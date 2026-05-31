//! Integration tests for the event-trace equivalence checker.
//!
//! These tests exercise the public API at a higher level than the inline
//! unit tests in `equivalence.rs`, covering realistic scenarios that a
//! user of the digital kernel would encounter.

use circuit_solver::digital::equivalence::{
    check_equivalence, EquivalenceMismatch, EquivalenceTolerance, Event, EventTrace, LogicValue,
};

// -----------------------------------------------------------------------
// Helper: build a trace from (time, net, value) tuples
// -----------------------------------------------------------------------

fn make_trace(pairs: &[(f64, &str, LogicValue)]) -> EventTrace {
    EventTrace::from_unsorted(pairs.iter().map(|&(t, n, v)| Event::new(t, n, v)).collect())
}

// -----------------------------------------------------------------------
// Scenario: two identical traces are equivalent
// -----------------------------------------------------------------------

#[test]
fn identical_traces_are_equivalent() {
    let t = make_trace(&[
        (0.0, "clk", LogicValue::One),
        (5e-9, "clk", LogicValue::Zero),
        (10e-9, "clk", LogicValue::One),
        (10e-9, "data", LogicValue::Zero),
    ]);
    let result = check_equivalence(&t, &t, &EquivalenceTolerance::exact());
    assert!(result.equivalent);
    assert!(result.first_mismatch.is_none());
}

// -----------------------------------------------------------------------
// Scenario: extra event in actual trace → length mismatch
// -----------------------------------------------------------------------

#[test]
fn extra_event_in_actual_gives_length_mismatch() {
    let actual = make_trace(&[
        (0.0, "clk", LogicValue::One),
        (5e-9, "clk", LogicValue::Zero),
        (10e-9, "clk", LogicValue::One),
    ]);
    let expected = make_trace(&[
        (0.0, "clk", LogicValue::One),
        (5e-9, "clk", LogicValue::Zero),
    ]);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
    match result.first_mismatch {
        Some(EquivalenceMismatch::LengthMismatch {
            actual_len: 3,
            expected_len: 2,
        }) => {}
        other => panic!("expected LengthMismatch(3, 2), got {:?}", other),
    }
}

// -----------------------------------------------------------------------
// Scenario: logic value differs at the same (time, net) position
// -----------------------------------------------------------------------

#[test]
fn wrong_logic_value_is_detected() {
    let actual = make_trace(&[
        (0.0, "clk", LogicValue::Zero), // wrong: should be One
    ]);
    let expected = make_trace(&[(0.0, "clk", LogicValue::One)]);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
    assert!(matches!(
        result.first_mismatch,
        Some(EquivalenceMismatch::ValueMismatch { index: 0, .. })
    ));
}

// -----------------------------------------------------------------------
// Scenario: time difference within tolerance → equivalent
// -----------------------------------------------------------------------

#[test]
fn timing_within_tolerance_is_equivalent() {
    let actual = make_trace(&[(1.000_000_001, "clk", LogicValue::One)]);
    let expected = make_trace(&[(1.0, "clk", LogicValue::One)]);
    let tol = EquivalenceTolerance::with_time_tolerance(1e-6);
    let result = check_equivalence(&actual, &expected, &tol);
    assert!(result.equivalent);
}

// -----------------------------------------------------------------------
// Scenario: time difference exceeds tolerance → mismatch
// -----------------------------------------------------------------------

#[test]
fn timing_exceeding_tolerance_is_mismatch() {
    let actual = make_trace(&[(1.002, "clk", LogicValue::One)]);
    let expected = make_trace(&[(1.0, "clk", LogicValue::One)]);
    let tol = EquivalenceTolerance::with_time_tolerance(1e-3);
    let result = check_equivalence(&actual, &expected, &tol);
    assert!(!result.equivalent);
    assert!(matches!(
        result.first_mismatch,
        Some(EquivalenceMismatch::TimeMismatch { index: 0, .. })
    ));
}

// -----------------------------------------------------------------------
// Scenario: net name differs → mismatch
// -----------------------------------------------------------------------

#[test]
fn different_net_name_is_detected() {
    let actual = make_trace(&[(1.0, "clk", LogicValue::One)]);
    let expected = make_trace(&[(1.0, "rst", LogicValue::One)]);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
    assert!(matches!(
        result.first_mismatch,
        Some(EquivalenceMismatch::NetMismatch { index: 0, .. })
    ));
}

// -----------------------------------------------------------------------
// Scenario: X and Z values must match exactly, never coerced
// -----------------------------------------------------------------------

#[test]
fn x_value_not_equal_to_zero_or_one() {
    for val in [LogicValue::Zero, LogicValue::One, LogicValue::Z] {
        let actual = make_trace(&[(1.0, "a", LogicValue::X)]);
        let expected = make_trace(&[(1.0, "a", val)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent, "X should not equal {:?}", val);
    }
}

#[test]
fn z_value_not_equal_to_zero_or_one_or_x() {
    for val in [LogicValue::Zero, LogicValue::One, LogicValue::X] {
        let actual = make_trace(&[(1.0, "a", LogicValue::Z)]);
        let expected = make_trace(&[(1.0, "a", val)]);
        let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
        assert!(!result.equivalent, "Z should not equal {:?}", val);
    }
}

// -----------------------------------------------------------------------
// Scenario: multi-net trace with mixed timing tolerance
// -----------------------------------------------------------------------

#[test]
fn multi_net_trace_with_tolerance() {
    let actual = make_trace(&[
        (0.0, "clk", LogicValue::One),
        (0.0, "data", LogicValue::Zero),
        (5.001e-9, "clk", LogicValue::Zero),
        (5.001e-9, "data", LogicValue::One),
        (10.0005e-9, "clk", LogicValue::One),
    ]);
    let expected = make_trace(&[
        (0.0, "clk", LogicValue::One),
        (0.0, "data", LogicValue::Zero),
        (5.000e-9, "clk", LogicValue::Zero),
        (5.000e-9, "data", LogicValue::One),
        (10.000e-9, "clk", LogicValue::One),
    ]);
    let tol = EquivalenceTolerance::with_time_tolerance(2e-12);
    let result = check_equivalence(&actual, &expected, &tol);
    assert!(result.equivalent);
}

// -----------------------------------------------------------------------
// Scenario: first mismatch is reported, not the last
// -----------------------------------------------------------------------

#[test]
fn first_mismatch_is_reported() {
    let actual = make_trace(&[
        (0.0, "clk", LogicValue::Zero),  // mismatch #1: value
        (5e-9, "clk", LogicValue::Zero), // mismatch #2: value (if #1 ignored)
    ]);
    let expected = make_trace(&[
        (0.0, "clk", LogicValue::One),
        (5e-9, "clk", LogicValue::One),
    ]);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
    assert!(matches!(
        result.first_mismatch,
        Some(EquivalenceMismatch::ValueMismatch { index: 0, .. })
    ));
}

// -----------------------------------------------------------------------
// Scenario: both empty traces → equivalent
// -----------------------------------------------------------------------

#[test]
fn empty_traces_are_equivalent() {
    let a = EventTrace::new();
    let e = EventTrace::new();
    let result = check_equivalence(&a, &e, &EquivalenceTolerance::exact());
    assert!(result.equivalent);
}

// -----------------------------------------------------------------------
// Scenario: one empty, one non-empty → length mismatch
// -----------------------------------------------------------------------

#[test]
fn one_empty_one_nonempty_is_length_mismatch() {
    let a = EventTrace::new();
    let e = make_trace(&[(0.0, "clk", LogicValue::One)]);
    let result = check_equivalence(&a, &e, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
    assert!(matches!(
        result.first_mismatch,
        Some(EquivalenceMismatch::LengthMismatch { .. })
    ));
}

// -----------------------------------------------------------------------
// Scenario: zero tolerance means exact time match required
// -----------------------------------------------------------------------

#[test]
fn zero_tolerance_requires_exact_times() {
    let expected = make_trace(&[(1.0, "a", LogicValue::One)]);
    let actual = make_trace(&[(1.0 + 1e-10, "a", LogicValue::One)]);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
}

// -----------------------------------------------------------------------
// Scenario: large trace — equivalence holds
// -----------------------------------------------------------------------

#[test]
fn large_equivalent_trace() {
    let mut actual_events = Vec::new();
    let mut expected_events = Vec::new();
    for i in 0..1000u64 {
        let t = i as f64 * 1e-9;
        actual_events.push(Event::new(
            t,
            "clk",
            if i % 2 == 0 {
                LogicValue::One
            } else {
                LogicValue::Zero
            },
        ));
        expected_events.push(Event::new(
            t,
            "clk",
            if i % 2 == 0 {
                LogicValue::One
            } else {
                LogicValue::Zero
            },
        ));
    }
    let actual = EventTrace::from_sorted(actual_events);
    let expected = EventTrace::from_sorted(expected_events);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(result.equivalent);
}

// -----------------------------------------------------------------------
// Scenario: large trace with one injected mismatch
// -----------------------------------------------------------------------

#[test]
fn large_trace_with_one_mismatch() {
    let mut actual_events = Vec::new();
    let mut expected_events = Vec::new();
    for i in 0..1000u64 {
        let t = i as f64 * 1e-9;
        let v = if i % 2 == 0 {
            LogicValue::One
        } else {
            LogicValue::Zero
        };
        actual_events.push(Event::new(t, "clk", v));
        if i == 500 {
            expected_events.push(Event::new(t, "clk", LogicValue::Zero));
        } else {
            expected_events.push(Event::new(t, "clk", v));
        }
    }
    let actual = EventTrace::from_sorted(actual_events);
    let expected = EventTrace::from_sorted(expected_events);
    let result = check_equivalence(&actual, &expected, &EquivalenceTolerance::exact());
    assert!(!result.equivalent);
    assert!(matches!(
        result.first_mismatch,
        Some(EquivalenceMismatch::ValueMismatch { index: 500, .. })
    ));
}

// -----------------------------------------------------------------------
// Scenario: EventTrace::push maintains sort order with many interleaved pushes
// -----------------------------------------------------------------------

#[test]
fn push_maintains_sort_with_interleaved_nets() {
    let mut trace = EventTrace::new();
    for i in (0..50).rev() {
        let t = i as f64 * 1e-9;
        trace.push(Event::new(t, "clk", LogicValue::One));
        trace.push(Event::new(t, "data", LogicValue::Zero));
        trace.push(Event::new(t, "rst", LogicValue::One));
    }
    for window in trace.iter().as_slice().windows(2) {
        assert!(
            window[0] <= window[1],
            "events not sorted: {:?} > {:?}",
            window[0],
            window[1]
        );
    }
    assert_eq!(trace.len(), 150);
}
