//! Native digital kernel — the in-process event-driven engine mandated by ADR-0006.
//!
//! ADR-0006 ("Native Event-Driven Digital Engine") replaces external
//! co-simulation (ADR-0004) with a native, in-process DEVS-style event
//! queue. The [`DigitalKernel`] is the top-level type that the Mixed-Signal
//! Scheduler drives via `run_until` — no IPC, no external process.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │            DigitalKernel                     │
//! │                                              │
//! │  ┌─────────────┐    ┌──────────────────┐    │
//! │  │ EventQueue   │    │ NetState         │    │
//! │  │ (min-heap)   │    │ (net → value)   │    │
//! │  └──────┬──────┘    └────────┬─────────┘    │
//! │         │                    │               │
//! │         └────────┬───────────┘              │
//! │                  │                          │
//! │         run_until(target)                    │
//! │         → RunUntilReport                    │
//! └─────────────────────────────────────────────┘
//! ```
//!
//! - [`EventQueue`] — binary min-heap of time-ordered events.
//! - [`NetState`] — current value of every digital net (wire).
//! - [`DigitalKernel`] — composes the two, exposes the `run_until` API.
//!
//! # Integration with analysis-orchestration
//!
//! The kernel is designed to implement the `DigitalSimulator` trait
//! (defined in `analysis-orchestration::mixed_signal`). Task #17 will
//! wire that impl; this crate provides the standalone kernel so that
//! task #12 (delta-cycle settling) and task #13 (checkpoint/restore)
//! can build on it without depending on the orchestration crate.

use circuit_solver_types::SimulationTime;
use core::fmt;

use crate::event_queue::{
    DigitalEvent, EventQueue, EventQueueCheckpoint, LogicValue, NetId,
};

// ---------------------------------------------------------------------------
// Net state
// ---------------------------------------------------------------------------

/// The current value of every digital net (wire) in the kernel.
///
/// Net state is updated as events are processed during `run_until`.
/// After processing, the net state reflects the values that combinational
/// settling (task #12) would compute. The kernel's checkpoint/restore
/// mechanism (task #13) snapshots net state alongside the event queue.
#[derive(Debug, Clone, PartialEq)]
pub struct NetState {
    /// Net values indexed by [`NetId::index`].
    values: Vec<LogicValue>,
}

impl Default for NetState {
    fn default() -> Self {
        Self::new()
    }
}

impl NetState {
    /// Create an empty net state.
    #[must_use]
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Create a net state pre-allocated for `n` nets, all initialized
    /// to [`LogicValue::Unknown`].
    #[must_use]
    pub fn with_nets(n: usize) -> Self {
        Self {
            values: vec![LogicValue::Unknown; n],
        }
    }

    /// Get the current value of net `id`.
    ///
    /// Returns [`LogicValue::Unknown`] for nets that have not been
    /// assigned.
    #[must_use]
    pub fn get(&self, id: NetId) -> LogicValue {
        self.values.get(id.index() as usize).copied().unwrap_or(LogicValue::Unknown)
    }

    /// Set the value of net `id`. Grows the internal vector if needed.
    pub fn set(&mut self, id: NetId, value: LogicValue) {
        let idx = id.index() as usize;
        if idx >= self.values.len() {
            self.values.resize(idx + 1, LogicValue::Unknown);
        }
        self.values[idx] = value;
    }

    /// Number of nets tracked (may include unassigned slots).
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// True iff no nets are tracked.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Checkpoint the net state for rollback.
    #[must_use]
    pub fn checkpoint(&self) -> NetStateCheckpoint {
        NetStateCheckpoint {
            values: self.values.clone(),
        }
    }

    /// Restore from a previously captured checkpoint.
    pub fn restore_from_checkpoint(&mut self, cp: NetStateCheckpoint) {
        self.values = cp.values;
    }
}

/// A snapshot of [`NetState`] for rollback (task #13).
#[derive(Debug, Clone, PartialEq)]
pub struct NetStateCheckpoint {
    values: Vec<LogicValue>,
}

// ---------------------------------------------------------------------------
// Kernel checkpoint
// ---------------------------------------------------------------------------

/// A combined checkpoint of the [`DigitalKernel`]'s event queue and net
/// state, sufficient for the optimistic rollback mechanism (task #13).
///
/// Produced by [`DigitalKernel::checkpoint`] and consumed by
/// [`DigitalKernel::restore_from_checkpoint`].
#[derive(Debug, Clone, PartialEq)]
pub struct KernelCheckpoint {
    /// Snapshot of the event queue.
    pub queue: EventQueueCheckpoint,
    /// Snapshot of the net state.
    pub net_state: NetStateCheckpoint,
}

// ---------------------------------------------------------------------------
// Digital kernel
// ---------------------------------------------------------------------------

/// The native, in-process event-driven digital kernel (ADR-0006).
///
/// The kernel composes an [`EventQueue`] (time-ordered event scheduling)
/// with a [`NetState`] (current value of every digital net). The
/// Mixed-Signal Scheduler drives the kernel via [`run_until`] — no IPC,
/// no external process.
///
/// # In-process run-until API
///
/// [`run_until(target)`] advances the kernel's simulation clock to
/// `target`, processing all scheduled events at or before `target`.
/// For each event, the kernel:
///
/// 1. Pops the event from the queue,
/// 2. Updates the net state (`net_state.set(net, value)`),
/// 3. Records the event in the processed-events trace.
///
/// Future tasks (#12, #13) will add:
/// - Delta-cycle combinational settling after each event,
/// - Checkpoint/restore for the optimistic rollback scheduler.
///
/// [`run_until`]: DigitalKernel::run_until
/// [`run_until(target)`]: DigitalKernel::run_until
#[derive(Debug, Clone)]
pub struct DigitalKernel {
    /// The event queue (scheduling and processing).
    queue: EventQueue,
    /// The current value of every digital net.
    net_state: NetState,
}

impl Default for DigitalKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalKernel {
    /// Create a new kernel with the simulation clock at t=0 and no
    /// nets or events.
    #[must_use]
    pub fn new() -> Self {
        Self {
            queue: EventQueue::new(),
            net_state: NetState::new(),
        }
    }

    /// Create a kernel pre-allocated for `n` nets, all initialized
    /// to [`LogicValue::Unknown`].
    #[must_use]
    pub fn with_nets(n: usize) -> Self {
        Self {
            queue: EventQueue::new(),
            net_state: NetState::with_nets(n),
        }
    }

    // ----- Queries -----

    /// The current simulation clock value.
    #[must_use]
    pub fn current_time(&self) -> SimulationTime {
        self.queue.current_time()
    }

    /// The earliest scheduled event time, or `None` if no events
    /// are pending.
    #[must_use]
    pub fn next_event_time(&self) -> Option<SimulationTime> {
        self.queue.next_event_time()
    }

    /// Get the current value of net `id`.
    #[must_use]
    pub fn net_value(&self, id: NetId) -> LogicValue {
        self.net_state.get(id)
    }

    /// Number of pending events in the queue.
    #[must_use]
    pub fn pending_event_count(&self) -> usize {
        self.queue.pending_count()
    }

    /// True iff no pending events remain.
    #[must_use]
    pub fn is_idle(&self) -> bool {
        self.queue.is_empty()
    }

    /// Reference to the underlying event queue (for inspection).
    #[must_use]
    pub fn queue(&self) -> &EventQueue {
        &self.queue
    }

    /// Reference to the underlying net state (for inspection).
    #[must_use]
    pub fn net_state(&self) -> &NetState {
        &self.net_state
    }

    // ----- Scheduling -----

    /// Schedule an event on the kernel's event queue.
    ///
    /// The event's net and value will be applied when the event is
    /// processed during [`run_until`].
    ///
    /// # Errors
    ///
    /// Returns [`crate::event_queue::EventQueueError::TimeTravel`]
    /// if the event time is before the current simulation clock.
    ///
    /// [`run_until`]: DigitalKernel::run_until
    pub fn schedule(&mut self, event: DigitalEvent) -> Result<(), crate::event_queue::EventQueueError> {
        self.queue.schedule(event)
    }

    // ----- In-process run-until -----

    /// Advance the kernel to `target`, processing all events at or
    /// before `target`.
    ///
    /// This is the **in-process run-until API** that ADR-0006 mandates.
    /// The Mixed-Signal Scheduler calls this directly — no IPC, no
    /// external process.
    ///
    /// For each event processed, the kernel:
    /// 1. Updates the net state (`net_state.set(event.net, event.value)`),
    /// 2. Records the event for trace assembly.
    ///
    /// Returns a [`KernelRunReport`] describing:
    /// - `time_reached`: the clock value after advance (equals `target`),
    /// - `events_processed`: all events processed during this call,
    /// - `next_event_time`: the next scheduled event after `target`, if any.
    ///
    /// # Panics
    ///
    /// Panics if `target` is before the current simulation clock.
    pub fn run_until(&mut self, target: SimulationTime) -> KernelRunReport {
        let report = self.queue.run_until(target);

        // Apply each processed event to the net state.
        for event in &report.events_processed {
            self.net_state.set(event.net, event.value);
        }

        KernelRunReport {
            time_reached: report.time_reached,
            events_processed: report.events_processed,
            next_event_time: report.next_event_time,
        }
    }

    // ----- Trace -----

    /// Drain the accumulated processed events, returning them in
    /// processing order.
    ///
    /// Called at end-of-run to assemble the digital event trace.
    #[must_use]
    pub fn take_processed_events(&mut self) -> Vec<DigitalEvent> {
        self.queue.take_processed_events()
    }

    // ----- Checkpoint / restore (task #13 foundation) -----

    /// Checkpoint the kernel's complete state (event queue + net state)
    /// for the optimistic rollback mechanism.
    #[must_use]
    pub fn checkpoint(&self) -> KernelCheckpoint {
        KernelCheckpoint {
            queue: self.queue.checkpoint(),
            net_state: self.net_state.checkpoint(),
        }
    }

    /// Restore the kernel to a previously captured checkpoint.
    ///
    /// Used by the rollback mechanism (task #13) to reset the kernel
    /// to a known-good state after a misprediction.
    pub fn restore_from_checkpoint(&mut self, cp: KernelCheckpoint) {
        self.queue.restore_from_checkpoint(cp.queue);
        self.net_state.restore_from_checkpoint(cp.net_state);
    }
}

// ---------------------------------------------------------------------------
// Kernel run report
// ---------------------------------------------------------------------------

/// Report returned by [`DigitalKernel::run_until`] describing what
/// happened during the advance.
#[derive(Debug, Clone, PartialEq)]
pub struct KernelRunReport {
    /// The simulation time the kernel was advanced to. Equals `target`
    /// on the normal path.
    pub time_reached: SimulationTime,
    /// Events processed during this `run_until` call, in the order
    /// they were processed.
    pub events_processed: Vec<DigitalEvent>,
    /// The next scheduled event time after `target`, if any.
    pub next_event_time: Option<SimulationTime>,
}

impl fmt::Display for KernelRunReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "run_until(t={}): {} events processed, next_event={}",
            self.time_reached,
            self.events_processed.len(),
            match self.next_event_time {
                Some(t) => format!("Some({t})"),
                None => "None".to_string(),
            }
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_kernel_starts_at_zero() {
        let k = DigitalKernel::new();
        assert_eq!(k.current_time(), SimulationTime::ZERO);
        assert!(k.is_idle());
        assert_eq!(k.next_event_time(), None);
    }

    #[test]
    fn with_nets_initializes_unknown() {
        let k = DigitalKernel::with_nets(4);
        for i in 0..4u32 {
            assert_eq!(k.net_value(NetId::new(i)), LogicValue::Unknown);
        }
    }

    #[test]
    fn schedule_and_run_until_applies_net_state() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let net_a = NetId::new(0);
        let net_b = NetId::new(1);

        k.schedule(DigitalEvent::new(t50, net_a, LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t50, net_b, LogicValue::Zero))
            .unwrap();

        let report = k.run_until(t50);
        assert_eq!(report.time_reached, t50);
        assert_eq!(report.events_processed.len(), 2);
        assert_eq!(k.net_value(net_a), LogicValue::One);
        assert_eq!(k.net_value(net_b), LogicValue::Zero);
    }

    #[test]
    fn run_until_partial_advance() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t100, NetId::new(1), LogicValue::Zero))
            .unwrap();

        // Advance to 50 ns: only the first event should fire.
        let report = k.run_until(t50);
        assert_eq!(report.events_processed.len(), 1);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::One);
        // Net 1 still unknown (not yet processed).
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Unknown);
        assert_eq!(report.next_event_time, Some(t100));
    }

    #[test]
    fn checkpoint_and_restore_roundtrip() {
        let mut k = DigitalKernel::with_nets(2);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        k.schedule(DigitalEvent::new(t100, NetId::new(1), LogicValue::Zero))
            .unwrap();

        // Checkpoint before running.
        let cp = k.checkpoint();

        // Run to 100 ns.
        let _ = k.run_until(t100);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::One);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Zero);
        assert_eq!(k.current_time(), t100);

        // Restore: kernel should be back to t=0 with 2 pending events.
        k.restore_from_checkpoint(cp);
        assert_eq!(k.current_time(), SimulationTime::ZERO);
        assert_eq!(k.pending_event_count(), 2);
        assert_eq!(k.net_value(NetId::new(0)), LogicValue::Unknown);
        assert_eq!(k.net_value(NetId::new(1)), LogicValue::Unknown);
    }

    #[test]
    fn take_processed_events_drains() {
        let mut k = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        k.schedule(DigitalEvent::new(t50, NetId::new(0), LogicValue::One))
            .unwrap();
        let _ = k.run_until(t50);

        let events = k.take_processed_events();
        assert_eq!(events.len(), 1);
        assert!(k.take_processed_events().is_empty());
    }

    #[test]
    fn net_state_auto_grows_on_set() {
        let mut ns = NetState::new();
        assert!(ns.is_empty());
        ns.set(NetId::new(5), LogicValue::One);
        assert_eq!(ns.len(), 6); // indices 0..=5
        assert_eq!(ns.get(NetId::new(5)), LogicValue::One);
        // Uninitialized slots are Unknown.
        assert_eq!(ns.get(NetId::new(0)), LogicValue::Unknown);
    }

    #[test]
    fn kernel_run_report_display() {
        let report = KernelRunReport {
            time_reached: SimulationTime::from_nanoseconds(50),
            events_processed: vec![DigitalEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(0),
                LogicValue::One,
            )],
            next_event_time: Some(SimulationTime::from_nanoseconds(100)),
        };
        let s = format!("{report}");
        assert!(s.contains("1 events processed"));
        assert!(s.contains("next_event=Some"));
    }
}
