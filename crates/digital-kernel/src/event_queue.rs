//! Event queue for the native digital simulation kernel.
//!
//! ADR-0006 ("Native Event-Driven Digital Engine") mandates a DEVS-style
//! event queue as the core scheduling primitive for the in-process digital
//! kernel. The queue maintains a time-ordered priority queue of scheduled
//! events and exposes a `run_until` API that the Mixed-Signal Scheduler
//! drives directly — no IPC, no external process.
//!
//! # Design
//!
//! The event queue is a binary min-heap keyed on [`SimulationTime`]. Events
//! are scheduled via [`EventQueue::schedule`] and drained in time order via
//! [`EventQueue::run_until`]. The `run_until` method advances the simulation
//! clock to `target`, processing all events at or before `target` and
//! returning a [`RunUntilReport`] describing what happened.
//!
//! # Invariants
//!
//! - Events are processed in non-decreasing time order.
//! - The simulation clock is monotonically non-decreasing.
//! - Events scheduled at the same time are processed in FIFO order
//!   (tie-breaking via sequence counter).

use circuit_solver_types::SimulationTime;
use core::fmt;
use std::collections::BinaryHeap;
use std::fmt::Write;

// ---------------------------------------------------------------------------
// Event types
// ---------------------------------------------------------------------------

/// A digital signal value — four-valued logic (0, 1, X, Z) per IEEE 1164.
///
/// The native kernel uses four-valued logic at its core so that
/// uninitialized and high-impedance states are representable from the
/// start. Downstream consumers (VCD writer, event-trace equivalence)
/// map these to their own representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicValue {
    /// Logic low.
    Zero,
    /// Logic high.
    One,
    /// Unknown / uninitialized.
    Unknown,
    /// High impedance.
    HighImpedance,
}

impl fmt::Display for LogicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Unknown => 'X',
            Self::HighImpedance => 'Z',
        };
        f.write_char(c)
    }
}

/// Identifier for a digital net (wire) in the native kernel.
///
/// Like [`circuit_solver_types::NodeId`] for the analog side, this is a
/// stable u32 index assigned during elaboration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetId(u32);

impl NetId {
    /// Construct from a raw u32 index.
    #[must_use]
    pub const fn new(idx: u32) -> Self {
        Self(idx)
    }

    /// Unwrap to a raw u32.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }
}

impl fmt::Display for NetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "net:{}", self.0)
    }
}

/// A scheduled event in the digital event queue.
///
/// An event is a (time, net, value) tuple: at `time`, the net `net`
/// transitions to `value`. The event queue processes these in time order,
/// and the kernel evaluates combinational logic after each event (delta-
/// cycle settling, task #12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalEvent {
    /// The time at which this event is scheduled to occur.
    pub time: SimulationTime,
    /// The net (wire) that transitions.
    pub net: NetId,
    /// The new value the net transitions to.
    pub value: LogicValue,
    /// Tie-breaking sequence counter for FIFO ordering within
    /// the same time step.
    seq: u64,
}

impl DigitalEvent {
    /// Construct a new event.
    #[must_use]
    pub fn new(time: SimulationTime, net: NetId, value: LogicValue) -> Self {
        Self {
            time,
            net,
            value,
            seq: 0, // overridden by EventQueue::schedule
        }
    }
}

// BinaryHeap is a max-heap; we need min-heap ordering by (time, seq).
impl PartialOrd for DigitalEvent {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DigitalEvent {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Reverse ordering: smaller (time, seq) = higher priority = "greater"
        // for the max-heap so it pops first.
        match other.time.cmp(&self.time) {
            core::cmp::Ordering::Equal => other.seq.cmp(&self.seq),
            ord => ord,
        }
    }
}

// ---------------------------------------------------------------------------
// Run-until report
// ---------------------------------------------------------------------------

/// Report returned by [`EventQueue::run_until`] describing what happened
/// during the advance.
#[derive(Debug, Clone, PartialEq)]
pub struct RunUntilReport {
    /// The simulation time the queue was advanced to. Equals `target`
    /// on the normal path.
    pub time_reached: SimulationTime,
    /// Events processed during this `run_until` call, in the order
    /// they were processed.
    pub events_processed: Vec<DigitalEvent>,
    /// The next scheduled event time after `target`, if any.
    /// The scheduler uses this for its `next_event_time` query.
    pub next_event_time: Option<SimulationTime>,
}

// ---------------------------------------------------------------------------
// Event queue errors
// ---------------------------------------------------------------------------

/// Errors from the event queue.
#[derive(Debug, Clone, PartialEq)]
pub enum EventQueueError {
    /// An event was scheduled at a time before the current simulation
    /// clock (time-travel violation).
    TimeTravel {
        /// The current simulation clock.
        current_time: SimulationTime,
        /// The attempted event time.
        attempted: SimulationTime,
    },
}

impl fmt::Display for EventQueueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeTravel {
                current_time,
                attempted,
            } => write!(
                f,
                "event at {attempted} is before current simulation time {current_time}"
            ),
        }
    }
}

impl std::error::Error for EventQueueError {}

// ---------------------------------------------------------------------------
// Event queue
// ---------------------------------------------------------------------------

/// DEVS-style event queue for the native digital kernel.
///
/// The queue maintains a binary min-heap of scheduled events keyed on
/// [`SimulationTime`] and a monotonically advancing simulation clock.
/// Events are scheduled via [`schedule`] and processed in time order
/// via [`run_until`].
///
/// # In-process run-until API (ADR-0006)
///
/// The Mixed-Signal Scheduler drives this queue directly — no IPC.
/// `run_until(target)` advances the clock to `target`, processing all
/// events at or before `target`, and returns a [`RunUntilReport`]
/// describing what happened. The report's `next_event_time` field lets
/// the scheduler predict the next synchronization boundary.
///
/// [`schedule`]: EventQueue::schedule
/// [`run_until`]: EventQueue::run_until
#[derive(Debug, Clone)]
pub struct EventQueue {
    /// Binary min-heap of scheduled events.
    heap: BinaryHeap<DigitalEvent>,
    /// Monotonically advancing simulation clock.
    current_time: SimulationTime,
    /// Monotonically increasing sequence counter for FIFO tie-breaking
    /// within the same time step.
    next_seq: u64,
    /// Record of all events processed since construction (for trace
    /// assembly). Cleared by [`take_processed_events`].
    ///
    /// [`take_processed_events`]: EventQueue::take_processed_events
    processed_events: Vec<DigitalEvent>,
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl EventQueue {
    /// Create an empty event queue with the clock at t=0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            current_time: SimulationTime::ZERO,
            next_seq: 0,
            processed_events: Vec::new(),
        }
    }

    /// The current simulation clock value.
    #[must_use]
    pub fn current_time(&self) -> SimulationTime {
        self.current_time
    }

    /// Number of events currently in the queue (not yet processed).
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.heap.len()
    }

    /// True iff no pending events remain.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Schedule an event for time `event.time` on net `event.net`.
    ///
    /// The event's internal sequence counter is assigned automatically
    /// for FIFO ordering within the same time step.
    ///
    /// # Errors
    ///
    /// Returns [`EventQueueError::TimeTravel`] if `event.time` is
    /// before the current simulation clock.
    pub fn schedule(&mut self, mut event: DigitalEvent) -> Result<(), EventQueueError> {
        if event.time < self.current_time {
            return Err(EventQueueError::TimeTravel {
                current_time: self.current_time,
                attempted: event.time,
            });
        }
        event.seq = self.next_seq;
        self.next_seq += 1;
        self.heap.push(event);
        Ok(())
    }

    /// The earliest scheduled event time, or `None` if the queue is empty.
    ///
    /// The scheduler calls this to predict the next synchronization
    /// boundary.
    #[must_use]
    pub fn next_event_time(&self) -> Option<SimulationTime> {
        self.heap.peek().map(|e| e.time)
    }

    /// Advance the simulation clock to `target`, processing all events
    /// at or before `target`.
    ///
    /// This is the **in-process run-until API** that ADR-0006 mandates.
    /// The Mixed-Signal Scheduler calls this directly — no IPC.
    ///
    /// Returns a [`RunUntilReport`] describing:
    /// - `time_reached`: the clock value after advance (equals `target`),
    /// - `events_processed`: all events processed during this call,
    /// - `next_event_time`: the next scheduled event after `target`, if any.
    ///
    /// # Panics
    ///
    /// Panics if `target` is before the current simulation clock
    /// (time-travel). Use [`can_advance_to`] to check first.
    ///
    /// [`can_advance_to`]: EventQueue::can_advance_to
    pub fn run_until(&mut self, target: SimulationTime) -> RunUntilReport {
        assert!(
            target >= self.current_time,
            "run_until target {target} is before current time {}",
            self.current_time
        );

        let mut events_processed = Vec::new();

        while let Some(event) = self.heap.peek() {
            if event.time > target {
                break;
            }
            let event = self
                .heap
                .pop()
                .expect("peek succeeded, pop must succeed");
            self.current_time = event.time;
            self.processed_events.push(event.clone());
            events_processed.push(event);
        }

        // Advance the clock to target even if no events were processed.
        self.current_time = target;

        RunUntilReport {
            time_reached: target,
            events_processed,
            next_event_time: self.next_event_time(),
        }
    }

    /// Check whether `target` is a valid `run_until` target (i.e., not
    /// before the current simulation clock).
    #[must_use]
    pub fn can_advance_to(&self, target: SimulationTime) -> bool {
        target >= self.current_time
    }

    /// Drain all processed events, returning them in processing order.
    ///
    /// Called by the kernel's `take_trace` implementation to assemble
    /// the digital event trace for the [`MixedSignalResult`].
    pub fn take_processed_events(&mut self) -> Vec<DigitalEvent> {
        std::mem::take(&mut self.processed_events)
    }

    /// Checkpoint the queue's current state for rollback (task #13).
    ///
    /// Returns a snapshot containing the pending events, the current
    /// clock, and the sequence counter, sufficient to restore the queue
    /// to this exact state via [`restore_from_checkpoint`].
    ///
    /// [`restore_from_checkpoint`]: EventQueue::restore_from_checkpoint
    #[must_use]
    pub fn checkpoint(&self) -> EventQueueCheckpoint {
        EventQueueCheckpoint {
            pending: self.heap.iter().cloned().collect(),
            current_time: self.current_time,
            next_seq: self.next_seq,
            processed_events: self.processed_events.clone(),
        }
    }

    /// Restore the queue to a previously captured checkpoint.
    ///
    /// Used by the rollback mechanism (task #13) to reset the queue
    /// to a known-good state after a misprediction.
    pub fn restore_from_checkpoint(&mut self, cp: EventQueueCheckpoint) {
        self.heap = cp.pending.into_iter().collect();
        self.current_time = cp.current_time;
        self.next_seq = cp.next_seq;
        self.processed_events = cp.processed_events;
    }
}

// ---------------------------------------------------------------------------
// Checkpoint
// ---------------------------------------------------------------------------

/// A snapshot of the [`EventQueue`] state, sufficient for rollback.
///
/// Produced by [`EventQueue::checkpoint`] and consumed by
/// [`EventQueue::restore_from_checkpoint`].
#[derive(Debug, Clone, PartialEq)]
pub struct EventQueueCheckpoint {
    /// Pending events in the heap at checkpoint time.
    pub pending: Vec<DigitalEvent>,
    /// The simulation clock at checkpoint time.
    pub current_time: SimulationTime,
    /// The sequence counter at checkpoint time.
    pub next_seq: u64,
    /// Events processed before checkpoint time (for trace recovery).
    pub processed_events: Vec<DigitalEvent>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_queue_has_no_next_event() {
        let q = EventQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.next_event_time(), None);
        assert_eq!(q.current_time(), SimulationTime::ZERO);
    }

    #[test]
    fn schedule_and_next_event_time() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        q.schedule(DigitalEvent::new(t50, NetId::new(1), LogicValue::One))
            .unwrap();
        assert_eq!(q.next_event_time(), Some(t50));
        assert_eq!(q.pending_count(), 1);
    }

    #[test]
    fn time_travel_rejected() {
        let mut q = EventQueue::new();
        // Advance clock to 50 ns first.
        let _ = q.run_until(SimulationTime::from_nanoseconds(50));
        // Then try scheduling at 30 ns — must fail.
        let result = q.schedule(DigitalEvent::new(
            SimulationTime::from_nanoseconds(30),
            NetId::new(1),
            LogicValue::One,
        ));
        assert!(matches!(result, Err(EventQueueError::TimeTravel { .. })));
    }

    #[test]
    fn run_until_processes_events_in_time_order() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);
        let t100 = SimulationTime::from_nanoseconds(100);

        q.schedule(DigitalEvent::new(t80, NetId::new(1), LogicValue::One))
            .unwrap();
        q.schedule(DigitalEvent::new(t50, NetId::new(2), LogicValue::Zero))
            .unwrap();
        q.schedule(DigitalEvent::new(t100, NetId::new(3), LogicValue::One))
            .unwrap();

        // Run to 80 ns: should process events at 50 ns and 80 ns.
        let report = q.run_until(t80);
        assert_eq!(report.time_reached, t80);
        assert_eq!(report.events_processed.len(), 2);
        // First processed: the 50 ns event.
        assert_eq!(report.events_processed[0].time, t50);
        assert_eq!(report.events_processed[0].net, NetId::new(2));
        // Second processed: the 80 ns event.
        assert_eq!(report.events_processed[1].time, t80);
        assert_eq!(report.events_processed[1].net, NetId::new(1));
        // Next event: 100 ns.
        assert_eq!(report.next_event_time, Some(t100));
    }

    #[test]
    fn run_until_with_no_events_advances_clock() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let report = q.run_until(t50);
        assert_eq!(report.time_reached, t50);
        assert!(report.events_processed.is_empty());
        assert_eq!(report.next_event_time, None);
        assert_eq!(q.current_time(), t50);
    }

    #[test]
    fn fifo_ordering_within_same_time_step() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);

        q.schedule(DigitalEvent::new(t50, NetId::new(1), LogicValue::One))
            .unwrap();
        q.schedule(DigitalEvent::new(t50, NetId::new(2), LogicValue::Zero))
            .unwrap();
        q.schedule(DigitalEvent::new(t50, NetId::new(3), LogicValue::Unknown))
            .unwrap();

        let report = q.run_until(t50);
        assert_eq!(report.events_processed.len(), 3);
        // FIFO: net 1, net 2, net 3.
        assert_eq!(report.events_processed[0].net, NetId::new(1));
        assert_eq!(report.events_processed[1].net, NetId::new(2));
        assert_eq!(report.events_processed[2].net, NetId::new(3));
    }

    #[test]
    fn processed_events_captured_for_trace() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        q.schedule(DigitalEvent::new(t50, NetId::new(1), LogicValue::One))
            .unwrap();
        let _ = q.run_until(t50);

        let events = q.take_processed_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].time, t50);
        // Second call returns empty (drained).
        assert!(q.take_processed_events().is_empty());
    }

    #[test]
    fn checkpoint_and_restore() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);

        q.schedule(DigitalEvent::new(t50, NetId::new(1), LogicValue::One))
            .unwrap();
        q.schedule(DigitalEvent::new(t100, NetId::new(2), LogicValue::Zero))
            .unwrap();

        // Checkpoint before running.
        let cp = q.checkpoint();

        // Run past 50 ns.
        let _ = q.run_until(t50);
        assert_eq!(q.current_time(), t50);
        assert_eq!(q.pending_count(), 1);

        // Restore: queue should be back to its pre-run state.
        q.restore_from_checkpoint(cp);
        assert_eq!(q.current_time(), SimulationTime::ZERO);
        assert_eq!(q.pending_count(), 2);
        assert_eq!(q.next_event_time(), Some(t50));
    }

    #[test]
    fn can_advance_to_validates_time_order() {
        let mut q = EventQueue::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let _ = q.run_until(t50);
        assert!(q.can_advance_to(SimulationTime::from_nanoseconds(80)));
        assert!(!q.can_advance_to(SimulationTime::from_nanoseconds(30)));
    }

    #[test]
    fn logic_value_display() {
        assert_eq!(format!("{}", LogicValue::Zero), "0");
        assert_eq!(format!("{}", LogicValue::One), "1");
        assert_eq!(format!("{}", LogicValue::Unknown), "X");
        assert_eq!(format!("{}", LogicValue::HighImpedance), "Z");
    }

    #[test]
    fn net_id_display() {
        assert_eq!(format!("{}", NetId::new(0)), "net:0");
        assert_eq!(format!("{}", NetId::new(42)), "net:42");
    }
}
