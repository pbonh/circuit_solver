//! Optimistic rollback checkpoint manager for the native digital kernel.
//!
//! ADR-0006 ("Native Event-Driven Digital Engine") retains the optimistic
//! checkpoint/rollback mechanism from ADR-0004, now applied to the native
//! kernel's event queue and net state rather than an external simulator.
//!
//! # Design
//!
//! The [`CheckpointManager`] maintains an ordered stack of timestamped
//! [`KernelCheckpoint`] snapshots. The Mixed-Signal Scheduler:
//!
//! 1. **Takes a checkpoint** before crossing each predicted digital event
//!    boundary (via [`CheckpointManager::take_checkpoint`]).
//! 2. **Advances** the kernel optimistically via `run_until`.
//! 3. **Rolls back** if the prediction was wrong (via
//!    [`CheckpointManager::rollback_to`]), restoring the kernel to the
//!    latest checkpoint at or before the corrected boundary.
//! 4. **Confirms** a time point once the analog solver has committed past
//!    it (via [`CheckpointManager::confirm`]), enabling pruning of
//!    checkpoints that are no longer needed.
//!
//! # Checkpoint lifecycle
//!
//! ```text
//! take_checkpoint  →  [cp0, cp1, cp2, ...]  ← newest last
//!                         ↑
//!                    rollback_to finds
//!                    latest cp at or before
//!                    the requested time
//!
//! confirm(t)  →  checkpoints before t are
//!                 eligible for pruning
//! prune       →  discard prunable checkpoints
//! ```
//!
//! # Trace handling on rollback
//!
//! When a checkpoint is restored, the kernel's processed-events buffer
//! is replaced with the checkpoint's snapshot — discarding any events
//! that were processed after the checkpoint time. Subsequent `run_until`
//! calls will re-accumulate events from the restored state, producing
//! a correct trace.

use circuit_solver_types::SimulationTime;

use crate::kernel::{DigitalKernel, KernelCheckpoint};

// ---------------------------------------------------------------------------
// Timestamped checkpoint
// ---------------------------------------------------------------------------

/// A [`KernelCheckpoint`] tagged with the simulation time at which it
/// was captured.
///
/// The time tag enables the [`CheckpointManager`] to locate the correct
/// checkpoint for rollback-to-time operations and to determine which
/// checkpoints can be pruned after confirmation.
#[derive(Debug, Clone, PartialEq)]
pub struct TimestampedCheckpoint {
    /// The simulation time at which this checkpoint was taken.
    pub time: SimulationTime,
    /// The kernel state snapshot.
    pub data: KernelCheckpoint,
}

// ---------------------------------------------------------------------------
// Checkpoint manager errors
// ---------------------------------------------------------------------------

/// Errors from the [`CheckpointManager`].
#[derive(Debug, Clone, PartialEq)]
pub enum CheckpointError {
    /// A rollback was requested but no checkpoint exists at or before
    /// the requested time.
    NoCheckpointAvailable {
        /// The requested rollback target time.
        requested_time: SimulationTime,
    },
}

impl core::fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoCheckpointAvailable { requested_time } => {
                write!(
                    f,
                    "no checkpoint available at or before time {requested_time}"
                )
            }
        }
    }
}

impl std::error::Error for CheckpointError {}

// ---------------------------------------------------------------------------
// Checkpoint manager
// ---------------------------------------------------------------------------

/// Manages the optimistic rollback checkpoint lifecycle for a
/// [`DigitalKernel`].
///
/// The manager maintains an ordered stack of [`TimestampedCheckpoint`]s
/// and exposes operations for the Mixed-Signal Scheduler's optimistic
/// time-advance protocol (ADR-0004, retained under ADR-0006):
///
/// - **Take checkpoint** before crossing a predicted digital event
///   boundary.
/// - **Roll back** to the latest checkpoint at or before a corrected
///   boundary time.
/// - **Confirm** a time point once the analog solver has committed past
///   it, enabling pruning of earlier checkpoints.
///
/// # Invariants
///
/// - Checkpoints are stored in non-decreasing time order.
/// - `confirmed_time` is monotonically non-decreasing.
/// - No checkpoint with `time > confirmed_time` is pruned.
/// - After rollback, the kernel is at the checkpoint's time with the
///   checkpoint's full state restored.
#[derive(Debug, Clone)]
pub struct CheckpointManager {
    /// Ordered stack of checkpoints, newest last. All entries have
    /// `time >= confirmed_time`.
    checkpoints: Vec<TimestampedCheckpoint>,
    /// The latest confirmed simulation time. Checkpoints before this
    /// time are eligible for pruning.
    confirmed_time: SimulationTime,
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckpointManager {
    /// Create a new checkpoint manager with no checkpoints and
    /// `confirmed_time = 0`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            confirmed_time: SimulationTime::ZERO,
        }
    }

    /// The latest confirmed simulation time.
    #[must_use]
    pub fn confirmed_time(&self) -> SimulationTime {
        self.confirmed_time
    }

    /// Number of stored checkpoints (including prunable ones not yet
    /// pruned).
    #[must_use]
    pub fn checkpoint_count(&self) -> usize {
        self.checkpoints.len()
    }

    /// The simulation time of the most recent checkpoint, or `None` if
    /// no checkpoints exist.
    #[must_use]
    pub fn latest_checkpoint_time(&self) -> Option<SimulationTime> {
        self.checkpoints.last().map(|cp| cp.time)
    }

    /// The simulation time of the earliest checkpoint still retained,
    /// or `None` if no checkpoints exist.
    #[must_use]
    pub fn earliest_checkpoint_time(&self) -> Option<SimulationTime> {
        self.checkpoints.first().map(|cp| cp.time)
    }

    /// Take a checkpoint of the kernel's current state.
    ///
    /// The checkpoint is tagged with `kernel.current_time()` and pushed
    /// onto the stack. The scheduler calls this before crossing each
    /// predicted digital event boundary.
    ///
    /// # Returns
    ///
    /// The simulation time at which the checkpoint was taken (i.e.,
    /// `kernel.current_time()`).
    pub fn take_checkpoint(&mut self, kernel: &DigitalKernel) -> SimulationTime {
        let time = kernel.current_time();
        let data = kernel.checkpoint();
        self.checkpoints.push(TimestampedCheckpoint { time, data });
        time
    }

    /// Roll the kernel back to the latest checkpoint at or before
    /// `target_time`.
    ///
    /// This is the core rollback operation of the optimistic
    /// time-advance protocol. The scheduler calls this when a
    /// misprediction is detected — the kernel is restored to the
    /// last known-good state before the corrected boundary.
    ///
    /// After rollback, the kernel's:
    /// - simulation clock is at the checkpoint's time,
    /// - event queue has the checkpoint's pending events,
    /// - net state has the checkpoint's net values,
    /// - processed-events buffer has the checkpoint's processed events
    ///   (discarding any events processed after the checkpoint).
    ///
    /// The restored checkpoint is **retained** in the stack so that the
    /// scheduler can roll back to it again if needed. Use
    /// [`confirm`] + [`prune`] to discard it once the analog solver
    /// commits past it.
    ///
    /// # Errors
    ///
    /// Returns [`CheckpointError::NoCheckpointAvailable`] if no
    /// checkpoint exists at or before `target_time`.
    ///
    /// [`confirm`]: CheckpointManager::confirm
    /// [`prune`]: CheckpointManager::prune
    pub fn rollback_to(
        &mut self,
        kernel: &mut DigitalKernel,
        target_time: SimulationTime,
    ) -> Result<SimulationTime, CheckpointError> {
        // Find the latest checkpoint at or before target_time.
        // Since checkpoints are in non-decreasing order, search
        // from the end.
        let idx = self
            .checkpoints
            .iter()
            .enumerate()
            .rev()
            .find(|(_, cp)| cp.time <= target_time)
            .map(|(i, _)| i);

        let Some(idx) = idx else {
            return Err(CheckpointError::NoCheckpointAvailable {
                requested_time: target_time,
            });
        };

        // Clone the checkpoint data (we retain the entry in the stack).
        let cp_data = self.checkpoints[idx].data.clone();
        let cp_time = self.checkpoints[idx].time;

        kernel.restore_from_checkpoint(cp_data);

        Ok(cp_time)
    }

    /// Mark a simulation time as confirmed.
    ///
    /// Once the analog solver has committed past time `t`, checkpoints
    /// at or before `t` will never be needed for rollback. Calling
    /// `confirm(t)` updates the confirmed time; a subsequent
    /// [`prune`] will discard those checkpoints.
    ///
    /// `confirm` is monotonically non-decreasing: if `t` is less than
    /// the current confirmed time, the call is a no-op.
    ///
    /// [`prune`]: CheckpointManager::prune
    pub fn confirm(&mut self, t: SimulationTime) {
        if t > self.confirmed_time {
            self.confirmed_time = t;
        }
    }

    /// Discard checkpoints at or before the confirmed time.
    ///
    /// After [`confirm`] is called, checkpoints before the confirmed
    /// boundary are no longer needed for rollback. `prune` removes
    /// them, freeing memory.
    ///
    /// Note: at least one checkpoint **at** the confirmed time is
    /// retained (if it exists) as a boundary anchor — the scheduler
    /// may still need it for a final rollback before fully committing.
    /// All checkpoints **before** the confirmed time are discarded.
    ///
    /// [`confirm`]: CheckpointManager::confirm
    pub fn prune(&mut self) {
        // Retain checkpoints with time > confirmed_time.
        // Also retain any checkpoint with time == confirmed_time
        // (the boundary anchor) — only one such entry is needed.
        let mut seen_confirmed = false;
        self.checkpoints.retain(|cp| {
            if cp.time > self.confirmed_time {
                true
            } else if cp.time == self.confirmed_time && !seen_confirmed {
                seen_confirmed = true;
                true
            } else {
                false
            }
        });
    }

    /// Get a reference to the checkpoint stack (for inspection).
    #[must_use]
    pub fn checkpoints(&self) -> &[TimestampedCheckpoint] {
        &self.checkpoints
    }

    /// Remove all checkpoints and reset confirmed time to zero.
    pub fn reset(&mut self) {
        self.checkpoints.clear();
        self.confirmed_time = SimulationTime::ZERO;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_queue::{DigitalEvent, LogicValue, NetId};

    // -- TimestampedCheckpoint --

    #[test]
    fn timestamped_checkpoint_fields() {
        let mut kernel = DigitalKernel::with_nets(2);
        let t50 = SimulationTime::from_nanoseconds(50);
        kernel
            .schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();

        let data = kernel.checkpoint();
        let ts = TimestampedCheckpoint {
            time: SimulationTime::ZERO,
            data,
        };
        assert_eq!(ts.time, SimulationTime::ZERO);
        // Data is a KernelCheckpoint with 1 pending event.
        assert_eq!(ts.data.queue.pending.len(), 1);
    }

    // -- CheckpointManager basics --

    #[test]
    fn new_manager_is_empty() {
        let mgr = CheckpointManager::new();
        assert_eq!(mgr.checkpoint_count(), 0);
        assert_eq!(mgr.confirmed_time(), SimulationTime::ZERO);
        assert_eq!(mgr.latest_checkpoint_time(), None);
        assert_eq!(mgr.earliest_checkpoint_time(), None);
    }

    #[test]
    fn take_checkpoint_records_time() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(2);
        // Kernel starts at t=0.
        let t0 = mgr.take_checkpoint(&kernel);
        assert_eq!(t0, SimulationTime::ZERO);
        assert_eq!(mgr.checkpoint_count(), 1);
        assert_eq!(mgr.latest_checkpoint_time(), Some(SimulationTime::ZERO));

        // Advance to t=50ns and take another.
        let t50 = SimulationTime::from_nanoseconds(50);
        kernel
            .schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t50);
        let t_cp = mgr.take_checkpoint(&kernel);
        assert_eq!(t_cp, t50);
        assert_eq!(mgr.checkpoint_count(), 2);
        assert_eq!(mgr.latest_checkpoint_time(), Some(t50));
    }

    // -- Rollback --

    #[test]
    fn rollback_to_restores_kernel_state() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(2);
        let net0 = NetId::new(0);
        let net1 = NetId::new(1);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        // Schedule two events and checkpoint before running.
        kernel
            .schedule(DigitalEvent::new(t50, net0, LogicValue::One))
            .unwrap();
        kernel
            .schedule(DigitalEvent::new(t100, net1, LogicValue::Zero))
            .unwrap();
        let cp_time = mgr.take_checkpoint(&kernel);
        assert_eq!(cp_time, SimulationTime::ZERO);

        // Run through both events.
        let _ = kernel.run_until(t100);
        assert_eq!(kernel.net_value(net0), LogicValue::One);
        assert_eq!(kernel.net_value(net1), LogicValue::Zero);
        assert_eq!(kernel.current_time(), t100);

        // Roll back to the checkpoint at t=0.
        let restored = mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();
        assert_eq!(restored, SimulationTime::ZERO);
        assert_eq!(kernel.current_time(), SimulationTime::ZERO);
        assert_eq!(kernel.pending_event_count(), 2);
        assert_eq!(kernel.net_value(net0), LogicValue::Unknown);
        assert_eq!(kernel.net_value(net1), LogicValue::Unknown);
    }

    #[test]
    fn rollback_to_finds_latest_at_or_before_target() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(2);
        let net0 = NetId::new(0);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        // Checkpoint at t=0.
        mgr.take_checkpoint(&kernel);

        // Advance to 50ns, schedule event at 100ns, checkpoint at 50ns.
        kernel
            .schedule(DigitalEvent::new(t50, net0, LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t50);
        mgr.take_checkpoint(&kernel);

        // Advance to 100ns, schedule event at 150ns, checkpoint at 100ns.
        kernel
            .schedule(DigitalEvent::new(t100, net0, LogicValue::Zero))
            .unwrap();
        let _ = kernel.run_until(t100);
        mgr.take_checkpoint(&kernel);

        assert_eq!(mgr.checkpoint_count(), 3);

        // Roll back to t=75ns — should find checkpoint at t=50ns.
        let restored = mgr
            .rollback_to(&mut kernel, SimulationTime::from_nanoseconds(75))
            .unwrap();
        assert_eq!(restored, t50);
        assert_eq!(kernel.current_time(), t50);
    }

    #[test]
    fn rollback_to_exact_time_returns_that_checkpoint() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(1);
        let net0 = NetId::new(0);
        let t50 = SimulationTime::from_nanoseconds(50);

        // Checkpoint at t=0.
        mgr.take_checkpoint(&kernel);

        // Advance and checkpoint at t=50.
        kernel
            .schedule(DigitalEvent::new(t50, net0, LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t50);
        mgr.take_checkpoint(&kernel);

        // Roll back to exactly t=50.
        let restored = mgr.rollback_to(&mut kernel, t50).unwrap();
        assert_eq!(restored, t50);
        assert_eq!(kernel.current_time(), t50);
    }

    #[test]
    fn rollback_with_no_checkpoint_returns_error() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);

        let result = mgr.rollback_to(&mut kernel, t50);
        assert!(matches!(
            result,
            Err(CheckpointError::NoCheckpointAvailable { .. })
        ));
    }

    #[test]
    fn rollback_to_time_before_all_checkpoints_returns_error() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(1);
        let t100 = SimulationTime::from_nanoseconds(100);

        // Advance and checkpoint at t=100.
        kernel
            .schedule(DigitalEvent::new(t100, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t100);
        mgr.take_checkpoint(&kernel);

        // Try to roll back to t=50 (before the only checkpoint at t=100).
        let result = mgr.rollback_to(&mut kernel, SimulationTime::from_nanoseconds(50));
        assert!(matches!(
            result,
            Err(CheckpointError::NoCheckpointAvailable { .. })
        ));
    }

    // -- Rollback preserves checkpoint stack --

    #[test]
    fn rollback_retains_checkpoint_in_stack() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(1);
        mgr.take_checkpoint(&kernel);

        let pre_rollback_count = mgr.checkpoint_count();
        let _ = mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();
        // The checkpoint is still in the stack.
        assert_eq!(mgr.checkpoint_count(), pre_rollback_count);
    }

    #[test]
    fn rollback_twice_to_same_time_works() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(1);
        let net0 = NetId::new(0);
        let t50 = SimulationTime::from_nanoseconds(50);

        // Schedule event FIRST, then checkpoint at t=0.
        kernel
            .schedule(DigitalEvent::new(t50, net0, LogicValue::One))
            .unwrap();
        mgr.take_checkpoint(&kernel);

        // Run to 50ns.
        let _ = kernel.run_until(t50);

        // Roll back once.
        let _ = mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();
        assert_eq!(kernel.current_time(), SimulationTime::ZERO);

        // Run again to 50ns.
        let _ = kernel.run_until(t50);

        // Roll back again — should still work.
        let _ = mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();
        assert_eq!(kernel.current_time(), SimulationTime::ZERO);
        assert_eq!(kernel.pending_event_count(), 1);
    }

    // -- Confirm and prune --

    #[test]
    fn confirm_advances_confirmed_time() {
        let mut mgr = CheckpointManager::new();
        assert_eq!(mgr.confirmed_time(), SimulationTime::ZERO);

        let t50 = SimulationTime::from_nanoseconds(50);
        mgr.confirm(t50);
        assert_eq!(mgr.confirmed_time(), t50);

        // Confirm with lower time is a no-op.
        mgr.confirm(SimulationTime::from_nanoseconds(30));
        assert_eq!(mgr.confirmed_time(), t50);
    }

    #[test]
    fn prune_removes_checkpoints_before_confirmed_time() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(1);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);
        let t150 = SimulationTime::from_nanoseconds(150);

        // Checkpoint at t=0.
        mgr.take_checkpoint(&kernel);

        // Advance to t=50, checkpoint.
        kernel
            .schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t50);
        mgr.take_checkpoint(&kernel);

        // Advance to t=100, checkpoint.
        kernel
            .schedule(DigitalEvent::new(t100, NetId::new(0), LogicValue::Zero))
            .unwrap();
        let _ = kernel.run_until(t100);
        mgr.take_checkpoint(&kernel);

        // Advance to t=150, checkpoint.
        kernel
            .schedule(DigitalEvent::new(t150, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t150);
        mgr.take_checkpoint(&kernel);

        assert_eq!(mgr.checkpoint_count(), 4);

        // Confirm t=100: checkpoints at t=0 and t=50 are prunable.
        // The checkpoint at t=100 is the boundary anchor.
        mgr.confirm(t100);
        mgr.prune();

        // Should retain checkpoint at t=100 (anchor) and t=150.
        assert_eq!(mgr.checkpoint_count(), 2);
        assert_eq!(mgr.earliest_checkpoint_time(), Some(t100));
        assert_eq!(mgr.latest_checkpoint_time(), Some(t150));
    }

    #[test]
    fn prune_keeps_boundary_anchor_at_confirmed_time() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(1);
        let t50 = SimulationTime::from_nanoseconds(50);

        // Checkpoint at t=0.
        mgr.take_checkpoint(&kernel);

        // Advance to t=50, checkpoint.
        kernel
            .schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = kernel.run_until(t50);
        mgr.take_checkpoint(&kernel);

        // Confirm t=50: should keep the t=50 checkpoint as anchor,
        // discard t=0.
        mgr.confirm(t50);
        mgr.prune();

        assert_eq!(mgr.checkpoint_count(), 1);
        assert_eq!(mgr.earliest_checkpoint_time(), Some(t50));
    }

    #[test]
    fn prune_on_unconfirmed_manager_does_nothing() {
        let mut mgr = CheckpointManager::new();
        let kernel = DigitalKernel::new();

        mgr.take_checkpoint(&kernel);
        mgr.take_checkpoint(&kernel);
        assert_eq!(mgr.checkpoint_count(), 2);

        // confirmed_time is still 0, all checkpoints at t=0.
        mgr.prune();
        // The anchor at t=0 is kept; the second t=0 is pruned
        // (duplicate confirmed_time entries are removed).
        assert_eq!(mgr.checkpoint_count(), 1);
    }

    // -- Full optimistic rollback scenario --

    #[test]
    fn optimistic_rollback_scenario() {
        // Simulate the full optimistic time-advance protocol:
        // 1. Scheduler predicts next digital event at t=100ns.
        // 2. Takes checkpoint, advances kernel.
        // 3. Digital event actually fires at t=80ns (misprediction).
        // 4. Scheduler rolls back to checkpoint, re-runs to t=80ns.
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(2);
        let net0 = NetId::new(0);
        let net1 = NetId::new(1);
        let t80 = SimulationTime::from_nanoseconds(80);
        let t100 = SimulationTime::from_nanoseconds(100);

        // Schedule the real event at t=80ns and a later one at t=100ns.
        kernel
            .schedule(DigitalEvent::new(t80, net0, LogicValue::One))
            .unwrap();
        kernel
            .schedule(DigitalEvent::new(t100, net1, LogicValue::Zero))
            .unwrap();

        // 1. Checkpoint before predicted boundary.
        let cp_time = mgr.take_checkpoint(&kernel);
        assert_eq!(cp_time, SimulationTime::ZERO);

        // 2. Optimistically advance to predicted t=100ns.
        let _ = kernel.run_until(t100);
        assert_eq!(kernel.net_value(net0), LogicValue::One);
        assert_eq!(kernel.net_value(net1), LogicValue::Zero);
        assert_eq!(kernel.current_time(), t100);

        // 3. Misprediction detected: actual event at t=80ns.
        //    Roll back to the checkpoint at t=0.
        let restored = mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();
        assert_eq!(restored, SimulationTime::ZERO);
        assert_eq!(kernel.current_time(), SimulationTime::ZERO);
        assert_eq!(kernel.pending_event_count(), 2);
        assert_eq!(kernel.net_value(net0), LogicValue::Unknown);
        assert_eq!(kernel.net_value(net1), LogicValue::Unknown);

        // 4. Re-run to the correct boundary at t=80ns.
        let report = kernel.run_until(t80);
        assert_eq!(kernel.current_time(), t80);
        assert_eq!(kernel.net_value(net0), LogicValue::One);
        assert_eq!(kernel.net_value(net1), LogicValue::Unknown); // t=100 event not yet processed
        assert_eq!(report.events_processed.len(), 1);
        assert_eq!(report.next_event_time, Some(t100));
    }

    #[test]
    fn rollback_preserves_processed_events_trace() {
        // After rollback and re-run, the processed events trace
        // should reflect only events from the restored state and
        // subsequent runs — not the discarded mispredicted events.
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(2);
        let net0 = NetId::new(0);
        let net1 = NetId::new(1);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        // Schedule two events.
        kernel
            .schedule(DigitalEvent::new(t50, net0, LogicValue::One))
            .unwrap();
        kernel
            .schedule(DigitalEvent::new(t100, net1, LogicValue::Zero))
            .unwrap();

        // Checkpoint at t=0.
        mgr.take_checkpoint(&kernel);

        // Run to t=100ns (both events processed).
        let _ = kernel.run_until(t100);
        let events_before_rollback = kernel.take_processed_events();
        assert_eq!(events_before_rollback.len(), 2);

        // Roll back to t=0.
        mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();

        // The processed events after rollback should be the checkpoint's
        // processed events (empty, since we checkpointed before any
        // run_until).
        let events_after_rollback = kernel.take_processed_events();
        assert!(events_after_rollback.is_empty());

        // Re-run to t=50ns.
        let _ = kernel.run_until(t50);
        let events_rerun = kernel.take_processed_events();
        assert_eq!(events_rerun.len(), 1);
        assert_eq!(events_rerun[0].net, net0);
    }

    #[test]
    fn multiple_rollback_re_run_cycles() {
        // Verify that repeated rollback + re-run cycles produce
        // consistent results.
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::with_nets(2);
        let net0 = NetId::new(0);
        let net1 = NetId::new(1);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        kernel
            .schedule(DigitalEvent::new(t50, net0, LogicValue::One))
            .unwrap();
        kernel
            .schedule(DigitalEvent::new(t100, net1, LogicValue::Zero))
            .unwrap();

        // Checkpoint at t=0.
        mgr.take_checkpoint(&kernel);

        // Cycle 1: advance to 100, rollback to 0.
        let _ = kernel.run_until(t100);
        mgr.rollback_to(&mut kernel, SimulationTime::ZERO).unwrap();

        // Cycle 2: advance to 50, checkpoint at 50, advance to 100,
        // rollback to 50.
        let _ = kernel.run_until(t50);
        mgr.take_checkpoint(&kernel);
        let _ = kernel.run_until(t100);
        mgr.rollback_to(&mut kernel, t50).unwrap();

        // After rollback to t=50, net0 should be One (event at 50
        // was processed before checkpoint), net1 should be Unknown.
        assert_eq!(kernel.current_time(), t50);
        assert_eq!(kernel.net_value(net0), LogicValue::One);
        assert_eq!(kernel.net_value(net1), LogicValue::Unknown);
        assert_eq!(kernel.pending_event_count(), 1);
    }

    // -- Reset --

    #[test]
    fn reset_clears_all_state() {
        let mut mgr = CheckpointManager::new();
        let kernel = DigitalKernel::new();
        mgr.take_checkpoint(&kernel);
        mgr.confirm(SimulationTime::from_nanoseconds(50));
        assert_eq!(mgr.checkpoint_count(), 1);

        mgr.reset();
        assert_eq!(mgr.checkpoint_count(), 0);
        assert_eq!(mgr.confirmed_time(), SimulationTime::ZERO);
    }

    // -- Edge cases --

    #[test]
    fn rollback_to_zero_with_checkpoint_at_zero() {
        let mut mgr = CheckpointManager::new();
        let mut kernel = DigitalKernel::new();
        mgr.take_checkpoint(&kernel);

        let result = mgr.rollback_to(&mut kernel, SimulationTime::ZERO);
        assert_eq!(result.unwrap(), SimulationTime::ZERO);
    }

    #[test]
    fn checkpoints_accessor_returns_slice() {
        let mut mgr = CheckpointManager::new();
        let kernel = DigitalKernel::new();
        mgr.take_checkpoint(&kernel);

        let cps = mgr.checkpoints();
        assert_eq!(cps.len(), 1);
        assert_eq!(cps[0].time, SimulationTime::ZERO);
    }

    #[test]
    fn checkpoint_error_display() {
        let err = CheckpointError::NoCheckpointAvailable {
            requested_time: SimulationTime::from_nanoseconds(42),
        };
        let msg = format!("{err}");
        assert!(msg.contains("42"));
        assert!(msg.contains("no checkpoint available"));
    }

    #[test]
    fn confirm_and_prune_with_no_checkpoints_is_safe() {
        let mut mgr = CheckpointManager::new();
        mgr.confirm(SimulationTime::from_nanoseconds(100));
        mgr.prune();
        assert_eq!(mgr.checkpoint_count(), 0);
    }
}
