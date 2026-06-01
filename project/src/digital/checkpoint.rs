//! Checkpoint/restore of the event queue and net state for optimistic rollback.
//!
//! Implements the `digital-engine#native-kernel-optimistic-rollback` spec
//! scenario: when the Mixed-Signal Scheduler rolls the analog side back past
//! a digital event that was optimistically processed, the kernel must restore
//! its event queue **and** net state to the checkpoint, consistent with the
//! superseding sync ADR (ADR-0006).
//!
//! # Architecture
//!
//! - `NetState` tracks the current `LogicValue` of every digital net.
//! - `KernelCheckpoint` combines an `EventQueueCheckpoint` with a snapshot of
//!   the net state, providing a single atomically-restorable unit.
//! - The `checkpoint_kernel` / `restore_kernel` free functions compose the
//!   per-component operations so the Mixed-Signal Scheduler (task #16) has a
//!   single call site.
//!
//! # Spec traceability
//!
//! - `digital-engine#native-kernel-optimistic-rollback`: "the kernel restores
//!   its event queue and net state to the checkpoint" → `restore_kernel`.

use std::collections::HashMap;

use super::event_queue::{EventQueue, EventQueueCheckpoint, LogicValue, NetId, TraceEntry};

// ---------------------------------------------------------------------------
// Net state
// ---------------------------------------------------------------------------

/// Current signal values on all digital nets.
///
/// The net state is the "memory" of the digital kernel — it records the
/// present `LogicValue` of every net that has been assigned by a processed
/// event. Unassigned nets have no entry (they are implicitly `LogicValue::X`
/// at the start of simulation).
///
/// Callers should call `apply_trace` after each `run_until` call to bring
/// the net state in sync with the events the kernel just processed.
#[derive(Debug, Clone)]
pub struct NetState {
    values: HashMap<NetId, LogicValue>,
}

impl NetState {
    /// Create an empty net state (no nets have been assigned yet).
    pub fn new() -> Self {
        NetState {
            values: HashMap::new(),
        }
    }

    /// Get the current value of a net, or `None` if it has never been assigned.
    ///
    /// Unassigned nets are implicitly in the `X` (unknown) state; callers
    /// that need the four-state value can use `get_or_x`.
    pub fn get(&self, net: NetId) -> Option<LogicValue> {
        self.values.get(&net).copied()
    }

    /// Get the current value of a net, returning `LogicValue::X` if unassigned.
    pub fn get_or_x(&self, net: NetId) -> LogicValue {
        self.get(net).unwrap_or(LogicValue::X)
    }

    /// Set the value of a net.
    pub fn set(&mut self, net: NetId, value: LogicValue) {
        self.values.insert(net, value);
    }

    /// Number of nets with an assigned value.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether any net has been assigned.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Apply a slice of processed trace entries to update net values.
    ///
    /// Each `TraceEntry` represents a `(time, net, value)` assignment that
    /// the kernel has already processed. Applying the trace brings the net
    /// state in sync with the event queue's processed events.
    ///
    /// Events should be applied in the order the kernel produced them
    /// (nondecreasing time, insertion-order within a delta) so that the
    /// last assignment to a given net wins, matching real-time semantics.
    pub fn apply_trace(&mut self, entries: &[TraceEntry]) {
        for entry in entries {
            self.set(entry.net, entry.value);
        }
    }

    // -----------------------------------------------------------------------
    // Checkpoint / restore
    // -----------------------------------------------------------------------

    /// Capture a checkpoint of the net state.
    pub fn checkpoint(&self) -> NetStateCheckpoint {
        NetStateCheckpoint {
            values: self.values.clone(),
        }
    }

    /// Restore the net state from a previously captured checkpoint.
    ///
    /// After this call, the net state is exactly what it was when
    /// `checkpoint()` was called.
    pub fn restore(&mut self, checkpoint: NetStateCheckpoint) {
        self.values = checkpoint.values;
    }
}

impl Default for NetState {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of a `NetState` for checkpoint/restore.
///
/// Produced by `NetState::checkpoint()`, consumed by `NetState::restore()`.
#[derive(Debug, Clone)]
pub struct NetStateCheckpoint {
    values: HashMap<NetId, LogicValue>,
}

// ---------------------------------------------------------------------------
// Combined kernel checkpoint (event queue + net state)
// ---------------------------------------------------------------------------

/// A combined checkpoint of the digital kernel's event queue **and** net
/// state, providing a single atomically-restorable unit for the optimistic
/// rollback mechanism.
///
/// This is what the Mixed-Signal Scheduler (task #16) uses when it rolls
/// back past a digital synchronization point: one `KernelCheckpoint`
/// captures everything needed to restore the digital kernel to a prior
/// state.
///
/// # Spec traceability
///
/// - `digital-engine#native-kernel-optimistic-rollback`: "the kernel
///   restores its event queue **and** net state to the checkpoint" →
///   `restore_kernel` uses both fields of this struct.
#[derive(Debug, Clone)]
pub struct KernelCheckpoint {
    /// Snapshot of the event queue (pending events, seq counter, current
    /// time, accumulated trace).
    pub queue_checkpoint: EventQueueCheckpoint,
    /// Snapshot of the net values at the time of checkpointing.
    pub net_state: HashMap<NetId, LogicValue>,
}

// ---------------------------------------------------------------------------
// Free-function checkpoint / restore API
// ---------------------------------------------------------------------------

/// Capture a combined checkpoint of the event queue and net state.
///
/// The returned `KernelCheckpoint` contains everything the scheduler needs
/// to roll the digital kernel back to this point.
///
/// # Spec traceability
///
/// - `digital-engine#native-kernel-optimistic-rollback`: "the kernel
///   restores its event queue and net state to the checkpoint" → this
///   function captures the data that `restore_kernel` will use.
pub fn checkpoint_kernel(queue: &EventQueue, net_state: &NetState) -> KernelCheckpoint {
    KernelCheckpoint {
        queue_checkpoint: queue.checkpoint(),
        net_state: net_state.checkpoint().values,
    }
}

/// Restore both the event queue and net state from a combined checkpoint.
///
/// After this call, the queue and net state are exactly as they were when
/// `checkpoint_kernel` (or manual construction of the `KernelCheckpoint`)
/// captured them. Any events processed or net values assigned since then
/// are discarded.
///
/// # Spec traceability
///
/// - `digital-engine#native-kernel-optimistic-rollback`: "When the analog
///   side rolls back past a digital event that was optimistically processed,
///   Then the kernel restores its event queue and net state to the
///   checkpoint" → this function implements the "Then" clause.
pub fn restore_kernel(queue: &mut EventQueue, net_state: &mut NetState, cp: KernelCheckpoint) {
    queue.restore(cp.queue_checkpoint);
    net_state.restore(NetStateCheckpoint {
        values: cp.net_state,
    });
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::digital::event_queue::{EventQueue, LogicValue, NetId, SimTime};

    // -- Spec scenario: native-kernel-optimistic-rollback -------------------

    #[test]
    fn optimistic_rollback_restores_queue_and_net_state() {
        // Given the native digital kernel under the Mixed-Signal Scheduler
        let mut queue = EventQueue::new(100);
        let mut net_state = NetState::new();

        // Schedule initial events and process them
        queue.schedule(SimTime(1.0), NetId(0), LogicValue::One);
        queue.schedule(SimTime(2.0), NetId(1), LogicValue::Zero);
        queue.schedule(SimTime(3.0), NetId(0), LogicValue::Zero);

        // Process up to t=2.0, updating net state
        let trace = queue.run_until(SimTime(2.0));
        net_state.apply_trace(&trace);

        // At this point: net0=One (from t=1), net1=Zero (from t=2)
        assert_eq!(net_state.get_or_x(NetId(0)), LogicValue::One);
        assert_eq!(net_state.get_or_x(NetId(1)), LogicValue::Zero);

        // Checkpoint before optimistically advancing
        let cp = checkpoint_kernel(&queue, &net_state);
        assert_eq!(queue.current_time(), SimTime(2.0));
        assert_eq!(queue.trace().len(), 2);

        // Optimistically process beyond the checkpoint (t=3)
        let trace3 = queue.run_until(SimTime(5.0));
        net_state.apply_trace(&trace3);

        // After optimistic advance: net0=Zero (from t=3), queue at t=3
        assert_eq!(net_state.get_or_x(NetId(0)), LogicValue::Zero);
        assert_eq!(queue.current_time(), SimTime(3.0));
        assert_eq!(queue.trace().len(), 3);

        // When the analog side rolls back past a digital event that was
        // optimistically processed
        restore_kernel(&mut queue, &mut net_state, cp);

        // Then the kernel restores its event queue and net state to the
        // checkpoint, consistent with the superseding sync ADR
        assert_eq!(queue.current_time(), SimTime(2.0));
        assert_eq!(queue.trace().len(), 2);
        assert_eq!(net_state.get_or_x(NetId(0)), LogicValue::One);
        assert_eq!(net_state.get_or_x(NetId(1)), LogicValue::Zero);
        // The t=3 event is back in the pending queue
        assert_eq!(queue.pending_count(), 1);
    }

    #[test]
    fn rollback_discards_events_and_net_changes_since_checkpoint() {
        let mut queue = EventQueue::new(100);
        let mut net_state = NetState::new();

        // Setup: process one event
        queue.schedule(SimTime(1.0), NetId(0), LogicValue::One);
        let trace = queue.run_until(SimTime(1.0));
        net_state.apply_trace(&trace);
        let cp = checkpoint_kernel(&queue, &net_state);

        // Advance past checkpoint: schedule and process more events
        queue.schedule(SimTime(2.0), NetId(5), LogicValue::Z);
        queue.schedule(SimTime(4.0), NetId(0), LogicValue::X);
        let trace2 = queue.run_until(SimTime(10.0));
        net_state.apply_trace(&trace2);

        // Verify we advanced
        assert_eq!(queue.current_time(), SimTime(4.0));
        assert_eq!(net_state.get_or_x(NetId(5)), LogicValue::Z);
        assert_eq!(net_state.get_or_x(NetId(0)), LogicValue::X);

        // Rollback
        restore_kernel(&mut queue, &mut net_state, cp);

        // The events at t=2 and t=4 are gone (they were scheduled after
        // the checkpoint and restored queue doesn't have them)
        assert_eq!(queue.current_time(), SimTime(1.0));
        assert_eq!(queue.pending_count(), 0);
        assert_eq!(net_state.get_or_x(NetId(0)), LogicValue::One);
        assert_eq!(net_state.get_or_x(NetId(5)), LogicValue::X); // never assigned
    }

    // -- NetState unit tests ------------------------------------------------

    #[test]
    fn net_state_starts_empty() {
        let ns = NetState::new();
        assert!(ns.is_empty());
        assert_eq!(ns.len(), 0);
        assert_eq!(ns.get_or_x(NetId(0)), LogicValue::X);
    }

    #[test]
    fn net_state_set_and_get() {
        let mut ns = NetState::new();
        ns.set(NetId(0), LogicValue::One);
        ns.set(NetId(42), LogicValue::Z);

        assert_eq!(ns.get(NetId(0)), Some(LogicValue::One));
        assert_eq!(ns.get(NetId(42)), Some(LogicValue::Z));
        assert_eq!(ns.get(NetId(99)), None);
        assert_eq!(ns.get_or_x(NetId(99)), LogicValue::X);
        assert_eq!(ns.len(), 2);
    }

    #[test]
    fn net_state_overwrite() {
        let mut ns = NetState::new();
        ns.set(NetId(0), LogicValue::One);
        assert_eq!(ns.get(NetId(0)), Some(LogicValue::One));

        ns.set(NetId(0), LogicValue::Zero);
        assert_eq!(ns.get(NetId(0)), Some(LogicValue::Zero));
        assert_eq!(ns.len(), 1); // still just one net
    }

    #[test]
    fn apply_trace_updates_nets() {
        let mut ns = NetState::new();
        let trace = vec![
            TraceEntry {
                time: SimTime(1.0),
                net: NetId(0),
                value: LogicValue::One,
            },
            TraceEntry {
                time: SimTime(2.0),
                net: NetId(1),
                value: LogicValue::Zero,
            },
            TraceEntry {
                time: SimTime(3.0),
                net: NetId(0),
                value: LogicValue::Z,
            },
        ];

        ns.apply_trace(&trace);
        assert_eq!(ns.get(NetId(0)), Some(LogicValue::Z)); // last assignment wins
        assert_eq!(ns.get(NetId(1)), Some(LogicValue::Zero));
    }

    #[test]
    fn net_state_checkpoint_restore() {
        let mut ns = NetState::new();
        ns.set(NetId(0), LogicValue::One);
        ns.set(NetId(1), LogicValue::Zero);

        let cp = ns.checkpoint();

        // Mutate
        ns.set(NetId(0), LogicValue::X);
        ns.set(NetId(2), LogicValue::Z);
        assert_eq!(ns.get(NetId(0)), Some(LogicValue::X));
        assert_eq!(ns.len(), 3);

        // Restore
        ns.restore(cp);
        assert_eq!(ns.get(NetId(0)), Some(LogicValue::One));
        assert_eq!(ns.get(NetId(1)), Some(LogicValue::Zero));
        assert_eq!(ns.get(NetId(2)), None); // not in checkpoint
        assert_eq!(ns.len(), 2);
    }

    // -- KernelCheckpoint integration tests ---------------------------------

    #[test]
    fn checkpoint_before_any_processing() {
        let queue = EventQueue::new(100);
        let net_state = NetState::new();
        let cp = checkpoint_kernel(&queue, &net_state);

        // Process events after checkpoint
        let mut q2 = queue;
        let mut ns2 = net_state;
        q2.schedule(SimTime(5.0), NetId(0), LogicValue::One);
        let trace = q2.run_until(SimTime(10.0));
        ns2.apply_trace(&trace);

        assert_eq!(q2.current_time(), SimTime(5.0));
        assert_eq!(ns2.get(NetId(0)), Some(LogicValue::One));

        // Restore to initial empty state
        restore_kernel(&mut q2, &mut ns2, cp);
        assert_eq!(q2.current_time(), SimTime::ZERO);
        assert_eq!(q2.pending_count(), 0);
        assert!(ns2.is_empty());
    }

    #[test]
    fn multiple_checkpoints_rollback_to_earliest() {
        let mut queue = EventQueue::new(100);
        let mut net_state = NetState::new();

        // Process event at t=1
        queue.schedule(SimTime(1.0), NetId(0), LogicValue::One);
        let trace = queue.run_until(SimTime(1.0));
        net_state.apply_trace(&trace);
        let cp1 = checkpoint_kernel(&queue, &net_state);

        // Process event at t=5
        queue.schedule(SimTime(5.0), NetId(1), LogicValue::Zero);
        let trace = queue.run_until(SimTime(5.0));
        net_state.apply_trace(&trace);
        let _cp2 = checkpoint_kernel(&queue, &net_state);

        // Process event at t=10
        queue.schedule(SimTime(10.0), NetId(2), LogicValue::Z);
        let trace = queue.run_until(SimTime(10.0));
        net_state.apply_trace(&trace);

        // Verify we advanced to t=10
        assert_eq!(queue.current_time(), SimTime(10.0));
        assert_eq!(net_state.len(), 3);

        // Rollback to earliest checkpoint (t=1)
        restore_kernel(&mut queue, &mut net_state, cp1);
        assert_eq!(queue.current_time(), SimTime(1.0));
        assert_eq!(queue.trace().len(), 1);
        assert_eq!(net_state.get(NetId(0)), Some(LogicValue::One));
        assert_eq!(net_state.get(NetId(1)), None); // not yet assigned at cp1
        assert_eq!(net_state.get(NetId(2)), None); // not yet assigned at cp1
        assert_eq!(queue.pending_count(), 0); // only t=1 was in queue at cp1
    }

    #[test]
    fn re_run_after_rollback_produces_same_trace() {
        let mut queue = EventQueue::new(100);
        let mut net_state = NetState::new();

        queue.schedule(SimTime(1.0), NetId(0), LogicValue::One);
        queue.schedule(SimTime(2.0), NetId(1), LogicValue::Zero);

        // First run: process up to t=2
        let trace1 = queue.run_until(SimTime(2.0));
        net_state.apply_trace(&trace1);
        let cp = checkpoint_kernel(&queue, &net_state);

        // Continue past checkpoint
        queue.schedule(SimTime(3.0), NetId(2), LogicValue::Z);
        let trace_extra = queue.run_until(SimTime(5.0));
        net_state.apply_trace(&trace_extra);

        // Rollback
        restore_kernel(&mut queue, &mut net_state, cp);

        // Re-run the same events from the checkpoint
        queue.schedule(SimTime(3.0), NetId(2), LogicValue::Z);
        let trace_rerun = queue.run_until(SimTime(5.0));
        net_state.apply_trace(&trace_rerun);

        // The extra trace from the re-run matches the original extra trace
        assert_eq!(trace_extra, trace_rerun);
        assert_eq!(net_state.get(NetId(2)), Some(LogicValue::Z));
    }

    #[test]
    fn rollback_with_multiple_nets_same_time() {
        let mut queue = EventQueue::new(100);
        let mut net_state = NetState::new();

        // Schedule multiple events at the same time
        queue.schedule(SimTime(1.0), NetId(0), LogicValue::One);
        queue.schedule(SimTime(1.0), NetId(1), LogicValue::Zero);
        queue.schedule(SimTime(1.0), NetId(2), LogicValue::Z);

        let trace = queue.run_until(SimTime(1.0));
        net_state.apply_trace(&trace);

        assert_eq!(net_state.len(), 3);
        let cp = checkpoint_kernel(&queue, &net_state);

        // Add more events
        queue.schedule(SimTime(2.0), NetId(0), LogicValue::X);
        let trace2 = queue.run_until(SimTime(2.0));
        net_state.apply_trace(&trace2);

        // Rollback
        restore_kernel(&mut queue, &mut net_state, cp);

        assert_eq!(net_state.get(NetId(0)), Some(LogicValue::One));
        assert_eq!(net_state.get(NetId(1)), Some(LogicValue::Zero));
        assert_eq!(net_state.get(NetId(2)), Some(LogicValue::Z));
        assert_eq!(queue.pending_count(), 0);
    }

    #[test]
    fn net_state_default_trait() {
        let ns = NetState::default();
        assert!(ns.is_empty());
    }
}
