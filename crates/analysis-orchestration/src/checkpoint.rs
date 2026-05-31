//! Sparse checkpoint manager (tasks.md item #43, ADR-0004).
//!
//! The Mixed-Signal Scheduler (tasks.md item #42, see
//! [`super::mixed_signal`]) drives the analog kernel forward
//! optimistically up to each digital-side prediction. **Before** the
//! scheduler asks the digital simulator to `confirm_event`, the
//! analog solver must save a *sparse checkpoint* at that predicted
//! boundary so the system can roll back if the prediction is wrong
//! (ADR-0004 commitment #2 and #4).
//!
//! This module is that checkpoint store. It is a **passive,
//! ordered, time-keyed cache of analog state**; it does not drive
//! the simulator and it does not know about the digital side.
//! Concretely, each [`SparseCheckpoint`] carries everything the
//! analog kernel needs to resume from the checkpointed time:
//!
//! - the **simulation time** at which the snapshot was taken,
//! - the **sparse node-voltage vector** (only non-ground nodes; the
//!   ground node `NodeId::GROUND` is constantly 0 V and never
//!   stored),
//! - the **reactive companion state** for each capacitor
//!   ([`CapacitorHistory`]) and inductor ([`InductorHistory`]) in
//!   the circuit.
//!
//! ADR-0004 deliberately leaves the *internal* format of a sparse
//! checkpoint to the numeric-solver context (full vector vs.
//! delta-encoded vs. Schur-complement). This first
//! implementation chooses the **full-vector sparse representation**
//! — pairs `(NodeId, voltage)` and `(ElementId, history)` — because:
//!
//! 1. it matches the wiki decision exactly ("typically node
//!    voltages and reactive-element companion-model state"),
//! 2. it is the smallest correct shape the rollback handler
//!    (tasks.md item #44) can consume without re-running pass-1
//!    flattening, and
//! 3. it does not commit to any encoding that would couple this
//!    module to the in-progress AC/transient flattened-structure
//!    code path.
//!
//! Future tasks may replace the storage layer with delta encoding
//! or hierarchical encoding without changing the public
//! [`SparseCheckpoint`] / [`SparseCheckpointManager`] surface.
//!
//! # Public surface (ADR-0010)
//!
//! Per ADR-0010, the v1 public API surface is **unstable**. The
//! types in this module are re-exported by `analysis-orchestration`'s
//! `lib.rs` so a breaking change is caught by downstream test
//! breakage. The integration witness
//! `tests/scenario_sparse_checkpoint_manager_item_43.rs` pins those
//! re-exports.
//!
//! # Relationship to [`super::mixed_signal::SchedulerError`]
//!
//! Failure to restore from a checkpoint is reported via
//! [`CheckpointError::NoCheckpointAtOrBefore`], which the rollback
//! handler (tasks.md #44) will lift into
//! [`super::mixed_signal::SchedulerError::NoCheckpoint`]. The
//! manager itself does not depend on scheduler types; this keeps
//! the dependency direction one-way (scheduler → checkpoint, not
//! the inverse).

use circuit_solver_types::{ElementId, NodeId, SimulationTime};
use core::fmt;
use numeric_solver::integration::{CapacitorHistory, InductorHistory};

// ---------------------------------------------------------------------------
// SparseCheckpoint — the saved-state record itself.
// ---------------------------------------------------------------------------

/// A single sparse snapshot of analog state at one predicted
/// digital event boundary.
///
/// Per ADR-0004 a sparse checkpoint carries "enough state to resume
/// from that time point if the prediction is wrong" — concretely
/// the **node voltages** at every non-ground node and the
/// **reactive companion state** for every capacitor and inductor.
/// MNA branch-augmentation rows (e.g., voltage sources, inductors
/// when carried as branch currents) live in `inductor_states`
/// because the inductor companion model is the one that needs the
/// previous branch current to advance.
///
/// # Sparsity convention
///
/// - **Node voltages.** Only non-ground nodes appear. The ground
///   node is constantly 0 V and is never stored. Producers should
///   filter `NodeId::GROUND` *before* calling
///   [`SparseCheckpoint::with_node_voltages`]; consumers may rely
///   on the absence of ground.
/// - **Reactive state.** Only elements with non-zero companion
///   contribution should appear. Producers may always include all
///   capacitors and inductors; the cost is linear in element
///   count, which is well-bounded.
///
/// # Field ordering
///
/// The node and element vectors are kept in **insertion order**.
/// The scheduler does not depend on any particular sort, and
/// keeping insertion order makes diffing the checkpoint against a
/// post-rollback re-solve mechanically simpler. If a future
/// consumer needs O(log n) lookup, wrap the public vectors in an
/// auxiliary index — do not break the insertion-order invariant.
///
/// # Equality semantics
///
/// `PartialEq` on this type compares times exactly (`SimulationTime`
/// is integer-backed) and voltages / histories bitwise via `f64`
/// `PartialEq`. Tests should rely on this only for *exact recall*
/// assertions (saved-then-restored, no integration in between); to
/// compare against a re-solved analog state, use the
/// `circuit-solver-types::convergence` envelope per ADR-0008.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseCheckpoint {
    /// The simulation time at which this state was snapshotted.
    /// Always equal to the predicted digital event boundary the
    /// scheduler asked the analog solver to run to.
    pub time: SimulationTime,
    /// Per-node DC voltage at `time`, in volts. The ground node
    /// (`NodeId::GROUND`) is *not* present in this vector.
    pub node_voltages: Vec<(NodeId, f64)>,
    /// Per-capacitor companion history at `time`.
    pub capacitor_states: Vec<(ElementId, CapacitorHistory)>,
    /// Per-inductor companion history at `time`. For an inductor
    /// carried as an MNA branch-augmentation row, this is the
    /// branch-current sample from the row's solution entry.
    pub inductor_states: Vec<(ElementId, InductorHistory)>,
}

impl SparseCheckpoint {
    /// Construct an empty checkpoint at `time` (no nodes, no
    /// reactive elements). Useful as a degenerate base case and
    /// for tests of the manager's bookkeeping in isolation.
    #[must_use]
    pub fn empty(time: SimulationTime) -> Self {
        Self {
            time,
            node_voltages: Vec::new(),
            capacitor_states: Vec::new(),
            inductor_states: Vec::new(),
        }
    }

    /// Replace the node-voltage vector with the given pairs.
    /// Consumers should already have filtered out the ground node;
    /// see "Sparsity convention" in the struct-level docs.
    #[must_use]
    pub fn with_node_voltages(mut self, voltages: Vec<(NodeId, f64)>) -> Self {
        self.node_voltages = voltages;
        self
    }

    /// Replace the capacitor-history vector with the given pairs.
    #[must_use]
    pub fn with_capacitor_states(mut self, states: Vec<(ElementId, CapacitorHistory)>) -> Self {
        self.capacitor_states = states;
        self
    }

    /// Replace the inductor-history vector with the given pairs.
    #[must_use]
    pub fn with_inductor_states(mut self, states: Vec<(ElementId, InductorHistory)>) -> Self {
        self.inductor_states = states;
        self
    }

    /// The simulation time this checkpoint covers.
    #[must_use]
    pub const fn time(&self) -> SimulationTime {
        self.time
    }

    /// Number of non-ground node voltages saved.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.node_voltages.len()
    }

    /// Number of capacitor history entries saved.
    #[must_use]
    pub fn capacitor_count(&self) -> usize {
        self.capacitor_states.len()
    }

    /// Number of inductor history entries saved.
    #[must_use]
    pub fn inductor_count(&self) -> usize {
        self.inductor_states.len()
    }

    /// True iff the checkpoint carries no node voltages and no
    /// reactive state.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.node_voltages.is_empty()
            && self.capacitor_states.is_empty()
            && self.inductor_states.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Errors.
// ---------------------------------------------------------------------------

/// Failures surfaced by [`SparseCheckpointManager`].
///
/// The variants here mirror what the rollback handler (tasks.md
/// item #44) will need to lift into
/// [`super::mixed_signal::SchedulerError`]:
///
/// - [`CheckpointError::NoCheckpointAtOrBefore`] → `SchedulerError::NoCheckpoint`
/// - [`CheckpointError::NonMonotonicSave`] → diagnostic + abort (the
///   scheduler never asks to save out of order on the
///   correct-prediction path; a non-monotonic save signals a
///   scheduler bug).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckpointError {
    /// A `restore_at_or_before(target)` call found no checkpoint
    /// whose time is `<= target`. The contained value is the
    /// requested `target`.
    NoCheckpointAtOrBefore(SimulationTime),
    /// A `save(checkpoint)` call provided a time strictly less
    /// than the time of the latest existing checkpoint. The
    /// manager refuses this because the rollback contract requires
    /// the saved sequence to be monotonically non-decreasing in
    /// time. Contained values are `(latest, attempted)`.
    NonMonotonicSave {
        /// The time of the most recently saved checkpoint.
        latest: SimulationTime,
        /// The time at which a new save was attempted.
        attempted: SimulationTime,
    },
}

impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCheckpointAtOrBefore(t) => {
                write!(f, "no checkpoint at or before {t}")
            }
            Self::NonMonotonicSave { latest, attempted } => {
                write!(
                    f,
                    "non-monotonic save: latest checkpoint at {latest}, attempted save at {attempted}"
                )
            }
        }
    }
}

impl std::error::Error for CheckpointError {}

// ---------------------------------------------------------------------------
// SparseCheckpointManager.
// ---------------------------------------------------------------------------

/// Ordered, time-keyed store of [`SparseCheckpoint`] snapshots.
///
/// The manager preserves the invariant that **stored checkpoints
/// are monotonically non-decreasing in `time`**. This matches
/// ADR-0004's optimistic-advance model: the scheduler always asks
/// the analog solver to step *forward* to the next predicted
/// digital event, and the analog solver saves a checkpoint *at*
/// that boundary. Rollback reduces the manager (via
/// [`SparseCheckpointManager::drop_after`]) but never inserts in
/// the middle.
///
/// # Memory model
///
/// Per ADR-0004 "Negative consequences": sparse-checkpoint memory
/// scales with the number of reactive elements and the digital
/// event rate. This first implementation places no automatic cap
/// on retained checkpoints; the rollback handler (tasks.md #44)
/// and the scheduler control loop (already in
/// [`super::mixed_signal`]) decide when to drop stale entries via
/// [`SparseCheckpointManager::drop_before`] — *commits* upstream
/// of the current advance are no longer rollback targets and may
/// be released. The decision page calls out a future lockstep
/// fallback if profiling shows checkpoint memory is prohibitive.
///
/// # Concurrency
///
/// `SparseCheckpointManager` is `!Sync` by default (it contains
/// owned `Vec`s). The Mixed-Signal Scheduler owns it singly per
/// ADR-0004 commitment #3 ("Shared scheduler ownership"); no
/// cross-thread sharing is required at v1.
#[derive(Debug, Default, Clone)]
pub struct SparseCheckpointManager {
    /// Stored checkpoints, monotonically non-decreasing in `time`.
    checkpoints: Vec<SparseCheckpoint>,
}

impl SparseCheckpointManager {
    /// Construct an empty manager.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
        }
    }

    /// Insert a checkpoint. The new checkpoint's `time` must be
    /// greater than or equal to the latest stored time.
    ///
    /// # Errors
    ///
    /// Returns [`CheckpointError::NonMonotonicSave`] if `checkpoint.time`
    /// is strictly less than the time of the currently latest
    /// stored checkpoint.
    pub fn save(&mut self, checkpoint: SparseCheckpoint) -> Result<(), CheckpointError> {
        if let Some(latest) = self.latest_time() {
            if checkpoint.time < latest {
                return Err(CheckpointError::NonMonotonicSave {
                    latest,
                    attempted: checkpoint.time,
                });
            }
        }
        self.checkpoints.push(checkpoint);
        Ok(())
    }

    /// Return a reference to the most recent checkpoint whose
    /// `time` is `<= target`.
    ///
    /// On the correct-prediction path the scheduler queries this
    /// at exactly a stored boundary (`target == saved_time`); on
    /// the rollback path (tasks.md #44) the query may target a
    /// strictly-earlier event time and must return the largest
    /// stored time `<= target`.
    ///
    /// # Errors
    ///
    /// Returns [`CheckpointError::NoCheckpointAtOrBefore`] if no
    /// stored checkpoint satisfies the bound (typically: the
    /// manager is empty, or all stored times are `> target`).
    pub fn restore_at_or_before(
        &self,
        target: SimulationTime,
    ) -> Result<&SparseCheckpoint, CheckpointError> {
        // Walk backwards; the vector is sorted ascending, so the
        // last element with time <= target is the answer.
        self.checkpoints
            .iter()
            .rev()
            .find(|c| c.time <= target)
            .ok_or(CheckpointError::NoCheckpointAtOrBefore(target))
    }

    /// Drop every checkpoint whose `time` is **strictly greater**
    /// than `target`. The checkpoint *at* `target`, if any, is
    /// retained; this matches the rollback semantics where the
    /// analog solver returns to the state saved *at* the rollback
    /// time and resumes forward from there.
    ///
    /// Returns the number of checkpoints removed.
    pub fn drop_after(&mut self, target: SimulationTime) -> usize {
        let before = self.checkpoints.len();
        self.checkpoints.retain(|c| c.time <= target);
        before - self.checkpoints.len()
    }

    /// Drop every checkpoint whose `time` is **strictly less**
    /// than `target`. Useful for releasing memory once the
    /// scheduler has committed past a boundary that can no longer
    /// be a rollback target.
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

    /// The `time` of the latest stored checkpoint, or `None` if
    /// empty.
    #[must_use]
    pub fn latest_time(&self) -> Option<SimulationTime> {
        self.checkpoints.last().map(|c| c.time)
    }

    /// The `time` of the earliest stored checkpoint, or `None` if
    /// empty.
    #[must_use]
    pub fn earliest_time(&self) -> Option<SimulationTime> {
        self.checkpoints.first().map(|c| c.time)
    }

    /// Borrow the full ordered checkpoint slice for read-only
    /// inspection (debug dumps, scheduler diagnostics).
    #[must_use]
    pub fn as_slice(&self) -> &[SparseCheckpoint] {
        &self.checkpoints
    }
}

// ---------------------------------------------------------------------------
// Tests — in-crate, fast, deterministic.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn t_ns(ns: i64) -> SimulationTime {
        SimulationTime::from_nanoseconds(ns)
    }

    fn cap(v: f64) -> CapacitorHistory {
        CapacitorHistory::new(v)
    }

    fn ind(i: f64) -> InductorHistory {
        InductorHistory::new(i)
    }

    fn checkpoint_at(time: SimulationTime, marker: f64) -> SparseCheckpoint {
        SparseCheckpoint::empty(time)
            .with_node_voltages(vec![(NodeId::new(1), marker)])
            .with_capacitor_states(vec![(ElementId::new(10), cap(marker))])
            .with_inductor_states(vec![(ElementId::new(20), ind(marker))])
    }

    // ---- SparseCheckpoint shape ---------------------------------------

    #[test]
    fn empty_checkpoint_is_empty() {
        let c = SparseCheckpoint::empty(t_ns(50));
        assert_eq!(c.time(), t_ns(50));
        assert_eq!(c.node_count(), 0);
        assert_eq!(c.capacitor_count(), 0);
        assert_eq!(c.inductor_count(), 0);
        assert!(c.is_empty());
    }

    #[test]
    fn with_setters_populate_state() {
        let c = SparseCheckpoint::empty(t_ns(100))
            .with_node_voltages(vec![(NodeId::new(1), 1.25), (NodeId::new(2), -0.5)])
            .with_capacitor_states(vec![(ElementId::new(7), cap(2.0))])
            .with_inductor_states(vec![(ElementId::new(9), ind(0.01))]);

        assert_eq!(c.node_count(), 2);
        assert_eq!(c.capacitor_count(), 1);
        assert_eq!(c.inductor_count(), 1);
        assert!(!c.is_empty());
        assert_eq!(c.node_voltages[0], (NodeId::new(1), 1.25));
        assert_eq!(c.node_voltages[1], (NodeId::new(2), -0.5));
        assert_eq!(c.capacitor_states[0].1.v_prev.to_bits(), 2.0_f64.to_bits());
        assert_eq!(c.inductor_states[0].1.i_prev.to_bits(), 0.01_f64.to_bits());
    }

    // ---- SparseCheckpointManager — save / monotonicity ---------------

    #[test]
    fn manager_starts_empty() {
        let mgr = SparseCheckpointManager::new();
        assert!(mgr.is_empty());
        assert_eq!(mgr.len(), 0);
        assert_eq!(mgr.latest_time(), None);
        assert_eq!(mgr.earliest_time(), None);
    }

    #[test]
    fn save_in_order_succeeds() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(150), 3.0)).unwrap();

        assert_eq!(mgr.len(), 3);
        assert_eq!(mgr.earliest_time(), Some(t_ns(50)));
        assert_eq!(mgr.latest_time(), Some(t_ns(150)));
    }

    #[test]
    fn save_at_equal_time_is_allowed() {
        // Equal-time saves model the scenario where a rollback's
        // re-solve lands at the same boundary; the manager
        // accepts and stores both. (Future tasks may decide to
        // dedupe; that's an explicit ADR change.)
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(50), 1.5)).unwrap();
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.latest_time(), Some(t_ns(50)));
    }

    #[test]
    fn save_out_of_order_is_rejected() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(100), 1.0)).unwrap();

        let err = mgr.save(checkpoint_at(t_ns(50), 2.0)).unwrap_err();
        assert_eq!(
            err,
            CheckpointError::NonMonotonicSave {
                latest: t_ns(100),
                attempted: t_ns(50),
            }
        );
        // The rejected save did not corrupt the store.
        assert_eq!(mgr.len(), 1);
        assert_eq!(mgr.latest_time(), Some(t_ns(100)));
    }

    // ---- restore_at_or_before ---------------------------------------

    #[test]
    fn restore_empty_is_error() {
        let mgr = SparseCheckpointManager::new();
        let err = mgr.restore_at_or_before(t_ns(50)).unwrap_err();
        assert_eq!(err, CheckpointError::NoCheckpointAtOrBefore(t_ns(50)));
    }

    #[test]
    fn restore_exact_match_returns_that_checkpoint() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(150), 3.0)).unwrap();

        let c = mgr.restore_at_or_before(t_ns(100)).unwrap();
        assert_eq!(c.time(), t_ns(100));
        assert_eq!(c.node_voltages[0], (NodeId::new(1), 2.0));
    }

    #[test]
    fn restore_between_returns_largest_le_target() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(150), 3.0)).unwrap();

        // Target 120 ns lies between saved 100 and 150 → return 100.
        let c = mgr.restore_at_or_before(t_ns(120)).unwrap();
        assert_eq!(c.time(), t_ns(100));
        assert_eq!(c.node_voltages[0], (NodeId::new(1), 2.0));
    }

    #[test]
    fn restore_before_all_is_error() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();

        let err = mgr.restore_at_or_before(t_ns(10)).unwrap_err();
        assert_eq!(err, CheckpointError::NoCheckpointAtOrBefore(t_ns(10)));
    }

    #[test]
    fn restore_after_all_returns_latest() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();

        let c = mgr.restore_at_or_before(t_ns(999)).unwrap();
        assert_eq!(c.time(), t_ns(100));
    }

    // ---- drop_after / drop_before ------------------------------------

    #[test]
    fn drop_after_retains_target_and_earlier() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(150), 3.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(200), 4.0)).unwrap();

        let dropped = mgr.drop_after(t_ns(100));
        assert_eq!(dropped, 2); // 150 and 200 were removed
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.latest_time(), Some(t_ns(100)));
        // The checkpoint *at* the target was retained.
        let c = mgr.restore_at_or_before(t_ns(100)).unwrap();
        assert_eq!(c.time(), t_ns(100));
    }

    #[test]
    fn drop_after_with_no_matches_is_noop() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();

        let dropped = mgr.drop_after(t_ns(200));
        assert_eq!(dropped, 0);
        assert_eq!(mgr.len(), 2);
    }

    #[test]
    fn drop_before_releases_stale_checkpoints() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(150), 3.0)).unwrap();

        let dropped = mgr.drop_before(t_ns(100));
        assert_eq!(dropped, 1); // only the t=50 entry
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.earliest_time(), Some(t_ns(100)));
    }

    #[test]
    fn save_after_drop_after_continues_monotonically() {
        // The rollback scenario: save up to t=200, roll back to
        // t=100 (drop_after(100)), then save a new checkpoint at
        // t=150 from the re-solve. This must succeed: the
        // manager's monotonicity invariant is relative to its
        // *current* contents, not its history.
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(200), 4.0)).unwrap();

        let dropped = mgr.drop_after(t_ns(100));
        assert_eq!(dropped, 1);

        // Now save at t=150 — strictly greater than the new
        // latest (t=100), so it must be accepted.
        mgr.save(checkpoint_at(t_ns(150), 2.5)).unwrap();
        assert_eq!(mgr.len(), 3);
        assert_eq!(mgr.latest_time(), Some(t_ns(150)));
    }

    // ---- as_slice / inspection ---------------------------------------

    #[test]
    fn as_slice_returns_ordered_view() {
        let mut mgr = SparseCheckpointManager::new();
        mgr.save(checkpoint_at(t_ns(50), 1.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(100), 2.0)).unwrap();
        mgr.save(checkpoint_at(t_ns(150), 3.0)).unwrap();

        let slice = mgr.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0].time(), t_ns(50));
        assert_eq!(slice[1].time(), t_ns(100));
        assert_eq!(slice[2].time(), t_ns(150));
    }

    // ---- Display on CheckpointError ----------------------------------

    #[test]
    fn error_displays_human_readable() {
        let e = CheckpointError::NoCheckpointAtOrBefore(t_ns(50));
        let msg = format!("{e}");
        assert!(msg.contains("no checkpoint"));
        assert!(msg.contains("50"));

        let e = CheckpointError::NonMonotonicSave {
            latest: t_ns(100),
            attempted: t_ns(50),
        };
        let msg = format!("{e}");
        assert!(msg.contains("non-monotonic"));
        assert!(msg.contains("100"));
        assert!(msg.contains("50"));
    }
}
