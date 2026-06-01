//! Rollback handler (tasks.md item #44, ADR-0004 commitment #4).
//!
//! The Mixed-Signal Scheduler (tasks.md item #42, see
//! [`super`]) drives the analog kernel forward optimistically up to
//! the next digital-predicted event boundary. When the digital
//! simulator subsequently reports that **no event occurred at the
//! predicted time** — but an event occurred *earlier* — the scheduler
//! must:
//!
//! 1. **find the nearest sparse checkpoint at or before the corrected
//!    event time** in the [`SparseCheckpointManager`] (tasks.md #43),
//! 2. **restore the analog solver state** to that checkpoint, by
//!    invoking [`super::AnalogSolver::rollback_to`] with the
//!    checkpoint's saved time,
//! 3. **drop every checkpoint strictly after** the restored time so
//!    the manager's monotonic-save invariant is preserved when the
//!    solver next saves a checkpoint,
//! 4. **re-issue `run_until` to the corrected event time**, and
//! 5. **record a [`circuit_solver_types::RollbackEvent`]** in the
//!    scheduler's metadata so downstream auditors (and the
//!    `optimistic-advance-with-misprediction-requiring-rollback`
//!    Gherkin scenario) can verify what happened.
//!
//! This module owns the [`RollbackHandler`] struct that performs
//! exactly that sequence. The scheduler delegates to it whenever the
//! digital adapter returns
//! [`super::DigitalStepReport::Mispredicted`].
//!
//! # Why a separate module
//!
//! Per the scientia design archive for sparse-checkpoint manager
//! (#43, parent task `t_79d0747d`), the manager is *passive* — it
//! does not drive the simulator. The rollback handler is the
//! *active* counterpart: it composes the manager with the analog
//! solver, but its dependency direction stays one-way (handler →
//! manager + analog solver), so the manager remains reusable by
//! sibling scenarios (boundary signal exchange, contract violation
//! detection in #49) without inheriting any rollback machinery.
//!
//! # ADR cross-references
//!
//! - **ADR-0004** (Optimistic Mixed-Signal Synchronization) —
//!   commitment #4 ("Rollback on misprediction") is what this module
//!   implements; the handler is the sole owner of the rollback
//!   command path so the analog solver and digital simulator stay
//!   decoupled.
//! - **ADR-0007** (Zero-Order Hold default at analog↔digital
//!   boundary) — orthogonal: boundary value rehydration on rollback
//!   is the boundary exchanger's job (item #45), not this handler's.
//! - **ADR-0008** (per-node max(rel,abs) tolerance envelope) —
//!   orthogonal: the handler restores *exact* recorded state; any
//!   tolerance comparison happens after re-solve in conformance.
//! - **ADR-0010** (unstable v1 public API) — the
//!   [`RollbackHandler`] surface is re-exported via the crate root
//!   so downstream tests/witnesses can construct one directly.

use circuit_solver_types::{RollbackEvent, SimulationTime};

use super::{AnalogSolver, AnalogStepReport, SchedulerError};
use crate::checkpoint::{CheckpointError, SparseCheckpointManager};

// ---------------------------------------------------------------------------
// RollbackHandler
// ---------------------------------------------------------------------------

/// Active coupler between a [`SparseCheckpointManager`] and an
/// [`AnalogSolver`] that implements ADR-0004 commitment #4
/// ("Rollback on misprediction").
///
/// # Lifecycle inside the scheduler
///
/// 1. The scheduler constructs a handler on entry to its synchronization
///    loop via [`RollbackHandler::new`].
/// 2. After every successful `analog.run_until(boundary)` call, the
///    scheduler hands the report's checkpoint (if any) to
///    [`RollbackHandler::observe_step`], which records it in the
///    underlying [`SparseCheckpointManager`] so a future rollback can
///    find it.
/// 3. When the digital simulator returns
///    [`super::DigitalStepReport::Mispredicted`] (or
///    [`super::DigitalStepReport::Postponed`], or a contract-violation
///    diagnostic in tasks.md #49), the scheduler calls
///    [`RollbackHandler::rollback_to`] with the *corrected* event time.
///    The handler then performs steps 1–5 above and returns the
///    populated [`RollbackEvent`] for the scheduler to push into
///    [`circuit_solver_types::SchedulerMetadata::rollbacks`].
/// 4. After the rollback returns the handler re-issues
///    `analog.run_until(corrected)` and exposes that
///    [`AnalogStepReport`] to the scheduler so it can record the new
///    boundary's checkpoint (closing the loop).
///
/// # Concurrency
///
/// `RollbackHandler` is `!Sync` (the contained
/// [`SparseCheckpointManager`] is `!Sync`); the scheduler owns it
/// singly. The handler holds an internal mutable manager and never
/// hands out raw `&mut` references, which keeps the rollback
/// invariants local to this module.
#[derive(Debug)]
pub struct RollbackHandler {
    manager: SparseCheckpointManager,
}

impl Default for RollbackHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RollbackHandler {
    /// Construct an empty handler. The scheduler typically calls this
    /// once at the top of its synchronization loop and feeds reports
    /// to it via [`RollbackHandler::observe_step`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            manager: SparseCheckpointManager::new(),
        }
    }

    /// Construct a handler that wraps a caller-supplied manager.
    /// Useful for tests that want to inspect or seed the manager
    /// directly.
    #[must_use]
    pub const fn with_manager(manager: SparseCheckpointManager) -> Self {
        Self { manager }
    }

    /// Borrow the underlying checkpoint manager. Read-only access is
    /// surfaced so callers can inspect the saved boundaries for
    /// diagnostics or testing; the rollback handler retains exclusive
    /// mutable ownership.
    #[must_use]
    pub fn manager(&self) -> &SparseCheckpointManager {
        &self.manager
    }

    /// Number of checkpoints currently retained. Convenience
    /// accessor for tests.
    #[must_use]
    pub fn checkpoint_count(&self) -> usize {
        self.manager.len()
    }

    /// Observe an [`AnalogStepReport`] from the analog solver. If the
    /// report carries a [`crate::checkpoint::SparseCheckpoint`] payload, the handler
    /// records it in the manager.
    ///
    /// `checkpoint_saved == true` with `checkpoint == None` is treated
    /// as a no-op — the contract permits implementations that have
    /// not yet plumbed a real payload, and the manager's monotonic
    /// invariant only constrains payloads that are actually stored.
    ///
    /// # Errors
    ///
    /// Returns [`CheckpointError::NonMonotonicSave`] if the carried
    /// checkpoint's time is strictly less than the latest stored
    /// time. Callers (the scheduler) should propagate this as a
    /// scheduler bug — on the correct-prediction path the solver
    /// only ever saves at strictly-increasing boundaries.
    pub fn observe_step(&mut self, report: &AnalogStepReport) -> Result<(), CheckpointError> {
        if let Some(checkpoint) = report.checkpoint.as_ref() {
            self.manager.save(checkpoint.clone())?;
        }
        Ok(())
    }

    /// Execute a full rollback to `corrected` time:
    ///
    /// 1. Look up the nearest checkpoint with `time <= corrected` in
    ///    the manager.
    /// 2. Tell the analog solver to roll back to that checkpoint's
    ///    time via [`AnalogSolver::rollback_to`].
    /// 3. Prune every checkpoint strictly after the restored time
    ///    from the manager.
    /// 4. Re-issue `run_until(corrected)` against the analog solver.
    /// 5. Record the new checkpoint (if the report carries one).
    /// 6. Return a populated [`RollbackOutcome`] describing the
    ///    restored checkpoint time, the corrected re-advance time,
    ///    and the new run-until report for the scheduler to consume.
    ///
    /// The scheduler is responsible for translating
    /// [`RollbackOutcome::event`] into a
    /// [`circuit_solver_types::SchedulerMetadata::rollbacks`] entry
    /// and committing the boundary.
    ///
    /// # Arguments
    ///
    /// - `analog` — the analog solver to drive (rollback + re-advance).
    /// - `mispredicted_at` — the original predicted boundary that
    ///   the digital simulator refuted.
    /// - `corrected` — the corrected event time the digital simulator
    ///   reported (must be `<= mispredicted_at` per ADR-0004's
    ///   misprediction semantics; `corrected == mispredicted_at` is
    ///   accepted as a degenerate edge case).
    /// - `reason` — human-readable label that ends up in the
    ///   [`RollbackEvent::reason`] field (e.g.,
    ///   `"no-event-confirmed"`, `"contract-violation"`,
    ///   `"postponed"`).
    ///
    /// # Errors
    ///
    /// - [`SchedulerError::NoCheckpoint`] if the manager has no
    ///   checkpoint at or before `corrected`.
    /// - Any [`SchedulerError`] returned by
    ///   [`AnalogSolver::rollback_to`] or
    ///   [`AnalogSolver::run_until`] is propagated verbatim.
    pub fn rollback_to<A>(
        &mut self,
        analog: &mut A,
        mispredicted_at: SimulationTime,
        corrected: SimulationTime,
        reason: &str,
    ) -> Result<RollbackOutcome, SchedulerError>
    where
        A: AnalogSolver + ?Sized,
    {
        // 1. Find the nearest checkpoint at or before `corrected`.
        let checkpoint_time = match self.manager.restore_at_or_before(corrected) {
            Ok(c) => c.time(),
            Err(CheckpointError::NoCheckpointAtOrBefore(t)) => {
                return Err(SchedulerError::NoCheckpoint(t));
            }
            // `restore_at_or_before` only returns NoCheckpointAtOrBefore
            // among `CheckpointError`'s variants; the other variants
            // come from save() and cannot reach this point.
            Err(other) => {
                return Err(SchedulerError::AnalogSolveFailed(format!(
                    "unexpected checkpoint manager failure during rollback: {other}"
                )));
            }
        };

        // 2. Tell the analog solver to restore to that checkpoint
        //    time. The solver's own state machine performs the
        //    actual hydration; the handler does not reach inside.
        analog.rollback_to(checkpoint_time)?;

        // 3. Drop every checkpoint strictly after the restored time
        //    so the monotonic-save invariant is preserved when the
        //    solver next saves at the corrected boundary.
        let pruned = self.manager.drop_after(checkpoint_time);

        // 4. Re-issue run-until to the corrected event time.
        let report = analog.run_until(corrected)?;

        // 5. Record the new checkpoint payload, if any. A
        //    NonMonotonicSave here would indicate a solver bug
        //    (returning a checkpoint at an earlier time than the
        //    restored one) — lift it into SchedulerError for the
        //    scheduler to surface.
        if let Err(err) = self.observe_step(&report) {
            return Err(SchedulerError::AnalogSolveFailed(format!(
                "rollback re-advance produced non-monotonic checkpoint: {err}"
            )));
        }

        // 6. Assemble the outcome.
        Ok(RollbackOutcome {
            event: RollbackEvent {
                mispredicted_at,
                corrected_to: corrected,
                checkpoint_at: checkpoint_time,
                reason: reason.to_string(),
            },
            re_advance: report,
            pruned_checkpoints: pruned,
        })
    }
}

// ---------------------------------------------------------------------------
// RollbackOutcome
// ---------------------------------------------------------------------------

/// The result of a [`RollbackHandler::rollback_to`] call.
///
/// The scheduler consumes this to:
///
/// - push [`RollbackOutcome::event`] into
///   [`circuit_solver_types::SchedulerMetadata::rollbacks`],
/// - inspect [`RollbackOutcome::re_advance`] to drive whatever
///   per-boundary commit bookkeeping it would normally do after a
///   plain `run_until`,
/// - and (in diagnostics) report
///   [`RollbackOutcome::pruned_checkpoints`] for visibility.
#[derive(Debug, Clone, PartialEq)]
pub struct RollbackOutcome {
    /// Event record for the scheduler's audit trail.
    pub event: RollbackEvent,
    /// The analog solver's report from the re-advance to the
    /// corrected event time.
    pub re_advance: AnalogStepReport,
    /// Number of post-rollback checkpoints discarded from the
    /// manager (those whose `time` was strictly greater than the
    /// restored checkpoint's time). Useful for diagnostic logs.
    pub pruned_checkpoints: usize,
}

// ---------------------------------------------------------------------------
// Tests — in-crate, fast, deterministic.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checkpoint::SparseCheckpoint;
    use circuit_solver_types::NodeId;

    fn t_ns(ns: i64) -> SimulationTime {
        SimulationTime::from_nanoseconds(ns)
    }

    fn checkpoint_at(time: SimulationTime, marker: f64) -> SparseCheckpoint {
        SparseCheckpoint::empty(time).with_node_voltages(vec![(NodeId::new(1), marker)])
    }

    /// Minimal `AnalogSolver` double for the rollback module's own
    /// tests. Records every `run_until` / `rollback_to` call and
    /// emits whichever `AnalogStepReport` the test scripted.
    struct ScriptedSolver {
        calls: Vec<Call>,
        scripted_reports: std::collections::VecDeque<AnalogStepReport>,
        rollback_should_fail: Option<SchedulerError>,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Call {
        RunUntil(SimulationTime),
        RollbackTo(SimulationTime),
    }

    impl ScriptedSolver {
        fn new(reports: impl IntoIterator<Item = AnalogStepReport>) -> Self {
            Self {
                calls: Vec::new(),
                scripted_reports: reports.into_iter().collect(),
                rollback_should_fail: None,
            }
        }
    }

    impl AnalogSolver for ScriptedSolver {
        fn run_until(
            &mut self,
            target: SimulationTime,
        ) -> Result<AnalogStepReport, SchedulerError> {
            self.calls.push(Call::RunUntil(target));
            self.scripted_reports.pop_front().ok_or_else(|| {
                SchedulerError::AnalogSolveFailed("scripted solver exhausted".into())
            })
        }

        fn rollback_to(&mut self, target: SimulationTime) -> Result<(), SchedulerError> {
            self.calls.push(Call::RollbackTo(target));
            if let Some(err) = self.rollback_should_fail.take() {
                return Err(err);
            }
            Ok(())
        }

        fn take_trace(&mut self) -> circuit_solver_types::AnalogTrace {
            circuit_solver_types::AnalogTrace::default()
        }
    }

    // ---- observe_step --------------------------------------------------

    #[test]
    fn observe_step_records_checkpoint_payload() {
        let mut handler = RollbackHandler::new();
        let report = AnalogStepReport::with_checkpoint(t_ns(50), checkpoint_at(t_ns(50), 1.0));
        handler.observe_step(&report).unwrap();
        assert_eq!(handler.checkpoint_count(), 1);
        assert_eq!(handler.manager().latest_time(), Some(t_ns(50)));
    }

    #[test]
    fn observe_step_with_no_checkpoint_payload_is_noop() {
        // saved_at sets `checkpoint = None`. The handler must not
        // panic and must leave the manager untouched.
        let mut handler = RollbackHandler::new();
        let report = AnalogStepReport::saved_at(t_ns(50));
        handler.observe_step(&report).unwrap();
        assert_eq!(handler.checkpoint_count(), 0);
    }

    #[test]
    fn observe_step_propagates_non_monotonic_save_error() {
        let mut handler = RollbackHandler::new();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(100),
                checkpoint_at(t_ns(100), 1.0),
            ))
            .unwrap();
        let err = handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(50),
                checkpoint_at(t_ns(50), 2.0),
            ))
            .unwrap_err();
        match err {
            CheckpointError::NonMonotonicSave { latest, attempted } => {
                assert_eq!(latest, t_ns(100));
                assert_eq!(attempted, t_ns(50));
            }
            CheckpointError::NoCheckpointAtOrBefore(_) => {
                panic!("expected NonMonotonicSave, got NoCheckpointAtOrBefore")
            }
        }
    }

    // ---- rollback_to: happy path ---------------------------------------

    #[test]
    fn rollback_finds_nearest_before_and_re_advances() {
        // Manager holds checkpoints at 50 ns and 100 ns. The
        // misprediction was at 100 ns, the corrected event is at
        // 80 ns. The nearest-before checkpoint is 50 ns.
        let mut handler = RollbackHandler::new();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(50),
                checkpoint_at(t_ns(50), 1.0),
            ))
            .unwrap();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(100),
                checkpoint_at(t_ns(100), 2.0),
            ))
            .unwrap();
        assert_eq!(handler.checkpoint_count(), 2);

        let re_advance_report =
            AnalogStepReport::with_checkpoint(t_ns(80), checkpoint_at(t_ns(80), 1.5));
        let mut analog = ScriptedSolver::new([re_advance_report.clone()]);

        let outcome = handler
            .rollback_to(&mut analog, t_ns(100), t_ns(80), "no-event-confirmed")
            .expect("rollback must succeed");

        // Event record carries the right (mispredicted, corrected,
        // checkpoint, reason) tuple.
        assert_eq!(outcome.event.mispredicted_at, t_ns(100));
        assert_eq!(outcome.event.corrected_to, t_ns(80));
        assert_eq!(outcome.event.checkpoint_at, t_ns(50));
        assert_eq!(outcome.event.reason, "no-event-confirmed");

        // Solver received exactly: rollback_to(50), run_until(80).
        assert_eq!(
            analog.calls,
            vec![Call::RollbackTo(t_ns(50)), Call::RunUntil(t_ns(80)),],
        );

        // The 100 ns checkpoint was pruned; the 80 ns one was added.
        assert_eq!(outcome.pruned_checkpoints, 1);
        assert_eq!(handler.checkpoint_count(), 2); // 50, 80
        assert_eq!(handler.manager().latest_time(), Some(t_ns(80)));

        // Re-advance report is propagated.
        assert_eq!(outcome.re_advance, re_advance_report);
    }

    #[test]
    fn rollback_to_exact_checkpoint_time_retains_it() {
        // Corrected == an existing checkpoint time: rollback_to
        // must target that exact checkpoint and re-advance to it
        // (degenerate but legal — the digital adapter may report the
        // event at exactly a stored boundary).
        let mut handler = RollbackHandler::new();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(50),
                checkpoint_at(t_ns(50), 1.0),
            ))
            .unwrap();

        let mut analog = ScriptedSolver::new([AnalogStepReport::with_checkpoint(
            t_ns(50),
            checkpoint_at(t_ns(50), 1.5),
        )]);

        let outcome = handler
            .rollback_to(&mut analog, t_ns(100), t_ns(50), "contract-violation")
            .unwrap();

        assert_eq!(outcome.event.checkpoint_at, t_ns(50));
        assert_eq!(outcome.event.corrected_to, t_ns(50));
        // 50 ns checkpoint stays (drop_after keeps the one at target).
        // The re-advance adds a new equal-time checkpoint, which
        // is allowed per checkpoint-manager monotonicity (>= latest).
        assert_eq!(handler.checkpoint_count(), 2);
    }

    // ---- rollback_to: error paths --------------------------------------

    #[test]
    fn rollback_with_empty_manager_returns_no_checkpoint() {
        let mut handler = RollbackHandler::new();
        let mut analog = ScriptedSolver::new([AnalogStepReport::with_checkpoint(
            t_ns(80),
            checkpoint_at(t_ns(80), 1.5),
        )]);
        let err = handler
            .rollback_to(&mut analog, t_ns(100), t_ns(80), "no-event-confirmed")
            .unwrap_err();
        assert_eq!(err, SchedulerError::NoCheckpoint(t_ns(80)));
        // Solver must NOT have been touched on the no-checkpoint path.
        assert!(analog.calls.is_empty());
    }

    #[test]
    fn rollback_when_only_checkpoints_are_after_target_returns_no_checkpoint() {
        let mut handler = RollbackHandler::new();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(100),
                checkpoint_at(t_ns(100), 1.0),
            ))
            .unwrap();
        let mut analog = ScriptedSolver::new([]);
        let err = handler
            .rollback_to(&mut analog, t_ns(100), t_ns(80), "no-event-confirmed")
            .unwrap_err();
        // 80 ns < 100 ns and no earlier checkpoint exists.
        assert_eq!(err, SchedulerError::NoCheckpoint(t_ns(80)));
        assert!(analog.calls.is_empty());
    }

    #[test]
    fn rollback_propagates_analog_rollback_failure() {
        let mut handler = RollbackHandler::new();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(50),
                checkpoint_at(t_ns(50), 1.0),
            ))
            .unwrap();
        let mut analog = ScriptedSolver::new([]);
        analog.rollback_should_fail = Some(SchedulerError::AnalogSolveFailed(
            "restore failed in solver".into(),
        ));
        let err = handler
            .rollback_to(&mut analog, t_ns(100), t_ns(80), "no-event-confirmed")
            .unwrap_err();
        match err {
            SchedulerError::AnalogSolveFailed(msg) => assert!(msg.contains("restore failed")),
            other => panic!("expected AnalogSolveFailed, got {other:?}"),
        }
        // run_until must NOT have been called after rollback failure.
        assert_eq!(analog.calls, vec![Call::RollbackTo(t_ns(50))]);
    }

    #[test]
    fn rollback_propagates_analog_re_advance_failure() {
        let mut handler = RollbackHandler::new();
        handler
            .observe_step(&AnalogStepReport::with_checkpoint(
                t_ns(50),
                checkpoint_at(t_ns(50), 1.0),
            ))
            .unwrap();
        // No scripted report -> run_until returns Err.
        let mut analog = ScriptedSolver::new([]);
        let err = handler
            .rollback_to(&mut analog, t_ns(100), t_ns(80), "no-event-confirmed")
            .unwrap_err();
        match err {
            SchedulerError::AnalogSolveFailed(msg) => assert!(msg.contains("scripted solver")),
            other => panic!("expected AnalogSolveFailed, got {other:?}"),
        }
        assert_eq!(
            analog.calls,
            vec![Call::RollbackTo(t_ns(50)), Call::RunUntil(t_ns(80))],
        );
    }

    #[test]
    fn rollback_prunes_multiple_post_boundary_checkpoints() {
        let mut handler = RollbackHandler::new();
        for (ns, marker) in [(20_i64, 0.5), (50, 1.0), (75, 1.25), (100, 2.0), (120, 2.5)] {
            handler
                .observe_step(&AnalogStepReport::with_checkpoint(
                    t_ns(ns),
                    checkpoint_at(t_ns(ns), marker),
                ))
                .unwrap();
        }
        let mut analog = ScriptedSolver::new([AnalogStepReport::with_checkpoint(
            t_ns(60),
            checkpoint_at(t_ns(60), 1.1),
        )]);
        let outcome = handler
            .rollback_to(&mut analog, t_ns(120), t_ns(60), "no-event-confirmed")
            .unwrap();
        // Nearest at-or-before 60 is 50.
        assert_eq!(outcome.event.checkpoint_at, t_ns(50));
        // Removed: 75, 100, 120 (3 checkpoints strictly > 50).
        assert_eq!(outcome.pruned_checkpoints, 3);
        // Remaining: 20, 50, plus the new 60 from re-advance.
        let times: Vec<_> = handler
            .manager()
            .as_slice()
            .iter()
            .map(SparseCheckpoint::time)
            .collect();
        assert_eq!(times, vec![t_ns(20), t_ns(50), t_ns(60)]);
    }
}
