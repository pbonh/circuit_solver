//! Event-trace equivalence checker over ordered (time, net, value) tuples
//! within tolerance.
//!
//! ADR-0006 mandates that the native digital kernel produce event traces
//! that can be checked for equivalence against a golden reference (e.g.,
//! Icarus Verilog). The `digital-equivalence#ordered-events-not-vcd`
//! scenario specifies that equivalence is checked at the level of
//! **ordered event tuples** — (time, net, value) — not at the level of
//! raw VCD bytes. This avoids false negatives from formatting differences,
//! signal ordering, or VCD header conventions.
//!
//! # Design
//!
//! - [`TraceEvent`] — a logical (time, net, value) tuple, stripped of
//!   the internal scheduling sequence counter. Two events from different
//!   simulators can be compared directly.
//! - [`EventTrace`] — an ordered sequence of [`TraceEvent`]s, sorted by
//!   (time, net). Constructed from the kernel's processed events or from
//!   a golden reference.
//! - [`EquivalenceTolerance`] — configuration for how strictly to compare
//!   event times. The `time_tolerance` field is a [`SimulationTime`] delta;
//!   corresponding events whose times differ by at most this amount are
//!   considered time-equivalent.
//! - [`EquivalenceResult`] — the outcome of [`check_equivalence`].
//! - [`check_equivalence`] — the main comparison function.
//!
//! # Algorithm
//!
//! Both traces are normalized (sorted by time, then net) and compared
//! element-by-element:
//!
//! 1. If the traces have different numbers of events, they are not
//!    equivalent.
//! 2. For each pair of corresponding events (same index after sorting):
//!    - The net IDs must match exactly.
//!    - The logic values must match exactly (four-valued logic has no
//!      "close enough" — 0 and 1 are distinct).
//!    - The times must differ by at most `time_tolerance`.
//! 3. If all pairs match, the traces are equivalent.

use circuit_solver_types::SimulationTime;
use core::fmt;

use crate::event_queue::{DigitalEvent, LogicValue, NetId};

// ---------------------------------------------------------------------------
// Trace event
// ---------------------------------------------------------------------------

/// A logical event in a digital event trace — (time, net, value).
///
/// Unlike [`DigitalEvent`] (which carries an internal scheduling sequence
/// counter for FIFO tie-breaking), `TraceEvent` is a pure (time, net,
/// value) tuple intended for cross-simulator comparison. Two traces from
/// different sources (native kernel vs. Icarus Verilog golden reference)
/// can be compared at this level without being sensitive to scheduling
/// order within the same time step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceEvent {
    /// The time at which this event occurs.
    pub time: SimulationTime,
    /// The net (wire) that transitions.
    pub net: NetId,
    /// The new value the net transitions to.
    pub value: LogicValue,
}

impl TraceEvent {
    /// Construct a new trace event.
    #[must_use]
    pub const fn new(time: SimulationTime, net: NetId, value: LogicValue) -> Self {
        Self { time, net, value }
    }

    /// Convert a [`DigitalEvent`] (from the kernel's processed-events
    /// trace) into a `TraceEvent`, stripping the internal sequence
    /// counter.
    #[must_use]
    pub fn from_digital_event(event: &DigitalEvent) -> Self {
        Self {
            time: event.time,
            net: event.net,
            value: event.value,
        }
    }
}

impl fmt::Display for TraceEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.time, self.net, self.value)
    }
}

impl PartialOrd for TraceEvent {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TraceEvent {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Primary sort: time (non-decreasing).
        // Secondary sort: net index (deterministic within same time).
        match self.time.cmp(&other.time) {
            core::cmp::Ordering::Equal => self.net.cmp(&other.net),
            ord => ord,
        }
    }
}

// ---------------------------------------------------------------------------
// Event trace
// ---------------------------------------------------------------------------

/// An ordered digital event trace for equivalence checking.
///
/// Events are stored in sorted order by (time, net), ensuring that
/// two traces from different simulators can be compared deterministically
/// regardless of the internal scheduling order within each simulator.
///
/// # Construction
///
/// - [`EventTrace::from_events`] — from a raw vector, which is sorted.
/// - [`EventTrace::from_digital_events`] — from the kernel's
///   `Vec<DigitalEvent>`, converting and sorting.
/// - [`EventTrace::from_sorted_events`] — from a pre-sorted vector
///   (avoids the sort if you know the data is already sorted).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTrace {
    /// Events sorted by (time, net).
    events: Vec<TraceEvent>,
}

impl EventTrace {
    /// Construct an event trace from an unsorted vector of events.
    ///
    /// The events are sorted by (time, net) on construction.
    #[must_use]
    pub fn from_events(events: Vec<TraceEvent>) -> Self {
        let mut trace = Self { events };
        trace.normalize();
        trace
    }

    /// Construct an event trace from a pre-sorted vector of events.
    ///
    /// # Safety (correctness)
    ///
    /// The caller must ensure the events are already sorted by (time, net).
    /// If they are not, equivalence checking will produce incorrect results.
    #[must_use]
    pub fn from_sorted_events(events: Vec<TraceEvent>) -> Self {
        Self { events }
    }

    /// Construct an event trace from the kernel's processed
    /// [`DigitalEvent`]s.
    ///
    /// Each `DigitalEvent` is converted to a [`TraceEvent`] (stripping
    /// the internal sequence counter), and the result is sorted by
    /// (time, net).
    #[must_use]
    pub fn from_digital_events(events: &[DigitalEvent]) -> Self {
        let trace_events: Vec<TraceEvent> =
            events.iter().map(TraceEvent::from_digital_event).collect();
        Self::from_events(trace_events)
    }

    /// An empty trace with no events.
    #[must_use]
    pub fn empty() -> Self {
        Self { events: Vec::new() }
    }

    /// The number of events in the trace.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// True iff the trace has no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Access the events as a slice (in sorted order).
    #[must_use]
    pub fn as_slice(&self) -> &[TraceEvent] {
        &self.events
    }

    /// Iterate over the events in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = &TraceEvent> {
        self.events.iter()
    }

    /// The set of unique nets that appear in this trace, sorted by
    /// [`NetId::index`].
    #[must_use]
    pub fn nets(&self) -> Vec<NetId> {
        let mut nets: Vec<NetId> = self.events.iter().map(|e| e.net).collect();
        nets.sort();
        nets.dedup();
        nets
    }

    /// Extract the sub-trace for a single net (preserving time order).
    #[must_use]
    pub fn for_net(&self, net: NetId) -> Self {
        let filtered: Vec<TraceEvent> = self
            .events
            .iter()
            .filter(|e| e.net == net)
            .copied()
            .collect();
        // Already sorted since the parent is sorted and we're filtering.
        Self::from_sorted_events(filtered)
    }

    /// Sort events by (time, net) and remove duplicates.
    fn normalize(&mut self) {
        self.events.sort();
        // Dedup: identical (time, net, value) tuples are redundant.
        self.events.dedup();
    }
}

impl fmt::Display for EventTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EventTrace({} events):", self.events.len())?;
        for event in &self.events {
            writeln!(f, "  {event}")?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Equivalence tolerance
// ---------------------------------------------------------------------------

/// Configuration for how strictly to compare event times in equivalence
/// checking.
///
/// The `time_tolerance` is a [`SimulationTime`] delta. Two corresponding
/// events whose times differ by at most this amount are considered
/// time-equivalent. Logic values must always match exactly — there is no
/// "close enough" for four-valued logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquivalenceTolerance {
    /// Maximum allowed time difference between corresponding events.
    pub time_tolerance: SimulationTime,
}

impl EquivalenceTolerance {
    /// Zero tolerance: event times must match exactly.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            time_tolerance: SimulationTime::ZERO,
        }
    }

    /// Tolerance of the given picosecond delta.
    #[must_use]
    pub const fn from_picoseconds(ps: i64) -> Self {
        Self {
            time_tolerance: SimulationTime::from_picoseconds(ps),
        }
    }

    /// Tolerance of the given nanosecond delta.
    #[must_use]
    pub const fn from_nanoseconds(ns: i64) -> Self {
        Self {
            time_tolerance: SimulationTime::from_nanoseconds(ns),
        }
    }

    /// Check whether two times are within this tolerance.
    #[must_use]
    pub fn times_match(&self, a: SimulationTime, b: SimulationTime) -> bool {
        let diff = if a >= b { a - b } else { b - a };
        diff <= self.time_tolerance
    }
}

impl Default for EquivalenceTolerance {
    fn default() -> Self {
        Self::exact()
    }
}

// ---------------------------------------------------------------------------
// Equivalence result
// ---------------------------------------------------------------------------

/// The outcome of event-trace equivalence checking.
#[derive(Debug, Clone, PartialEq)]
pub enum EquivalenceResult {
    /// The two traces are equivalent within the specified tolerance.
    Equivalent,

    /// The two traces are NOT equivalent.
    NotEquivalent {
        /// Human-readable description of the first mismatch.
        reason: String,
        /// Index in the reference trace where the first mismatch occurs,
        /// if applicable.
        mismatch_index: Option<usize>,
    },
}

impl EquivalenceResult {
    /// True iff the traces are equivalent.
    #[must_use]
    pub fn is_equivalent(&self) -> bool {
        matches!(self, Self::Equivalent)
    }
}

impl fmt::Display for EquivalenceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equivalent => write!(f, "EQUIVALENT"),
            Self::NotEquivalent { reason, mismatch_index } => {
                write!(f, "NOT EQUIVALENT: {reason}")?;
                if let Some(idx) = mismatch_index {
                    write!(f, " (at index {idx})")?;
                }
                Ok(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Equivalence checker
// ---------------------------------------------------------------------------

/// Compare two event traces for equivalence within the given tolerance.
///
/// Both traces are compared element-by-element after their canonical
/// (time, net) sort. For each pair of corresponding events:
///
/// - The net IDs must match exactly.
/// - The logic values must match exactly.
/// - The times must differ by at most `tolerance.time_tolerance`.
///
/// # Arguments
///
/// - `reference` — the golden reference trace (e.g., from Icarus Verilog).
/// - `candidate` — the trace to check (e.g., from the native kernel).
/// - `tolerance` — how strictly to compare event times.
///
/// # Returns
///
/// An [`EquivalenceResult`] indicating whether the traces match.
pub fn check_equivalence(
    reference: &EventTrace,
    candidate: &EventTrace,
    tolerance: EquivalenceTolerance,
) -> EquivalenceResult {
    let ref_events = reference.as_slice();
    let cand_events = candidate.as_slice();

    // Different event counts → not equivalent.
    if ref_events.len() != cand_events.len() {
        return EquivalenceResult::NotEquivalent {
            reason: format!(
                "event count mismatch: reference has {}, candidate has {}",
                ref_events.len(),
                cand_events.len()
            ),
            mismatch_index: None,
        };
    }

    // Both empty → equivalent.
    if ref_events.is_empty() {
        return EquivalenceResult::Equivalent;
    }

    // Element-by-element comparison.
    for (i, (ref_ev, cand_ev)) in ref_events.iter().zip(cand_events.iter()).enumerate() {
        // Net must match exactly.
        if ref_ev.net != cand_ev.net {
            return EquivalenceResult::NotEquivalent {
                reason: format!(
                    "net mismatch at index {i}: reference has {}, candidate has {}",
                    ref_ev.net, cand_ev.net
                ),
                mismatch_index: Some(i),
            };
        }

        // Value must match exactly.
        if ref_ev.value != cand_ev.value {
            return EquivalenceResult::NotEquivalent {
                reason: format!(
                    "value mismatch at index {i} on {}: reference has {}, candidate has {}",
                    ref_ev.net, ref_ev.value, cand_ev.value
                ),
                mismatch_index: Some(i),
            };
        }

        // Time must match within tolerance.
        if !tolerance.times_match(ref_ev.time, cand_ev.time) {
            return EquivalenceResult::NotEquivalent {
                reason: format!(
                    "time mismatch at index {i} on {}: reference at {}, candidate at {} (tolerance {})",
                    ref_ev.net,
                    ref_ev.time,
                    cand_ev.time,
                    tolerance.time_tolerance
                ),
                mismatch_index: Some(i),
            };
        }
    }

    EquivalenceResult::Equivalent
}

// ---------------------------------------------------------------------------
// Per-net equivalence (supplementary)
// ---------------------------------------------------------------------------

/// Compare two event traces for equivalence on a per-net basis.
///
/// This is a more robust comparison than [`check_equivalence`] when the
/// two traces may have events on different nets interleaved in different
/// orders at the same time step. Per-net comparison extracts the event
/// sub-trace for each net and compares them independently.
///
/// # Algorithm
///
/// 1. Collect the union of nets from both traces.
/// 2. For each net, extract the sub-trace from both traces.
/// 3. Compare each pair of sub-traces using [`check_equivalence`].
/// 4. If any net's sub-traces are not equivalent, the whole comparison
///    fails.
///
/// # Arguments
///
/// Same as [`check_equivalence`].
///
/// # Returns
///
/// An [`EquivalenceResult`]. On mismatch, the `reason` identifies the
/// net where the first mismatch was found.
pub fn check_equivalence_per_net(
    reference: &EventTrace,
    candidate: &EventTrace,
    tolerance: EquivalenceTolerance,
) -> EquivalenceResult {
    // Collect union of nets from both traces.
    let mut all_nets: Vec<NetId> = reference.nets();
    for net in candidate.nets() {
        if !all_nets.contains(&net) {
            all_nets.push(net);
        }
    }
    all_nets.sort();

    // Compare each net's sub-trace.
    for net in all_nets {
        let ref_sub = reference.for_net(net);
        let cand_sub = candidate.for_net(net);

        let result = check_equivalence(&ref_sub, &cand_sub, tolerance);
        if let EquivalenceResult::NotEquivalent { reason, mismatch_index } = result {
            return EquivalenceResult::NotEquivalent {
                reason: format!("net {net}: {reason}"),
                mismatch_index,
            };
        }
    }

    EquivalenceResult::Equivalent
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ----- TraceEvent -----

    #[test]
    fn trace_event_display() {
        let ev = TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        );
        assert_eq!(format!("{ev}"), "(50 ns, net:1, 1)");
    }

    #[test]
    fn trace_event_ordering_by_time_then_net() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);
        let ev_a = TraceEvent::new(t50, NetId::new(1), LogicValue::One);
        let ev_b = TraceEvent::new(t50, NetId::new(2), LogicValue::Zero);
        let ev_c = TraceEvent::new(t80, NetId::new(0), LogicValue::One);

        // Same time: sorted by net.
        assert!(ev_a < ev_b);
        // Different time: sorted by time.
        assert!(ev_a < ev_c);
        assert!(ev_b < ev_c);
    }

    #[test]
    fn trace_event_from_digital_event_strips_seq() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let de = DigitalEvent::new(t50, NetId::new(3), LogicValue::Unknown);
        let te = TraceEvent::from_digital_event(&de);
        assert_eq!(te.time, t50);
        assert_eq!(te.net, NetId::new(3));
        assert_eq!(te.value, LogicValue::Unknown);
    }

    // ----- EventTrace -----

    #[test]
    fn empty_trace() {
        let trace = EventTrace::empty();
        assert!(trace.is_empty());
        assert_eq!(trace.len(), 0);
    }

    #[test]
    fn from_events_sorts_and_dedups() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);
        let events = vec![
            TraceEvent::new(t80, NetId::new(1), LogicValue::One),
            TraceEvent::new(t50, NetId::new(2), LogicValue::Zero),
            // Duplicate of the first event (after seq stripping).
            TraceEvent::new(t80, NetId::new(1), LogicValue::One),
        ];
        let trace = EventTrace::from_events(events);
        // Deduped: 2 events.
        assert_eq!(trace.len(), 2);
        // Sorted: (50ns, net:2) then (80ns, net:1).
        assert_eq!(trace.as_slice()[0].time, t50);
        assert_eq!(trace.as_slice()[0].net, NetId::new(2));
        assert_eq!(trace.as_slice()[1].time, t80);
        assert_eq!(trace.as_slice()[1].net, NetId::new(1));
    }

    #[test]
    fn from_digital_events_converts_and_sorts() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let t100 = SimulationTime::from_nanoseconds(100);
        let digital_events = vec![
            DigitalEvent::new(t100, NetId::new(0), LogicValue::Zero),
            DigitalEvent::new(t50, NetId::new(1), LogicValue::One),
        ];
        let trace = EventTrace::from_digital_events(&digital_events);
        assert_eq!(trace.len(), 2);
        // Sorted by time: 50 ns first.
        assert_eq!(trace.as_slice()[0].time, t50);
        assert_eq!(trace.as_slice()[1].time, t100);
    }

    #[test]
    fn for_net_filters_correctly() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);
        let trace = EventTrace::from_events(vec![
            TraceEvent::new(t50, NetId::new(1), LogicValue::One),
            TraceEvent::new(t50, NetId::new(2), LogicValue::Zero),
            TraceEvent::new(t80, NetId::new(1), LogicValue::Zero),
        ]);
        let net1 = trace.for_net(NetId::new(1));
        assert_eq!(net1.len(), 2);
        assert_eq!(net1.as_slice()[0].time, t50);
        assert_eq!(net1.as_slice()[0].value, LogicValue::One);
        assert_eq!(net1.as_slice()[1].time, t80);
        assert_eq!(net1.as_slice()[1].value, LogicValue::Zero);
    }

    #[test]
    fn nets_returns_sorted_unique_nets() {
        let trace = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(3),
                LogicValue::One,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::Zero,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(80),
                NetId::new(3),
                LogicValue::Zero,
            ),
        ]);
        let nets = trace.nets();
        assert_eq!(nets, vec![NetId::new(1), NetId::new(3)]);
    }

    #[test]
    fn trace_display() {
        let trace = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(0),
            LogicValue::One,
        )]);
        let s = format!("{trace}");
        assert!(s.contains("1 events"));
        assert!(s.contains("(50 ns, net:0, 1)"));
    }

    // ----- EquivalenceTolerance -----

    #[test]
    fn exact_tolerance_requires_perfect_time_match() {
        let tol = EquivalenceTolerance::exact();
        let t50 = SimulationTime::from_nanoseconds(50);
        let t51 = SimulationTime::from_nanoseconds(51);
        assert!(tol.times_match(t50, t50));
        assert!(!tol.times_match(t50, t51));
    }

    #[test]
    fn nanosecond_tolerance_allows_delta() {
        let tol = EquivalenceTolerance::from_nanoseconds(5);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t54 = SimulationTime::from_nanoseconds(54);
        let t55 = SimulationTime::from_nanoseconds(55);
        let t56 = SimulationTime::from_nanoseconds(56);
        // 4 ns diff < 5 ns tolerance → matches.
        assert!(tol.times_match(t50, t54));
        // 5 ns diff == 5 ns tolerance → matches (<=).
        assert!(tol.times_match(t50, t55));
        // 6 ns diff > 5 ns tolerance → does not match.
        assert!(!tol.times_match(t50, t56));
    }

    #[test]
    fn tolerance_is_symmetric() {
        let tol = EquivalenceTolerance::from_nanoseconds(5);
        let t50 = SimulationTime::from_nanoseconds(50);
        let t53 = SimulationTime::from_nanoseconds(53);
        assert!(tol.times_match(t50, t53));
        assert!(tol.times_match(t53, t50));
    }

    // ----- check_equivalence -----

    #[test]
    fn empty_traces_are_equivalent() {
        let a = EventTrace::empty();
        let b = EventTrace::empty();
        let result = check_equivalence(&a, &b, EquivalenceTolerance::exact());
        assert!(result.is_equivalent());
    }

    #[test]
    fn identical_traces_are_equivalent() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);
        let trace = EventTrace::from_events(vec![
            TraceEvent::new(t50, NetId::new(1), LogicValue::One),
            TraceEvent::new(t80, NetId::new(2), LogicValue::Zero),
        ]);
        let result = check_equivalence(&trace, &trace, EquivalenceTolerance::exact());
        assert!(result.is_equivalent());
    }

    #[test]
    fn different_event_counts_are_not_equivalent() {
        let a = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let b = EventTrace::empty();
        let result = check_equivalence(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
        let reason = match result {
            EquivalenceResult::NotEquivalent { reason, .. } => reason,
            _ => unreachable!(),
        };
        assert!(reason.contains("event count mismatch"));
    }

    #[test]
    fn net_mismatch_is_detected() {
        let a = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let b = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(2),
            LogicValue::One,
        )]);
        let result = check_equivalence(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
        let reason = match result {
            EquivalenceResult::NotEquivalent { reason, .. } => reason,
            _ => unreachable!(),
        };
        assert!(reason.contains("net mismatch"));
    }

    #[test]
    fn value_mismatch_is_detected() {
        let a = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let b = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::Zero,
        )]);
        let result = check_equivalence(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
        let reason = match result {
            EquivalenceResult::NotEquivalent { reason, .. } => reason,
            _ => unreachable!(),
        };
        assert!(reason.contains("value mismatch"));
    }

    #[test]
    fn time_mismatch_with_exact_tolerance() {
        let a = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let b = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(51),
            NetId::new(1),
            LogicValue::One,
        )]);
        let result = check_equivalence(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
        let reason = match result {
            EquivalenceResult::NotEquivalent { reason, .. } => reason,
            _ => unreachable!(),
        };
        assert!(reason.contains("time mismatch"));
    }

    #[test]
    fn time_within_tolerance_is_equivalent() {
        let a = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let b = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_picoseconds(50_100), // 50.1 ns
            NetId::new(1),
            LogicValue::One,
        )]);
        // 1 ns tolerance: 50 ns and 50.1 ns differ by 100 ps = 0.1 ns.
        let tol = EquivalenceTolerance::from_nanoseconds(1);
        let result = check_equivalence(&a, &b, tol);
        assert!(result.is_equivalent());
    }

    #[test]
    fn time_beyond_tolerance_is_not_equivalent() {
        let a = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let b = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(55),
            NetId::new(1),
            LogicValue::One,
        )]);
        // 2 ns tolerance: 50 ns and 55 ns differ by 5 ns.
        let tol = EquivalenceTolerance::from_nanoseconds(2);
        let result = check_equivalence(&a, &b, tol);
        assert!(!result.is_equivalent());
    }

    #[test]
    fn multi_event_traces_match_within_tolerance() {
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);
        let t100 = SimulationTime::from_nanoseconds(100);

        let a = EventTrace::from_events(vec![
            TraceEvent::new(t50, NetId::new(1), LogicValue::One),
            TraceEvent::new(t80, NetId::new(2), LogicValue::Zero),
            TraceEvent::new(t100, NetId::new(1), LogicValue::Zero),
        ]);
        // Slight time shifts within 1 ns tolerance.
        let b = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_picoseconds(50_500),
                NetId::new(1),
                LogicValue::One,
            ),
            TraceEvent::new(t80, NetId::new(2), LogicValue::Zero),
            TraceEvent::new(
                SimulationTime::from_picoseconds(100_500),
                NetId::new(1),
                LogicValue::Zero,
            ),
        ]);
        let tol = EquivalenceTolerance::from_nanoseconds(1);
        let result = check_equivalence(&a, &b, tol);
        assert!(result.is_equivalent());
    }

    #[test]
    fn mismatch_index_is_reported() {
        let a = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(80),
                NetId::new(2),
                LogicValue::Zero,
            ),
        ]);
        let b = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
            // Value mismatch at index 1.
            TraceEvent::new(
                SimulationTime::from_nanoseconds(80),
                NetId::new(2),
                LogicValue::One,
            ),
        ]);
        let result = check_equivalence(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
        let idx = match result {
            EquivalenceResult::NotEquivalent { mismatch_index, .. } => mismatch_index,
            _ => unreachable!(),
        };
        assert_eq!(idx, Some(1));
    }

    // ----- check_equivalence_per_net -----

    #[test]
    fn per_net_equivalent_when_interleaving_differs() {
        // Same events on net 1 and net 2, but the flat ordering after
        // (time, net) sort should make them equivalent.
        let a = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(2),
                LogicValue::Zero,
            ),
        ]);
        let b = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(2),
                LogicValue::Zero,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
        ]);
        // Both sort to the same canonical form, so both flat and
        // per-net check should pass.
        let result = check_equivalence_per_net(&a, &b, EquivalenceTolerance::exact());
        assert!(result.is_equivalent());
    }

    #[test]
    fn per_net_detects_mismatch_on_single_net() {
        let a = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(80),
                NetId::new(2),
                LogicValue::Zero,
            ),
        ]);
        let b = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
            // Value mismatch on net 2.
            TraceEvent::new(
                SimulationTime::from_nanoseconds(80),
                NetId::new(2),
                LogicValue::One,
            ),
        ]);
        let result = check_equivalence_per_net(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
        let reason = match result {
            EquivalenceResult::NotEquivalent { reason, .. } => reason,
            _ => unreachable!(),
        };
        assert!(reason.contains("net net:2"));
        assert!(reason.contains("value mismatch"));
    }

    #[test]
    fn per_net_detects_missing_net_in_candidate() {
        let a = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_nanoseconds(50),
                NetId::new(1),
                LogicValue::One,
            ),
            TraceEvent::new(
                SimulationTime::from_nanoseconds(80),
                NetId::new(2),
                LogicValue::Zero,
            ),
        ]);
        // Candidate is missing events on net 2.
        let b = EventTrace::from_events(vec![TraceEvent::new(
            SimulationTime::from_nanoseconds(50),
            NetId::new(1),
            LogicValue::One,
        )]);
        let result = check_equivalence_per_net(&a, &b, EquivalenceTolerance::exact());
        assert!(!result.is_equivalent());
    }

    // ----- End-to-end: kernel → trace → equivalence -----

    #[test]
    fn kernel_trace_matches_golden_within_tolerance() {
        use crate::kernel::DigitalKernel;

        let mut kernel = DigitalKernel::new();
        let t50 = SimulationTime::from_nanoseconds(50);
        let t80 = SimulationTime::from_nanoseconds(80);

        kernel
            .schedule(DigitalEvent::new(t50, NetId::new(1), LogicValue::One))
            .unwrap();
        kernel
            .schedule(DigitalEvent::new(t80, NetId::new(2), LogicValue::Zero))
            .unwrap();
        let _ = kernel.run_until(t80);

        let candidate_events = kernel.take_processed_events();
        let candidate = EventTrace::from_digital_events(&candidate_events);

        // Golden reference: same events but with slight time jitter.
        let golden = EventTrace::from_events(vec![
            TraceEvent::new(
                SimulationTime::from_picoseconds(50_200),
                NetId::new(1),
                LogicValue::One,
            ),
            TraceEvent::new(
                SimulationTime::from_picoseconds(80_200),
                NetId::new(2),
                LogicValue::Zero,
            ),
        ]);

        // Exact tolerance: should fail.
        let exact = check_equivalence(&golden, &candidate, EquivalenceTolerance::exact());
        assert!(!exact.is_equivalent());

        // 1 ns tolerance: 200 ps = 0.2 ns < 1 ns, should pass.
        let tol = EquivalenceTolerance::from_nanoseconds(1);
        let within_tol = check_equivalence(&golden, &candidate, tol);
        assert!(within_tol.is_equivalent());
    }

    // ----- EquivalenceResult display -----

    #[test]
    fn equivalence_result_display_equivalent() {
        let r = EquivalenceResult::Equivalent;
        assert_eq!(format!("{r}"), "EQUIVALENT");
    }

    #[test]
    fn equivalence_result_display_not_equivalent() {
        let r = EquivalenceResult::NotEquivalent {
            reason: "value mismatch".into(),
            mismatch_index: Some(3),
        };
        let s = format!("{r}");
        assert!(s.contains("NOT EQUIVALENT"));
        assert!(s.contains("value mismatch"));
        assert!(s.contains("at index 3"));
    }
}
