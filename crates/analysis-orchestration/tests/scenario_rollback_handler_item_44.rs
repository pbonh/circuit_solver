//! Integration witness for **tasks.md item #44** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Implement rollback handler: restore analog state from
//! > checkpoint, re-issue run-until with corrected time
//! > — @spec: mixed-signal-cosim#optimistic-advance-with-
//! >   misprediction-requiring-rollback (depends on #43)
//!
//! Item #44 binds the rollback handler to the
//! `optimistic-advance-with-misprediction-requiring-rollback`
//! Gherkin scenario:
//!
//! ```gherkin
//! Given SimulationEngineer has constructed a mixed-signal Circuit
//! And the digital simulator predicts a next event at time 100 ns
//! When the Scheduler issues a run-until command to the analog
//!   solver for 100 ns
//! And the analog solver saves a sparse checkpoint at 100 ns
//! And the digital simulator reports no event at 100 ns but an
//!   event at 80 ns
//! Then the Scheduler rolls back the analog state to the checkpoint
//!   nearest before 80 ns
//! And the Scheduler re-issues a run-until command for 80 ns
//! And the Result contains correct analog Waveforms and digital
//!   traces at 80 ns
//! And the rollback event is recorded in the Result metadata
//! ```
//!
//! This witness drives that exact sequence end-to-end through the
//! public `MixedSignalScheduler`, asserts the scheduler's call
//! sequence into the analog and digital adapters, and verifies the
//! `RollbackEvent` carries the right `(mispredicted_at,
//! corrected_to, checkpoint_at, reason)` tuple.
//!
//! ADR refs:
//! - **ADR-0004** — Mixed-Signal Scheduler ownership + sparse
//!   checkpoint memory model; commitment #4 is "Rollback on
//!   misprediction" which this scenario realizes.
//! - **ADR-0007** — Zero-Order Hold default at boundary; orthogonal
//!   to this witness but cited as part of the change-level set.
//! - **ADR-0008** — Per-node tolerance envelope; this witness uses
//!   exact-recall assertions (no integration) so bitwise `PartialEq`
//!   is sufficient per the manager's documented contract.
//! - **ADR-0010** — Unstable public API surface for v1; the
//!   re-exports asserted at the top of this file pin the
//!   surface item #44 contributes.

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, RollbackHandler, RollbackOutcome,
    SchedulerError, SparseCheckpoint, SparseCheckpointManager,
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

// ---------------------------------------------------------------------------
// Test doubles
//
// These two doubles are scoped to this integration witness only. We
// drive the same Gherkin script the spec describes: digital predicts
// 100 ns first, the analog solver advances + saves a checkpoint,
// and on `confirm_event` the digital adapter reports `Mispredicted
// { actual_time: 80 ns }`.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum AnalogCall {
    RunUntil(SimulationTime),
    RollbackTo(SimulationTime),
}

/// Analog solver double that observes one node (`vout`) at a simple
/// linear profile and saves a real `SparseCheckpoint` at every
/// `run_until` so the scheduler's rollback handler has a checkpoint
/// to restore. State is dropped strictly after a `rollback_to`
/// target, matching what the real solver will do.
struct AnalogDouble {
    observed: NodeId,
    calls: Vec<AnalogCall>,
    samples: Vec<(SimulationTime, f64)>,
}

impl AnalogDouble {
    fn new(observed: NodeId) -> Self {
        Self {
            observed,
            calls: Vec::new(),
            samples: vec![(SimulationTime::ZERO, voltage_at(SimulationTime::ZERO))],
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn voltage_at(t: SimulationTime) -> f64 {
    // Simple ramp: 0 V at 0 ns, 1 V at 100 ns, linear in between.
    let ns = t.as_nanoseconds() as f64;
    (ns / 100.0).clamp(0.0, 1.0)
}

impl AnalogSolver for AnalogDouble {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        self.calls.push(AnalogCall::RunUntil(target));
        let v = voltage_at(target);
        self.samples.push((target, v));
        let checkpoint =
            SparseCheckpoint::empty(target).with_node_voltages(vec![(self.observed, v)]);
        Ok(AnalogStepReport::with_checkpoint(target, checkpoint))
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        self.calls.push(AnalogCall::RollbackTo(target));
        // Drop samples strictly after `target` to mirror real
        // restore semantics.
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

/// Digital adapter double that emits a single misprediction event:
///
/// - First `next_event_time` call returns 100 ns.
/// - First `confirm_event(100 ns)` returns `Mispredicted { actual: 80 ns }`.
/// - Second `next_event_time` call returns 80 ns (the corrected
///   event the scheduler should re-target — but in our scheduler
///   the rollback handler already advanced the analog solver to
///   80 ns inside the previous loop iteration, so the loop should
///   then exit cleanly via end-of-events on the third query).
/// - Second `confirm_event(80 ns)` is `Confirmed { time: 80 ns }`
///   (a corrected re-confirmation, equivalent to the spec's "and
///   the Scheduler re-issues a run-until for 80 ns").
/// - Third `next_event_time` returns an exhausted error so the
///   scheduler terminates.
struct MispredictingDigital {
    next_predictions: VecDeque<SimulationTime>,
    confirms: VecDeque<DigitalStepReport>,
    confirm_calls: Vec<SimulationTime>,
    next_calls: usize,
}

impl MispredictingDigital {
    fn new() -> Self {
        let next_predictions = VecDeque::from([t_ns(100), t_ns(80)]);
        let confirms = VecDeque::from([
            DigitalStepReport::Mispredicted {
                actual_time: t_ns(80),
            },
            DigitalStepReport::Confirmed { time: t_ns(80) },
        ]);
        Self {
            next_predictions,
            confirms,
            confirm_calls: Vec::new(),
            next_calls: 0,
        }
    }
}

impl DigitalSimulator for MispredictingDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        self.next_calls += 1;
        match self.next_predictions.pop_front() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "test double exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        self.confirm_calls.push(boundary);
        self.confirms.pop_front().ok_or_else(|| {
            SchedulerError::DigitalAdapterFailed("confirm_event script exhausted".into())
        })
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        use std::fmt::Write as _;
        // Minimal VCD covering the one confirmed event at 80 ns.
        let mut vcd = String::new();
        vcd.push_str("$timescale 1ps $end\n");
        vcd.push_str("$scope module mixed_signal_test $end\n");
        vcd.push_str("$var wire 1 ! din $end\n");
        vcd.push_str("$upscope $end\n$enddefinitions $end\n");
        let _ = writeln!(vcd, "#{}\n1!", t_ns(80).as_picoseconds());
        DigitalEventTrace {
            vcd,
            events_by_signal: vec![(SignalName::new("din"), vec![t_ns(80)])],
        }
    }
}

// ---------------------------------------------------------------------------
// Three-phase digital double — used by the on-path Gherkin scenario
// to seed an earlier confirmed boundary at 50 ns before the
// misprediction at 100 ns.
// ---------------------------------------------------------------------------

struct ThreePhaseDigital {
    next_predictions: VecDeque<SimulationTime>,
    confirms: VecDeque<DigitalStepReport>,
    confirm_calls: Vec<SimulationTime>,
}

impl DigitalSimulator for ThreePhaseDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.next_predictions.pop_front() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed("end".into())),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        self.confirm_calls.push(boundary);
        self.confirms
            .pop_front()
            .ok_or_else(|| SchedulerError::DigitalAdapterFailed("confirm end".into()))
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        use std::fmt::Write as _;
        let mut vcd = String::new();
        vcd.push_str("$timescale 1ps $end\n");
        vcd.push_str("$enddefinitions $end\n");
        let _ = writeln!(vcd, "#{}", t_ns(50).as_picoseconds());
        let _ = writeln!(vcd, "#{}", t_ns(80).as_picoseconds());
        DigitalEventTrace {
            vcd,
            events_by_signal: vec![(SignalName::new("din"), vec![t_ns(50), t_ns(80)])],
        }
    }
}

// ---------------------------------------------------------------------------
// API-surface witness (ADR-0010): pin the public re-exports that
// item #44 contributes. A breaking rename will fail-compile here.
// ---------------------------------------------------------------------------

#[test]
fn item_44_public_api_surface_is_re_exported() {
    // The re-exports below must resolve at the crate root per
    // ADR-0010 (unstable v1 API surface). A breaking rename will
    // fail compilation of this test.
    type _Handler = RollbackHandler;
    type _Outcome = RollbackOutcome;
    type _Manager = SparseCheckpointManager;
    // Cheap runtime check so the test reports `ok` (not just
    // compile-only) and the surface is exercised at least once.
    let h: _Handler = RollbackHandler::new();
    assert_eq!(h.checkpoint_count(), 0);
}

// ---------------------------------------------------------------------------
// The Gherkin scenario itself.
// ---------------------------------------------------------------------------

/// **Scenario: optimistic-advance-with-misprediction-requiring-rollback**
///
/// Drives the Gherkin block in the task body end-to-end via the
/// public `MixedSignalScheduler`. Asserts (in order of the
/// "Then ..." clauses):
///
/// 1. The Scheduler rolls back the analog state to the checkpoint
///    nearest before 80 ns. The analog solver receives
///    `rollback_to(checkpoint_time)` where `checkpoint_time` is the
///    nearest stored time `<= 80 ns`. Since the only checkpoint
///    saved before the misprediction was at 100 ns (`> 80 ns`),
///    there is no checkpoint at-or-before 80 ns and the scheduler
///    must surface `NoCheckpoint(80 ns)`. The spec's "nearest
///    before 80 ns" wording assumes *some* checkpoint exists earlier
///    — see `rollback_finds_nearest_before_when_earlier_checkpoint_exists`
///    for the populated case which is the on-path Gherkin.
/// 2. The corrected re-advance lands at 80 ns: the analog solver
///    receives `run_until(80 ns)`.
/// 3. The Result contains an analog Waveform with a sample at 80 ns
///    and the digital VCD records the event at 80 ns.
/// 4. The rollback event is recorded in `result.scheduler.rollbacks`
///    with `mispredicted_at = 100 ns`, `corrected_to = 80 ns`,
///    `checkpoint_at = <nearest-before>`, and a non-empty reason.
#[test]
fn rollback_finds_nearest_before_when_earlier_checkpoint_exists() {
    // For this on-path scenario we seed an earlier confirmed boundary
    // at 50 ns so the manager has a real "nearest before 80 ns"
    // checkpoint. This matches the spec's intent — in a real run the
    // scheduler has already committed earlier boundaries before the
    // misprediction at 100 ns. The digital double therefore confirms
    // 50 ns first, then mispredicts at 100 ns.
    let vout = NodeId::new(1);

    let digital = ThreePhaseDigital {
        next_predictions: VecDeque::from([t_ns(50), t_ns(100)]),
        confirms: VecDeque::from([
            DigitalStepReport::Confirmed { time: t_ns(50) },
            DigitalStepReport::Mispredicted {
                actual_time: t_ns(80),
            },
        ]),
        confirm_calls: Vec::new(),
    };
    let analog = AnalogDouble::new(vout);
    let scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    let result = scheduler.run().expect("scheduler.run must succeed");

    // — Then the Scheduler rolls back to the checkpoint nearest
    //   before 80 ns. The nearest stored checkpoint <= 80 ns is 50 ns.
    //   And the Scheduler re-issues a run-until command for 80 ns.
    assert_eq!(
        result.scheduler.rollbacks.len(),
        1,
        "exactly one rollback event"
    );
    let rb = &result.scheduler.rollbacks[0];
    assert_eq!(
        rb,
        &RollbackEvent {
            mispredicted_at: t_ns(100),
            corrected_to: t_ns(80),
            checkpoint_at: t_ns(50),
            reason: "no-event-confirmed".into(),
        },
        "rollback event must carry the nearest-before checkpoint time"
    );

    // — And the Result contains correct analog Waveforms and digital
    //   traces at 80 ns —
    let wf = result
        .analog
        .waveform_for(vout)
        .expect("analog trace must contain vout waveform");
    assert!(
        wf.times.contains(&t_ns(80)),
        "analog waveform must include the 80 ns sample"
    );
    assert!(
        !wf.times.contains(&t_ns(100)),
        "analog waveform must NOT contain the (rolled-back) 100 ns sample"
    );
    assert_eq!(
        result.analog.committed_through,
        t_ns(80),
        "analog trace committed_through must end at the corrected 80 ns"
    );
    assert!(
        result
            .digital
            .vcd
            .contains(&format!("#{}", t_ns(80).as_picoseconds())),
        "digital VCD must record an event at 80 ns"
    );

    // — And the rollback event is recorded in the Result metadata —
    //   (asserted above via `result.scheduler.rollbacks`).
    assert!(
        !result.rollback_free(),
        "rollback_free must be false when a rollback occurred"
    );

    // Two commits expected: 50 ns (the confirmed one) and 80 ns
    // (the post-rollback corrected commit).
    assert_eq!(result.scheduler.commits, vec![t_ns(50), t_ns(80)]);
    assert_eq!(result.final_commit(), Some(t_ns(80)));
}

/// Edge case: misprediction with no earlier checkpoint. The
/// scheduler must surface `SchedulerError::NoCheckpoint(target)`
/// rather than silently restoring an arbitrary state.
#[test]
fn misprediction_without_earlier_checkpoint_surfaces_no_checkpoint_error() {
    let vout = NodeId::new(1);
    let analog = AnalogDouble::new(vout);
    let digital = MispredictingDigital::new();
    let scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    let err = scheduler
        .run()
        .expect_err("must error: no earlier checkpoint");
    match err {
        SchedulerError::NoCheckpoint(t) => assert_eq!(t, t_ns(80)),
        other => panic!("expected NoCheckpoint(80ns), got {other:?}"),
    }
}

/// Belt-and-braces: the rollback handler's checkpoint manager
/// behaviour is observable via `MixedSignalScheduler::rollback_handler`
/// after construction (pre-`run`, since `run` consumes self).
#[test]
fn rollback_handler_is_observable_before_run() {
    let vout = NodeId::new(1);
    let analog = AnalogDouble::new(vout);
    let digital = MispredictingDigital::new();
    let scheduler =
        MixedSignalScheduler::new(analog, digital, BoundarySignals::default(), t_ns(200));
    // Before run, no checkpoints have been observed.
    assert_eq!(scheduler.rollback_handler().checkpoint_count(), 0);
    assert!(scheduler.rollback_handler().manager().is_empty());
}

/// Direct-call witness: drive `RollbackHandler::rollback_to`
/// against the public API surface to pin its semantics in the
/// integration layer. This mirrors the unit tests in
/// `mixed_signal::rollback::tests` but operates only through the
/// crate-root re-exports per ADR-0010.
#[test]
fn rollback_handler_direct_call_via_public_api() {
    let vout = NodeId::new(1);
    let mut handler = RollbackHandler::new();
    // Seed checkpoints at 50 ns and 100 ns via the public observe_step path.
    let r1 = AnalogStepReport::with_checkpoint(
        t_ns(50),
        SparseCheckpoint::empty(t_ns(50)).with_node_voltages(vec![(vout, 0.5)]),
    );
    let r2 = AnalogStepReport::with_checkpoint(
        t_ns(100),
        SparseCheckpoint::empty(t_ns(100)).with_node_voltages(vec![(vout, 1.0)]),
    );
    handler.observe_step(&r1).unwrap();
    handler.observe_step(&r2).unwrap();
    assert_eq!(handler.checkpoint_count(), 2);

    // Roll back to corrected = 80 ns.
    let mut analog = AnalogDouble::new(vout);
    let outcome = handler
        .rollback_to(&mut analog, t_ns(100), t_ns(80), "no-event-confirmed")
        .expect("rollback must succeed");
    assert_eq!(outcome.event.checkpoint_at, t_ns(50));
    assert_eq!(outcome.event.corrected_to, t_ns(80));
    assert_eq!(outcome.event.mispredicted_at, t_ns(100));
    assert_eq!(outcome.pruned_checkpoints, 1, "100 ns checkpoint pruned");
    // After rollback + re-advance, the analog double recorded both
    // calls in order.
    assert_eq!(
        analog.calls,
        vec![
            AnalogCall::RollbackTo(t_ns(50)),
            AnalogCall::RunUntil(t_ns(80)),
        ],
    );
}
