//! Event queue with an in-process `run-until` API.
//!
//! Implements the core of the Native Digital Kernel (ADR-0006): a DEVS-style
//! event queue that processes scheduled events in nondecreasing time order.
//! The Mixed-Signal Scheduler drives this kernel via the in-process `run_until`
//! method — no cross-process IPC.
//!
//! # Spec traceability
//!
//! - `digital-engine#native-kernel-event-queue`: the kernel advances by
//!   processing the event queue in nondecreasing time order.
//!
//! # Shared contract
//!
//! This module produces the `digital.DigitalKernel` contract: an in-process
//! run-until event queue the scheduler drives (ratified by ADR-0006).

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// ---------------------------------------------------------------------------
// Simulation time
// ---------------------------------------------------------------------------

/// Simulation time expressed as a real-valued quantity (seconds).
///
/// Uses a newtype so we can enforce ordering semantics and later swap the
/// representation (e.g. fixed-point) without changing the API.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SimTime(pub f64);

impl SimTime {
    /// The simulation origin (t = 0).
    pub const ZERO: SimTime = SimTime(0.0);

    /// Returns the inner f64 value.
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl std::fmt::Display for SimTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}

// ---------------------------------------------------------------------------
// Net identifiers and signal values
// ---------------------------------------------------------------------------

/// Identifier for a digital net (wire / signal) within the kernel.
///
/// Opaque 32-bit index — the kernel does not prescribe how nets are named;
/// that is the frontend's responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetId(pub u32);

impl std::fmt::Display for NetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "net{}", self.0)
    }
}

/// Digital signal value on a net.
///
/// For the event queue core we model the standard four-state logic: 0, 1, X
/// (unknown / conflict), and Z (high-impedance). Resolution is outside the
/// queue's scope — it belongs in the settle module (task #12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicValue {
    /// Logic low.
    Zero,
    /// Logic high.
    One,
    /// Unknown / conflict (used during settling).
    X,
    /// High-impedance.
    Z,
}

impl std::fmt::Display for LogicValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicValue::Zero => write!(f, "0"),
            LogicValue::One => write!(f, "1"),
            LogicValue::X => write!(f, "X"),
            LogicValue::Z => write!(f, "Z"),
        }
    }
}

// ---------------------------------------------------------------------------
// Scheduled event
// ---------------------------------------------------------------------------

/// A scheduled event: at `time`, assign `value` to `net`.
///
/// Events are ordered by time (nondecreasing); ties are broken by insertion
/// order so that events scheduled earlier in wall-clock time fire first
/// (first-scheduled, first-served within a delta).
#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    /// Simulation time at which this event fires.
    pub time: SimTime,
    /// Target net.
    pub net: NetId,
    /// New value to assign.
    pub value: LogicValue,
    /// Monotonic insertion counter — breaks ties within the same SimTime.
    seq: u64,
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.seq == other.seq
    }
}

impl Eq for ScheduledEvent {}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledEvent {
    /// Reverse ordering so BinaryHeap (a max-heap) yields the *smallest*
    /// time first — i.e. it behaves as a min-heap by event time.
    fn cmp(&self, other: &Self) -> Ordering {
        // Primary: earlier time = higher priority (so reverse for max-heap).
        match other.time.partial_cmp(&self.time) {
            Some(Ordering::Equal) | None => {}
            Some(ord) => return ord,
        }
        // Secondary: lower seq = earlier scheduled = higher priority.
        other.seq.cmp(&self.seq)
    }
}

// ---------------------------------------------------------------------------
// Event trace entry (for spec-scenario verification & golden comparison)
// ---------------------------------------------------------------------------

/// A record of a processed event, used for event-trace equivalence checking.
///
/// The trace records (time, net, value) tuples in the order they were
/// processed by the kernel — this is the "ordered events" representation
/// required by `digital-equivalence#ordered-events-not-vcd`.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceEntry {
    /// Simulation time at which the event was processed.
    pub time: SimTime,
    /// Net that changed.
    pub net: NetId,
    /// Value assigned.
    pub value: LogicValue,
}

// ---------------------------------------------------------------------------
// Event queue (the core data structure)
// ---------------------------------------------------------------------------

/// DEVS-style event queue with in-process `run_until` API.
///
/// This is the core of the `digital.DigitalKernel` contract (ADR-0006). The
/// Mixed-Signal Scheduler calls `run_until(target_time)` to advance the
/// simulation; the kernel processes all events up to (and including) the
/// target time in nondecreasing order.
///
/// # Checkpoint / restore (task #13 integration)
///
/// The queue exposes `checkpoint()` and `restore()` methods so that the
/// optimistic rollback mechanism (superseding ADR-0004) can save and recover
/// queue + net state. The full checkpoint including net-value state lives in
/// `checkpoint.rs` (task #13); here we provide the primitives.
pub struct EventQueue {
    /// Priority queue of pending scheduled events (min-heap by time, then seq).
    heap: BinaryHeap<ScheduledEvent>,
    /// Monotonic insertion counter — next seq assigned to a new event.
    next_seq: u64,
    /// Current simulation time (the time of the last processed event, or
    /// `SimTime::ZERO` before any event is processed).
    current_time: SimTime,
    /// Accumulated trace of processed events (for equivalence checking).
    trace: Vec<TraceEntry>,
    /// Maximum delta cycles allowed at a single time point before oscillation
    /// is reported. Zero-delay settling (task #12) will use this.
    max_delta_cycles: u32,
}

impl EventQueue {
    /// Create a new, empty event queue.
    ///
    /// `max_delta_cycles` is the maximum number of zero-delay (same-time)
    /// event rounds before the kernel reports an oscillation instead of
    /// looping forever. A reasonable default is 100.
    pub fn new(max_delta_cycles: u32) -> Self {
        EventQueue {
            heap: BinaryHeap::new(),
            next_seq: 0,
            current_time: SimTime::ZERO,
            trace: Vec::new(),
            max_delta_cycles,
        }
    }

    /// Schedule an event: at `time`, assign `value` to `net`.
    ///
    /// The event is inserted into the priority queue and will be processed
    /// when the kernel advances past `time` via `run_until` or `step`.
    pub fn schedule(&mut self, time: SimTime, net: NetId, value: LogicValue) {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.heap.push(ScheduledEvent {
            time,
            net,
            value,
            seq,
        });
    }

    /// Current simulation time.
    pub fn current_time(&self) -> SimTime {
        self.current_time
    }

    /// Number of pending events in the queue.
    pub fn pending_count(&self) -> usize {
        self.heap.len()
    }

    /// Peek at the next event time without processing it.
    ///
    /// Returns `None` if the queue is empty.
    pub fn next_event_time(&self) -> Option<SimTime> {
        self.heap.peek().map(|e| e.time)
    }

    /// Process a single event (the earliest pending one).
    ///
    /// Returns `Some(TraceEntry)` if an event was processed, or `None` if
    /// the queue is empty. Advances `current_time` to the processed event's
    /// time.
    pub fn step(&mut self) -> Option<TraceEntry> {
        let event = self.heap.pop()?;
        self.current_time = event.time;
        let entry = TraceEntry {
            time: event.time,
            net: event.net,
            value: event.value,
        };
        self.trace.push(entry.clone());
        Some(entry)
    }

    /// Run the event queue until `target_time`, processing all events whose
    /// time is ≤ target_time, in nondecreasing time order.
    ///
    /// This is the primary API the Mixed-Signal Scheduler uses to drive the
    /// digital kernel (in-process, no IPC — per ADR-0006).
    ///
    /// Returns the trace entries for all events processed during this call.
    /// If the queue is empty or the next event is past `target_time`, returns
    /// an empty vec without advancing time.
    ///
    /// # Spec traceability
    ///
    /// - `digital-engine#native-kernel-event-queue`: "the kernel is run until
    ///   a target simulation time" → this method.
    pub fn run_until(&mut self, target_time: SimTime) -> Vec<TraceEntry> {
        let mut processed = Vec::new();
        while let Some(next_time) = self.next_event_time() {
            if next_time > target_time {
                break;
            }
            if let Some(entry) = self.step() {
                processed.push(entry);
            }
        }
        // If we processed at least one event, current_time was advanced.
        // If nothing was processed, current_time stays where it was.
        processed
    }

    /// Run the event queue until it is empty (process all pending events).
    ///
    /// Useful for draining the queue in tests or when the target is
    /// "end of simulation".
    pub fn run_to_completion(&mut self) -> Vec<TraceEntry> {
        let mut processed = Vec::new();
        while let Some(entry) = self.step() {
            processed.push(entry);
        }
        processed
    }

    /// Access the accumulated event trace (all events processed so far).
    pub fn trace(&self) -> &[TraceEntry] {
        &self.trace
    }

    /// Maximum delta cycles before oscillation detection kicks in.
    pub fn max_delta_cycles(&self) -> u32 {
        self.max_delta_cycles
    }

    // -----------------------------------------------------------------------
    // Checkpoint / restore primitives (for task #13)
    // -----------------------------------------------------------------------

    /// Capture a checkpoint of the event queue's state.
    ///
    /// The checkpoint includes all pending events, the monotonic counter,
    /// current time, and the accumulated trace. This is sufficient for the
    /// optimistic rollback mechanism to restore the kernel to an earlier
    /// state.
    ///
    /// # Spec traceability
    ///
    /// - `digital-engine#native-kernel-optimistic-rollback`: "the kernel
    ///   restores its event queue and net state to the checkpoint" →
    ///   `restore()` uses the data captured here.
    pub fn checkpoint(&self) -> EventQueueCheckpoint {
        EventQueueCheckpoint {
            heap: self.heap.clone(),
            next_seq: self.next_seq,
            current_time: self.current_time,
            trace: self.trace.clone(),
        }
    }

    /// Restore the event queue from a previously captured checkpoint.
    ///
    /// After this call, the queue is in exactly the state it was when
    /// `checkpoint()` was called. Any events processed since then are
    /// discarded.
    pub fn restore(&mut self, checkpoint: EventQueueCheckpoint) {
        self.heap = checkpoint.heap;
        self.next_seq = checkpoint.next_seq;
        self.current_time = checkpoint.current_time;
        self.trace = checkpoint.trace;
    }
}

/// Snapshot of an `EventQueue`'s state for checkpoint/restore.
///
/// Opaque outside the digital module; produced by `EventQueue::checkpoint()`
/// and consumed by `EventQueue::restore()`.
#[derive(Debug, Clone)]
pub struct EventQueueCheckpoint {
    heap: BinaryHeap<ScheduledEvent>,
    next_seq: u64,
    current_time: SimTime,
    trace: Vec<TraceEntry>,
}

// ---------------------------------------------------------------------------
// DigitalKernel contract (digital.DigitalKernel — ADR-0006)
// ---------------------------------------------------------------------------

/// The `digital.DigitalKernel` shared contract.
///
/// This trait defines the interface that the Mixed-Signal Scheduler (task #16)
/// uses to drive the digital kernel. The concrete `EventQueue` implements it;
/// the trait exists so the scheduler can mock or substitute the kernel.
///
/// # Contract owner
///
/// - Component: `digital`
/// - Ratified by: ADR-0006
pub trait DigitalKernel {
    /// Schedule an event at the given time on the given net.
    fn schedule(&mut self, time: SimTime, net: NetId, value: LogicValue);

    /// Run the kernel until `target_time`, returning processed trace entries.
    fn run_until(&mut self, target_time: SimTime) -> Vec<TraceEntry>;

    /// Current simulation time inside the kernel.
    fn current_time(&self) -> SimTime;

    /// Capture a checkpoint for optimistic rollback.
    fn checkpoint(&self) -> EventQueueCheckpoint;

    /// Restore from a checkpoint (rollback).
    fn restore(&mut self, checkpoint: EventQueueCheckpoint);
}

impl DigitalKernel for EventQueue {
    fn schedule(&mut self, time: SimTime, net: NetId, value: LogicValue) {
        EventQueue::schedule(self, time, net, value);
    }

    fn run_until(&mut self, target_time: SimTime) -> Vec<TraceEntry> {
        EventQueue::run_until(self, target_time)
    }

    fn current_time(&self) -> SimTime {
        EventQueue::current_time(self)
    }

    fn checkpoint(&self) -> EventQueueCheckpoint {
        EventQueue::checkpoint(self)
    }

    fn restore(&mut self, checkpoint: EventQueueCheckpoint) {
        EventQueue::restore(self, checkpoint);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Helper: schedule events in a convenient shorthand ------------------

    fn sched(queue: &mut EventQueue, time: f64, net: u32, value: LogicValue) {
        queue.schedule(SimTime(time), NetId(net), value);
    }

    // -- Spec scenario: native-kernel-event-queue ---------------------------

    #[test]
    fn events_processed_in_nondecreasing_time_order() {
        // Given a gate-level testbench loaded into the native kernel
        // (modelled as a set of scheduled events)
        let mut q = EventQueue::new(100);

        // Schedule events out of order
        sched(&mut q, 3.0, 0, LogicValue::One);
        sched(&mut q, 1.0, 1, LogicValue::Zero);
        sched(&mut q, 2.0, 2, LogicValue::One);
        sched(&mut q, 1.0, 3, LogicValue::One); // same time as net 1

        // When the kernel is run until a target simulation time
        let trace = q.run_until(SimTime(5.0));

        // Then events are processed in nondecreasing time order
        assert_eq!(trace.len(), 4);
        assert!(
            trace.windows(2).all(|w| w[0].time <= w[1].time),
            "trace must be in nondecreasing time order"
        );

        // First two events are at t=1.0, then t=2.0, then t=3.0
        assert_eq!(trace[0].time, SimTime(1.0));
        assert_eq!(trace[1].time, SimTime(1.0));
        assert_eq!(trace[2].time, SimTime(2.0));
        assert_eq!(trace[3].time, SimTime(3.0));
    }

    #[test]
    fn run_until_stops_at_target_time() {
        let mut q = EventQueue::new(100);

        sched(&mut q, 1.0, 0, LogicValue::One);
        sched(&mut q, 5.0, 1, LogicValue::Zero);
        sched(&mut q, 10.0, 2, LogicValue::One);

        // Run until t=3 — should process only the t=1 event
        let trace = q.run_until(SimTime(3.0));
        assert_eq!(trace.len(), 1);
        assert_eq!(trace[0].time, SimTime(1.0));
        assert_eq!(q.current_time(), SimTime(1.0));

        // Run until t=7 — should process the t=5 event
        let trace2 = q.run_until(SimTime(7.0));
        assert_eq!(trace2.len(), 1);
        assert_eq!(trace2[0].time, SimTime(5.0));
        assert_eq!(q.current_time(), SimTime(5.0));

        // t=10 is still pending
        assert_eq!(q.pending_count(), 1);
    }

    #[test]
    fn run_until_empty_queue_is_noop() {
        let mut q = EventQueue::new(100);
        let trace = q.run_until(SimTime(10.0));
        assert!(trace.is_empty());
        assert_eq!(q.current_time(), SimTime::ZERO);
    }

    #[test]
    fn run_until_no_events_before_target() {
        let mut q = EventQueue::new(100);
        sched(&mut q, 20.0, 0, LogicValue::One);
        let trace = q.run_until(SimTime(10.0));
        assert!(trace.is_empty());
        assert_eq!(q.current_time(), SimTime::ZERO);
    }

    #[test]
    fn run_to_completion_processes_all() {
        let mut q = EventQueue::new(100);
        sched(&mut q, 1.0, 0, LogicValue::One);
        sched(&mut q, 5.0, 1, LogicValue::Zero);
        sched(&mut q, 3.0, 2, LogicValue::X);

        let trace = q.run_to_completion();
        assert_eq!(trace.len(), 3);
        assert!(trace.windows(2).all(|w| w[0].time <= w[1].time));
        assert_eq!(q.pending_count(), 0);
    }

    #[test]
    fn insertion_order_breaks_time_ties() {
        // Events at the same time should fire in insertion (scheduling) order.
        let mut q = EventQueue::new(100);
        sched(&mut q, 1.0, 10, LogicValue::One); // seq=0
        sched(&mut q, 1.0, 20, LogicValue::Zero); // seq=1
        sched(&mut q, 1.0, 30, LogicValue::X); // seq=2

        let trace = q.run_to_completion();
        assert_eq!(trace.len(), 3);
        assert_eq!(trace[0].net, NetId(10));
        assert_eq!(trace[1].net, NetId(20));
        assert_eq!(trace[2].net, NetId(30));
    }

    // -- Checkpoint / restore (primitives for task #13) ---------------------

    #[test]
    fn checkpoint_restore_reverts_to_saved_state() {
        let mut q = EventQueue::new(100);
        sched(&mut q, 1.0, 0, LogicValue::One);
        sched(&mut q, 2.0, 1, LogicValue::Zero);
        sched(&mut q, 3.0, 2, LogicValue::One);

        // Process first event
        q.run_until(SimTime(1.0));
        let cp = q.checkpoint();
        assert_eq!(q.current_time(), SimTime(1.0));
        assert_eq!(q.trace().len(), 1);

        // Process remaining events
        q.run_to_completion();
        assert_eq!(q.trace().len(), 3);

        // Restore — should go back to the checkpoint state
        q.restore(cp);
        assert_eq!(q.current_time(), SimTime(1.0));
        assert_eq!(q.trace().len(), 1);
        assert_eq!(q.pending_count(), 2); // t=2 and t=3 still pending
    }

    #[test]
    fn checkpoint_before_any_events() {
        let mut q = EventQueue::new(100);
        let cp = q.checkpoint();

        sched(&mut q, 1.0, 0, LogicValue::One);
        q.run_to_completion();
        assert_eq!(q.current_time(), SimTime(1.0));

        q.restore(cp);
        assert_eq!(q.current_time(), SimTime::ZERO);
        assert_eq!(q.pending_count(), 0);
        assert!(q.trace().is_empty());
    }

    // -- DigitalKernel trait contract ---------------------------------------

    #[test]
    fn digital_kernel_trait_dispatch() {
        // Verify the EventQueue implements DigitalKernel and can be used
        // through the trait object.
        let mut kernel: Box<dyn DigitalKernel> = Box::new(EventQueue::new(100));
        kernel.schedule(SimTime(5.0), NetId(0), LogicValue::One);
        kernel.schedule(SimTime(10.0), NetId(1), LogicValue::Zero);

        let trace = kernel.run_until(SimTime(7.0));
        assert_eq!(trace.len(), 1);
        assert_eq!(kernel.current_time(), SimTime(5.0));

        let cp = kernel.checkpoint();
        kernel.run_until(SimTime(20.0));
        assert_eq!(kernel.current_time(), SimTime(10.0));

        kernel.restore(cp);
        assert_eq!(kernel.current_time(), SimTime(5.0));
    }

    // -- Edge cases --------------------------------------------------------

    #[test]
    fn step_processes_one_event() {
        let mut q = EventQueue::new(100);
        sched(&mut q, 1.0, 0, LogicValue::One);
        sched(&mut q, 2.0, 1, LogicValue::Zero);

        let entry = q.step().unwrap();
        assert_eq!(entry.time, SimTime(1.0));
        assert_eq!(entry.net, NetId(0));
        assert_eq!(q.pending_count(), 1);
        assert_eq!(q.current_time(), SimTime(1.0));

        let entry2 = q.step().unwrap();
        assert_eq!(entry2.time, SimTime(2.0));

        assert!(q.step().is_none());
    }

    #[test]
    fn next_event_time_on_empty_queue() {
        let q = EventQueue::new(100);
        assert!(q.next_event_time().is_none());
    }

    #[test]
    fn trace_accumulates_across_run_until_calls() {
        let mut q = EventQueue::new(100);
        sched(&mut q, 1.0, 0, LogicValue::One);
        sched(&mut q, 2.0, 1, LogicValue::Zero);

        q.run_until(SimTime(1.0));
        assert_eq!(q.trace().len(), 1);

        q.run_until(SimTime(5.0));
        assert_eq!(q.trace().len(), 2);
    }
}
