//! Icarus Verilog golden-trace harness wired to the event-trace-equivalence checker.
//!
//! # Spec traceability
//!
//! - Scenario: `digital-equivalence#ordered-events-not-vcd`
//! - Task #29: Icarus Verilog golden-trace harness wired to the event-trace-
//!   equivalence checker. (depends on #24, #25)
//!
//! # Design
//!
//! This test harness exercises the full pipeline:
//!
//! 1. Write a Verilog source (+ testbench) to a temp directory.
//! 2. Compile with `iverilog` and simulate with `vvp`, producing a VCD file.
//! 3. Parse the VCD output via `digital::vcd::parse_vcd` into an `EventTrace`.
//! 4. Build a manually-expected `EventTrace` from known circuit behaviour.
//! 5. Compare the two traces with `check_equivalence`.
//!
//! Equivalence is judged on **ordered events**, not byte-level VCD identity
//! (per scenario `digital-equivalence#ordered-events-not-vcd`).  A small
//! timing tolerance accounts for floating-point rounding when Icarus Verilog
//! converts integer tick counts to real-valued timestamps.
//!
//! The tests in the "ordered-events-not-vcd" section (tests 10–13) directly
//! verify the core guarantee: two VCD files with different byte-level layouts
//! but the same ordered (time, net, value) event sequences are equivalent,
//! while two VCD files that happen to share byte patterns but differ in
//! event semantics are *not* equivalent.

use std::fs;
use std::process::Command;

use circuit_solver::digital::equivalence::{
    check_equivalence, EquivalenceTolerance, Event, EventTrace, LogicValue,
};
use circuit_solver::digital::vcd::parse_vcd;

// ---------------------------------------------------------------------------
// Harness helpers
// ---------------------------------------------------------------------------

/// Run `iverilog` to compile `source` into `output_vvp`, then `vvp` to
/// simulate and produce `output_vcd`.  Returns the VCD content as a String.
///
/// # Panics
///
/// Panics if `iverilog` or `vvp` cannot be found on PATH, or if either
/// command exits with a non-zero status.
fn simulate_verilog(verilog: &str, testbench: &str) -> String {
    let dir = tempfile::tempdir().expect("create temp dir");
    let src_path = dir.path().join("dut.v");
    let tb_path = dir.path().join("tb.v");
    let vvp_path = dir.path().join("sim.vvp");
    let vcd_path = dir.path().join("sim.vcd");

    fs::write(&src_path, verilog).expect("write dut.v");
    fs::write(&tb_path, testbench).expect("write tb.v");

    // Compile
    let compile = Command::new("iverilog")
        .arg("-o")
        .arg(&vvp_path)
        .arg(&src_path)
        .arg(&tb_path)
        .output()
        .expect("iverilog not found on PATH — install Icarus Verilog");

    if !compile.status.success() {
        let stderr = String::from_utf8_lossy(&compile.stderr);
        panic!("iverilog compilation failed:\n{}", stderr);
    }

    // Simulate — run vvp from the temp directory so $dumpfile("sim.vcd")
    // writes the VCD next to the .vvp file.
    let simulate = Command::new("vvp")
        .current_dir(dir.path())
        .arg(&vvp_path)
        .output()
        .expect("vvp not found on PATH — install Icarus Verilog");

    if !simulate.status.success() {
        let stderr = String::from_utf8_lossy(&simulate.stderr);
        panic!("vvp simulation failed:\n{}", stderr);
    }

    fs::read_to_string(&vcd_path)
        .unwrap_or_else(|e| panic!("failed to read VCD output at {}: {}", vcd_path.display(), e))
}

/// Parse a VCD string into an `EventTrace`, panicking on parse errors.
fn parse_vcd_trace(vcd: &str) -> EventTrace {
    let result = parse_vcd(vcd).unwrap_or_else(|e| panic!("VCD parse error: {}", e));
    result.trace
}

/// Convenience: build an `EventTrace` from (time, net, value) tuples.
fn make_trace(pairs: &[(f64, &str, LogicValue)]) -> EventTrace {
    EventTrace::from_unsorted(pairs.iter().map(|&(t, n, v)| Event::new(t, n, v)).collect())
}

/// Tolerance used for Icarus-Verilog golden-trace comparison.
///
/// 1e-15 seconds covers ULP-level rounding from integer-tick-to-float
/// conversion (e.g. `30u64 * 1e-9_f64` may differ by 1 ULP from the
/// literal `30e-9_f64`) while being far below any practical timing
/// resolution.
fn icarus_tolerance() -> EquivalenceTolerance {
    EquivalenceTolerance::with_time_tolerance(1e-15)
}

// ===========================================================================
// Test 1: Inverter (NOT gate)
// ===========================================================================

#[test]
fn inverter_golden_trace() {
    let dut = r#"
module inverter(input wire a, output wire y);
    assign y = ~a;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a;
    wire y;

    inverter uut(.a(a), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0;
        #10;
        a = 1;
        #10;
        a = 0;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // With `timescale 1ns / 1ps`, Icarus writes $timescale 1ps to VCD.
    // VCD timestamps are in picoseconds; parse_vcd converts to seconds.
    // #10 in Verilog = 10ns = 10000ps → 10000 * 1e-12 = 10e-9.
    //
    // Expected events (time in seconds, net names from Icarus scoping):
    //   t=0:   tb.a=0, tb.y=1  (initial + combinational output)
    //   t=10ns: tb.a=1, tb.y=0
    //   t=20ns: tb.a=0, tb.y=1
    let expected = make_trace(&[
        (0.0, "tb.a", LogicValue::Zero),
        (0.0, "tb.y", LogicValue::One),
        (10e-9, "tb.a", LogicValue::One),
        (10e-9, "tb.y", LogicValue::Zero),
        (20e-9, "tb.a", LogicValue::Zero),
        (20e-9, "tb.y", LogicValue::One),
    ]);

    let result = check_equivalence(&actual, &expected, &icarus_tolerance());
    assert!(
        result.equivalent,
        "inverter golden trace mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 2: AND gate
// ===========================================================================

#[test]
fn and_gate_golden_trace() {
    let dut = r#"
module and_gate(input wire a, input wire b, output wire y);
    assign y = a & b;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a, b;
    wire y;

    and_gate uut(.a(a), .b(b), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0; b = 0;
        #10;
        a = 0; b = 1;
        #10;
        a = 1; b = 0;
        #10;
        a = 1; b = 1;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // Expected: y = a & b
    // t=0:   a=0 b=0 y=0
    // t=10ns: b=1, y stays 0 (a=0)
    // t=20ns: a=1 b=0, y stays 0
    // t=30ns: b=1, y=1 (a=1 & b=1)
    //
    // Icarus only emits VCD value-change records, so:
    //   at t=0:  a=0, b=0, y=0
    //   at t=10: b=1  (a and y unchanged — y stays 0 because a=0)
    //   at t=20: b=0, a=1  (y stays 0 because b=0)
    //   at t=30: b=1, y=1
    let expected = make_trace(&[
        (0.0, "tb.a", LogicValue::Zero),
        (0.0, "tb.b", LogicValue::Zero),
        (0.0, "tb.y", LogicValue::Zero),
        (10e-9, "tb.b", LogicValue::One),
        (20e-9, "tb.a", LogicValue::One),
        (20e-9, "tb.b", LogicValue::Zero),
        (30e-9, "tb.b", LogicValue::One),
        (30e-9, "tb.y", LogicValue::One),
    ]);

    let result = check_equivalence(&actual, &expected, &icarus_tolerance());
    assert!(
        result.equivalent,
        "AND gate golden trace mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 3: Clock divider (register-based)
// ===========================================================================

#[test]
fn clock_divider_golden_trace() {
    let dut = r#"
module clock_divider(input wire clk, output reg q);
    initial q = 0;
    always @(posedge clk)
        q <= ~q;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg clk;
    wire q;

    clock_divider uut(.clk(clk), .q(q));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        clk = 0;
        forever #5 clk = ~clk;
    end

    initial begin
        #50;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // Clock toggles every 5ns.  q toggles on every rising edge of clk:
    //   t=0:   clk=0, q=0  (initial)
    //   t=5ns: clk=1 → q=1 (posedge)
    //   t=10ns: clk=0 (negedge, q unchanged)
    //   t=15ns: clk=1 → q=0 (posedge)
    //   t=20ns: clk=0
    //   t=25ns: clk=1 → q=1 (posedge)
    //   t=30ns: clk=0
    //   t=35ns: clk=1 → q=0 (posedge)
    //   t=40ns: clk=0
    //   t=45ns: clk=1 → q=1 (posedge)
    //
    // Icarus only emits value-change records. After initial dumpvars,
    // subsequent changes are:
    let expected = make_trace(&[
        (0.0, "tb.clk", LogicValue::Zero),
        (0.0, "tb.q", LogicValue::Zero),
        (5e-9, "tb.q", LogicValue::One),
        (5e-9, "tb.clk", LogicValue::One),
        (10e-9, "tb.clk", LogicValue::Zero),
        (15e-9, "tb.q", LogicValue::Zero),
        (15e-9, "tb.clk", LogicValue::One),
        (20e-9, "tb.clk", LogicValue::Zero),
        (25e-9, "tb.q", LogicValue::One),
        (25e-9, "tb.clk", LogicValue::One),
        (30e-9, "tb.clk", LogicValue::Zero),
        (35e-9, "tb.q", LogicValue::Zero),
        (35e-9, "tb.clk", LogicValue::One),
        (40e-9, "tb.clk", LogicValue::Zero),
        (45e-9, "tb.q", LogicValue::One),
        (45e-9, "tb.clk", LogicValue::One),
        (50e-9, "tb.clk", LogicValue::Zero),
    ]);

    let result = check_equivalence(&actual, &expected, &icarus_tolerance());
    assert!(
        result.equivalent,
        "clock divider golden trace mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 4: X propagation through a buffer (initial X → resolved)
// ===========================================================================

#[test]
fn x_propagation_golden_trace() {
    let dut = r#"
module buffer(input wire a, output wire y);
    assign y = a;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a;
    wire y;

    buffer uut(.a(a), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 1'bx;
        #10;
        a = 1;
        #10;
        a = 0;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // t=0:   a=X, y=X (X propagates through buffer)
    // t=10ns: a=1, y=1
    // t=20ns: a=0, y=0
    let expected = make_trace(&[
        (0.0, "tb.a", LogicValue::X),
        (0.0, "tb.y", LogicValue::X),
        (10e-9, "tb.a", LogicValue::One),
        (10e-9, "tb.y", LogicValue::One),
        (20e-9, "tb.a", LogicValue::Zero),
        (20e-9, "tb.y", LogicValue::Zero),
    ]);

    let result = check_equivalence(&actual, &expected, &icarus_tolerance());
    assert!(
        result.equivalent,
        "X propagation golden trace mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 5: Z (high-impedance) on tri-state buffer
// ===========================================================================

#[test]
fn tristate_z_golden_trace() {
    let dut = r#"
module tristate(input wire a, input wire en, output wire y);
    assign y = en ? a : 1'bz;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a, en;
    wire y;

    tristate uut(.a(a), .en(en), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 1; en = 1;
        #10;
        en = 0;
        #10;
        a = 0;
        #10;
        en = 1;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // t=0:   a=1, en=1, y=1 (driven)
    // t=10ns: en=0, y=Z (high-impedance)
    // t=20ns: a=0 (y stays Z because en=0)
    // t=30ns: en=1, y=0 (driven by a=0)
    let expected = make_trace(&[
        (0.0, "tb.a", LogicValue::One),
        (0.0, "tb.en", LogicValue::One),
        (0.0, "tb.y", LogicValue::One),
        (10e-9, "tb.en", LogicValue::Zero),
        (10e-9, "tb.y", LogicValue::Z),
        (20e-9, "tb.a", LogicValue::Zero),
        (30e-9, "tb.en", LogicValue::One),
        (30e-9, "tb.y", LogicValue::Zero),
    ]);

    let result = check_equivalence(&actual, &expected, &icarus_tolerance());
    assert!(
        result.equivalent,
        "tri-state Z golden trace mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 6: Multi-module hierarchy — nested scopes in VCD
// ===========================================================================

#[test]
fn multi_module_hierarchy_golden_trace() {
    let dut = r#"
module half_adder(input wire a, input wire b, output wire sum, output wire cout);
    assign sum = a ^ b;
    assign cout = a & b;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a, b;
    wire sum, cout;

    half_adder uut(.a(a), .b(b), .sum(sum), .cout(cout));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0; b = 0;
        #10;
        a = 1;
        #10;
        b = 1;
        #10;
        a = 0;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // Half adder: sum = a ^ b, cout = a & b
    // t=0:   a=0 b=0 sum=0 cout=0
    // t=10ns: a=1, sum=1 (b stays 0, cout stays 0)
    // t=20ns: b=1, sum=0, cout=1
    // t=30ns: a=0, sum=1, cout=0 (b stays 1)
    let expected = make_trace(&[
        (0.0, "tb.a", LogicValue::Zero),
        (0.0, "tb.b", LogicValue::Zero),
        (0.0, "tb.cout", LogicValue::Zero),
        (0.0, "tb.sum", LogicValue::Zero),
        (10e-9, "tb.a", LogicValue::One),
        (10e-9, "tb.sum", LogicValue::One),
        (20e-9, "tb.b", LogicValue::One),
        (20e-9, "tb.cout", LogicValue::One),
        (20e-9, "tb.sum", LogicValue::Zero),
        (30e-9, "tb.a", LogicValue::Zero),
        (30e-9, "tb.cout", LogicValue::Zero),
        (30e-9, "tb.sum", LogicValue::One),
    ]);

    let result = check_equivalence(&actual, &expected, &icarus_tolerance());
    assert!(
        result.equivalent,
        "half-adder hierarchy golden trace mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 7: Equivalence with relaxed timing tolerance
// ===========================================================================

#[test]
fn equivalence_with_relaxed_tolerance() {
    let dut = r#"
module inverter(input wire a, output wire y);
    assign y = ~a;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a;
    wire y;

    inverter uut(.a(a), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0;
        #10;
        a = 1;
        #10;
        a = 0;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // Build an "expected" trace that's shifted by a tiny amount.
    // With 1ps tolerance, traces that differ by up to 1ps should still
    // be considered equivalent.
    let expected = make_trace(&[
        (0.0, "tb.a", LogicValue::Zero),
        (0.0, "tb.y", LogicValue::One),
        (10e-9 + 0.5e-12, "tb.a", LogicValue::One), // 0.5ps off
        (10e-9 + 0.5e-12, "tb.y", LogicValue::Zero),
        (20e-9 + 0.3e-12, "tb.a", LogicValue::Zero), // 0.3ps off
        (20e-9 + 0.3e-12, "tb.y", LogicValue::One),
    ]);

    let tol = EquivalenceTolerance::with_time_tolerance(1e-12); // 1ps
    let result = check_equivalence(&actual, &expected, &tol);
    assert!(
        result.equivalent,
        "relaxed-tolerance inverter mismatch: {}",
        result
    );
}

// ===========================================================================
// Test 8: Detect mismatch — wrong expected value
// ===========================================================================

#[test]
fn detects_value_mismatch_against_golden() {
    let dut = r#"
module inverter(input wire a, output wire y);
    assign y = ~a;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a;
    wire y;

    inverter uut(.a(a), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0;
        #10;
        a = 1;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // Deliberately set wrong expected value for y at t=0:
    // inverter output should be 1 when a=0, but we expect 0.
    let wrong_expected = make_trace(&[
        (0.0, "tb.a", LogicValue::Zero),
        (0.0, "tb.y", LogicValue::Zero), // wrong! should be One
        (10e-9, "tb.a", LogicValue::One),
        (10e-9, "tb.y", LogicValue::One), // wrong! should be Zero
    ]);

    let result = check_equivalence(&actual, &wrong_expected, &icarus_tolerance());
    assert!(
        !result.equivalent,
        "should detect value mismatch against golden trace"
    );
}

// ===========================================================================
// Test 9: Detect timing mismatch beyond tolerance
// ===========================================================================

#[test]
fn detects_timing_mismatch_beyond_tolerance() {
    let dut = r#"
module inverter(input wire a, output wire y);
    assign y = ~a;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a;
    wire y;

    inverter uut(.a(a), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0;
        #10;
        a = 1;
        #10;
        $finish;
    end
endmodule
"#;

    let vcd = simulate_verilog(dut, tb);
    let actual = parse_vcd_trace(&vcd);

    // Build an expected trace with times shifted by 100ps — beyond the
    // default 1e-15 tolerance.
    let shifted = make_trace(&[
        (0.0, "tb.a", LogicValue::Zero),
        (0.0, "tb.y", LogicValue::One),
        (10e-9 + 100e-12, "tb.a", LogicValue::One), // 100ps off
        (10e-9 + 100e-12, "tb.y", LogicValue::Zero),
    ]);

    let result = check_equivalence(&actual, &shifted, &icarus_tolerance());
    assert!(
        !result.equivalent,
        "should detect timing mismatch beyond tolerance"
    );
}

// ===========================================================================
// Tests 10–13: ordered-events-not-vcd
//
// These tests directly verify the spec scenario guarantee: equivalence is
// judged on ordered (time, net, value) event sequences, NOT on byte-level
// VCD identity.  Two VCD files that differ in header comments, $var
// declaration order, whitespace, or timescale magnitude but encode the same
// ordered events must be equivalent.  Conversely, VCD files that happen to
// share byte patterns but carry different event semantics must NOT be
// equivalent.
// ===========================================================================

// ---------------------------------------------------------------------------
// Test 10: VCD with reordered $var declarations produces equivalent trace
// ---------------------------------------------------------------------------

#[test]
fn vcd_reordered_vars_equivalent() {
    // VCD-A: standard Icarus-like output order (a before y)
    let vcd_a = r#"$date
   Mon Jan 1 00:00:00 2026
$end
$version
   Icarus Verilog
$end
$timescale 1ps $end
$scope module tb $end
$var wire 1 a a $end
$var wire 1 y y $end
$upscope $end
$enddefinitions $end
$dumpvars
0a
1y
$end
#10000
1a
0y
#20000
0a
1y
"#;

    // VCD-B: same circuit, but $var declarations in opposite order (y before a)
    let vcd_b = r#"$date
   Mon Jan 1 00:00:00 2026
$end
$version
   Different Simulator v2.0
$end
$timescale 1ps $end
$scope module tb $end
$var wire 1 y y $end
$var wire 1 a a $end
$upscope $end
$enddefinitions $end
$dumpvars
0a
1y
$end
#10000
1a
0y
#20000
0a
1y
"#;

    let trace_a = parse_vcd_trace(vcd_a);
    let trace_b = parse_vcd_trace(vcd_b);

    // The VCD files have different $var ordering and different $version
    // strings, but the ordered events are identical.
    let result = check_equivalence(&trace_a, &trace_b, &icarus_tolerance());
    assert!(
        result.equivalent,
        "reordered VCD vars should produce equivalent event traces: {}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 11: VCD with different timescale magnitude but same physical time
// ---------------------------------------------------------------------------

#[test]
fn vcd_different_timescale_magnitude_equivalent() {
    // VCD-A: timescale 1ps, timestamps in picoseconds
    //   #10000 = 10000ps = 10ns
    //   #20000 = 20000ps = 20ns
    let vcd_a = r#"$timescale 1ps $end
$scope module tb $end
$var wire 1 a a $end
$upscope $end
$enddefinitions $end
$dumpvars
0a
$end
#10000
1a
#20000
0a
$end
"#;

    // VCD-B: timescale 10ps, timestamps in deka-picoseconds
    //   #1000 = 1000 * 10ps = 10000ps = 10ns
    //   #2000 = 2000 * 10ps = 20000ps = 20ns
    // Same physical events, different VCD encoding.
    let vcd_b = r#"$timescale 10ps $end
$scope module tb $end
$var wire 1 a a $end
$upscope $end
$enddefinitions $end
$dumpvars
0a
$end
#1000
1a
#2000
0a
$end
"#;

    let trace_a = parse_vcd_trace(vcd_a);
    let trace_b = parse_vcd_trace(vcd_b);

    let result = check_equivalence(&trace_a, &trace_b, &icarus_tolerance());
    assert!(
        result.equivalent,
        "VCD with different timescale magnitude but same physical time should be equivalent: {}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 12: VCD with extra header comments and whitespace — still equivalent
// ---------------------------------------------------------------------------

#[test]
fn vcd_extra_header_whitespace_equivalent() {
    // VCD-A: minimal VCD
    let vcd_a = r#"$timescale 1ns $end
$scope module tb $end
$var wire 1 clk clk $end
$var wire 1 q q $end
$upscope $end
$enddefinitions $end
$dumpvars
0clk
0q
$end
#5
1clk
1q
#10
0clk
$end
"#;

    // VCD-B: same events, but with $comment, $date, extra blank lines, and
    // a different $version string.  The byte-level content is very different.
    let vcd_b = r#"$comment
   This file was generated by a custom simulation backend.
   It has extra metadata that should not affect equivalence.
$end
$date
   Tue Feb 28 12:34:56 2026
$end
$version
   CustomSim 3.14
$end
$timescale 1ns $end

$scope module tb $end

$var wire 1 clk clk $end
$var wire 1 q q $end

$upscope $end
$enddefinitions $end

$dumpvars
0clk
0q
$end
#5
1clk
1q
#10
0clk
$end
"#;

    let trace_a = parse_vcd_trace(vcd_a);
    let trace_b = parse_vcd_trace(vcd_b);

    let result = check_equivalence(&trace_a, &trace_b, &icarus_tolerance());
    assert!(
        result.equivalent,
        "VCD with extra header/comments/whitespace should be equivalent: {}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 13: Icarus VCD vs hand-crafted VCD — same ordered events
//          (full pipeline: iverilog → VCD → parse → compare with manual VCD)
// ---------------------------------------------------------------------------

#[test]
fn icarus_vs_manual_vcd_ordered_events_equivalent() {
    // Run Icarus Verilog to produce a real VCD
    let dut = r#"
module inverter(input wire a, output wire y);
    assign y = ~a;
endmodule
"#;

    let tb = r#"
`timescale 1ns / 1ps
module tb;
    reg a;
    wire y;

    inverter uut(.a(a), .y(y));

    initial begin
        $dumpfile("sim.vcd");
        $dumpvars(1, tb);
        a = 0;
        #10;
        a = 1;
        #10;
        a = 0;
        #10;
        $finish;
    end
endmodule
"#;

    let icarus_vcd = simulate_verilog(dut, tb);
    let icarus_trace = parse_vcd_trace(&icarus_vcd);

    // Hand-craft a VCD that encodes the same ordered events but with a
    // completely different byte-level representation: different header,
    // different $var shorthand identifiers, different comment, different
    // timescale magnitude encoding the same physical time.
    //
    // Icarus uses `1ps` timescale; we use `100ps` with timestamps scaled
    // by 1/10.  Icarus uses shorthand 'a'/'y'; we use '!'/'@'.  The net
    // names after the shorthand are what the parser resolves, and those
    // net names are what equivalence checks — not the shorthand IDs.
    let manual_vcd = r#"$comment Manually crafted VCD for equivalence test $end
$timescale 100ps $end
$scope module tb $end
$var wire 1 ! a $end
$var wire 1 @ y $end
$upscope $end
$enddefinitions $end
$dumpvars
0!
1@
$end
#100
1!
0@
#200
0!
1@
"#;

    let manual_trace = parse_vcd_trace(manual_vcd);

    let result = check_equivalence(&icarus_trace, &manual_trace, &icarus_tolerance());
    assert!(
        result.equivalent,
        "Icarus VCD and manually-crafted VCD with same ordered events should be equivalent: {}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 14: VCD byte similarity does NOT imply event equivalence
//          (negative case — the "not VCD" part of ordered-events-not-vcd)
// ---------------------------------------------------------------------------

#[test]
fn vcd_byte_similar_but_event_different_not_equivalent() {
    // Two VCD files that share net names and have the same number of events,
    // but the events carry different values.  Byte-level VCD is nearly
    // identical (only 0↔1 swapped on one net at one timestep), but the
    // ordered event sequences differ — so they must NOT be equivalent.

    let vcd_correct = r#"$timescale 1ns $end
$scope module tb $end
$var wire 1 a a $end
$var wire 1 y y $end
$upscope $end
$enddefinitions $end
$dumpvars
0a
1y
$end
#10
1a
0y
$end
"#;

    // Same structure, but y=0 at t=0 and y=1 at t=10ns — inverted output
    // is wrong (NOT an inverter).  Only 2 bytes differ in the VCD.
    let vcd_wrong = r#"$timescale 1ns $end
$scope module tb $end
$var wire 1 a a $end
$var wire 1 y y $end
$upscope $end
$enddefinitions $end
$dumpvars
0a
0y
$end
#10
1a
1y
$end
"#;

    let trace_correct = parse_vcd_trace(vcd_correct);
    let trace_wrong = parse_vcd_trace(vcd_wrong);

    let result = check_equivalence(&trace_correct, &trace_wrong, &icarus_tolerance());
    assert!(
        !result.equivalent,
        "VCD files that are byte-similar but carry different event values must NOT be equivalent"
    );
}
