//! Integration tests for the VCD parser (`digital::vcd`).
//!
//! These tests exercise the public API of `parse_vcd` through the
//! `circuit_solver::digital::vcd` module, verifying that VCD files are
//! correctly parsed into `EventTrace` objects that interoperate with the
//! equivalence checker from `digital::equivalence`.

use circuit_solver::digital::equivalence::{
    check_equivalence, EquivalenceTolerance, Event, EventTrace, LogicValue,
};
use circuit_solver::digital::vcd::{parse_vcd, TimescaleMagnitude, TimescaleUnit, VcdParseError};

// ---------------------------------------------------------------------------
// Happy-path: round-trip with the equivalence checker
// ---------------------------------------------------------------------------

#[test]
fn vcd_clock_trace_matches_manual() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinitions $end
$dumpvars
0!
$end
#10
1!
#20
0!
#30
1!
#40
0!
";
    let parsed = parse_vcd(vcd).unwrap();
    let manual = EventTrace::from_sorted(vec![
        Event::new(0.0, "top.clk", LogicValue::Zero),
        Event::new(10e-9, "top.clk", LogicValue::One),
        Event::new(20e-9, "top.clk", LogicValue::Zero),
        Event::new(30e-9, "top.clk", LogicValue::One),
        Event::new(40e-9, "top.clk", LogicValue::Zero),
    ]);

    let result = check_equivalence(
        &parsed.trace,
        &manual,
        &EquivalenceTolerance::with_time_tolerance(1e-15),
    );
    assert!(
        result.equivalent,
        "clock trace from VCD should be equivalent to manual trace"
    );
}

#[test]
fn vcd_multi_signal_trace_matches_manual() {
    let vcd = "\
$timescale 1 ps $end
$scope module dut $end
$var wire 1 A en $end
$var wire 1 B data $end
$var wire 1 C ready $end
$upscope $end
$enddefinitions $end
$dumpvars
0A
0B
0C
$end
#1000
1A
#2000
1B
#3000
1C
0A
#4000
0B
0C
";
    let parsed = parse_vcd(vcd).unwrap();

    // ps timescale: #1000 = 1000ps = 1ns = 1e-9s
    let manual = EventTrace::from_unsorted(vec![
        Event::new(0.0, "dut.en", LogicValue::Zero),
        Event::new(0.0, "dut.data", LogicValue::Zero),
        Event::new(0.0, "dut.ready", LogicValue::Zero),
        Event::new(1e-9, "dut.en", LogicValue::One),
        Event::new(2e-9, "dut.data", LogicValue::One),
        Event::new(3e-9, "dut.ready", LogicValue::One),
        Event::new(3e-9, "dut.en", LogicValue::Zero),
        Event::new(4e-9, "dut.data", LogicValue::Zero),
        Event::new(4e-9, "dut.ready", LogicValue::Zero),
    ]);

    let result = check_equivalence(
        &parsed.trace,
        &manual,
        &EquivalenceTolerance::with_time_tolerance(1e-15),
    );
    assert!(
        result.equivalent,
        "multi-signal VCD trace should match manual trace"
    );
}

// ---------------------------------------------------------------------------
// Four-state logic values
// ---------------------------------------------------------------------------

#[test]
fn vcd_x_and_z_values() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$var wire 1 \" b $end
$upscope $end
$enddefinitions $end
$dumpvars
x!
z\"
$end
#5
0!
1\"
";
    let parsed = parse_vcd(vcd).unwrap();
    let events = parsed.trace.as_slice();

    // dumpvars: a=X, b=Z at t=0
    assert_eq!(events[0].net, "m.a");
    assert_eq!(events[0].value, LogicValue::X);
    assert_eq!(events[1].net, "m.b");
    assert_eq!(events[1].value, LogicValue::Z);

    // #5: a=0, b=1
    assert_eq!(events[2].value, LogicValue::Zero);
    assert_eq!(events[3].value, LogicValue::One);
}

#[test]
fn vcd_uppercase_x_z() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$var wire 1 \" b $end
$upscope $end
$enddefinitions $end
#0
X!
Z\"
";
    let parsed = parse_vcd(vcd).unwrap();
    let events = parsed.trace.as_slice();
    assert_eq!(events[0].value, LogicValue::X);
    assert_eq!(events[1].value, LogicValue::Z);
}

// ---------------------------------------------------------------------------
// Scope nesting and dotted net names
// ---------------------------------------------------------------------------

#[test]
fn vcd_nested_scopes() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$scope module sub1 $end
$var wire 1 ! a $end
$upscope $end
$scope module sub2 $end
$var wire 1 \" b $end
$upscope $end
$upscope $end
$enddefinitions $end
#0
1!
0\"
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.header.signals["!"], "top.sub1.a");
    assert_eq!(parsed.header.signals["\""], "top.sub2.b");
}

#[test]
fn vcd_deeply_nested_scopes() {
    let vcd = "\
$timescale 1 ns $end
$scope module a $end
$scope module b $end
$scope module c $end
$var wire 1 ! sig $end
$upscope $end
$upscope $end
$upscope $end
$enddefinitions $end
#1
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.header.signals["!"], "a.b.c.sig");
}

// ---------------------------------------------------------------------------
// Timescale handling
// ---------------------------------------------------------------------------

#[test]
fn vcd_timescale_10ps() {
    let vcd = "\
$timescale 10 ps $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#100
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    // 100 * 10ps = 1000ps = 1ns = 1e-9
    assert!((parsed.trace.as_slice()[0].time - 1e-9).abs() < 1e-20);
    assert_eq!(parsed.header.timescale.magnitude, TimescaleMagnitude::Ten);
    assert_eq!(parsed.header.timescale.unit, TimescaleUnit::Picosecond);
}

#[test]
fn vcd_timescale_100us() {
    let vcd = "\
$timescale 100 us $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#1
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    // 1 * 100us = 100us = 100e-6
    assert!((parsed.trace.as_slice()[0].time - 100e-6).abs() < 1e-15);
}

#[test]
fn vcd_timescale_1s() {
    let vcd = "\
$timescale 1 s $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#5
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    assert!((parsed.trace.as_slice()[0].time - 5.0).abs() < 1e-12);
}

#[test]
fn vcd_default_timescale_when_missing() {
    let vcd = "\
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#5
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.header.timescale.unit, TimescaleUnit::Nanosecond);
    assert_eq!(parsed.header.timescale.magnitude, TimescaleMagnitude::One);
}

// ---------------------------------------------------------------------------
// Error cases
// ---------------------------------------------------------------------------

#[test]
fn vcd_empty_input() {
    assert!(matches!(parse_vcd(""), Err(VcdParseError::EmptyInput)));
}

#[test]
fn vcd_real_var_rejected() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var real 64 ! voltage $end
$upscope $end
$enddefinitions $end
";
    assert!(matches!(
        parse_vcd(vcd),
        Err(VcdParseError::RealVarNotSupported { .. })
    ));
}

#[test]
fn vcd_dumpoff_rejected() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
$dumpoff
x!
$end
";
    assert!(matches!(
        parse_vcd(vcd),
        Err(VcdParseError::UnsupportedDumpSection { .. })
    ));
}

#[test]
fn vcd_dumpon_rejected() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#10
$dumpon
1!
$end
";
    assert!(matches!(
        parse_vcd(vcd),
        Err(VcdParseError::UnsupportedDumpSection { .. })
    ));
}

#[test]
fn vcd_dumpall_rejected() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
$dumpall
1!
$end
";
    assert!(matches!(
        parse_vcd(vcd),
        Err(VcdParseError::UnsupportedDumpSection { .. })
    ));
}

#[test]
fn vcd_invalid_timescale() {
    let vcd = "\
$timescale 5 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
";
    assert!(matches!(
        parse_vcd(vcd),
        Err(VcdParseError::InvalidTimescale { .. })
    ));
}

// ---------------------------------------------------------------------------
// Unknown signal IDs silently ignored
// ---------------------------------------------------------------------------

#[test]
fn vcd_unknown_signal_id_ignored() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#5
1!
1@
";
    let parsed = parse_vcd(vcd).unwrap();
    // Only the declared signal '!' produces an event
    assert_eq!(parsed.trace.len(), 1);
    assert_eq!(parsed.trace.as_slice()[0].net, "m.a");
}

// ---------------------------------------------------------------------------
// No timestamps → empty trace
// ---------------------------------------------------------------------------

#[test]
fn vcd_header_only_no_events() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
";
    let parsed = parse_vcd(vcd).unwrap();
    assert!(parsed.trace.is_empty());
}

// ---------------------------------------------------------------------------
// $dumpvars without prior timestamp (t=0)
// ---------------------------------------------------------------------------

#[test]
fn vcd_dumpvars_implies_time_zero() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
$dumpvars
1!
$end
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.trace.len(), 1);
    assert_eq!(parsed.trace.as_slice()[0].time, 0.0);
    assert_eq!(parsed.trace.as_slice()[0].value, LogicValue::One);
}

// ---------------------------------------------------------------------------
// Multi-character signal IDs
// ---------------------------------------------------------------------------

#[test]
fn vcd_multi_char_signal_ids() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 abc sig1 $end
$var wire 1 def sig2 $end
$upscope $end
$enddefinitions $end
#1
1abc
0def
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.header.signals["abc"], "m.sig1");
    assert_eq!(parsed.header.signals["def"], "m.sig2");
    assert_eq!(parsed.trace.len(), 2);
}

// ---------------------------------------------------------------------------
// Interoperability: parsed VCD trace vs. equivalence checker with tolerance
// ---------------------------------------------------------------------------

#[test]
fn vcd_equivalence_with_tolerance() {
    let vcd = "\
$timescale 1 ps $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#1000
1!
#2000
0!
";
    let parsed = parse_vcd(vcd).unwrap();

    // Build a "same" trace with slightly different times (within tolerance)
    let manual = EventTrace::from_sorted(vec![
        Event::new(1.000001e-9, "m.a", LogicValue::One),
        Event::new(2.000001e-9, "m.a", LogicValue::Zero),
    ]);

    let tol = EquivalenceTolerance::with_time_tolerance(1e-12);
    let result = check_equivalence(&parsed.trace, &manual, &tol);
    assert!(
        result.equivalent,
        "VCD trace should be equivalent within 1ps tolerance"
    );
}

// ---------------------------------------------------------------------------
// Var types (wire, reg, integer, tri, etc.)
// ---------------------------------------------------------------------------

#[test]
fn vcd_var_types_wire_and_reg() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! clk $end
$var reg 1 \" data $end
$upscope $end
$enddefinitions $end
#1
1!
0\"
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.header.signals.len(), 2);
    assert_eq!(parsed.header.signals["!"], "m.clk");
    assert_eq!(parsed.header.signals["\""], "m.data");
}

#[test]
fn vcd_var_types_supply0_supply1() {
    let vcd = "\
$timescale 1 ns $end
$scope module m $end
$var supply0 1 ! gnd $end
$var supply1 1 \" vdd $end
$upscope $end
$enddefinitions $end
#0
0!
1\"
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.header.signals["!"], "m.gnd");
    assert_eq!(parsed.header.signals["\""], "m.vdd");
}

// ---------------------------------------------------------------------------
// Comments in header are ignored
// ---------------------------------------------------------------------------

#[test]
fn vcd_comments_ignored() {
    let vcd = "\
$comment This is a test VCD $end
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#1
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.trace.len(), 1);
}

// ---------------------------------------------------------------------------
// $date and $version headers are ignored
// ---------------------------------------------------------------------------

#[test]
fn vcd_date_and_version_ignored() {
    let vcd = "\
$date Fri May 29 2026 $end
$version circuit-solver 0.1 $end
$timescale 1 ns $end
$scope module m $end
$var wire 1 ! a $end
$upscope $end
$enddefinitions $end
#5
1!
";
    let parsed = parse_vcd(vcd).unwrap();
    assert_eq!(parsed.trace.len(), 1);
    assert!((parsed.trace.as_slice()[0].time - 5e-9).abs() < 1e-20);
}
