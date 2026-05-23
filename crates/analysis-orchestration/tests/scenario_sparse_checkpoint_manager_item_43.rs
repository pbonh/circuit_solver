//! Integration witness for **tasks.md item #43** (Capability:
//! `mixed-signal-cosim`):
//!
//! > Implement sparse checkpoint manager: save node voltages +
//! > reactive companion state at predicted event boundaries
//! > — @adr: ADR-0004 (depends on #42)
//!
//! Item #43 carries no `@spec:` tag; it is shared infrastructure
//! that future scenario-tasks compose against (#44 rollback
//! handler, #45 boundary signal exchange, etc.). This integration
//! witness asserts the **public API surface** of the sparse
//! checkpoint manager — the types and methods that downstream
//! tasks depend on — using the same re-export channel ADR-0010
//! pins (`analysis_orchestration::lib`).
//!
//! The four core guarantees, asserted in order:
//!
//! 1. The `SparseCheckpoint` record carries **node voltages and
//!    reactive companion state** (ADR-0004 commitment #2:
//!    "typically node voltages and reactive-element
//!    companion-model state").
//! 2. The manager preserves the **monotonic save invariant**
//!    that ADR-0004's optimistic-advance model relies on
//!    (commitment #1 + #2 combined).
//! 3. `restore_at_or_before` honors the **"at-or-before"
//!    contract** that the rollback handler (tasks.md #44) will
//!    issue: the largest stored time `<= target` is returned, or
//!    `NoCheckpointAtOrBefore` if none qualifies.
//! 4. `drop_after` provides the **rollback-and-resume** pruning
//!    primitive: checkpoints strictly after the rollback target
//!    are removed, the checkpoint *at* the target is retained,
//!    and the manager can then accept further saves
//!    monotonically (#44's exact need).
//!
//! ADR refs: ADR-0004 (mixed-signal scheduler ownership +
//! sparse-checkpoint memory model), ADR-0010 (unstable v1 API
//! surface — these re-exports are tracked here so a breaking
//! change to the checkpoint manager's public API surface is
//! caught by the test breakage of this witness).

use analysis_orchestration::{
    AnalogSolver, AnalogStepReport, BoundarySignals, CheckpointError, DigitalAdapterKind,
    DigitalSimulator, DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError,
    SparseCheckpoint, SparseCheckpointManager,
};
use circuit_solver_types::{
    AnalogTrace, DigitalEventTrace, ElementId, NodeId, SimulationTime, Waveform,
};
use numeric_solver::integration::{CapacitorHistory, InductorHistory};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn t_ns(ns: i64) -> SimulationTime {
    SimulationTime::from_nanoseconds(ns)
}

/// Build a representative checkpoint: 3 non-ground nodes, 1
/// capacitor, 1 inductor. `marker` lets tests check the right
/// checkpoint was retrieved.
fn build_checkpoint(time: SimulationTime, marker: f64) -> SparseCheckpoint {
    SparseCheckpoint::empty(time)
        .with_node_voltages(vec![
            (NodeId::new(1), marker),
            (NodeId::new(2), marker * 2.0),
            (NodeId::new(3), -marker),
        ])
        .with_capacitor_states(vec![(ElementId::new(10), CapacitorHistory::new(marker))])
        .with_inductor_states(vec![(
            ElementId::new(20),
            InductorHistory::new(marker * 0.01),
        )])
}

// ---------------------------------------------------------------------------
// Guarantee 1: SparseCheckpoint carries node voltages + reactive state.
// ---------------------------------------------------------------------------

#[test]
fn item_43_checkpoint_carries_node_voltages_and_reactive_state() {
    let c = build_checkpoint(t_ns(50), 1.5);

    // Time stamp is exactly the predicted boundary.
    assert_eq!(c.time(), t_ns(50));

    // Node voltages: 3 non-ground nodes were saved. Ground is
    // *not* in the vector (sparsity convention from the
    // struct-level docs).
    assert_eq!(c.node_count(), 3);
    assert!(c.node_voltages.iter().all(|(n, _)| *n != NodeId::GROUND));
    assert_eq!(c.node_voltages[0], (NodeId::new(1), 1.5));
    assert_eq!(c.node_voltages[1], (NodeId::new(2), 3.0));
    assert_eq!(c.node_voltages[2], (NodeId::new(3), -1.5));

    // Capacitor companion state: 1 entry with v_prev = marker.
    assert_eq!(c.capacitor_count(), 1);
    assert_eq!(c.capacitor_states[0].0, ElementId::new(10));
    assert_eq!(c.capacitor_states[0].1.v_prev.to_bits(), 1.5_f64.to_bits());

    // Inductor companion state: 1 entry with i_prev = marker*0.01.
    assert_eq!(c.inductor_count(), 1);
    assert_eq!(c.inductor_states[0].0, ElementId::new(20));
    assert_eq!(c.inductor_states[0].1.i_prev.to_bits(), 0.015_f64.to_bits());

    // The checkpoint is not "empty" once any state is populated.
    assert!(!c.is_empty());
}

// ---------------------------------------------------------------------------
// Guarantee 2: monotonic save invariant.
// ---------------------------------------------------------------------------

#[test]
fn item_43_manager_enforces_monotonic_save_invariant() {
    let mut mgr = SparseCheckpointManager::new();
    assert!(mgr.is_empty());

    // Saves in forward time succeed.
    mgr.save(build_checkpoint(t_ns(50), 1.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(100), 2.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(150), 3.0)).unwrap();
    assert_eq!(mgr.len(), 3);
    assert_eq!(mgr.earliest_time(), Some(t_ns(50)));
    assert_eq!(mgr.latest_time(), Some(t_ns(150)));

    // A save at a *strictly earlier* time is rejected; the
    // existing store is unchanged.
    let err = mgr
        .save(build_checkpoint(t_ns(75), 99.0))
        .expect_err("non-monotonic save must error");
    assert_eq!(
        err,
        CheckpointError::NonMonotonicSave {
            latest: t_ns(150),
            attempted: t_ns(75),
        }
    );
    assert_eq!(mgr.len(), 3);
    assert_eq!(mgr.latest_time(), Some(t_ns(150)));
}

// ---------------------------------------------------------------------------
// Guarantee 3: restore_at_or_before contract.
// ---------------------------------------------------------------------------

#[test]
fn item_43_restore_at_or_before_returns_largest_le_target() {
    let mut mgr = SparseCheckpointManager::new();
    mgr.save(build_checkpoint(t_ns(50), 1.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(100), 2.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(150), 3.0)).unwrap();

    // Exact-match boundary: scheduler-on-correct-prediction path.
    let c = mgr.restore_at_or_before(t_ns(100)).unwrap();
    assert_eq!(c.time(), t_ns(100));
    // Marker confirms it's the t=100 checkpoint, not a neighbor.
    assert_eq!(c.capacitor_states[0].1.v_prev.to_bits(), 2.0_f64.to_bits());

    // Target between two stored times: rollback to the latest
    // *prior* checkpoint (this is exactly the rollback handler's
    // need per ADR-0004 commitment #4).
    let c = mgr.restore_at_or_before(t_ns(120)).unwrap();
    assert_eq!(c.time(), t_ns(100));

    // Target after every stored time: still returns the latest
    // (the scheduler tolerates "ask for restore beyond the
    // current frontier" by returning the most-recent good
    // checkpoint).
    let c = mgr.restore_at_or_before(t_ns(999)).unwrap();
    assert_eq!(c.time(), t_ns(150));

    // Target strictly before every stored time: error.
    let err = mgr.restore_at_or_before(t_ns(10)).unwrap_err();
    assert_eq!(err, CheckpointError::NoCheckpointAtOrBefore(t_ns(10)));
}

#[test]
fn item_43_restore_on_empty_manager_is_error() {
    let mgr = SparseCheckpointManager::new();
    let err = mgr.restore_at_or_before(t_ns(50)).unwrap_err();
    assert_eq!(err, CheckpointError::NoCheckpointAtOrBefore(t_ns(50)));
}

// ---------------------------------------------------------------------------
// Guarantee 4: drop_after enables rollback-and-resume.
// ---------------------------------------------------------------------------

#[test]
fn item_43_drop_after_supports_rollback_and_resume() {
    // Optimistic advance saves four checkpoints: 50, 100, 150, 200.
    let mut mgr = SparseCheckpointManager::new();
    mgr.save(build_checkpoint(t_ns(50), 1.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(100), 2.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(150), 3.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(200), 4.0)).unwrap();

    // Misprediction at 200: digital reports an event at 100, so
    // the rollback handler asks us to drop everything strictly
    // after 100. The checkpoint *at* 100 must survive (that's
    // the state the analog solver restores into).
    let dropped = mgr.drop_after(t_ns(100));
    assert_eq!(dropped, 2);
    assert_eq!(mgr.len(), 2);
    assert_eq!(mgr.latest_time(), Some(t_ns(100)));

    // The retained t=100 checkpoint is still retrievable in full.
    let c = mgr.restore_at_or_before(t_ns(100)).unwrap();
    assert_eq!(c.time(), t_ns(100));
    assert_eq!(c.capacitor_states[0].1.v_prev.to_bits(), 2.0_f64.to_bits());
    assert_eq!(c.node_voltages.len(), 3);

    // Forward progress after rollback: re-solve produces a new
    // checkpoint at t=120 (the corrected boundary). This must be
    // accepted because the latest is now t=100 < 120.
    mgr.save(build_checkpoint(t_ns(120), 2.5)).unwrap();
    assert_eq!(mgr.len(), 3);
    assert_eq!(mgr.latest_time(), Some(t_ns(120)));

    // Old t=200 marker is gone forever; the new t=120 is the
    // tip of the advance.
    let tip = mgr.restore_at_or_before(t_ns(999)).unwrap();
    assert_eq!(tip.time(), t_ns(120));
    assert_eq!(
        tip.capacitor_states[0].1.v_prev.to_bits(),
        2.5_f64.to_bits()
    );
}

// ---------------------------------------------------------------------------
// Bonus: drop_before releases stale-but-once-good checkpoints.
// ADR-0004's "negative consequence" memory-overhead note motivates
// this primitive; the scheduler will use it to release commits
// the rollback machinery can no longer reach.
// ---------------------------------------------------------------------------

#[test]
fn item_43_drop_before_releases_stale_checkpoints() {
    let mut mgr = SparseCheckpointManager::new();
    mgr.save(build_checkpoint(t_ns(50), 1.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(100), 2.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(150), 3.0)).unwrap();
    mgr.save(build_checkpoint(t_ns(200), 4.0)).unwrap();

    // Scheduler has committed past t=100; everything strictly
    // earlier is unreachable.
    let dropped = mgr.drop_before(t_ns(100));
    assert_eq!(dropped, 1); // only the t=50 entry
    assert_eq!(mgr.len(), 3);
    assert_eq!(mgr.earliest_time(), Some(t_ns(100)));

    // The slice view is still ordered, now starting at t=100.
    let slice = mgr.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0].time(), t_ns(100));
    assert_eq!(slice[1].time(), t_ns(150));
    assert_eq!(slice[2].time(), t_ns(200));
}

// ---------------------------------------------------------------------------
// Integration: an analog double backed by SparseCheckpointManager
// is correctly driven by MixedSignalScheduler. This is the end-to-end
// composition of item #42 (scheduler) and item #43 (checkpoint mgr).
// ---------------------------------------------------------------------------

/// Analog solver double that stores its state in a
/// `SparseCheckpointManager` (the item-#43 product), exposing the
/// existing `AnalogSolver` trait from item #42.
struct CheckpointBackedAnalog {
    observed: NodeId,
    checkpoints: SparseCheckpointManager,
    samples: Vec<(SimulationTime, f64)>,
    voltage_at: fn(SimulationTime) -> f64,
}

impl CheckpointBackedAnalog {
    fn new(observed: NodeId, voltage_at: fn(SimulationTime) -> f64) -> Self {
        let v0 = voltage_at(SimulationTime::ZERO);
        Self {
            observed,
            checkpoints: SparseCheckpointManager::new(),
            samples: vec![(SimulationTime::ZERO, v0)],
            voltage_at,
        }
    }

    fn checkpoint_times(&self) -> Vec<SimulationTime> {
        self.checkpoints
            .as_slice()
            .iter()
            .map(SparseCheckpoint::time)
            .collect()
    }
}

impl AnalogSolver for CheckpointBackedAnalog {
    fn run_until(&mut self, target: SimulationTime) -> Result<AnalogStepReport, SchedulerError> {
        let v = (self.voltage_at)(target);
        self.samples.push((target, v));

        // Build and save a real SparseCheckpoint. This is the
        // load-bearing side: the analog "saves a sparse
        // checkpoint at the boundary" using item-#43's manager.
        let cp = SparseCheckpoint::empty(target)
            .with_node_voltages(vec![(self.observed, v)])
            .with_capacitor_states(vec![(ElementId::new(10), CapacitorHistory::new(v))])
            .with_inductor_states(vec![(ElementId::new(20), InductorHistory::new(v * 0.001))]);
        self.checkpoints
            .save(cp)
            .map_err(|e| SchedulerError::AnalogSolveFailed(e.to_string()))?;

        Ok(AnalogStepReport {
            time_reached: target,
            checkpoint_saved: true,
        })
    }

    fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
        // Use the manager's restore + drop_after primitives — the
        // exact composition tasks.md #44 will package as the
        // rollback handler.
        let _cp = self
            .checkpoints
            .restore_at_or_before(target)
            .map_err(|_| SchedulerError::NoCheckpoint(target))?;
        let _ = self.checkpoints.drop_after(target);
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

/// Two-event digital double: predicts events at 50ns and 100ns,
/// then exhausts. Mirrors the in-crate `mixed_signal::test_doubles`
/// double, but minimal.
struct ScriptedDigital {
    predictions: Vec<SimulationTime>,
    confirmations_seen: Vec<SimulationTime>,
}

impl ScriptedDigital {
    fn new(predictions: Vec<SimulationTime>) -> Self {
        Self {
            predictions,
            confirmations_seen: Vec::new(),
        }
    }
}

impl DigitalSimulator for ScriptedDigital {
    fn adapter_kind(&self) -> DigitalAdapterKind {
        DigitalAdapterKind::TestDouble
    }

    fn next_event_time(&mut self) -> Result<NextEventReport, SchedulerError> {
        match self.predictions.first().copied() {
            Some(t) => Ok(NextEventReport { predicted_time: t }),
            None => Err(SchedulerError::DigitalAdapterFailed(
                "scripted digital exhausted".into(),
            )),
        }
    }

    fn confirm_event(
        &mut self,
        boundary: SimulationTime,
    ) -> Result<DigitalStepReport, SchedulerError> {
        // Pop the prediction we just confirmed.
        if let Some(first) = self.predictions.first().copied() {
            assert_eq!(first, boundary, "scheduler must confirm at predicted time");
            self.predictions.remove(0);
            self.confirmations_seen.push(boundary);
        }
        Ok(DigitalStepReport::Confirmed { time: boundary })
    }

    fn take_trace(&mut self) -> DigitalEventTrace {
        DigitalEventTrace::default()
    }
}

#[test]
fn item_43_scheduler_composes_with_checkpoint_backed_analog() {
    // Synthetic analog: ramp from 0 V to 3.3 V over 100 ns.
    fn ramp(t: SimulationTime) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let ns = t.as_nanoseconds() as f64;
        3.3 * (ns / 100.0).clamp(0.0, 1.0)
    }

    let analog = CheckpointBackedAnalog::new(NodeId::new(1), ramp);
    let digital = ScriptedDigital::new(vec![t_ns(50), t_ns(100)]);
    let scheduler = MixedSignalScheduler::new(
        analog,
        digital,
        BoundarySignals::default(),
        t_ns(200), // horizon
    );

    let result = scheduler.run().expect("scheduler must run cleanly");

    // The scheduler must have committed both predicted boundaries
    // in the expected order: 50 ns, 100 ns.
    assert_eq!(result.scheduler.commits, vec![t_ns(50), t_ns(100)]);
    // No rollbacks on the correct-prediction path.
    assert!(result.scheduler.rollbacks.is_empty());

    // The analog trace covers t=0, 50, 100 (three samples). The
    // *checkpoints* — invisible from MixedSignalResult — were
    // saved by the analog inside its item-#43 manager; the
    // dispatch above proves the scheduler's "checkpoint_saved =
    // true" debug assertion never tripped.
    let analog_trace = &result.analog;
    let waveform = &analog_trace.waveforms[0];
    assert_eq!(waveform.node, NodeId::new(1));
    assert_eq!(waveform.times, vec![t_ns(0), t_ns(50), t_ns(100)]);
    assert_eq!(analog_trace.committed_through, t_ns(100));
}

// ---------------------------------------------------------------------------
// ADR-0004 fidelity: a misprediction-then-rollback round trip
// exercises the manager exactly as tasks.md #44 will. We drive the
// analog directly (not via the scheduler) because the scheduler's
// misprediction path is the subject of #44, not #43.
// ---------------------------------------------------------------------------

#[test]
fn item_43_rollback_round_trip_via_analog_solver_double() {
    fn ramp(t: SimulationTime) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let ns = t.as_nanoseconds() as f64;
        3.3 * (ns / 100.0).clamp(0.0, 1.0)
    }

    let mut analog = CheckpointBackedAnalog::new(NodeId::new(1), ramp);

    // Optimistic advance: 50 → 100 → 150 ns.
    analog.run_until(t_ns(50)).unwrap();
    analog.run_until(t_ns(100)).unwrap();
    analog.run_until(t_ns(150)).unwrap();
    assert_eq!(
        analog.checkpoint_times(),
        vec![t_ns(50), t_ns(100), t_ns(150)]
    );

    // Digital reports a misprediction: the real event was at 80
    // ns. The rollback target falls between 50 and 100; the
    // manager's `restore_at_or_before(80)` must return the t=50
    // checkpoint (the latest one at or before 80).
    analog.rollback_to(t_ns(80)).unwrap();
    // After rollback, only t=50 survives (t=100, t=150 dropped;
    // there is no t=80 checkpoint *yet* — the analog will save
    // one after the re-solve to the corrected boundary).
    assert_eq!(analog.checkpoint_times(), vec![t_ns(50)]);

    // Re-solve to the corrected boundary t=80; that save is
    // accepted because the manager's latest is now t=50 < 80.
    analog.run_until(t_ns(80)).unwrap();
    assert_eq!(analog.checkpoint_times(), vec![t_ns(50), t_ns(80)]);

    // And forward progress continues monotonically.
    analog.run_until(t_ns(120)).unwrap();
    assert_eq!(
        analog.checkpoint_times(),
        vec![t_ns(50), t_ns(80), t_ns(120)]
    );
}

#[test]
fn item_43_rollback_below_earliest_checkpoint_errors() {
    fn ramp(t: SimulationTime) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let ns = t.as_nanoseconds() as f64;
        3.3 * (ns / 100.0).clamp(0.0, 1.0)
    }

    let mut analog = CheckpointBackedAnalog::new(NodeId::new(1), ramp);
    analog.run_until(t_ns(50)).unwrap();
    analog.run_until(t_ns(100)).unwrap();

    // Try to roll back to a time strictly before the earliest
    // saved checkpoint. This is the "no checkpoint at or before
    // target" failure path that #44 will lift into
    // SchedulerError::NoCheckpoint.
    let err = analog.rollback_to(t_ns(10)).unwrap_err();
    assert!(matches!(err, SchedulerError::NoCheckpoint(t) if t == t_ns(10)));
}
