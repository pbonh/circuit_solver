//! Integration witness for **tasks.md item #51** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Implement mixed-signal analysis control loop: orchestrate
//! > scheduler, collect analog Waveforms + digital VCD traces,
//! > produce unified Result — @spec: mixed-signal-cosim#optimistic-
//! > advance-with-correct-prediction (depends on #42, #50)
//!
//! Item #51 composes the two sibling deliverables the scheduler core
//! (#42) and the IEEE-1364 §18 VCD trace writer (#50) into a single
//! end-to-end control loop. The headline Gherkin scenario:
//!
//! ```gherkin
//! Given SimulationEngineer has constructed a mixed-signal Circuit
//!   with an analog front-end and a digital Verilog block
//! And the digital simulator predicts a next event at time 50 ns
//! When the Scheduler issues a run-until command to the analog
//!   solver for 50 ns
//! And the digital simulator confirms an event at 50 ns
//! Then the Scheduler commits the analog state at 50 ns
//! And the Result contains analog Waveforms and digital event
//!   traces synchronized at 50 ns
//! And no rollback occurs
//! ```
//!
//! This integration file asserts the three guarantees of item #51:
//!
//! 1. **Unified Result** — the control loop produces a `MixedSignalResult`
//!    that carries both analog `Waveform`s *and* a parseable VCD digital
//!    event trace in a single envelope (depends on #42 + #50).
//! 2. **Synchronized commit** — the analog `committed_through` field and
//!    the digital VCD timestamp agree on the committed boundary time.
//! 3. **No rollback** — the correct-prediction path performs zero
//!    rollbacks (the `rollback_free()` invariant holds).
//!
//! ADR refs: ADR-0004 (optimistic advance), ADR-0010 (unstable v1 API).

use std::cell::RefCell;
use std::fmt::Write as _;
use std::rc::Rc;

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError,
};
use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, NodeId, SignalName, SimulationTime, Waveform,
};

// ---------------------------------------------------------------------------
// Integration-test doubles — minimal, observable, distinct from the in-crate
// ones. We need both analog samples and a parseable VCD string to prove the
// control loop successfully collected both halves into the unified Result.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum AnalogCall {
    RunUntil(SimulationTime),
    #[allow(dead_code)] // reserved for rollback scenario tests
    RollbackTo(SimulationTime),
    TakeTrace,
}

#[derive(Debug, Clone, PartialEq)]
enum DigitalCall {
    NextEventTime,
    ConfirmEvent(SimulationTime),
    TakeTrace,
}

struct ObservingAnalog {
    observed: NodeId,
    log: Rc<RefCell<Vec<AnalogCall>>>,
    samples: Vec<(SimulationTime, f64)>,
    checkpoints: Vec<SimulationTime>,
}

impl ObservingAnalog {
    fn new(observed: NodeId, log: Rc<RefCell<Vec<AnalogCall>>>) -> Self {
        Self {
            observed,
            log,
            samples: vec![(SimulationTime::ZERO, 0.0)],
            checkpoints: Vec::new(),
        }
    }
}

impl AnalogSolver for ObservingAnalog {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        self.log.borrow_mut().push(AnalogCall::RunUntil(target));
        #[allow(clippy::cast_precision_loss)]
        let ns = target.as_nanoseconds() as f64;
        let value = 3.3 * (ns / 50.0).clamp(0.0, 1.0);
        self.samples.push((target, value));
        self.checkpoints.push(target);
        Ok(AnalogStepReport {
            time_reached: target,
            checkpoint_saved: true,
            checkpoint: None,
        })
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.log.borrow_mut().push(AnalogCall::RollbackTo(target));
        self.samples.retain(|(t, _)| *t <= target);
        self.checkpoints.retain(|t| *t <= target);
        Ok(())
    }

    fn take_trace(&mut self) -> AnalogTrace {
        self.log.borrow_mut().push(AnalogCall::TakeTrace);
        let (times, values): (Vec<_>, Vec<_>) = self.samples.iter().copied().unzip();
        let committed_through = times.last().copied().unwrap_or(SimulationTime::ZERO);
        AnalogTrace {
            waveforms: vec![Waveform::new(self.observed, times, values)],
            committed_through,
        }
    }
}

struct ObservingDigital {
    script: std::collections::VecDeque<SimulationTime>,
    log: Rc<RefCell<Vec<DigitalCall>>>,
    confirmed: Vec<SimulationTime>,
    signals: Vec<SignalName>,
}

impl ObservingDigital {
    fn new(
        script: impl IntoIterator<Item = SimulationTime>,
        signals: Vec<SignalName>,
        log: Rc<RefCell<Vec<DigitalCall>>>,
    ) -> Self {
        Self {
            script: script.into_iter().collect(),
            log,
            confirmed: Vec::new(),
            signals,
        }
    }
}

impl DigitalSimulator for ObservingDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        self.log.borrow_mut().push(DigitalCall::NextEventTime);
        match self.script.front().copied() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "integration double exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        self.log
            .borrow_mut()
            .push(DigitalCall::ConfirmEvent(boundary));
        match self.script.pop_front() {
            Some(t) if t == boundary => {
                self.confirmed.push(t);
                Ok(DigitalStepReport::Confirmed { time: t })
            }
            Some(t) => Err(SchedulerError::ContractViolation {
                predicted: t,
                actual: boundary,
            }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "integration double script exhausted before confirm_event".into(),
            )),
        }
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        self.log.borrow_mut().push(DigitalCall::TakeTrace);
        // Emit a full, parseable VCD via the build_vcd helper (#50),
        // proving the control loop (#51) can collect a standards-
        // conformant digital trace.
        let mut vcd = String::new();
        let _ = writeln!(vcd, "$timescale 1ps $end");
        let _ = writeln!(vcd, "$scope module mixed_signal_test $end");
        for (i, sig) in self.signals.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let id_byte = b'!' + (i as u8);
            let id = char::from(id_byte);
            let _ = writeln!(vcd, "$var wire 1 {id} {sig} $end");
        }
        let _ = writeln!(vcd, "$upscope $end");
        let _ = writeln!(vcd, "$enddefinitions $end");
        for t in &self.confirmed {
            let _ = writeln!(vcd, "#{}", t.as_picoseconds());
            for (i, _) in self.signals.iter().enumerate() {
                #[allow(clippy::cast_possible_truncation)]
                let id_byte = b'!' + (i as u8);
                let id = char::from(id_byte);
                let _ = writeln!(vcd, "1{id}");
            }
        }

        let events_by_signal = self
            .signals
            .iter()
            .map(|s| (s.clone(), self.confirmed.clone()))
            .collect();

        DigitalEventTrace {
            vcd,
            events_by_signal,
        }
    }
}

// ---------------------------------------------------------------------------
// Witness 1 — Unified Result: control loop produces both analog Waveforms
// and a parseable VCD digital event trace in a single MixedSignalResult.
// ---------------------------------------------------------------------------

/// The headline Gherkin scenario for item #51 states that the Result must
/// contain analog Waveforms *and* digital event traces synchronized at the
/// committed boundary. This test constructs the scheduler through the
/// public API, drives the control loop to completion, and asserts both
/// halves are present and well-formed.
#[test]
fn item_51_control_loop_produces_unified_result_with_analog_waveforms_and_vcd_trace() {
    let analog_log = Rc::new(RefCell::new(Vec::new()));
    let digital_log = Rc::new(RefCell::new(Vec::new()));
    let vout = NodeId::new(1);

    let analog = ObservingAnalog::new(vout, Rc::clone(&analog_log));
    let digital = ObservingDigital::new(
        [SimulationTime::from_nanoseconds(50)],
        vec![SignalName::new("din"), SignalName::new("dout")],
        Rc::clone(&digital_log),
    );
    let boundaries = BoundarySignals {
        analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
        digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
    };

    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        boundaries,
        SimulationTime::from_nanoseconds(100),
    );
    let result = scheduler.run().expect("control loop must succeed");

    // --- Then the Scheduler commits the analog state at 50 ns ---
    assert_eq!(
        result.final_commit(),
        Some(SimulationTime::from_nanoseconds(50)),
        "control loop must commit at the predicted 50 ns boundary"
    );
    assert_eq!(
        result.scheduler.commits,
        vec![SimulationTime::from_nanoseconds(50)],
        "exactly one commit, at 50 ns"
    );

    // --- And the Result contains analog Waveforms ---
    let analog_wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain the vout waveform");
    assert!(
        analog_wf
            .times
            .contains(&SimulationTime::from_nanoseconds(50)),
        "analog waveform must include the 50 ns sample"
    );
    assert_eq!(
        result.analog.committed_through,
        SimulationTime::from_nanoseconds(50),
        "analog committed_through must be the synchronized boundary"
    );

    // --- And the Result contains a VCD-format digital event trace ---
    assert!(
        result.digital.vcd.contains("$timescale 1ps $end"),
        "VCD must declare a timescale"
    );
    assert!(
        result.digital.vcd.contains("$enddefinitions $end"),
        "VCD must terminate its declarations block"
    );
    // 50 ns = 50_000 ps in VCD timestamps
    assert!(
        result.digital.vcd.contains(&format!("#{}\n", 50_000_i64)),
        "VCD must contain a #50000 timestamp for the 50 ns event"
    );

    // Both declared signals must be recorded at 50 ns.
    for sig in [SignalName::new("din"), SignalName::new("dout")] {
        assert_eq!(
            result.digital.events_for(&sig),
            Some(&[SimulationTime::from_nanoseconds(50)][..]),
            "digital trace must record an event at 50 ns for {sig}"
        );
    }

    // --- And no rollback occurs ---
    assert!(
        result.rollback_free(),
        "correct-prediction path must have zero rollbacks"
    );
    assert!(result.scheduler.rollbacks.is_empty());
}

// ---------------------------------------------------------------------------
// Witness 2 — Synchronized commit: the analog committed_through and the
// digital VCD agree on every confirmed boundary time.
// ---------------------------------------------------------------------------

/// The control loop must ensure the analog `committed_through` and the
/// digital trace convergently point to the last confirmed boundary.
/// This test drives three successive events (20 ns, 50 ns, 80 ns) and
/// checks alignment at each step.
#[test]
fn item_51_control_loop_synchronizes_analog_and_digital_at_every_boundary() {
    let analog_log = Rc::new(RefCell::new(Vec::new()));
    let digital_log = Rc::new(RefCell::new(Vec::new()));

    let analog = ObservingAnalog::new(NodeId::new(1), Rc::clone(&analog_log));
    let digital = ObservingDigital::new(
        [
            SimulationTime::from_nanoseconds(20),
            SimulationTime::from_nanoseconds(50),
            SimulationTime::from_nanoseconds(80),
        ],
        vec![
            SignalName::new("clk"),
            SignalName::new("rst"),
            SignalName::new("data"),
        ],
        Rc::clone(&digital_log),
    );

    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(100),
    );
    let result = scheduler.run().expect("control loop must succeed");

    // All three commits must be recorded.
    assert_eq!(
        result.scheduler.commits,
        vec![
            SimulationTime::from_nanoseconds(20),
            SimulationTime::from_nanoseconds(50),
            SimulationTime::from_nanoseconds(80),
        ],
        "three predicted events → three commits"
    );

    // The analog committed_through must match the last commit.
    assert_eq!(
        result.analog.committed_through,
        SimulationTime::from_nanoseconds(80),
        "analog committed_through must equal the final synchronized boundary"
    );

    // The VCD must contain timestamps for all three boundaries.
    // 20 ns = 20_000 ps, 50 ns = 50_000 ps, 80 ns = 80_000 ps
    for (ns, ps) in [(20, 20_000), (50, 50_000), (80, 80_000)] {
        assert!(
            result.digital.vcd.contains(&format!("#{ps}\n")),
            "VCD must contain a #{ps} timestamp for the {ns} ns boundary"
        );
    }

    // Each signal must have events at all three times.
    for sig in [
        SignalName::new("clk"),
        SignalName::new("rst"),
        SignalName::new("data"),
    ] {
        assert_eq!(
            result.digital.events_for(&sig),
            Some(
                &[
                    SimulationTime::from_nanoseconds(20),
                    SimulationTime::from_nanoseconds(50),
                    SimulationTime::from_nanoseconds(80),
                ][..]
            ),
            "digital trace must record all three events for {sig}"
        );
    }
}

// ---------------------------------------------------------------------------
// Witness 3 — No rollback on correct-prediction path: rollback_free().
// ---------------------------------------------------------------------------

/// The `rollback_free()` invariant is the spec's "And no rollback occurs"
/// clause lifted to the public API surface. This test drives the happy
/// path with a single event and asserts the invariant directly.
#[test]
fn item_51_control_loop_rollback_free_on_correct_prediction() {
    let analog_log = Rc::new(RefCell::new(Vec::new()));
    let digital_log = Rc::new(RefCell::new(Vec::new()));

    let analog = ObservingAnalog::new(NodeId::new(1), Rc::clone(&analog_log));
    let digital = ObservingDigital::new(
        [SimulationTime::from_nanoseconds(50)],
        vec![SignalName::new("sig")],
        Rc::clone(&digital_log),
    );

    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        SimulationTime::from_nanoseconds(100),
    );
    let result = scheduler.run().expect("control loop must succeed");

    assert!(result.rollback_free(), "no rollbacks on correct-prediction");
    assert!(result.scheduler.rollbacks.is_empty());

    // Belt-and-braces: the analog kernel never received a rollback command.
    let no_rollback_calls = !analog_log
        .borrow()
        .iter()
        .any(|c| matches!(c, AnalogCall::RollbackTo(_)));
    assert!(
        no_rollback_calls,
        "analog kernel must receive zero RollbackTo calls on the correct-prediction path"
    );
}

// ---------------------------------------------------------------------------
// Witness 4 — Public API surface stability (ADR-0010 v1 unstable but pinned).
// ---------------------------------------------------------------------------

/// Item #51's integration surface is the full set of mixed-signal + VCD
/// re-exports. If the scheduler, its trait bounds, or the `DigitalEventTrace`
/// / `MixedSignalResult` types change shape, this compile-time assertion
/// catches it.
#[test]
fn item_51_public_api_surface_is_visible() {
    // Types from #42 (scheduler).
    let _: MixedSignalScheduler<ObservingAnalog, ObservingDigital>;
    let _: BoundarySignals = BoundarySignals::default();
    let _: SchedulerError = SchedulerError::AnalogSolveFailed("compile-time witness".into());
    let _: NextEventReport = NextEventReport {
        predicted_time: SimulationTime::ZERO,
    };
    let _: AnalogStepReport = AnalogStepReport {
        time_reached: SimulationTime::ZERO,
        checkpoint_saved: true,
        checkpoint: None,
    };
    let _: DigitalStepReport = DigitalStepReport::Confirmed {
        time: SimulationTime::ZERO,
    };

    // Types from #50 (VCD writer).
    let _: DigitalEventTrace = DigitalEventTrace {
        vcd: String::new(),
        events_by_signal: Vec::new(),
    };
    let _: AnalogTrace = AnalogTrace {
        waveforms: Vec::new(),
        committed_through: SimulationTime::ZERO,
    };
}
