//! Event model + event-trace-equivalence checker over ordered (time, net, value) tuples.
//!
//! # Spec traceability
//!
//! - Scenario: `digital-equivalence#ordered-events-not-vcd`
//! - Task #20: Event model + event-trace-equivalence checker over ordered
//!   (time, net, value) tuples within tolerance.
//!
//! # Design
//!
//! Equivalence is judged on **ordered events**, not byte-level VCD identity.
//! Two traces are equivalent iff their ordered `(time, net, value)` sequences
//! agree within a configurable timing tolerance.  VCD is treated as an
//! interchange format only — no acceptance criterion depends on VCD byte
//! layout (that is task #21's concern).

use std::fmt;

// ---------------------------------------------------------------------------
// Digital value
// ---------------------------------------------------------------------------

/// A digital signal value.
///
/// Models the four standard logic levels (0, 1, X, Z) used in gate-level
/// simulation.  `X` (unknown) and `Z` (high-impedance) only compare equal
/// to themselves; they never match a definite logic level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LogicValue {
    /// Logic low (driven 0).
    Zero,
    /// Logic high (driven 1).
    One,
    /// Unknown / unresolved.
    X,
    /// High-impedance (undriven).
    Z,
}

impl fmt::Display for LogicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicValue::Zero => write!(f, "0"),
            LogicValue::One => write!(f, "1"),
            LogicValue::X => write!(f, "X"),
            LogicValue::Z => write!(f, "Z"),
        }
    }
}

impl LogicValue {
    /// Parse a single character into a `LogicValue`.
    ///
    /// Accepts `'0'`, `'1'`, `'x'`/`'X'`, `'z'`/`'Z'`.
    /// Returns `None` for any other character.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(LogicValue::Zero),
            '1' => Some(LogicValue::One),
            'x' | 'X' => Some(LogicValue::X),
            'z' | 'Z' => Some(LogicValue::Z),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Event
// ---------------------------------------------------------------------------

/// A single digital event: a value change on a named net at a specific time.
///
/// Events are the atomic unit of a digital trace.  Two traces are compared
/// by matching their ordered event sequences within a timing tolerance.
///
/// # Ordering
///
/// Events are ordered **lexicographically by (time, net)** so that the
/// equivalence check can walk both traces in lock-step.  When two events
/// share the same time and net the value must also agree.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    /// Simulation time at which the event occurs (seconds).
    pub time: f64,
    /// Net (signal) name on which the value change occurs.
    pub net: String,
    /// The new logic value after the transition.
    pub value: LogicValue,
}

impl Event {
    /// Construct a new event.
    pub fn new(time: f64, net: impl Into<String>, value: LogicValue) -> Self {
        Event {
            time,
            net: net.into(),
            value,
        }
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    /// Lexicographic ordering by (time, net).
    ///
    /// This is the canonical sort order for event traces so that two
    /// equivalently-sorted traces can be compared in linear time.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time
            .partial_cmp(&other.time)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.net.cmp(&other.net))
    }
}

// ---------------------------------------------------------------------------
// Event trace
// ---------------------------------------------------------------------------

/// An ordered sequence of digital events, sorted by (time, net).
///
/// Invariant: events are always stored in non-decreasing `(time, net)` order.
/// Methods that add events maintain this invariant.
#[derive(Clone, Debug, PartialEq)]
pub struct EventTrace {
    events: Vec<Event>,
}

impl EventTrace {
    /// Create an empty trace.
    pub fn new() -> Self {
        EventTrace { events: Vec::new() }
    }

    /// Create a trace from a pre-sorted vector of events.
    ///
    /// The caller guarantees that `events` is already sorted in
    /// non-decreasing `(time, net)` order and contains no exact duplicates
    /// (same time, net, value).  If in doubt, use [`EventTrace::from_unsorted`].
    pub fn from_sorted(events: Vec<Event>) -> Self {
        EventTrace { events }
    }

    /// Create a trace from an arbitrary vector of events.
    ///
    /// Sorts and deduplicates the events in-place.
    pub fn from_unsorted(mut events: Vec<Event>) -> Self {
        events.sort();
        events.dedup();
        EventTrace { events }
    }

    /// Push a single event, maintaining the sorted invariant.
    ///
    /// This is O(n) due to the insert.  For bulk construction prefer
    /// [`EventTrace::from_unsorted`].
    pub fn push(&mut self, event: Event) {
        let idx = self.events.partition_point(|e| e < &event);
        self.events.insert(idx, event);
    }

    /// Iterate over the events in order.
    pub fn iter(&self) -> std::slice::Iter<'_, Event> {
        self.events.iter()
    }

    /// Access the events as a slice.
    pub fn as_slice(&self) -> &[Event] {
        &self.events
    }

    /// Number of events in the trace.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Access event at index.
    pub fn get(&self, index: usize) -> Option<&Event> {
        self.events.get(index)
    }

    /// Consume the trace, returning the underlying `Vec<Event>`.
    pub fn into_events(self) -> Vec<Event> {
        self.events
    }
}

impl Default for EventTrace {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Equivalence result
// ---------------------------------------------------------------------------

/// Detailed result of comparing two event traces.
#[derive(Clone, Debug, PartialEq)]
pub enum EquivalenceMismatch {
    /// The traces have different numbers of events.
    LengthMismatch {
        actual_len: usize,
        expected_len: usize,
    },
    /// An event's logic value differs at the same (time, net) position.
    ValueMismatch {
        index: usize,
        actual: LogicValue,
        expected: LogicValue,
        time: f64,
        net: String,
    },
    /// An event's time falls outside the allowed tolerance.
    TimeMismatch {
        index: usize,
        actual_time: f64,
        expected_time: f64,
        tolerance: f64,
        delta: f64,
        net: String,
    },
    /// A net name differs at the same index (indicates fundamentally
    /// different trace structure).
    NetMismatch {
        index: usize,
        actual_net: String,
        expected_net: String,
        time: f64,
    },
}

impl fmt::Display for EquivalenceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EquivalenceMismatch::LengthMismatch {
                actual_len,
                expected_len,
            } => write!(
                f,
                "length mismatch: actual {} events, expected {}",
                actual_len, expected_len
            ),
            EquivalenceMismatch::ValueMismatch {
                index,
                actual,
                expected,
                time,
                net,
            } => write!(
                f,
                "value mismatch at index {}: actual={} expected={} (t={}, net={})",
                index, actual, expected, time, net
            ),
            EquivalenceMismatch::TimeMismatch {
                index,
                actual_time,
                expected_time,
                tolerance,
                delta,
                net,
            } => write!(
                f,
                "time mismatch at index {}: actual_t={} expected_t={} delta={} > tolerance={} (net={})",
                index, actual_time, expected_time, delta, tolerance, net
            ),
            EquivalenceMismatch::NetMismatch {
                index,
                actual_net,
                expected_net,
                time,
            } => write!(
                f,
                "net mismatch at index {}: actual_net={} expected_net={} (t={})",
                index, actual_net, expected_net, time
            ),
        }
    }
}

/// The outcome of an event-trace equivalence check.
#[derive(Clone, Debug)]
pub struct EquivalenceResult {
    /// `true` when the traces are equivalent within tolerance.
    pub equivalent: bool,
    /// The first mismatch found, if any.
    pub first_mismatch: Option<EquivalenceMismatch>,
}

impl EquivalenceResult {
    fn ok() -> Self {
        EquivalenceResult {
            equivalent: true,
            first_mismatch: None,
        }
    }

    fn mismatch(m: EquivalenceMismatch) -> Self {
        EquivalenceResult {
            equivalent: false,
            first_mismatch: Some(m),
        }
    }
}

impl fmt::Display for EquivalenceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.equivalent {
            write!(f, "EQUIVALENT")
        } else {
            write!(
                f,
                "NOT EQUIVALENT: {}",
                self.first_mismatch.as_ref().unwrap()
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Tolerance configuration
// ---------------------------------------------------------------------------

/// Configuration for the equivalence check.
#[derive(Clone, Debug)]
pub struct EquivalenceTolerance {
    /// Maximum allowed difference in event time (seconds).
    ///
    /// Two events that agree on `(net, value)` but whose times differ by
    /// more than this value are considered a mismatch.
    ///
    /// Defaults to 0.0 (exact timing match required).
    pub time_tolerance: f64,
}

impl Default for EquivalenceTolerance {
    fn default() -> Self {
        EquivalenceTolerance {
            time_tolerance: 0.0,
        }
    }
}

impl EquivalenceTolerance {
    /// Exact-match tolerance (no timing slack).
    pub fn exact() -> Self {
        EquivalenceTolerance {
            time_tolerance: 0.0,
        }
    }

    /// Tolerance with the given timing slack in seconds.
    pub fn with_time_tolerance(t: f64) -> Self {
        EquivalenceTolerance { time_tolerance: t }
    }
}

// ---------------------------------------------------------------------------
// Equivalence checker
// ---------------------------------------------------------------------------

/// Check two event traces for equivalence within tolerance.
///
/// # Algorithm
///
/// Both traces must already be sorted in non-decreasing `(time, net)` order
/// (which [`EventTrace`] guarantees).  The checker walks both traces in
/// lock-step:
///
/// 1. If the traces have different lengths → `LengthMismatch`.
/// 2. For each event pair at index *i*:
///    a. If net names differ → `NetMismatch`.
///    b. If logic values differ → `ValueMismatch`.
///    c. If `|actual.time - expected.time| > tolerance` → `TimeMismatch`.
/// 3. If all event pairs pass → `Equivalent`.
///
/// Returns [`EquivalenceResult`] with the first mismatch encountered (if any).
///
/// # Spec reference
///
/// > Equivalence holds iff the ordered (time, net, value) event sequences
/// > agree within the timing tolerance (not byte-level VCD identity).
/// > — `digital-equivalence#ordered-events-not-vcd`
pub fn check_equivalence(
    actual: &EventTrace,
    expected: &EventTrace,
    tolerance: &EquivalenceTolerance,
) -> EquivalenceResult {
    if actual.len() != expected.len() {
        return EquivalenceResult::mismatch(EquivalenceMismatch::LengthMismatch {
            actual_len: actual.len(),
            expected_len: expected.len(),
        });
    }

    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        // 1. Net name must match exactly.
        if a.net != e.net {
            return EquivalenceResult::mismatch(EquivalenceMismatch::NetMismatch {
                index: i,
                actual_net: a.net.clone(),
                expected_net: e.net.clone(),
                time: a.time,
            });
        }

        // 2. Logic value must match exactly.
        if a.value != e.value {
            return EquivalenceResult::mismatch(EquivalenceMismatch::ValueMismatch {
                index: i,
                actual: a.value,
                expected: e.value,
                time: a.time,
                net: a.net.clone(),
            });
        }

        // 3. Time must be within tolerance.
        let delta = (a.time - e.time).abs();
        if delta > tolerance.time_tolerance {
            return EquivalenceResult::mismatch(EquivalenceMismatch::TimeMismatch {
                index: i,
                actual_time: a.time,
                expected_time: e.time,
                tolerance: tolerance.time_tolerance,
                delta,
                net: a.net.clone(),
            });
        }
    }

    EquivalenceResult::ok()
}

// ---------------------------------------------------------------------------
// Unit tests (inline)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logic_value_from_char_roundtrip() {
        assert_eq!(LogicValue::from_char('0'), Some(LogicValue::Zero));
        assert_eq!(LogicValue::from_char('1'), Some(LogicValue::One));
        assert_eq!(LogicValue::from_char('x'), Some(LogicValue::X));
        assert_eq!(LogicValue::from_char('X'), Some(LogicValue::X));
        assert_eq!(LogicValue::from_char('z'), Some(LogicValue::Z));
        assert_eq!(LogicValue::from_char('Z'), Some(LogicValue::Z));
        assert_eq!(LogicValue::from_char('?'), None);
    }

    #[test]
    fn logic_value_display() {
        assert_eq!(format!("{}", LogicValue::Zero), "0");
        assert_eq!(format!("{}", LogicValue::One), "1");
        assert_eq!(format!("{}", LogicValue::X), "X");
        assert_eq!(format!("{}", LogicValue::Z), "Z");
    }

    #[test]
    fn event_ordering_by_time_then_net() {
        let e1 = Event::new(1.0, "a", LogicValue::One);
        let e2 = Event::new(2.0, "a", LogicValue::Zero);
        let e3 = Event::new(1.0, "b", LogicValue::One);

        assert!(e1 < e2); // same net, earlier time
        assert!(e1 < e3); // same time, "a" < "b"
        assert!(e3 < e2); // t=1 < t=2
    }

    #[test]
    fn trace_from_unsorted_sorts_and_dedupes() {
        let events = vec![
            Event::new(2.0, "a", LogicValue::One),
            Event::new(1.0, "b", LogicValue::Zero),
            Event::new(1.0, "a", LogicValue::One),
            Event::new(2.0, "a", LogicValue::One), // duplicate
        ];
        let trace = EventTrace::from_unsorted(events);
        assert_eq!(trace.len(), 3);
        assert_eq!(trace.get(0).unwrap().time, 1.0);
        assert_eq!(trace.get(0).unwrap().net, "a");
        assert_eq!(trace.get(1).unwrap().time, 1.0);
        assert_eq!(trace.get(1).unwrap().net, "b");
        assert_eq!(trace.get(2).unwrap().time, 2.0);
    }

    #[test]
    fn trace_push_maintains_sort() {
        let mut trace = EventTrace::new();
        trace.push(Event::new(3.0, "c", LogicValue::Zero));
        trace.push(Event::new(1.0, "a", LogicValue::One));
        trace.push(Event::new(2.0, "b", LogicValue::Zero));
        assert_eq!(trace.len(), 3);
        assert_eq!(trace.get(0).unwrap().net, "a");
        assert_eq!(trace.get(1).unwrap().net, "b");
        assert_eq!(trace.get(2).unwrap().net, "c");
    }

    #[test]
    fn equivalence_exact_match() {
        let actual = EventTrace::from_sorted(vec![
            Event::new(1.0, "a", LogicValue::One),
            Event::new(2.0, "b", LogicValue::Zero),
        ]);
        let expected = EventTrace::from_sorted(vec![
            Event::new(1.0, "a", LogicValue::One),
            Event::new(2.0, "b", LogicValue::Zero),
        ]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(result.equivalent);
    }

    #[test]
    fn equivalence_length_mismatch() {
        let actual = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::One)]);
        let expected = EventTrace::from_sorted(vec![
            Event::new(1.0, "a", LogicValue::One),
            Event::new(2.0, "b", LogicValue::Zero),
        ]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent);
        assert!(matches!(
            result.first_mismatch,
            Some(EquivalenceMismatch::LengthMismatch { .. })
        ));
    }

    #[test]
    fn equivalence_value_mismatch() {
        let actual = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::One)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::Zero)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent);
        assert!(matches!(
            result.first_mismatch,
            Some(EquivalenceMismatch::ValueMismatch { index: 0, .. })
        ));
    }

    #[test]
    fn equivalence_time_mismatch_exact_tolerance() {
        let actual = EventTrace::from_sorted(vec![Event::new(1.01, "a", LogicValue::One)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.00, "a", LogicValue::One)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent);
        assert!(matches!(
            result.first_mismatch,
            Some(EquivalenceMismatch::TimeMismatch { index: 0, .. })
        ));
    }

    #[test]
    fn equivalence_time_within_tolerance() {
        let actual = EventTrace::from_sorted(vec![Event::new(1.01, "a", LogicValue::One)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.00, "a", LogicValue::One)]);
        let result = check_equivalence(
            &actual,
            &expected,
            &EquivalenceTolerance::with_time_tolerance(0.02),
        );
        assert!(result.equivalent);
    }

    #[test]
    fn equivalence_time_just_outside_tolerance() {
        let actual = EventTrace::from_sorted(vec![Event::new(1.03, "a", LogicValue::One)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.00, "a", LogicValue::One)]);
        let result = check_equivalence(
            &actual,
            &expected,
            &EquivalenceTolerance::with_time_tolerance(0.02),
        );
        assert!(!result.equivalent);
    }

    #[test]
    fn equivalence_net_mismatch() {
        let actual = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::One)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.0, "b", LogicValue::One)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent);
        assert!(matches!(
            result.first_mismatch,
            Some(EquivalenceMismatch::NetMismatch { index: 0, .. })
        ));
    }

    #[test]
    fn equivalence_empty_traces() {
        let a = EventTrace::new();
        let e = EventTrace::new();
        let result = check_equivalence(&a, &e, &EquivalenceTolerance::exact());
        assert!(result.equivalent);
    }

    #[test]
    fn equivalence_x_and_z_values_must_match_exactly() {
        // X != Zero
        let actual = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::X)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::Zero)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent);

        // Z != One
        let actual = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::Z)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::One)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent);

        // X == X
        let actual = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::X)]);
        let expected = EventTrace::from_sorted(vec![Event::new(1.0, "a", LogicValue::X)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(result.equivalent);
    }

    #[test]
    fn equivalence_multi_net_trace() {
        let actual = EventTrace::from_unsorted(vec![
            Event::new(0.0, "clk", LogicValue::One),
            Event::new(0.0, "data", LogicValue::Zero),
            Event::new(5e-9, "clk", LogicValue::Zero),
            Event::new(5e-9, "data", LogicValue::One),
            Event::new(10e-9, "clk", LogicValue::One),
        ]);
        let expected = EventTrace::from_unsorted(vec![
            Event::new(0.0, "clk", LogicValue::One),
            Event::new(0.0, "data", LogicValue::Zero),
            Event::new(5e-9, "clk", LogicValue::Zero),
            Event::new(5e-9, "data", LogicValue::One),
            Event::new(10e-9, "clk", LogicValue::One),
        ]);
        let result = check_equivalence(
            &actual,
            &expected,
            &EquivalenceTolerance::with_time_tolerance(1e-12),
        );
        assert!(result.equivalent);
    }

    #[test]
    fn mismatch_display_formatting() {
        let m = EquivalenceMismatch::TimeMismatch {
            index: 3,
            actual_time: 1.03,
            expected_time: 1.0,
            tolerance: 0.02,
            delta: 0.03,
            net: "clk".into(),
        };
        let s = format!("{}", m);
        assert!(s.contains("time mismatch"));
        assert!(s.contains("index 3"));
        assert!(s.contains("net=clk"));
    }

    #[test]
    fn result_display_equivalent() {
        let r = EquivalenceResult::ok();
        assert_eq!(format!("{}", r), "EQUIVALENT");
    }

    #[test]
    fn result_display_not_equivalent() {
        let r = EquivalenceResult::mismatch(EquivalenceMismatch::LengthMismatch {
            actual_len: 1,
            expected_len: 2,
        });
        let s = format!("{}", r);
        assert!(s.starts_with("NOT EQUIVALENT"));
        assert!(s.contains("length mismatch"));
    }
}
