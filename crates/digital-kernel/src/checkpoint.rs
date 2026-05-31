//! Checkpoint manager for optimistic rollback of the digital kernel.
//!
//! ADR-0006 ("Native Event-Driven Digital Engine") mandates that the
//! native digital kernel support checkpoint/restore for the Mixed-Signal
//! Scheduler's optimistic time-advance strategy. When the analog side
//! rolls back past a digital event that was optimistically processed, the
//! kernel must restore its event queue and net state to a previously
//! captured checkpoint.
//!
//! # Design
//!
//! [`DigitalCheckpointManager`] stores multiple [`TimedKernelCheckpoint`]s
//! keyed by simulation time. It mirrors the analysis-orchestration crate's
//! `SparseCheckpointManager` API shape, adapted for digital-kernel state:
//!
//! - **Monotonic saves**: checkpoints are stored in non-decreasing time
//!   order, matching the scheduler's forward-only advance model.
//! - **Nearest-before restore**: `restore_at_or_before(target)` finds the
//!   latest checkpoint whose time is `<= target`.
//! - **Pruning**: `drop_after` and `drop_before` remove checkpoints that
//!   are no longer needed (post-rollback invalidation or commit cleanup).
//!
//! # Integration with `DigitalKernel`
//!
//! The scheduler calls `kernel.checkpoint()` to snapshot state, then
//! `manager.save(timed_checkpoint)` to record it. On rollback, it calls
//! `manager.restore_at_or_before(target)` to find the right checkpoint,
//! then `kernel.restore_from_checkpoint(cp)` to apply it.

use circuit_solver_types::SimulationTime;
use core::fmt;

use crate::kernel::KernelCheckpoint;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Failures surfaced by [`DigitalCheckpointManager`].
///
/// These mirror the error variants in the orchestration crate's
/// `CheckpointError`, ensuring a consistent error shape across both
/// the analog and digital checkpoint managers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitalCheckpointError {
    /// A `restore_at_or_before(target)` call found no checkpoint whose
    /// time is `<= target`. The contained value is the requested `target`.
    NoCheckpointAtOrBefore(SimulationTime),
    /// A `save` call provided a time strictly less than the time of
    /// the latest existing checkpoint. The manager refuses this because
    /// the rollback contract requires the saved sequence to be
    /// monotonically non-decreasing in time. Contained values are
    /// `(latest, attempted)`.
    NonMonotonicSave {
        /// The time of the most recently saved checkpoint.
        latest: SimulationTime,
        /// The time at which a new save was attempted.
        attempted: SimulationTime,
    },
}

impl fmt::Display for DigitalCheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCheckpointAtOrBefore(t) => {
                write!(f, "no checkpoint at or before {t}")
            }
            Self::NonMonotonicSave { latest, attempted } => {
                write!(
                    f,
                    "non-monotonic save: latest checkpoint at {latest}, \
                     attempted save at {attempted}"
                )
            }
        }
    }
}

impl std::error::Error for DigitalCheckpointError {}

// ---------------------------------------------------------------------------
// TimedKernelCheckpoint
// ---------------------------------------------------------------------------

/// A [`KernelCheckpoint`] tagged with the simulation time at which it
/// was captured.
///
/// The `time` field is the kernel's simulation clock at the moment of
/// checkpoint capture. This is the key used by [`DigitalCheckpointManager`]
/// for lookup and pruning.
#[derive(Debug, Clone, PartialEq)]
pub struct TimedKernelCheckpoint {
    /// The simulation time at which this checkpoint was captured.
    pub time: SimulationTime,
    /// The captured kernel state (event queue + net state).
    pub checkpoint: KernelCheckpoint,
}

// ---------------------------------------------------------------------------
// DigitalCheckpointManager
// ---------------------------------------------------------------------------

/// Ordered, time-keyed store of [`TimedKernelCheckpoint`] snapshots for
/// the native digital kernel's optimistic-rollback mechanism.
///
/// The manager preserves the invariant that **stored checkpoints are
/// monotonically non-decreasing in `time`**. This matches ADR-0004/0006's
/// optimistic-advance model: the scheduler always advances the kernel
/// *forward* to the next predicted digital event and saves a checkpoint
/// *at* that boundary. Rollback reduces the manager (via
/// [`DigitalCheckpointManager::drop_after`]) but never inserts in the
/// middle.
///
/// # Memory model
///
/// Per ADR-0004, sparse-checkpoint memory scales with the digital event
/// rate. This implementation places no automatic cap on retained
/// checkpoints; the scheduler decides when to drop stale entries via
/// [`DigitalCheckpointManager::drop_before`] — commits upstream of the
/// current advance are no longer rollback targets and may be released.
///
/// # Concurrency
///
/// `DigitalCheckpointManager` is `!Sync` by default (it contains owned
/// `Vec`s). The Mixed-Signal Scheduler owns it singly; no cross-thread
/// sharing is required at v1.
#[derive(Debug, Default, Clone)]
pub struct DigitalCheckpointManager {
    /// Stored checkpoints, monotonically non-decreasing in `time`.
    checkpoints: Vec<TimedKernelCheckpoint>,
}

impl DigitalCheckpointManager {
    /// Construct an empty manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
        }
    }

    /// Insert a timed checkpoint. The new checkpoint's `time` must be
    /// greater than or equal to the latest stored time.
    ///
    /// # Errors
    ///
    /// Returns [`DigitalCheckpointError::NonMonotonicSave`] if the
    /// provided checkpoint's `time` is strictly less than the time of
    /// the currently latest stored checkpoint.
    pub fn save(&mut self, cp: TimedKernelCheckpoint) -> Result<(), DigitalCheckpointError> {
        if let Some(latest) = self.latest_time() {
            if cp.time < latest {
                return Err(DigitalCheckpointError::NonMonotonicSave {
                    latest,
                    attempted: cp.time,
                });
            }
        }
        self.checkpoints.push(cp);
        Ok(())
    }

    /// Return a reference to the most recent checkpoint whose `time`
    /// is `<= target`.
    ///
    /// On the correct-prediction path the scheduler queries this at
    /// exactly a stored boundary (`target == saved_time`); on the
    /// rollback path the query may target a strictly-earlier event
    /// time and must return the largest stored time `<= target`.
    ///
    /// # Errors
    ///
    /// Returns [`DigitalCheckpointError::NoCheckpointAtOrBefore`] if no
    /// stored checkpoint satisfies the bound (typically: the manager
    /// is empty, or all stored times are `> target`).
    pub fn restore_at_or_before(
        &self,
        target: SimulationTime,
    ) -> Result<&TimedKernelCheckpoint, DigitalCheckpointError> {
        // Walk backwards; the vector is sorted ascending, so the
        // last element with time <= target is the answer.
        self.checkpoints
            .iter()
            .rev()
            .find(|c| c.time <= target)
            .ok_or(DigitalCheckpointError::NoCheckpointAtOrBefore(target))
    }

    /// Drop every checkpoint whose `time` is **strictly greater**
    /// than `target`. The checkpoint *at* `target`, if any, is
    /// retained; this matches the rollback semantics where the
    /// kernel returns to the state saved *at* the rollback time and
    /// resumes forward from there.
    ///
    /// Returns the number of checkpoints removed.
    pub fn drop_after(&mut self, target: SimulationTime) -> usize {
        let before = self.checkpoints.len();
        self.checkpoints.retain(|c| c.time <= target);
        before - self.checkpoints.len()
    }

    /// Drop every checkpoint whose `time` is **strictly less**
    /// than `target`. Useful for releasing memory once the scheduler
    /// has committed past a boundary that can no longer be a rollback
    /// target.
    ///
    /// Returns the number of checkpoints removed.
    pub fn drop_before(&mut self, target: SimulationTime) -> usize {
        let before = self.checkpoints.len();
        self.checkpoints.retain(|c| c.time >= target);
        before - self.checkpoints.len()
    }

    /// Number of stored checkpoints.
    #[must_use]
    pub fn len(&self) -> usize {
        self.checkpoints.len()
    }

    /// `true` iff no checkpoints are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.checkpoints.is_empty()
    }

    /// The `time` of the latest stored checkpoint, or `None` if empty.
    #[must_use]
    pub fn latest_time(&self) -> Option<SimulationTime> {
        self.checkpoints.last().map(|c| c.time)
    }

    /// The `time` of the earliest stored checkpoint, or `None` if empty.
    #[must_use]
    pub fn earliest_time(&self) -> Option<SimulationTime> {
        self.checkpoints.first().map(|c| c.time)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use circuit_solver_types::SimulationTime;

    use super::*;
    use crate::event_queue::{DigitalEvent, LogicValue, NetId};
    use crate::kernel::DigitalKernel;

    /// Helper: build a TimedKernelCheckpoint from a kernel at its current time.
    fn checkpoint_kernel(kernel: &DigitalKernel) -> TimedKernelCheckpoint {
        let time = kernel.current_time();
        TimedKernelCheckpoint {
            time,
            checkpoint: kernel.checkpoint(),
        }
    }

    /// Helper: simulation time from nanoseconds.
    fn ns(n: i64) -> SimulationTime {
        SimulationTime::from_nanoseconds(n)
    }

    // -- Basic construction tests --

    #[test]
    fn new_manager_is_empty() {
        let mgr = DigitalCheckpointManager::new();
        assert!(mgr.is_empty());
        assert_eq!(mgr.len(), 0);
        assert_eq!(mgr.latest_time(), None);
        assert_eq!(mgr.earliest_time(), None);
    }

    #[test]
    fn default_is_same_as_new() {
        let mgr = DigitalCheckpointManager::default();
        assert!(mgr.is_empty());
    }

    // -- Save tests --

    #[test]
    fn save_single_checkpoint() {
        let mut mgr = DigitalCheckpointManager::new();
        let k = DigitalKernel::new();
        let cp = checkpoint_kernel(&k);
        assert!(mgr.save(cp).is_ok());
        assert_eq!(mgr.len(), 1);
        assert_eq!(mgr.earliest_time(), Some(SimulationTime::ZERO));
        assert_eq!(mgr.latest_time(), Some(SimulationTime::ZERO));
    }

    #[test]
    fn save_monotonic_checkpoints_succeeds() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(3);

        // Schedule events so we can advance through meaningful state.
        k.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(ns(100), NetId::new(1), LogicValue::One))
            .unwrap();

        // Checkpoint at t=0
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Advance and checkpoint at t=50
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Advance and checkpoint at t=100
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        assert_eq!(mgr.len(), 3);
        assert_eq!(mgr.earliest_time(), Some(SimulationTime::ZERO));
        assert_eq!(mgr.latest_time(), Some(ns(100)));
    }

    #[test]
    fn save_at_same_time_as_latest_succeeds() {
        let mut mgr = DigitalCheckpointManager::new();
        let k = DigitalKernel::new();

        // Save at t=0 twice — allowed (non-decreasing, not strictly increasing)
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());
        assert_eq!(mgr.len(), 2);
    }

    #[test]
    fn save_non_monotonic_errors() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        // Save at t=0
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Advance to t=100 and save
        k.schedule(DigitalEvent::new(ns(100), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Now create a checkpoint at t=50 (would be out of order)
        let k2 = DigitalKernel::with_nets(2);
        let cp_50 = {
            let mut k2 = k2;
            k2.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
                .unwrap();
            let _ = k2.run_until(ns(50));
            checkpoint_kernel(&k2)
        };

        let result = mgr.save(cp_50);
        assert!(matches!(
            result,
            Err(DigitalCheckpointError::NonMonotonicSave {
                latest,
                attempted
            }) if latest == ns(100) && attempted == ns(50)
        ));
        // The invalid checkpoint was NOT stored.
        assert_eq!(mgr.len(), 2);
    }

    // -- Restore tests --

    #[test]
    fn restore_at_or_before_finds_exact_match() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        // Save at t=0 and t=50
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());
        k.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        let found = mgr.restore_at_or_before(ns(50)).expect("should find checkpoint");
        assert_eq!(found.time, ns(50));
    }

    #[test]
    fn restore_at_or_before_finds_nearest_before() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        // Save at t=0 and t=100
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());
        k.schedule(DigitalEvent::new(ns(100), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Query for t=50 → should find t=0 (the latest <= 50)
        let found = mgr.restore_at_or_before(ns(50)).expect("should find checkpoint");
        assert_eq!(found.time, SimulationTime::ZERO);
    }

    #[test]
    fn restore_at_or_before_empty_manager_errors() {
        let mgr = DigitalCheckpointManager::new();
        let result = mgr.restore_at_or_before(ns(50));
        assert!(matches!(
            result,
            Err(DigitalCheckpointError::NoCheckpointAtOrBefore(t))
            if t == ns(50)
        ));
    }

    #[test]
    fn restore_at_or_before_all_after_target_errors() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        k.schedule(DigitalEvent::new(ns(100), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Query for t=50 → no checkpoint at or before 50
        let result = mgr.restore_at_or_before(ns(50));
        assert!(matches!(
            result,
            Err(DigitalCheckpointError::NoCheckpointAtOrBefore(t))
            if t == ns(50)
        ));
    }

    // -- Drop tests --

    #[test]
    fn drop_after_removes_later_checkpoints() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=0
        k.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=50
        k.schedule(DigitalEvent::new(ns(100), NetId::new(1), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=100

        assert_eq!(mgr.drop_after(ns(50)), 1); // drops t=100
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.latest_time(), Some(ns(50)));
    }

    #[test]
    fn drop_after_retains_checkpoint_at_target() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=0
        k.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=50

        assert_eq!(mgr.drop_after(ns(50)), 0); // nothing to drop
        assert_eq!(mgr.len(), 2);
    }

    #[test]
    fn drop_before_removes_earlier_checkpoints() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=0
        k.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=50
        k.schedule(DigitalEvent::new(ns(100), NetId::new(1), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=100

        assert_eq!(mgr.drop_before(ns(50)), 1); // drops t=0
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.earliest_time(), Some(ns(50)));
    }

    #[test]
    fn drop_before_retains_checkpoint_at_target() {
        let mut mgr = DigitalCheckpointManager::new();
        let mut k = DigitalKernel::with_nets(2);

        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=0
        k.schedule(DigitalEvent::new(ns(50), NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok()); // t=50

        assert_eq!(mgr.drop_before(ns(50)), 1); // drops t=0
        assert_eq!(mgr.len(), 1);
        assert_eq!(mgr.earliest_time(), Some(ns(50)));
    }

    // -- Optimistic rollback scenario test --
    // This directly verifies the gherkin scenario:
    //   Given the native digital kernel under the Mixed-Signal Scheduler
    //   When the analog side rolls back past a digital event that was
    //     optimistically processed
    //   Then the kernel restores its event queue and net state to the
    //     checkpoint, consistent with the superseding sync ADR

    #[test]
    fn optimistic_rollback_restores_event_queue_and_net_state() {
        // Set up a kernel with 3 nets.
        let mut k = DigitalKernel::with_nets(3);
        let net_a = NetId::new(0);
        let net_b = NetId::new(1);
        let net_c = NetId::new(2);

        let mut mgr = DigitalCheckpointManager::new();

        // -- Advance to t=50 and save checkpoint --
        // Schedule an event to advance time, then schedule future events.
        k.schedule(DigitalEvent::new(ns(50), net_a, LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        // At t=50, net_a = One (just processed).
        assert_eq!(k.net_value(net_a), LogicValue::One);

        // Schedule events that will fire after the checkpoint.
        k.schedule(DigitalEvent::new(ns(75), net_b, LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(ns(100), net_c, LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(ns(150), net_a, LogicValue::Zero))
            .unwrap();

        // Save checkpoint at t=50.
        let cp_50 = checkpoint_kernel(&k);
        assert_eq!(cp_50.time, ns(50));
        assert!(mgr.save(cp_50).is_ok());

        // -- Optimistically advance to t=100 (process events at 75 and 100) --
        let report = k.run_until(ns(100));
        assert_eq!(report.time_reached, ns(100));
        // The event at t=75 changed net_b to One.
        assert_eq!(k.net_value(net_b), LogicValue::One);
        // The event at t=100 changed net_c to One.
        assert_eq!(k.net_value(net_c), LogicValue::One);
        // net_a is still One (the event at t=150 hasn't fired yet).
        assert_eq!(k.net_value(net_a), LogicValue::One);

        // Save checkpoint at t=100.
        let cp_100 = checkpoint_kernel(&k);
        assert!(mgr.save(cp_100).is_ok());

        // -- Simulate rollback: analog side rolls back to t=50 --
        // The analog solver discovered a misprediction and needs to
        // go back before the digital event at t=75.
        let cp_data = {
            let found = mgr.restore_at_or_before(ns(50)).expect("checkpoint at t=50");
            assert_eq!(found.time, ns(50));
            found.checkpoint.clone()
        }; // immutable borrow on mgr ends here

        // Drop checkpoints after the rollback target (they are now invalid).
        let dropped = mgr.drop_after(ns(50));
        assert_eq!(dropped, 1); // The t=100 checkpoint is dropped

        // Restore the kernel to the t=50 checkpoint.
        k.restore_from_checkpoint(cp_data);

        // -- Verify the kernel state is exactly as it was at t=50 --
        assert_eq!(k.current_time(), ns(50));
        // net_a should still be One (it was set at t=50 before checkpoint).
        assert_eq!(k.net_value(net_a), LogicValue::One);
        // net_b and net_c should be Unknown (events at t=75 and t=100 undone).
        assert_eq!(k.net_value(net_b), LogicValue::Unknown);
        assert_eq!(k.net_value(net_c), LogicValue::Unknown);

        // The pending events at t=75, t=100, and t=150 should be back
        // in the queue (they were captured in the checkpoint).
        assert_eq!(k.pending_event_count(), 3);
        assert_eq!(k.next_event_time(), Some(ns(75)));

        // -- Re-advance from t=50 with corrected analog inputs --
        let report2 = k.run_until(ns(100));
        assert_eq!(report2.time_reached, ns(100));
        assert_eq!(k.net_value(net_b), LogicValue::One);
        assert_eq!(k.net_value(net_c), LogicValue::One);
    }

    #[test]
    fn rollback_with_multiple_checkpoints_finds_nearest_before() {
        let mut k = DigitalKernel::with_nets(2);
        let net_a = NetId::new(0);
        let mut mgr = DigitalCheckpointManager::new();

        // Checkpoint at t=0
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Schedule events at t=25 and t=50, advance to t=25, checkpoint
        k.schedule(DigitalEvent::new(ns(25), net_a, LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(ns(50), net_a, LogicValue::Zero))
            .unwrap();
        let _ = k.run_until(ns(25));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());
        // At t=25: net_a=One, t=50 event still pending

        // Advance past t=50 to t=75, checkpoint
        let _ = k.run_until(ns(75));
        assert_eq!(k.net_value(net_a), LogicValue::Zero);
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Roll back to t=30 — nearest checkpoint is at t=25
        let cp_data = {
            let found = mgr.restore_at_or_before(ns(30)).expect("should find t=25");
            assert_eq!(found.time, ns(25));
            found.checkpoint.clone()
        };

        // Restore kernel
        k.restore_from_checkpoint(cp_data);
        assert_eq!(k.current_time(), ns(25));
        // At t=25, net_a was One.
        assert_eq!(k.net_value(net_a), LogicValue::One);
        // The event at t=50 is back in the queue (it was in the
        // pending queue when the t=25 checkpoint was captured).
        assert_eq!(k.pending_event_count(), 1);
        assert_eq!(k.next_event_time(), Some(ns(50)));
    }

    #[test]
    fn rollback_then_commit_drops_stale_checkpoints() {
        let mut k = DigitalKernel::with_nets(2);
        let net_a = NetId::new(0);
        let mut mgr = DigitalCheckpointManager::new();

        // Checkpoint at t=0
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Advance and checkpoint at t=50
        k.schedule(DigitalEvent::new(ns(50), net_a, LogicValue::One))
            .unwrap();
        let _ = k.run_until(ns(50));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // Advance and checkpoint at t=100
        k.schedule(DigitalEvent::new(ns(100), net_a, LogicValue::Zero))
            .unwrap();
        let _ = k.run_until(ns(100));
        assert!(mgr.save(checkpoint_kernel(&k)).is_ok());

        // After a commit (scheduler confirms no rollback needed before t=50),
        // drop checkpoints before t=50 to release memory.
        assert_eq!(mgr.drop_before(ns(50)), 1); // drops t=0
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.earliest_time(), Some(ns(50)));
    }

    #[test]
    fn error_display_formatting() {
        let e1 = DigitalCheckpointError::NoCheckpointAtOrBefore(ns(42));
        assert!(format!("{e1}").contains("no checkpoint at or before"));

        let e2 = DigitalCheckpointError::NonMonotonicSave {
            latest: ns(100),
            attempted: ns(50),
        };
        let msg = format!("{e2}");
        assert!(msg.contains("non-monotonic save"));
        assert!(msg.contains("100"));
        assert!(msg.contains("50"));
    }
}
