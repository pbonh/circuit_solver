//! Priority queue of digital transition events for transient analysis.
//!
//! [`EventScheduler`] holds a min-heap of [`DigitalEvent`]s ordered by time.
//! The transient driver peeks at [`EventScheduler::next_event_time`] before
//! each step so the timestep `h` is capped to land exactly on the next digital
//! boundary rather than stepping over it.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

// ── DigitalEvent ──────────────────────────────────────────────────────────────

/// A single digital transition event at a specific simulation time.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalEvent {
    /// Time (seconds) at which the event should fire.
    pub time: f64,
    /// Name of the net or signal whose value changes at this time.
    pub signal: String,
    /// New logical value after the transition.
    pub value: bool,
}

// ── Ord / PartialOrd for DigitalEvent ─────────────────────────────────────────
//
// We need `Ord` so `DigitalEvent` can live inside `Reverse<DigitalEvent>` in a
// `BinaryHeap`.  `BinaryHeap` in Rust is a max-heap, so wrapping in `Reverse`
// gives min-heap semantics (earliest time on top).
//
// NaN in `f64` makes a total order impossible in general, but simulation times
// are always finite, so we use `total_cmp` which defines a complete ordering
// consistent with `PartialOrd`.

impl Eq for DigitalEvent {}

impl PartialOrd for DigitalEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DigitalEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary sort: time (ascending).
        // Secondary sort: signal name (for deterministic ordering of ties).
        self.time
            .total_cmp(&other.time)
            .then_with(|| self.signal.cmp(&other.signal))
    }
}

// ── EventScheduler ───────────────────────────────────────────────────────────

/// Min-heap priority queue of [`DigitalEvent`]s ordered by time.
///
/// # Example
///
/// ```
/// use circuit_solver_delta::event_scheduler::{DigitalEvent, EventScheduler};
///
/// let mut sched = EventScheduler::new();
/// sched.push(DigitalEvent { time: 3e-9, signal: "clk".into(), value: true });
/// sched.push(DigitalEvent { time: 1e-9, signal: "rst".into(), value: false });
/// sched.push(DigitalEvent { time: 2e-9, signal: "clk".into(), value: false });
///
/// assert_eq!(sched.next_event_time(), Some(1e-9));
/// let e = sched.pop().unwrap();
/// assert_eq!(e.time, 1e-9);
/// ```
#[derive(Debug, Default)]
pub struct EventScheduler {
    heap: BinaryHeap<Reverse<DigitalEvent>>,
}

impl EventScheduler {
    /// Create an empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a digital event into the priority queue.
    pub fn push(&mut self, event: DigitalEvent) {
        self.heap.push(Reverse(event));
    }

    /// Peek at the time of the earliest pending event without removing it.
    ///
    /// Returns `None` when the queue is empty.
    pub fn next_event_time(&self) -> Option<f64> {
        self.heap.peek().map(|Reverse(e)| e.time)
    }

    /// Remove and return the earliest pending event.
    ///
    /// Returns `None` when the queue is empty.
    pub fn pop(&mut self) -> Option<DigitalEvent> {
        self.heap.pop().map(|Reverse(e)| e)
    }

    /// Return `true` if the queue contains no events.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Number of events currently in the queue.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Insert 3 events out-of-order; pop should return them in chronological order.
    #[test]
    fn event_scheduler_chronological_order() {
        let mut sched = EventScheduler::new();

        // Insert out-of-order: 3 ns, 1 ns, 2 ns.
        sched.push(DigitalEvent { time: 3e-9, signal: "clk".into(), value: true });
        sched.push(DigitalEvent { time: 1e-9, signal: "rst".into(), value: false });
        sched.push(DigitalEvent { time: 2e-9, signal: "clk".into(), value: false });

        // next_event_time should peek at the earliest without consuming.
        assert_eq!(sched.next_event_time(), Some(1e-9));
        assert_eq!(sched.len(), 3, "peek must not consume");

        let e1 = sched.pop().expect("first pop");
        assert_eq!(e1.time, 1e-9, "first event should be at 1 ns");

        let e2 = sched.pop().expect("second pop");
        assert_eq!(e2.time, 2e-9, "second event should be at 2 ns");

        let e3 = sched.pop().expect("third pop");
        assert_eq!(e3.time, 3e-9, "third event should be at 3 ns");

        assert!(sched.pop().is_none(), "queue should be empty");
        assert_eq!(sched.next_event_time(), None);
    }

    /// is_empty and len reflect queue state correctly.
    #[test]
    fn event_scheduler_is_empty_and_len() {
        let mut sched = EventScheduler::new();
        assert!(sched.is_empty());
        assert_eq!(sched.len(), 0);

        sched.push(DigitalEvent { time: 1e-9, signal: "a".into(), value: true });
        assert!(!sched.is_empty());
        assert_eq!(sched.len(), 1);

        sched.pop();
        assert!(sched.is_empty());
    }
}
