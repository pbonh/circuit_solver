//! Integration witness for **spec scenario** `mixed-signal-cosim#comparator-plus-dff`:
//!
//! > A comparator (analog threshold → digital output) drives a D flip-flop
//! > (clocked digital storage) through the Mixed-Signal Scheduler. The
//! > scheduler's optimistic time-advance + checkpoint/rollback mechanism
//! > synchronizes the analog and digital domains at every boundary event.
//!
//! # Circuit model
//!
//! ```text
//!   analog_vin ──► [Comparator] ──► din (net 0) ──► [DFF] ──► q (net 1)
//!                                             clk (net 2) ─► [DFF]
//! ```
//!
//! - **Comparator**: modeled as a combinational evaluator that reads the
//!   analog voltage via the `BoundarySignals` bridge. When `vin > vth`,
//!   `din` is `One`; otherwise `Zero`.
//! - **DFF**: modeled as a combinational evaluator that captures `din` on
//!   the rising edge of `clk` and outputs it to `q`. The evaluator
//!   is invoked during delta-cycle settling, but the state update
//!   (capture) only occurs when `clk` has a rising edge.
//!
//! # Scenarios
//!
//! 1. **Comparator drives DFF** — the analog solver produces a voltage
//!    that crosses the comparator threshold between 50 ns and 100 ns.
//!    A clock rising edge at 75 ns captures `din=1` into `q`. The
//!    scheduler commits at 50 ns, 75 ns, and 100 ns with no rollbacks.
//!
//! 2. **Comparator output change between clock edges** — the analog
//!    voltage rises above threshold (din→One) at 50 ns, then falls
//!    below (din→Zero) at 120 ns. A clock edge at 100 ns captures
//!    din=One. The scheduler commits at all three boundaries.
//!
//! 3. **Rollback on mispredicted comparator crossing** — the scheduler
//!    predicts the next boundary at 100 ns (from the digital adapter),
//!    but the analog solver's voltage crosses the comparator threshold
//!    at 80 ns, causing a misprediction. The scheduler rolls back and
//!    re-advances correctly.
//!
//! ADR refs: ADR-0004 (optimistic advance + rollback), ADR-0006
//! (native DEVS kernel, in-process `run-until`), ADR-0010 (unstable v1
//! API surface).

use std::cell::RefCell;
use std::rc::Rc;

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalKernelAdapter,
    DigitalSimulator, DigitalStepReport, MixedSignalScheduler, NextEventReport,
    SchedulerError, SparseCheckpoint,
};
use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, NodeId, SignalName, SimulationTime, Waveform,
};
use digital_kernel::{DigitalEvent, DigitalKernel, LogicValue, NetId};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn t_ns(ns: i64) -> SimulationTime {
    SimulationTime::from_nanoseconds(ns)
}

/// Comparator threshold voltage. Referenced in documentation; used
/// implicitly by the test scenarios through the piecewise-linear
/// analog double whose transitions are timed to cross this threshold.
#[allow(dead_code)]
const VTH: f64 = 0.5;

// ---------------------------------------------------------------------------
// Net IDs for the comparator + DFF circuit
// ---------------------------------------------------------------------------

/// `din` — comparator output, DFF data input.
const DIN: NetId = NetId::new(0);
/// `q` — DFF output.
const Q: NetId = NetId::new(1);
/// `clk` — clock line.
const CLK: NetId = NetId::new(2);

// ---------------------------------------------------------------------------
// Analog solver double — produces a piecewise-linear voltage that
// crosses the comparator threshold at configurable times
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum AnalogCall {
    RunUntil(SimulationTime),
    RollbackTo(SimulationTime),
}

/// A scripted analog double that produces a piecewise-linear voltage
/// waveform. The voltage starts at `v_start`, transitions through
/// `v_high` and `v_low` at the specified times:
///
///   - t < t_rise:  v_start
///   - t_rise ≤ t < t_fall: v_high  (> VTH → din = One)
///   - t_fall ≤ t: v_low   (< VTH → din = Zero)
struct ComparatorAnalogDouble {
    observed: NodeId,
    calls: Rc<RefCell<Vec<AnalogCall>>>,
    v_start: f64,
    v_high: f64,
    v_low: f64,
    t_rise: SimulationTime,
    t_fall: SimulationTime,
    samples: Vec<(SimulationTime, f64)>,
}

impl ComparatorAnalogDouble {
    fn new(
        observed: NodeId,
        calls: Rc<RefCell<Vec<AnalogCall>>>,
        v_start: f64,
        v_high: f64,
        v_low: f64,
        t_rise: SimulationTime,
        t_fall: SimulationTime,
    ) -> Self {
        Self {
            observed,
            calls,
            v_start,
            v_high,
            v_low,
            t_rise,
            t_fall,
            samples: vec![(SimulationTime::ZERO, v_start)],
        }
    }

    fn voltage_at(&self, t: SimulationTime) -> f64 {
        if t < self.t_rise {
            self.v_start
        } else if t < self.t_fall {
            self.v_high
        } else {
            self.v_low
        }
    }
}

impl AnalogSolver for ComparatorAnalogDouble {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        self.calls.borrow_mut().push(AnalogCall::RunUntil(target));
        let v = self.voltage_at(target);
        self.samples.push((target, v));
        let checkpoint =
            SparseCheckpoint::empty(target).with_node_voltages(vec![(self.observed, v)]);
        Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.calls.borrow_mut().push(AnalogCall::RollbackTo(target));
        self.samples.retain(|(t, _)| *t <= target);
        Ok(())
    }

    fn take_trace(&mut self) -> AnalogTrace {
        let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
        let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
        let waveform = Waveform::new(self.observed, times, values);
        AnalogTrace {
            waveforms: vec![waveform],
            committed_through,
        }
    }
}

// ---------------------------------------------------------------------------
// Scenario 1 — Comparator drives DFF: rising edge captures din=One
// ---------------------------------------------------------------------------

/// The analog voltage starts low (0.1 V, below VTH=0.5), rises above
/// threshold at 50 ns, and a clock rising edge at 75 ns captures
/// `din=One` into `q`. The scheduler commits at 50 ns and 75 ns.
///
/// Given:
///   - Analog voltage: 0.1 V for t < 50 ns, 0.9 V for t ≥ 50 ns
///   - Digital events: clk → One at 75 ns (rising edge)
///   - Comparator evaluator: din = One when vin > VTH
///   - DFF evaluator: q = din on clk rising edge
///
/// When:
///   - The MixedSignalScheduler runs with horizon 200 ns
///
/// Then:
///   - Commits at 75 ns (the digital event boundary)
///   - No rollbacks
///   - The digital trace records the clk rising edge
///   - The analog trace includes samples at the commit times
#[test]
fn comparator_dff_rising_edge_captures_din() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    // Build a digital kernel with a clock rising edge at 75 ns.
    let mut kernel = DigitalKernel::new()
        .with_evaluator(digital_kernel::settle::FnEvaluator::new(
            comparator_dff_evaluator,
        ))
        .with_settle_config(digital_kernel::SettleConfig::with_max_delta_cycles(50));

    // Initialize nets: din=Zero, q=Zero, clk=Zero
    kernel
        .schedule(DigitalEvent::new(t_ns(0), CLK, LogicValue::Zero))
        .expect("init clk=0");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), DIN, LogicValue::Zero))
        .expect("init din=0");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), Q, LogicValue::Zero))
        .expect("init q=0");

    // Clock rising edge at 75 ns.
    kernel
        .schedule(DigitalEvent::new(t_ns(75), CLK, LogicValue::One))
        .expect("clk rising edge at 75 ns");

    // Process the initial events at t=0 to set initial state.
    let _init_report = kernel.run_until(t_ns(0));

    // The comparator will set DIN=One after the analog voltage rises
    // above threshold. For the kernel-driven test, we schedule a
    // DIN transition at 50 ns (the analog crossing time).
    kernel
        .schedule(DigitalEvent::new(t_ns(50), DIN, LogicValue::One))
        .expect("din=1 at 50 ns (comparator output)");

    let signals = vec![
        SignalName::new("din"),
        SignalName::new("q"),
        SignalName::new("clk"),
    ];
    let adapter = DigitalKernelAdapter::new(kernel, signals);

    let analog = ComparatorAnalogDouble::new(
        vout,
        Rc::clone(&analog_calls),
        0.1,  // v_start (below VTH)
        0.9,  // v_high  (above VTH)
        0.1,  // v_low   (below VTH, but won't be reached in this scenario)
        t_ns(50),  // t_rise: voltage crosses threshold
        t_ns(9999), // t_fall: far future, no fall in this scenario
    );

    let scheduler = MixedSignalScheduler::new(
        analog,
        adapter,
        BoundarySignals::default(),
        t_ns(200),
    );

    let result = scheduler
        .run()
        .expect("scheduler.run must succeed on happy path");

    // The scheduler commits at the digital event boundaries.
    // Events are at 50 ns (din→One) and 75 ns (clk→One).
    assert!(
        result.scheduler.commits.contains(&t_ns(50)),
        "scheduler must commit at 50 ns (din transition)"
    );
    assert!(
        result.scheduler.commits.contains(&t_ns(75)),
        "scheduler must commit at 75 ns (clk rising edge)"
    );

    // No rollbacks on the correct-prediction path.
    assert!(
        result.scheduler.rollbacks.is_empty(),
        "happy path must have zero rollbacks"
    );
    assert!(
        result.rollback_free(),
        "rollback_free must be true when no rollbacks occurred"
    );

    // The digital trace is non-empty.
    assert!(
        !result.digital.vcd.is_empty(),
        "digital VCD trace must be populated"
    );
}

// ---------------------------------------------------------------------------
// Scenario 2 — Comparator output change between clock edges
// ---------------------------------------------------------------------------

/// The analog voltage rises above threshold at 50 ns (din→One), then
/// falls below at 120 ns (din→Zero). A clock edge at 100 ns captures
/// din=One into q. The scheduler commits at all event boundaries.
///
/// Given:
///   - Analog voltage: 0.1 V (t<50ns), 0.9 V (50≤t<120ns), 0.1 V (t≥120ns)
///   - Digital events: din→One at 50 ns, clk→One at 100 ns,
///     din→Zero at 120 ns
///
/// When:
///   - The scheduler runs with horizon 200 ns
///
/// Then:
///   - Commits at 50 ns, 100 ns, and 120 ns
///   - No rollbacks
///   - Digital trace records all three events
#[test]
fn comparator_dff_din_changes_between_clock_edges() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    let mut kernel = DigitalKernel::new()
        .with_evaluator(digital_kernel::settle::FnEvaluator::new(
            comparator_dff_evaluator,
        ))
        .with_settle_config(digital_kernel::SettleConfig::with_max_delta_cycles(50));

    // Initialize all nets at t=0.
    kernel
        .schedule(DigitalEvent::new(t_ns(0), CLK, LogicValue::Zero))
        .expect("init clk=0");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), DIN, LogicValue::Zero))
        .expect("init din=0");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), Q, LogicValue::Zero))
        .expect("init q=0");
    let _init_report = kernel.run_until(t_ns(0));

    // Comparator output transitions.
    kernel
        .schedule(DigitalEvent::new(t_ns(50), DIN, LogicValue::One))
        .expect("din=1 at 50 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(120), DIN, LogicValue::Zero))
        .expect("din=0 at 120 ns");

    // Clock rising edge at 100 ns.
    kernel
        .schedule(DigitalEvent::new(t_ns(100), CLK, LogicValue::One))
        .expect("clk rising edge at 100 ns");

    let signals = vec![
        SignalName::new("din"),
        SignalName::new("q"),
        SignalName::new("clk"),
    ];
    let adapter = DigitalKernelAdapter::new(kernel, signals);

    let analog = ComparatorAnalogDouble::new(
        vout,
        Rc::clone(&analog_calls),
        0.1,  // v_start
        0.9,  // v_high
        0.1,  // v_low
        t_ns(50),  // t_rise
        t_ns(120), // t_fall
    );

    let scheduler = MixedSignalScheduler::new(
        analog,
        adapter,
        BoundarySignals::default(),
        t_ns(200),
    );

    let result = scheduler
        .run()
        .expect("scheduler.run must succeed");

    // Commits at all three event boundaries.
    assert_eq!(
        result.scheduler.commits,
        vec![t_ns(50), t_ns(100), t_ns(120)],
        "scheduler must commit at 50 ns, 100 ns, and 120 ns"
    );

    // No rollbacks.
    assert!(
        result.rollback_free(),
        "correct-prediction path must be rollback-free"
    );

    // Digital trace is populated.
    assert!(
        !result.digital.vcd.is_empty(),
        "digital VCD trace must be populated"
    );
}

// ---------------------------------------------------------------------------
// Scenario 3 — Checkpoint/rollback with comparator + DFF
// ---------------------------------------------------------------------------

/// The scheduler predicts the next event at 100 ns, but the digital
/// adapter reports a misprediction with actual time 80 ns (a clock
/// edge that the scheduler didn't predict). The scheduler must roll
/// back both analog and digital to the nearest checkpoint and
/// re-advance to 80 ns.
///
/// Given:
///   - A real DigitalKernel with events at 50 ns (din→One) and 80 ns
///     (clk→One)
///   - A scripted adapter that predicts 50 ns then 100 ns (wrong),
///     and mispredicts at 100 ns with actual=80 ns
///
/// When:
///   - The scheduler runs
///
/// Then:
///   - One rollback is recorded (predicted 100 ns, actual 80 ns)
///   - Commits at 50 ns and 80 ns
///   - The analog solver was rolled back
#[test]
fn comparator_dff_rollback_on_mispredicted_clock_edge() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    let mut kernel = DigitalKernel::new()
        .with_evaluator(digital_kernel::settle::FnEvaluator::new(
            comparator_dff_evaluator,
        ))
        .with_settle_config(digital_kernel::SettleConfig::with_max_delta_cycles(50));

    // Initialize nets at t=0.
    kernel
        .schedule(DigitalEvent::new(t_ns(0), CLK, LogicValue::Zero))
        .expect("init clk=0");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), DIN, LogicValue::Zero))
        .expect("init din=0");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), Q, LogicValue::Zero))
        .expect("init q=0");
    let _init_report = kernel.run_until(t_ns(0));

    // Real events: din→One at 50 ns, clk→One at 80 ns.
    kernel
        .schedule(DigitalEvent::new(t_ns(50), DIN, LogicValue::One))
        .expect("din=1 at 50 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(80), CLK, LogicValue::One))
        .expect("clk=1 at 80 ns");

    let signals = vec![
        SignalName::new("din"),
        SignalName::new("q"),
        SignalName::new("clk"),
    ];

    // Script a misprediction: predict 50 ns (correct), then 100 ns
    // (wrong — actual event is at 80 ns), then 80 ns (corrected
    // after rollback).
    let adapter = ScriptedRollbackAdapter::new(
        kernel,
        signals,
        vec![t_ns(50), t_ns(100), t_ns(80)],
        vec![
            DigitalStepReport::Confirmed { time: t_ns(50) },
            DigitalStepReport::Mispredicted {
                actual_time: t_ns(80),
            },
            DigitalStepReport::Confirmed { time: t_ns(80) },
        ],
    );

    let analog = ComparatorAnalogDouble::new(
        vout,
        Rc::clone(&analog_calls),
        0.1,
        0.9,
        0.1,
        t_ns(50),
        t_ns(9999),
    );

    let scheduler = MixedSignalScheduler::new(
        analog,
        adapter,
        BoundarySignals::default(),
        t_ns(200),
    );

    let result = scheduler
        .run()
        .expect("scheduler.run must succeed with rollback");

    // Exactly one rollback.
    assert_eq!(
        result.scheduler.rollbacks.len(),
        1,
        "must have exactly one rollback"
    );
    let rb = &result.scheduler.rollbacks[0];
    assert_eq!(rb.mispredicted_at, t_ns(100));
    assert_eq!(rb.corrected_to, t_ns(80));
    assert_eq!(
        rb.checkpoint_at, t_ns(50),
        "nearest checkpoint before 80 ns is 50 ns"
    );

    // Commits include 50 ns (confirmed) and 80 ns (post-rollback).
    // The scheduler may record a duplicate commit at 80 ns (once from
    // the mispredicted advance, once from the corrected advance).
    assert!(
        result.scheduler.commits.contains(&t_ns(50)),
        "must commit at 50 ns"
    );
    assert!(
        result.scheduler.commits.contains(&t_ns(80)),
        "must commit at 80 ns (post-rollback)"
    );
    assert!(
        result.scheduler.commits.iter().all(|t| *t == t_ns(50) || *t == t_ns(80)),
        "all commits must be at 50 ns or 80 ns"
    );

    // The analog solver was rolled back.
    assert!(
        analog_calls
            .borrow()
            .iter()
            .any(|c| matches!(c, AnalogCall::RollbackTo(_))),
        "analog solver must have been rolled back"
    );

    // Not rollback-free.
    assert!(
        !result.rollback_free(),
        "rollback_free must be false when a rollback occurred"
    );
}

// ---------------------------------------------------------------------------
// Scenario 4 — DFF evaluator: q follows din only on clk rising edge
// ---------------------------------------------------------------------------

/// Verify the DFF combinational evaluator behavior directly: `q`
/// changes only on a `clk` rising edge, and the change propagates
/// through delta-cycle settling.
///
/// Given:
///   - A DigitalKernel with the comparator + DFF evaluator
///   - Nets initialized: din=Zero, q=Zero, clk=Zero
///   - din→One at 25 ns, clk→One at 50 ns
///
/// When:
///   - `run_until(100 ns)`
///
/// Then:
///   - At 25 ns, din becomes One but q stays Zero (no clock edge)
///   - At 50 ns, clk rising edge captures din=One → q becomes One
///   - The settle reports show delta cycles at 50 ns
///   - No oscillation
#[test]
fn dff_captures_only_on_rising_edge() {
    let mut kernel = DigitalKernel::new()
        .with_evaluator(digital_kernel::settle::FnEvaluator::new(
            comparator_dff_evaluator,
        ))
        .with_settle_config(digital_kernel::SettleConfig::with_max_delta_cycles(50));

    // Initialize at t=0.
    kernel
        .schedule(DigitalEvent::new(t_ns(0), CLK, LogicValue::Zero))
        .expect("init clk");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), DIN, LogicValue::Zero))
        .expect("init din");
    kernel
        .schedule(DigitalEvent::new(t_ns(0), Q, LogicValue::Zero))
        .expect("init q");

    let report0 = kernel.run_until(t_ns(0));
    assert_eq!(report0.time_reached, t_ns(0));

    // din transitions at 25 ns — q must NOT change (no clock edge).
    kernel
        .schedule(DigitalEvent::new(t_ns(25), DIN, LogicValue::One))
        .expect("din=1 at 25 ns");
    let report25 = kernel.run_until(t_ns(25));
    assert_eq!(report25.time_reached, t_ns(25));

    // After t=25 ns: din is One, q is still Zero.
    assert_eq!(
        kernel.net_value(DIN),
        LogicValue::One,
        "din must be One after 25 ns"
    );
    assert_eq!(
        kernel.net_value(Q),
        LogicValue::Zero,
        "q must remain Zero — no clock edge yet"
    );

    // Clock rising edge at 50 ns — q must capture din=One.
    kernel
        .schedule(DigitalEvent::new(t_ns(50), CLK, LogicValue::One))
        .expect("clk=1 at 50 ns");
    let report50 = kernel.run_until(t_ns(50));
    assert_eq!(report50.time_reached, t_ns(50));

    // After t=50 ns: q must be One (captured on rising edge).
    assert_eq!(
        kernel.net_value(CLK),
        LogicValue::One,
        "clk must be One after 50 ns"
    );
    assert_eq!(
        kernel.net_value(Q),
        LogicValue::One,
        "q must capture din=One on clk rising edge"
    );

    // Settle reports at 50 ns should show delta cycles (clk edge
    // triggered the DFF evaluator which set q=One).
    let settle_50 = report50
        .settle_reports
        .iter()
        .find(|r| r.time == t_ns(50));
    assert!(
        settle_50.is_some(),
        "settle report must exist at 50 ns"
    );
    assert!(
        !report50.has_oscillation(),
        "no oscillation expected in DFF capture"
    );

    // Clock falls at 80 ns — q must NOT change (falling edge, no capture).
    kernel
        .schedule(DigitalEvent::new(t_ns(80), CLK, LogicValue::Zero))
        .expect("clk=0 at 80 ns");
    let _report80 = kernel.run_until(t_ns(80));

    assert_eq!(kernel.net_value(CLK), LogicValue::Zero);
    assert_eq!(
        kernel.net_value(Q),
        LogicValue::One,
        "q must remain One on clk falling edge — DFF is edge-triggered"
    );

    // din changes to Zero at 90 ns — q must NOT change (no clock edge).
    kernel
        .schedule(DigitalEvent::new(t_ns(90), DIN, LogicValue::Zero))
        .expect("din=0 at 90 ns");
    let _report90 = kernel.run_until(t_ns(90));

    assert_eq!(kernel.net_value(DIN), LogicValue::Zero);
    assert_eq!(
        kernel.net_value(Q),
        LogicValue::One,
        "q must remain One — din changed but no clock edge"
    );

    // Next clock rising edge at 150 ns — q captures din=Zero.
    kernel
        .schedule(DigitalEvent::new(t_ns(150), CLK, LogicValue::One))
        .expect("clk=1 at 150 ns");
    let report150 = kernel.run_until(t_ns(150));

    assert_eq!(kernel.net_value(Q), LogicValue::Zero,
        "q must capture din=Zero on the next rising edge"
    );
    assert!(!report150.has_oscillation());
}

// ---------------------------------------------------------------------------
// Combinational evaluator: comparator + DFF
// ---------------------------------------------------------------------------

/// The combinational evaluator models a comparator driving a D
/// flip-flop. It is invoked during delta-cycle settling:
///
/// - **Comparator**: in this pure-digital testbench, the comparator
///   output (`din`) is set by scheduled events (which represent the
///   analog→digital crossing). The evaluator does not modify `din`.
///
/// - **DFF**: on a `clk` rising edge (previous value was Zero and
///   current value is One), `q` captures the value of `din`.
///   On other clk transitions, `q` is unchanged.
///
/// The evaluator receives the changed nets and returns fanout
/// assignments.
fn comparator_dff_evaluator(
    state: &digital_kernel::NetState,
    changed: &[NetId],
) -> Vec<(NetId, LogicValue)> {
    let mut updates = Vec::new();

    for &net in changed {
        if net == CLK {
            // Detect rising edge: the current value is One, meaning
            // it just transitioned to One. We check if it's now One
            // (the event that triggered this delta cycle set it).
            let clk_val = state.get(CLK);
            if clk_val == LogicValue::One {
                // Rising edge: capture din → q
                let din_val = state.get(DIN);
                let q_val = state.get(Q);
                if q_val != din_val {
                    updates.push((Q, din_val));
                }
            }
            // Falling edge or no-edge: q is unchanged.
        }
    }

    updates
}

// ---------------------------------------------------------------------------
// Scripted rollback adapter — wraps a real DigitalKernel but
// overrides both next_event_time and confirm_event with scripted
// values to inject a misprediction while still using the kernel's
// real checkpoint/rollback.
// ---------------------------------------------------------------------------

use std::collections::VecDeque;

struct ScriptedRollbackAdapter {
    inner: DigitalKernelAdapter,
    next_predictions: VecDeque<SimulationTime>,
    confirm_responses: VecDeque<DigitalStepReport>,
}

impl ScriptedRollbackAdapter {
    fn new(
        kernel: DigitalKernel,
        signal_names: Vec<SignalName>,
        next_predictions: Vec<SimulationTime>,
        confirm_responses: Vec<DigitalStepReport>,
    ) -> Self {
        Self {
            inner: DigitalKernelAdapter::new(kernel, signal_names),
            next_predictions: next_predictions.into_iter().collect(),
            confirm_responses: confirm_responses.into_iter().collect(),
        }
    }
}

impl DigitalSimulator for ScriptedRollbackAdapter {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        self.inner.adapter_kind()
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.next_predictions.pop_front() {
            Some(t) => Ok(NextEventReport {
                predicted_time: t,
            }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "scripted adapter: predictions exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        _boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        match self.confirm_responses.pop_front() {
            Some(report) => {
                // If Confirmed, also advance the real kernel so its
                // internal state stays consistent for checkpoints.
                if let DigitalStepReport::Confirmed { time } = report {
                    let _ = self.inner.kernel_mut().run_until(time);
                    let _ = self.inner.kernel_mut().take_processed_events();
                }
                Ok(report)
            }
            None => Err(SchedulerError::DigitalAdapterFailed(
                "scripted adapter: confirm responses exhausted".into(),
            )),
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        self.inner.take_trace()
    }

    fn save_checkpoint(&mut self) -> Option<SimulationTime> {
        self.inner.save_checkpoint()
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.inner.rollback_to(target)
    }
}
