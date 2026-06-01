//! Integration tests for the VCD interchange parser.
//!
//! These tests exercise the public VCD API at a higher level than the inline
//! unit tests, covering realistic VCD files and the spec scenario
//! `digital-equivalence#vcd-interchange-only`.

use circuit_solver::digital::equivalence::{
    check_equivalence, EquivalenceTolerance, Event, EventTrace, LogicValue,
};
use circuit_solver::digital::vcd::{trace_to_vcd, VcdParseError, VcdParser};

// -----------------------------------------------------------------------
// Helper: a realistic clock-data VCD snippet
// -----------------------------------------------------------------------

fn clock_data_vcd() -> &'static str {
    "\
$date
   2026-01-01
$end
$version
   circuit-solver test
$end
$timescale
   1 ns
$end
$scope module top $end
$var wire 1 ! clk $end
$var wire 1 \" data $end
$upscope $end
$enddefinition $end
#0
0!
0\"
#5
1!
#10
0!
1\"
#15
1!
0\"
#20
0!
"
}

// -----------------------------------------------------------------------
// Scenario: parse realistic VCD and check event count
// -----------------------------------------------------------------------

#[test]
fn parse_realistic_vcd_event_count() {
    let trace = VcdParser::parse(clock_data_vcd()).unwrap();
    // clk: 5 transitions (#0, #5, #10, #15, #20)
    // data: 3 transitions (#0, #10, #15)
    assert_eq!(trace.len(), 8);
}

// -----------------------------------------------------------------------
// Scenario: VCD events match hand-built expected trace
// -----------------------------------------------------------------------

#[test]
fn vcd_events_match_expected_trace() {
    let trace = VcdParser::parse(clock_data_vcd()).unwrap();
    let expected = EventTrace::from_unsorted(vec![
        Event::new(0.0, "top.clk", LogicValue::Zero),
        Event::new(0.0, "top.data", LogicValue::Zero),
        Event::new(5e-9, "top.clk", LogicValue::One),
        Event::new(10e-9, "top.clk", LogicValue::Zero),
        Event::new(10e-9, "top.data", LogicValue::One),
        Event::new(15e-9, "top.clk", LogicValue::One),
        Event::new(15e-9, "top.data", LogicValue::Zero),
        Event::new(20e-9, "top.clk", LogicValue::Zero),
    ]);
    let result = check_equivalence(&trace, &expected, &EquivalenceTolerance::with_time_tolerance(1e-20));
    assert!(result.equivalent, "mismatch: {}", result);
}

// -----------------------------------------------------------------------
// Scenario: round-trip preserves semantic equivalence
// -----------------------------------------------------------------------

#[test]
fn round_trip_preserves_semantic_equivalence() {
    let trace = VcdParser::parse(clock_data_vcd()).unwrap();
    let vcd_out = trace_to_vcd(&trace, "1 ns");
    let trace2 = VcdParser::parse(&vcd_out).unwrap();

    let result = check_equivalence(&trace, &trace2, &EquivalenceTolerance::exact());
    assert!(result.equivalent, "round-trip failed: {}", result);
}

// -----------------------------------------------------------------------
// Scenario: VCD byte layout does NOT matter for equivalence
// -----------------------------------------------------------------------

#[test]
fn vcd_byte_layout_irrelevant_for_equivalence() {
    // Two VCDs with different formatting/whitespace/comments must produce
    // equivalent event traces.
    let vcd_a = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0
0!
#5
1!
#10
0!
";

    let vcd_b = "\
$date 2026-01-01 $end
$version test $end
$timescale   1   ns   $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
// comments are fine
#0
0!
#5
1!
#10
0!
";

    let trace_a = VcdParser::parse(vcd_a).unwrap();
    let trace_b = VcdParser::parse(vcd_b).unwrap();

    let result = check_equivalence(&trace_a, &trace_b, &EquivalenceTolerance::exact());
    assert!(result.equivalent, "byte layout should not matter: {}", result);
}

// -----------------------------------------------------------------------
// Scenario: picosecond timescale converts correctly
// -----------------------------------------------------------------------

#[test]
fn picosecond_timescale_converts_correctly() {
    let vcd = "\
$timescale 10 ps $end
$scope module top $end
$var wire 1 ! sig $end
$upscope $end
$enddefinition $end
#0
0!
#100
1!
";
    let trace = VcdParser::parse(vcd).unwrap();
    let events = trace.as_slice();

    // 100 VCD units * 10ps = 1000ps = 1ns = 1e-9 s
    assert!((events[1].time - 1e-9).abs() < 1e-20);
}

// -----------------------------------------------------------------------
// Scenario: nested scopes produce dot-separated names
// -----------------------------------------------------------------------

#[test]
fn nested_scopes_produce_dot_separated_names() {
    let vcd = "\
$timescale 1 ns $end
$scope module cpu $end
$scope module alu $end
$var wire 1 ! result $end
$upscope $end
$scope module regs $end
$var wire 1 \" r0 $end
$upscope $end
$upscope $end
$enddefinition $end
#0
1!
0\"
";
    let trace = VcdParser::parse(vcd).unwrap();
    let events = trace.as_slice();

    assert_eq!(events[0].net, "cpu.alu.result");
    assert_eq!(events[1].net, "cpu.regs.r0");
}

// -----------------------------------------------------------------------
// Scenario: multi-bit bus produces per-bit events
// -----------------------------------------------------------------------

#[test]
fn bus_produces_per_bit_events() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 3 ! addr $end
$upscope $end
$enddefinition $end
#0
b101 !
";
    let trace = VcdParser::parse(vcd).unwrap();
    assert_eq!(trace.len(), 3);

    let events = trace.as_slice();
    // addr[2]=1, addr[1]=0, addr[0]=1 (MSB first in VCD)
    assert_eq!(events[0].net, "top.addr[0]");
    assert_eq!(events[0].value, LogicValue::One);
    assert_eq!(events[1].net, "top.addr[1]");
    assert_eq!(events[1].value, LogicValue::Zero);
    assert_eq!(events[2].net, "top.addr[2]");
    assert_eq!(events[2].value, LogicValue::One);
}

// -----------------------------------------------------------------------
// Scenario: bus with leading zeros pads correctly
// -----------------------------------------------------------------------

#[test]
fn bus_with_leading_zeros() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 8 ! data $end
$upscope $end
$enddefinition $end
#0
b1 !
";
    let trace = VcdParser::parse(vcd).unwrap();
    // 8-bit bus, value "1" → padded to "00000001" → 8 events
    assert_eq!(trace.len(), 8);

    let events = trace.as_slice();
    // data[0] = 1, data[1..7] = 0
    assert_eq!(events[0].net, "top.data[0]");
    assert_eq!(events[0].value, LogicValue::One);
    for i in 1..8 {
        assert_eq!(events[i].value, LogicValue::Zero,
            "top.data[{}] should be Zero", i);
    }
}

// -----------------------------------------------------------------------
// Scenario: X and Z in bus values
// -----------------------------------------------------------------------

#[test]
fn bus_with_x_and_z_values() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 4 ! bus $end
$upscope $end
$enddefinition $end
#0
bX1Z0 !
";
    let trace = VcdParser::parse(vcd).unwrap();
    assert_eq!(trace.len(), 4);

    let events = trace.as_slice();
    // bus[3]=X, bus[2]=1, bus[1]=Z, bus[0]=0
    assert_eq!(events[0].net, "top.bus[0]");
    assert_eq!(events[0].value, LogicValue::Zero);
    assert_eq!(events[1].net, "top.bus[1]");
    assert_eq!(events[1].value, LogicValue::Z);
    assert_eq!(events[2].net, "top.bus[2]");
    assert_eq!(events[2].value, LogicValue::One);
    assert_eq!(events[3].net, "top.bus[3]");
    assert_eq!(events[3].value, LogicValue::X);
}

// -----------------------------------------------------------------------
// Scenario: empty input is rejected
// -----------------------------------------------------------------------

#[test]
fn empty_input_rejected() {
    let result = VcdParser::parse("");
    assert!(matches!(result, Err(VcdParseError::EmptyInput)));

    let result = VcdParser::parse("   \n\n  ");
    assert!(matches!(result, Err(VcdParseError::EmptyInput)));
}

// -----------------------------------------------------------------------
// Scenario: missing timescale is rejected
// -----------------------------------------------------------------------

#[test]
fn missing_timescale_rejected() {
    let vcd = "\
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0
0!
";
    let result = VcdParser::parse(vcd);
    assert!(matches!(result, Err(VcdParseError::MissingHeader(_))));
}

// -----------------------------------------------------------------------
// Scenario: unknown identifier in value change is rejected
// -----------------------------------------------------------------------

#[test]
fn unknown_identifier_rejected() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#0
1#
";
    let result = VcdParser::parse(vcd);
    assert!(matches!(result, Err(VcdParseError::UnknownIdentifier(_))));
}

// -----------------------------------------------------------------------
// Scenario: invalid time value is rejected
// -----------------------------------------------------------------------

#[test]
fn invalid_time_rejected() {
    let vcd = "\
$timescale 1 ns $end
$scope module top $end
$var wire 1 ! clk $end
$upscope $end
$enddefinition $end
#abc
1!
";
    let result = VcdParser::parse(vcd);
    assert!(matches!(result, Err(VcdParseError::InvalidTime(_))));
}

// -----------------------------------------------------------------------
// Scenario: error display formatting
// -----------------------------------------------------------------------

#[test]
fn error_display_formatting() {
    let e = VcdParseError::EmptyInput;
    assert!(format!("{}", e).contains("empty"));

    let e = VcdParseError::UnknownIdentifier("!!".into());
    assert!(format!("{}", e).contains("!!"));

    let e = VcdParseError::InvalidTimescale("bad".into());
    assert!(format!("{}", e).contains("bad"));
}

// -----------------------------------------------------------------------
// Scenario: large VCD with repeated clock pattern
// -----------------------------------------------------------------------

#[test]
fn large_vcd_repeated_clock() {
    let mut vcd = String::from(
        "$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n$upscope $end\n$enddefinition $end\n",
    );

    for i in 0..1000u64 {
        let value = if i % 2 == 0 { "0!" } else { "1!" };
        vcd.push_str(&format!("#{}\n{}\n", i * 5, value));
    }

    let trace = VcdParser::parse(&vcd).unwrap();
    assert_eq!(trace.len(), 1000);

    // Verify first and last events
    let events = trace.as_slice();
    assert_eq!(events[0].time, 0.0);
    assert_eq!(events[0].value, LogicValue::Zero);
    assert!((events[999].time - 4995e-9).abs() < 1e-20);
}

// -----------------------------------------------------------------------
// Scenario: trace_to_vcd produces parseable output
// -----------------------------------------------------------------------

#[test]
fn trace_to_vcd_produces_parseable_output() {
    // Flat nets (no scope prefix) should round-trip correctly.
    let trace = EventTrace::from_unsorted(vec![
        Event::new(0.0, "sig", LogicValue::Zero),
        Event::new(5e-9, "sig", LogicValue::One),
        Event::new(10e-9, "sig", LogicValue::X),
    ]);
    let vcd = trace_to_vcd(&trace, "1 ns");
    let trace2 = VcdParser::parse(&vcd).unwrap();

    let result = check_equivalence(&trace, &trace2, &EquivalenceTolerance::exact());
    assert!(result.equivalent, "trace_to_vcd round-trip: {}", result);

    // Scoped nets should also round-trip correctly.
    let trace_scoped = EventTrace::from_unsorted(vec![
        Event::new(0.0, "top.clk", LogicValue::Zero),
        Event::new(5e-9, "top.clk", LogicValue::One),
    ]);
    let vcd_scoped = trace_to_vcd(&trace_scoped, "1 ns");
    let trace3 = VcdParser::parse(&vcd_scoped).unwrap();

    let result2 = check_equivalence(&trace_scoped, &trace3, &EquivalenceTolerance::exact());
    assert!(result2.equivalent, "scoped round-trip: {}", result2);
}
