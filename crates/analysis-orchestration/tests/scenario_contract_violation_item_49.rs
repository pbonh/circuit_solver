//! Integration witness for **tasks.md item #49** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Implement digital contract violation detection: detect event
//! > earlier than predicted next-event-time, rollback + log
//! > diagnostic warning
//! > — @spec: mixed-signal-cosim#digital-simulator-violates-next-
//! >   event-time-contract (depends on #44)
//!
//! Item #49 binds the contract-violation detection to the
//! `digital-simulator-violates-next-event-time-contract` Gherkin
//! scenario:
//!
//! ```gherkin
//! Given SimulationEngineer has configured a mixed-signal
//!   simulation
//! And the digital simulator reports an event at a time earlier
//!   than its previously predicted next-event-time
//! When the Scheduler detects the contract violation
//! Then the Scheduler rolls back to the last committed checkpoint
//!   before the early event time
//! And the Scheduler logs a diagnostic warning about the next-
//!   event-time contract violation
//! And the simulation continues from the corrected point
//! ```
//!
//! This witness drives that exact sequence end-to-end through the
//! public `MixedSignalScheduler`. The digital double first commits
//! a known-good boundary at 50 ns (seeding a checkpoint), then
//! predicts 100 ns but on `confirm_event` reports the event at 80 ns
//! — a contract violation since 80 < 100. The scheduler must roll
//! back to the nearest checkpoint before 80 ns (50 ns), log a
//! diagnostic, and continue (producing a Result, not an error).
//!
//! ADR refs:
//! - **ADR-0004** — Mixed-Signal Scheduler ownership + sparse
//!   checkpoint memory model; commitment #4 is "Rollback on
//!   misprediction" which this scenario extends with the contract-
//!   violation diagnostic.
//! - **ADR-0007** — Zero-Order Hold default at boundary; orthogonal.
//! - **ADR-0008** — Per-node tolerance envelope; orthogonal.
//! - **ADR-0010** — Unstable public API surface for v1; the
//!   re-exports asserted by item #44 pin the surface this test uses.

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError, SparseCheckpoint,
};
use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, NodeId, RollbackEvent, SignalName, SimulationTime, Waveform,
};
use std::collections::VecDeque;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn t_ns(ns: i64) -> SimulationTime {
    SimulationTime::from_nanoseconds(ns)
}

#[allow(clippy::cast_precision_loss)]
fn voltage_at(t: SimulationTime) -> f64 {
    let ns = t.as_nanoseconds() as f64;
    (ns / 100.0).clamp(0.0, 1.0)
}

// ---------------------------------------------------------------------------
// Test doubles
// ---------------------------------------------------------------------------

/// Analog solver double. Records calls and saves real
/// `SparseCheckpoint` payloads so the rollback handler can find them.
struct AnalogDouble {
    observed: NodeId,
    samples: Vec<(SimulationTime, f64)>,
}

impl AnalogDouble {
    fn new(observed: NodeId) -> Self {
        Self {
            observed,
            samples: vec![(SimulationTime::ZERO, voltage_at(SimulationTime::ZERO))],
        }
    }
}

impl AnalogSolver for AnalogDouble {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        let v = voltage_at(target);
        self.samples.push((target, v));
        let checkpoint =
            SparseCheckpoint::empty(target).with_node_voltages(vec![(self.observed, v)]);
        Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
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

/// Digital double: commits 50 ns, then violates the contract at
/// 100 ns by reporting the event at 80 ns (earlier than predicted).
struct ContractViolatingDigital {
    next_predictions: VecDeque<SimulationTime>,
    confirms: VecDeque<DigitalStepReport>,
}

impl ContractViolatingDigital {
    fn new() -> Self {
        Self {
            next_predictions: VecDeque::from([t_ns(50), t_ns(100)]),
            confirms: VecDeque::from([
                DigitalStepReport::Confirmed { time: t_ns(50) },
                DigitalStepReport::Mispredicted {
                    actual_time: t_ns(80),
                },
            ]),
        }
    }
}

impl DigitalSimulator for ContractViolatingDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.next_predictions.pop_front() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "test double exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        _boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        self.confirms
            .pop_front()
            .ok_or_else(|| SchedulerError::DigitalAdapterFailed("confirm end".into()))
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        use std::fmt::Write as _;
        let mut vcd = String::new();
        vcd.push_str("$timescale 1ps $end\n");
        vcd.push_str("$scope module mixed_signal_test $end\n");
        vcd.push_str("$var wire 1 ! din $end\n");
        vcd.push_str("$upscope $end\n$enddefinitions $end\n");
        let _ = writeln!(vcd, "#{}\n1!", t_ns(50).as_picoseconds());
        let _ = writeln!(vcd, "#{}\n1!", t_ns(80).as_picoseconds());
        DigitalEventTrace {
            vcd,
            events_by_signal: vec![(SignalName::new("din"), vec![t_ns(50), t_ns(80)])],
        }
    }
}

/// Digital double for the edge-case test: predict 100 ns, report
/// event at 80 ns (contract violation), with NO prior checkpoint.
struct BareViolatingDigital {
    next_predictions: VecDeque<SimulationTime>,
    confirms: VecDeque<DigitalStepReport>,
}

impl DigitalSimulator for BareViolatingDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.next_predictions.pop_front() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed("exhausted".into())),
        }
    }

    fn confirm_event(
        &mut self,
        _boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        self.confirms
            .pop_front()
            .ok_or_else(|| SchedulerError::DigitalAdapterFailed("exhausted".into()))
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        DigitalEventTrace::default()
    }
}

// ---------------------------------------------------------------------------
// API-surface witness (ADR-0010)
// ---------------------------------------------------------------------------

#[test]
fn item_49_public_api_surface_is_re_exported() {
    type _Scheduler = MixedSignalScheduler<AnalogDouble, ContractViolatingDigital>;
    let vout = NodeId::new(1);
    let analog = AnalogDouble::new(vout);
    let digital = ContractViolatingDigital::new();
    let sched: _Scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    assert_eq!(sched.horizon(), t_ns(200));
}

// ---------------------------------------------------------------------------
// The Gherkin scenario itself.
// ---------------------------------------------------------------------------

/// **Scenario: digital-simulator-violates-next-event-time-contract**
///
/// Drives the Gherkin block in the task body end-to-end via the
/// public `MixedSignalScheduler`. Asserts:
///
/// 1. The Scheduler rolls back to the last committed checkpoint
///    before the early event time (50 ns ≤ 80 ns).
/// 2. The Scheduler logs a diagnostic warning about the next-event-
///    time contract violation.
/// 3. The simulation continues from the corrected point (produces a
///    `MixedSignalResult`, not an error).
#[test]
fn digital_simulator_violates_next_event_time_contract() {
    let vout = NodeId::new(1);
    let analog = AnalogDouble::new(vout);
    let digital = ContractViolatingDigital::new();

    let scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    let result = scheduler
        .run()
        .expect("scheduler must continue after contract violation — no error");

    // ── Then: the Scheduler rolls back to the last committed
    //    checkpoint before the early event time.
    assert_eq!(
        result.scheduler.rollbacks.len(),
        1,
        "exactly one rollback event from the contract violation"
    );
    let rb = &result.scheduler.rollbacks[0];
    assert_eq!(
        rb,
        &RollbackEvent {
            mispredicted_at: t_ns(100),
            corrected_to: t_ns(80),
            checkpoint_at: t_ns(50),
            reason: "contract-violation".into(),
        },
        "rollback event must carry contract-violation reason and correct checkpoint"
    );

    // ── Then: the Scheduler logs a diagnostic warning about the
    //    next-event-time contract violation.
    let diag_contains_contract = result
        .scheduler
        .diagnostics
        .iter()
        .any(|d| d.contains("contract violation"));
    assert!(
        diag_contains_contract,
        "diagnostics must contain a contract-violation warning; got {:?}",
        result.scheduler.diagnostics
    );

    // ── Then: the simulation continues from the corrected point.
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        wf.times.contains(&t_ns(80)),
        "analog waveform must include the corrected 80 ns sample"
    );
    assert!(
        !wf.times.contains(&t_ns(100)),
        "analog waveform must NOT contain the (rolled-back) 100 ns sample"
    );
    assert_eq!(
        result.analog.committed_through,
        t_ns(80),
        "analog trace committed_through must be at the corrected 80 ns"
    );

    assert_eq!(result.scheduler.commits, vec![t_ns(50), t_ns(80)]);
    assert_eq!(result.final_commit(), Some(t_ns(80)));

    // Digital trace records events at both boundaries.
    assert!(
        result
            .digital
            .vcd
            .contains(&format!("#{}", t_ns(50).as_picoseconds())),
        "digital VCD must record 50 ns event"
    );
    assert!(
        result
            .digital
            .vcd
            .contains(&format!("#{}", t_ns(80).as_picoseconds())),
        "digital VCD must record 80 ns event"
    );

    assert!(!result.rollback_free());
}

/// Edge case: contract violation when no prior checkpoint exists.
/// The scheduler must surface `NoCheckpoint(80ns)` rather than
/// silently continuing from an arbitrary state.
#[test]
fn contract_violation_without_prior_checkpoint_errors() {
    let vout = NodeId::new(1);
    let analog = AnalogDouble::new(vout);
    let digital = BareViolatingDigital {
        next_predictions: VecDeque::from([t_ns(100)]),
        confirms: VecDeque::from([DigitalStepReport::Mispredicted {
            actual_time: t_ns(80),
        }]),
    };

    let scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    let err = scheduler
        .run()
        .expect_err("must error: no checkpoint before early event time");
    match err {
        SchedulerError::NoCheckpoint(t) => assert_eq!(t, t_ns(80)),
        other => panic!("expected NoCheckpoint(80ns), got {other:?}"),
    }
}

/// Verify that the scheduler records the contract violation
/// diagnostic with the exact prefix in `SchedulerMetadata::diagnostics`.
#[test]
fn contract_violation_diagnostic_is_recorded_in_metadata() {
    let vout = NodeId::new(1);
    let analog = AnalogDouble::new(vout);
    let digital = ContractViolatingDigital::new();

    let scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    let result = scheduler.run().expect("scheduler must continue");

    let has_diag = result
        .scheduler
        .diagnostics
        .iter()
        .any(|d| d.starts_with("digital next-event-time contract violation"));
    assert!(
        has_diag,
        "diagnostics must contain the exact contract-violation message prefix; got {:?}",
        result.scheduler.diagnostics
    );
}
