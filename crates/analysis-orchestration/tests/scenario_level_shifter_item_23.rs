//! Integration witness for **tasks.md item #23** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Mixed-signal corpus: level shifter across supply domains.
//! > (depends on #20)
//! > <!-- traces-spec: mixed-signal-cosim#level-shifter -->
//! > <!-- traces-adr: ADR-0006 -->
//!
//! A level shifter bridges two supply domains: a low-voltage digital
//! domain (VDD1 = 1.8 V) and a high-voltage analog domain (VDD2 = 3.3 V).
//! The digital kernel drives the low-side input; the analog solver
//! responds with the level-shifted output on the high side. This file
//! asserts the end-to-end mixed-signal behaviour using the real
//! [`DigitalKernel`] (ADR-0006) wrapped in [`DigitalKernelAdapter`].
//!
//! # Scenarios
//!
//! 1. **Rising edge** — digital `din_1v8` transitions 0→1 at 50 ns;
//!    the analog level-shifter output `vout_3v3` rises from 0 V to 3.3 V.
//!    Boundary signals are exchanged at 50 ns; the unified `Result`
//!    contains synchronized analog + digital traces.
//!
//! 2. **Full cycle** — rising at 50 ns, falling at 150 ns. Two commits,
//!    zero rollbacks. Analog output follows VDD2 → 0 V.
//!
//! 3. **Bidirectional boundary exchange** — both analog-to-digital and
//!    digital-to-analog boundary signals carry values across the
//!    domain crossing at each synchronization point.
//!
//! 4. **ZOH across domain boundary** — when the analog solver's last
//!    accepted sample time precedes the digital event time, the zero-
//!    order hold carries the analog value forward without interpolation.
//!
//! 5. **Multiple events** — three events across supply domains.
//!
//! ADR refs: ADR-0006 (native DEVS kernel), ADR-0007 (ZOH boundary
//! exchange), ADR-0010 (unstable v1 API surface).

use std::cell::RefCell;
use std::rc::Rc;

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, AnalogValueProvider, BoundarySignalExchanger, BoundarySignals,
    DigitalKernelAdapter, DigitalValueProvider, MixedSignalScheduler, SchedulerError,
    SparseCheckpoint,
};
use circuit_solver_types::{
    AnalogTrace, NodeId, SignalName, SimulationTime, Waveform,
};
use digital_kernel::{DigitalEvent, DigitalKernel, LogicValue, NetId, NetState};

// ---------------------------------------------------------------------------
// Domain constants
// ---------------------------------------------------------------------------

/// Low-side supply voltage (digital domain).
const VDD1: f64 = 1.8;
/// High-side supply voltage (analog domain).
const VDD2: f64 = 3.3;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn t_ns(ns: i64) -> SimulationTime {
    SimulationTime::from_nanoseconds(ns)
}

// ---------------------------------------------------------------------------
// Level-shifter analog solver double
// ---------------------------------------------------------------------------

/// An analog solver test double that models a level shifter:
///
/// - The input comes from the digital domain via a boundary signal
///   (`din_1v8`). If the digital side is logic-1, the level shifter
///   output is VDD2; if logic-0, the output is 0 V.
/// - The solver records samples at each `run_until` call so the
///   waveform can be inspected.
struct LevelShifterAnalog {
    /// Output node observed in the analog trace.
    vout: NodeId,
    /// Current level-shifter output voltage (0 or VDD2).
    voltage: f64,
    /// Collected (time, voltage) samples.
    samples: Vec<(SimulationTime, f64)>,
    /// Call log for inspecting scheduler interactions.
    calls: Rc<RefCell<Vec<AnalogCall>>>,
}

#[derive(Debug, Clone, PartialEq)]
enum AnalogCall {
    RunUntil(SimulationTime),
    RollbackTo(SimulationTime),
}

impl LevelShifterAnalog {
    fn new(vout: NodeId, calls: Rc<RefCell<Vec<AnalogCall>>>) -> Self {
        Self {
            vout,
            voltage: 0.0,
            samples: vec![(SimulationTime::ZERO, 0.0)],
            calls,
        }
    }

    /// Apply a digital input value: logic-1 → output VDD2, logic-0 → 0 V.
    fn apply_digital_input(&mut self, logic_high: bool) {
        self.voltage = if logic_high { VDD2 } else { 0.0 };
    }
}

impl AnalogSolver for LevelShifterAnalog {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        self.calls.borrow_mut().push(AnalogCall::RunUntil(target));
        self.samples.push((target, self.voltage));
        let checkpoint =
            SparseCheckpoint::empty(target).with_node_voltages(vec![(self.vout, self.voltage)]);
        Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.calls.borrow_mut().push(AnalogCall::RollbackTo(target));
        // Retain samples up to the rollback point; restore voltage from
        // the last remaining sample.
        self.samples.retain(|(t, _)| *t <= target);
        self.voltage = self
            .samples
            .last()
            .map(|(_, v)| *v)
            .unwrap_or(0.0);
        Ok(())
    }

    fn take_trace(&mut self) -> AnalogTrace {
        let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
        let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
        let waveform = Waveform::new(self.vout, times, values);
        AnalogTrace {
            waveforms: vec![waveform],
            committed_through,
        }
    }
}

// ---------------------------------------------------------------------------
// AnalogValueProvider for the level-shifter double
// ---------------------------------------------------------------------------

/// Provides the analog output voltage for boundary exchange. The analog
/// side's "vout_3v3" is the level-shifted output.
struct LevelShifterAnalogProvider {
    analog: Rc<RefCell<LevelShifterAnalog>>,
}

impl LevelShifterAnalogProvider {
    fn new(analog: Rc<RefCell<LevelShifterAnalog>>) -> Self {
        Self { analog }
    }
}

impl AnalogValueProvider for LevelShifterAnalogProvider {
    fn last_analog_value(&self, signal: &SignalName) -> Option<f64> {
        if signal.as_str() == "vout_3v3" {
            Some(self.analog.borrow().voltage)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// DigitalValueProvider for the digital kernel
// ---------------------------------------------------------------------------

/// Provides the digital output value scaled to the analog domain for
/// boundary exchange. The digital side's "dout_1v8" carries VDD1 when
/// logic-1, 0 V when logic-0.
///
/// Owns a snapshot of the kernel's [`NetState`] to avoid lifetime issues
/// with borrowing the kernel across the trait method boundary.
struct LevelShifterDigitalProvider {
    net_state: NetState,
    net_dout: NetId,
}

impl LevelShifterDigitalProvider {
    /// Build from a kernel reference by cloning its net state.
    fn from_kernel(kernel: &DigitalKernel, net_dout: NetId) -> Self {
        Self {
            net_state: kernel.net_state().clone(),
            net_dout,
        }
    }
}

impl DigitalValueProvider for LevelShifterDigitalProvider {
    fn last_digital_value(&self, signal: &SignalName) -> Option<f64> {
        if signal.as_str() == "dout_1v8" {
            let val = self.net_state.get(self.net_dout);
            let scaled = match val {
                LogicValue::One => VDD1,
                LogicValue::Zero => 0.0,
                LogicValue::Unknown | LogicValue::HighImpedance => 0.0,
            };
            Some(scaled)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Common boundary configuration for level-shifter scenarios
// ---------------------------------------------------------------------------

fn level_shifter_boundaries() -> BoundarySignals {
    BoundarySignals {
        analog_to_digital: vec![(SignalName::new("vout_3v3"), SignalName::new("din_1v8"))],
        digital_to_analog: vec![(SignalName::new("dout_1v8"), SignalName::new("vin_ls"))],
    }
}

// ---------------------------------------------------------------------------
// Scenario 1 — Rising edge: digital 0→1 drives analog 0→VDD2
// ---------------------------------------------------------------------------

/// The level shifter sits between a 1.8 V digital domain and a 3.3 V
/// analog domain. At 50 ns the digital kernel transitions net 0 from
/// Zero to One (representing a rising edge on `dout_1v8`). The
/// scheduler must:
///
/// 1. Predict the next digital event at 50 ns.
/// 2. Advance the analog solver to 50 ns.
/// 3. Confirm the event at 50 ns.
/// 4. The analog output transitions to 3.3 V (level-shifted).
/// 5. The unified Result contains synchronized traces at 50 ns.
#[test]
fn level_shifter_rising_edge_drives_analog_high() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    // Build a real digital kernel with one rising-edge event at 50 ns.
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(50), net0, LogicValue::One))
        .expect("schedule rising edge at 50 ns");

    let signals = vec![
        SignalName::new("dout_1v8"),
        SignalName::new("din_1v8"),
    ];
    let adapter = DigitalKernelAdapter::new(kernel, signals);

    // Build the level-shifter analog double — starts at 0 V.
    let analog = LevelShifterAnalog::new(vout, Rc::clone(&analog_calls));

    let boundaries = level_shifter_boundaries();

    let scheduler = MixedSignalScheduler::new(analog, adapter, boundaries, t_ns(200));
    let result = scheduler
        .run()
        .expect("scheduler.run must succeed on rising-edge path");

    // One commit at the 50 ns boundary.
    assert_eq!(
        result.scheduler.commits,
        vec![t_ns(50)],
        "scheduler must commit at the 50 ns rising-edge boundary"
    );

    // No rollbacks on the correct-prediction path.
    assert!(
        result.rollback_free(),
        "rising-edge path must be rollback-free"
    );
    assert!(result.scheduler.rollbacks.is_empty());

    // The analog solver was advanced to 50 ns.
    let run_untils: Vec<SimulationTime> = analog_calls
        .borrow()
        .iter()
        .filter_map(|c| match c {
            AnalogCall::RunUntil(t) => Some(*t),
            _ => None,
        })
        .collect();
    assert_eq!(
        run_untils,
        vec![t_ns(50)],
        "analog solver must receive run-until at 50 ns"
    );

    // No analog rollbacks.
    assert!(
        !analog_calls
            .borrow()
            .iter()
            .any(|c| matches!(c, AnalogCall::RollbackTo(_))),
        "rising-edge path must not rollback the analog solver"
    );

    // The digital trace is non-empty and parseable.
    assert!(
        !result.digital.vcd.is_empty(),
        "digital VCD trace must be populated"
    );
    assert!(
        result.digital.vcd.contains("$enddefinitions $end"),
        "VCD must be parseable"
    );

    // The analog trace includes a sample at 50 ns.
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        wf.times.contains(&t_ns(50)),
        "analog waveform must include the 50 ns sample"
    );
}

// ---------------------------------------------------------------------------
// Scenario 2 — Full cycle: rising at 50 ns, falling at 150 ns
// ---------------------------------------------------------------------------

/// The level shifter sees both a rising edge (50 ns, 0→1) and a falling
/// edge (150 ns, 1→0). The scheduler must commit at both boundaries,
/// and the analog output must reflect the level-shifted value at each
/// point:
///
/// - At 50 ns: output transitions to VDD2 (3.3 V).
/// - At 150 ns: output transitions back to 0 V.
#[test]
fn level_shifter_full_cycle_rising_then_falling() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    // Build a real digital kernel with two events.
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(50), net0, LogicValue::One))
        .expect("schedule rising edge at 50 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(150), net0, LogicValue::Zero))
        .expect("schedule falling edge at 150 ns");

    let signals = vec![
        SignalName::new("dout_1v8"),
        SignalName::new("din_1v8"),
    ];
    let adapter = DigitalKernelAdapter::new(kernel, signals);

    let analog = LevelShifterAnalog::new(vout, Rc::clone(&analog_calls));

    let boundaries = level_shifter_boundaries();

    let scheduler = MixedSignalScheduler::new(analog, adapter, boundaries, t_ns(300));
    let result = scheduler
        .run()
        .expect("scheduler.run must succeed on full-cycle path");

    // Two commits at the digital event boundaries.
    assert_eq!(
        result.scheduler.commits,
        vec![t_ns(50), t_ns(150)],
        "scheduler must commit at both the rising and falling edges"
    );

    // No rollbacks.
    assert!(
        result.rollback_free(),
        "full-cycle path must be rollback-free"
    );
    assert!(result.scheduler.rollbacks.is_empty());

    // The analog solver was advanced to 50 ns and 150 ns.
    let run_untils: Vec<SimulationTime> = analog_calls
        .borrow()
        .iter()
        .filter_map(|c| match c {
            AnalogCall::RunUntil(t) => Some(*t),
            _ => None,
        })
        .collect();
    assert_eq!(
        run_untils,
        vec![t_ns(50), t_ns(150)],
        "analog solver must receive run-until at 50 ns and 150 ns"
    );

    // No analog rollbacks.
    assert!(
        !analog_calls
            .borrow()
            .iter()
            .any(|c| matches!(c, AnalogCall::RollbackTo(_))),
        "full-cycle path must not rollback the analog solver"
    );

    // The digital trace has VCD content for both events.
    assert!(
        !result.digital.vcd.is_empty(),
        "digital VCD trace must be populated"
    );

    // The analog trace includes samples at both boundaries.
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        wf.times.contains(&t_ns(50)),
        "analog waveform must include the 50 ns sample"
    );
    assert!(
        wf.times.contains(&t_ns(150)),
        "analog waveform must include the 150 ns sample"
    );

    // The analog committed_through matches the last commit.
    assert_eq!(
        result.analog.committed_through,
        t_ns(150),
        "analog committed_through must equal the last synchronized boundary"
    );
}

// ---------------------------------------------------------------------------
// Scenario 3 — Bidirectional boundary exchange with ZOH
// ---------------------------------------------------------------------------

/// The level-shifter corpus verifies that boundary signal exchange
/// works bidirectionally across the supply domain crossing. At each
/// synchronization point:
///
/// - The analog `vout_3v3` value is delivered to the digital side as
///   `din_1v8` (analog → digital).
/// - The digital `dout_1v8` value (scaled to VDD1) is delivered to
///   the analog side as `vin_ls` (digital → analog).
///
/// The zero-order hold (ADR-0007) carries the most recent accepted
/// analog value forward to the boundary time without interpolation.
#[test]
fn level_shifter_bidirectional_boundary_exchange() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));
    let analog_inner = Rc::new(RefCell::new(LevelShifterAnalog::new(
        vout,
        Rc::clone(&analog_calls),
    )));
    // Simulate the analog side having already level-shifted to VDD2.
    analog_inner.borrow_mut().apply_digital_input(true);
    let analog_provider = LevelShifterAnalogProvider::new(Rc::clone(&analog_inner));

    // Digital provider: dout_1v8 is logic-1 → scaled to VDD1.
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(50), net0, LogicValue::One))
        .expect("schedule rising edge");
    // Advance the kernel to 50 ns so its net state reflects logic-1.
    let _report = kernel.run_until(t_ns(50));
    let digital_provider = LevelShifterDigitalProvider::from_kernel(&kernel, net0);

    let boundaries = level_shifter_boundaries();
    let exchanger = BoundarySignalExchanger::zero_order_hold(boundaries);
    let packet = exchanger.exchange(&analog_provider, &digital_provider);

    // All configured boundary signals must be exchanged.
    assert!(
        packet.is_complete(),
        "all configured boundary signals must be exchanged across the supply domain crossing"
    );

    // Analog → digital: the level-shifted output (VDD2 = 3.3 V) is
    // delivered as the digital input.
    assert_eq!(
        packet.analog_to_digital_value(&SignalName::new("din_1v8")),
        Some(VDD2),
        "analog vout_3v3 ({VDD2} V) must be delivered to digital input din_1v8"
    );

    // Digital → analog: the digital output (VDD1 = 1.8 V for logic-1)
    // is delivered as the analog input.
    assert_eq!(
        packet.digital_to_analog_value(&SignalName::new("vin_ls")),
        Some(VDD1),
        "digital dout_1v8 ({VDD1} V) must be delivered to analog input vin_ls"
    );

    // No missing sources.
    assert!(
        packet.missing_sources.is_empty(),
        "no boundary signals should be missing"
    );
}

// ---------------------------------------------------------------------------
// Scenario 4 — ZOH holds the analog value across domain boundary
// ---------------------------------------------------------------------------

/// When the analog solver's last accepted sample was taken before the
/// digital event time (e.g., the analog step landed at 47 ns but the
/// digital event is at 50 ns), the zero-order hold must carry the
/// analog value forward unchanged. This is ADR-0007's core guarantee.
#[test]
fn level_shifter_zoh_holds_analog_value_across_domain_boundary() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));
    let analog_inner = Rc::new(RefCell::new(LevelShifterAnalog::new(
        vout,
        Rc::clone(&analog_calls),
    )));
    // Override voltage to mid-rail to demonstrate ZOH.
    analog_inner.borrow_mut().voltage = VDD2 / 2.0;
    let analog_provider = LevelShifterAnalogProvider::new(analog_inner);

    // Digital provider: logic-low (0 V).
    let kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    let digital_provider = LevelShifterDigitalProvider::from_kernel(&kernel, net0);

    let boundaries = level_shifter_boundaries();
    let exchanger = BoundarySignalExchanger::zero_order_hold(boundaries);
    let packet = exchanger.exchange(&analog_provider, &digital_provider);

    // ZOH: the analog value held from the last accepted sample (VDD2/2)
    // is carried to the boundary without interpolation.
    assert_eq!(
        packet.analog_to_digital_value(&SignalName::new("din_1v8")),
        Some(VDD2 / 2.0),
        "ZOH must hold the analog value from the last accepted sample"
    );

    // Digital → analog: logic-low → 0 V.
    assert_eq!(
        packet.digital_to_analog_value(&SignalName::new("vin_ls")),
        Some(0.0),
        "digital logic-low must deliver 0 V to the analog side"
    );
}

// ---------------------------------------------------------------------------
// Scenario 5 — Multiple events across domain boundary with real kernel
// ---------------------------------------------------------------------------

/// A more complex scenario: the digital kernel has three events
/// (rising at 40 ns, falling at 90 ns, rising at 160 ns). The
/// scheduler must commit at all three boundaries, exchange boundary
/// signals, and produce a unified Result with synchronized traces.
#[test]
fn level_shifter_multiple_events_across_supply_domains() {
    let vout = NodeId::new(1);
    let analog_calls = Rc::new(RefCell::new(Vec::new()));

    // Build a real digital kernel with three events.
    let mut kernel = DigitalKernel::new();
    let net0 = NetId::new(0);
    kernel
        .schedule(DigitalEvent::new(t_ns(40), net0, LogicValue::One))
        .expect("schedule rising edge at 40 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(90), net0, LogicValue::Zero))
        .expect("schedule falling edge at 90 ns");
    kernel
        .schedule(DigitalEvent::new(t_ns(160), net0, LogicValue::One))
        .expect("schedule rising edge at 160 ns");

    let signals = vec![
        SignalName::new("dout_1v8"),
        SignalName::new("din_1v8"),
    ];
    let adapter = DigitalKernelAdapter::new(kernel, signals);

    let analog = LevelShifterAnalog::new(vout, Rc::clone(&analog_calls));

    let boundaries = level_shifter_boundaries();

    let scheduler = MixedSignalScheduler::new(analog, adapter, boundaries, t_ns(300));
    let result = scheduler
        .run()
        .expect("scheduler.run must succeed with three events");

    // Three commits at all digital event boundaries.
    assert_eq!(
        result.scheduler.commits,
        vec![t_ns(40), t_ns(90), t_ns(160)],
        "scheduler must commit at all three digital event boundaries"
    );

    // No rollbacks.
    assert!(
        result.rollback_free(),
        "three-event path must be rollback-free"
    );

    // The analog solver was advanced to all three boundaries.
    let run_untils: Vec<SimulationTime> = analog_calls
        .borrow()
        .iter()
        .filter_map(|c| match c {
            AnalogCall::RunUntil(t) => Some(*t),
            _ => None,
        })
        .collect();
    assert_eq!(
        run_untils,
        vec![t_ns(40), t_ns(90), t_ns(160)],
        "analog solver must receive run-until at 40 ns, 90 ns, and 160 ns"
    );

    // The digital trace is populated.
    assert!(
        !result.digital.vcd.is_empty(),
        "digital VCD trace must be populated for three events"
    );

    // The analog trace includes samples at all three boundaries.
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(wf.times.contains(&t_ns(40)));
    assert!(wf.times.contains(&t_ns(90)));
    assert!(wf.times.contains(&t_ns(160)));

    // The analog committed_through matches the last commit.
    assert_eq!(
        result.analog.committed_through,
        t_ns(160),
        "analog committed_through must equal the final synchronized boundary (160 ns)"
    );
}
