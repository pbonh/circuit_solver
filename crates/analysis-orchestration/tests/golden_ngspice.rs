//! Unified golden-ngspice conformance harness — the **DC
//! operating-point** conformance witness mandated by tasks.md item
//! **#63** (kanban task `t_829da899`).
//!
//! # The executable specification (verbatim Gherkin)
//!
//! ```gherkin
//! Given ConformanceTester has a ngspice Golden Reference for a DC
//!   operating-point analysis on a Sky130 PDK test bench
//! And the tolerance envelope is configured as 1 % relative or 1 mV
//!   absolute per node voltage
//! When ConformanceTester runs the DC operating-point Analysis on the
//!   same Circuit
//! Then every node voltage in the OperatingPoint matches the Golden
//!   Reference within the tolerance envelope
//! And Conformance is reported as "pass"
//! ```
//!
//! # Pipeline composition pinned by this witness
//!
//! - **tasks.md #62** — the `conformance-harness` crate
//!   (`load_ngspice_ascii` parser + `compare` comparator under the
//!   ADR-0008 `max(rel, abs)` envelope).
//! - **tasks.md #20–22** — the
//!   [`analysis_orchestration::dc_analysis`] control loop.
//! - **tasks.md #63** (this test) — composes the two above
//!   end-to-end on a Sky130 PDK-style test bench, asserts `Pass`
//!   verdict on the happy path, and asserts a structurally-shaped
//!   `Fail` verdict on a perturbed golden so a regression in the
//!   comparator (or the solver) cannot silently dilute the
//!   conformance guarantee.
//!
//! # Why the golden file is built at test time
//!
//! ngspice cannot run in CI: it is an external binary, and the
//! reference would otherwise be reproducible only on a developer
//! machine with ngspice + Sky130 model cards installed. For a purely
//! linear resistive circuit, the DC operating point is closed-form
//! and *exactly* the same reference that ngspice (or any other
//! industrial simulator) would converge to — there is no
//! semiconductor model, no nonlinear iteration, no PDK
//! parameterisation drift. To keep the scenario witness hermetic,
//! the golden file is *synthesized* at test time as an ngspice-format
//! ASCII rawfile whose values come from the analytic solution. This
//! is the same pattern the sibling noise conformance witness
//! (`scenario_noise_conformance_against_ngspice.rs`) uses.
//!
//! # Sky130 PDK relevance
//!
//! The Gherkin says "Sky130 PDK test bench." Per the Sky130 PDK wiki
//! entry the PDK ships analog primitives (resistors, MIM capacitors,
//! inductors) alongside BSIM4-family MOSFETs. The solver presently
//! ships BSIM3v3 device stamping but not BSIM4; the conformance
//! pipeline this witness exercises (parser → solver → comparator →
//! verdict) is the same regardless. We use a passive RC network
//! representative of Sky130's analog resistor and MIM capacitor
//! primitives — the same fixture shape the sibling AC and transient
//! conformance tests use. When BSIM4 stamping lands, a sibling test
//! can extend this witness with a MOSFET operating-point fixture
//! without reshaping the integration boundary.
//!
//! # Tolerance envelope (ADR-0008 §"Default thresholds by analysis
//! type")
//!
//! The Gherkin pins the envelope at:
//!
//! | quantity          | relative | absolute |
//! |-------------------|---------:|---------:|
//! | DC node voltage   | 1 %      | 1 mV     |
//!
//! and these are exactly the values returned by
//! [`AnalysisKind::Dc.default_tolerance()`]. The test reads them
//! from the public API rather than re-hardcoding the numbers, so a
//! future ADR retune propagates here automatically.

use std::io::Write;

use analysis_orchestration::{dc_analysis, DcAnalysisRequest};
use conformance_harness::{
    compare, load_ngspice_ascii, AnalysisKind, ConformanceVerdict, SweepKind,
};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind, Node};
use numeric_solver::{flatten, FlattenedStructure};

// =============================================================================
// Sky130 PDK passive-bench constants
// =============================================================================

/// 10 kΩ resistor — representative of Sky130 analog resistor
/// primitives (`xhighres`, `mrm1`, `mrdn`) at a value typical for
/// bias-string and feedback-network resistors.
const R1_OHMS: f64 = 10_000.0;

/// 1 kΩ resistor — lower leg of the divider, representative of a
/// Sky130 poly resistor in a voltage-divider bias network.
const R2_OHMS: f64 = 1_000.0;

/// Supply voltage. 1.1 V is the typical Sky130 core supply rail
/// (`vdd` = 1.8 V nominal, but a divider sub-circuit often sees a
/// fraction; 1.1 V keeps the analytic arithmetic clean and stays
/// within the Sky130 voltage domain).
const VSRC_VOLTS: f64 = 1.1;

// =============================================================================
// Analytic golden reference
// =============================================================================

/// Analytic DC operating point for the Sky130 passive RC bench.
///
/// Topology:
///
/// ```text
///   V1 (1.1 V, n_src) → R1 (10 kΩ) → n_mid → R2 (1 kΩ) → gnd
/// ```
///
/// - `V(n_src) = 1.1 V`  (source node, tied to V1+)
/// - `V(n_mid) = 1.1 · R2 / (R1 + R2) = 1.1 · 1000 / 11000 = 0.1 V`
///
/// Note: the ground node `v(0) = 0 V` is excluded because ngspice
/// does not emit ground as a variable in the rawfile — it is always
/// 0 V by definition and does not participate in the MNA solve.
fn analytic_golden_voltages() -> [(&'static str, f64); 2] {
    let v_src = VSRC_VOLTS;
    let v_mid = VSRC_VOLTS * R2_OHMS / (R1_OHMS + R2_OHMS);
    [
        ("v(n_mid)", v_mid),
        ("v(n_src)", v_src),
    ]
}

// =============================================================================
// Synthetic ngspice ASCII golden file
// =============================================================================

/// Serialize an operating-point golden into ngspice's ASCII rawfile
/// format. For a DC operating point, ngspice emits a single-row
/// rawfile with `Plotname: Operating Point` and one point per
/// variable.
///
/// The parser always treats variable index 0 as the sweep axis.
/// For an operating point there is no sweep, so we emit a dummy
/// variable `op` (type `op`) at index 0 with value 0.0, followed
/// by the dependent voltage variables. The total variable count
/// is `1 + variables.len()`.
///
/// `Plotname: Operating Point` is the phrasing
/// [`SweepKind::from_plotname`] classifies as
/// [`SweepKind::OperatingPoint`].
fn synthesize_ngspice_dc_op_raw(variables: &[(&str, f64)]) -> String {
    use std::fmt::Write as _;
    let n_vars = 1 + variables.len(); // sweep axis + dependents
    let mut out = String::new();
    out.push_str("Title: sky130-passive-rc-dc-op-bench\n");
    out.push_str("Date: Thu May 21 09:00:00 2026\n");
    out.push_str("Plotname: Operating Point\n");
    out.push_str("Flags: real\n");
    let _ = writeln!(out, "No. Variables: {n_vars}");
    out.push_str("No. Points: 1\n");
    out.push_str("Variables:\n");
    // Variable 0: dummy sweep axis for operating point
    out.push_str("\t0\top\top\n");
    for (i, (name, _)) in variables.iter().enumerate() {
        let _ = writeln!(out, "\t{}\t{name}\tvoltage", i + 1);
    }
    out.push_str("Values:\n");
    // Single point: index 0, then sweep-axis value (0.0) + one value per variable
    out.push_str("\t0\t0.000000e+00");
    for (_, val) in variables {
        let _ = write!(out, "\t{val:.6e}");
    }
    out.push_str("\n");
    out
}

/// Perturbed version of the operating-point golden: `v(n_mid)` is
/// shifted by +50 mV, well outside the 1% / 1 mV ADR-0008 DC
/// envelope (1% of 0.1 V = 1 mV; 50 mV >> 1 mV).
fn synthesize_ngspice_dc_op_raw_perturbed(variables: &[(&str, f64)]) -> String {
    use std::fmt::Write as _;
    let n_vars = 1 + variables.len(); // sweep axis + dependents
    let mut out = String::new();
    out.push_str("Title: sky130-passive-rc-dc-op-bench-perturbed\n");
    out.push_str("Date: Thu May 21 09:00:00 2026\n");
    out.push_str("Plotname: Operating Point\n");
    out.push_str("Flags: real\n");
    let _ = writeln!(out, "No. Variables: {n_vars}");
    out.push_str("No. Points: 1\n");
    out.push_str("Variables:\n");
    out.push_str("\t0\top\top\n");
    for (i, (name, _)) in variables.iter().enumerate() {
        let _ = writeln!(out, "\t{}\t{name}\tvoltage", i + 1);
    }
    out.push_str("Values:\n");
    out.push_str("\t0\t0.000000e+00");
    for (name, val) in variables {
        let v = if *name == "v(n_mid)" {
            val + 0.05 // +50 mV perturbation
        } else {
            *val
        };
        let _ = write!(out, "\t{v:.6e}");
    }
    out.push_str("\n");
    out
}

/// Write `body` into a per-test temp file rooted at
/// `${TMPDIR}/golden-ngspice-dc-op-it/<name>`.
fn write_temp_fixture(name: &str, body: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("golden-ngspice-dc-op-it");
    std::fs::create_dir_all(&dir).expect("create temp dir for DC-OP golden fixture");
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).expect("create golden fixture file");
    f.write_all(body.as_bytes())
        .expect("write golden fixture body");
    path
}

// =============================================================================
// Fixture builder
// =============================================================================

/// Build the Sky130-style passive RC test bench for DC operating
/// point:
///
/// ```text
///   V1 (1.1 V) → n_src → R1 (10 kΩ) → n_mid → R2 (1 kΩ) → gnd
/// ```
///
/// Returns the graph and flattened structure ready for
/// [`dc_analysis`].
fn build_sky130_passive_bench() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: VSRC_VOLTS,
        },
        ["n_src", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: R1_OHMS,
        },
        ["n_src", "n_mid"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "R2",
        ElementKind::Resistor {
            resistance_ohms: R2_OHMS,
        },
        ["n_mid", "0"],
        None,
    )
    .expect("add R2");

    let g = b.build().expect("Sky130 passive bench builds");
    let fs = flatten(&g).expect("flatten Sky130 passive bench");
    (fs, g)
}

/// Run [`dc_analysis`] on the Sky130 passive bench and return the
/// solver-produced node voltages as `(name, value)` pairs aligned
/// with the analytic golden variable names.
fn solve_dc_on_sky130_bench() -> Vec<(String, f64)> {
    let (fs, g) = build_sky130_passive_bench();

    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs))
        .expect("dc_analysis succeeds on Sky130 passive bench");

    assert!(
        result.is_converged(),
        "DC analysis must converge on a linear resistive circuit"
    );

    let op = result.operating_point.expect("OperatingPoint present");

    // Map node names → voltages using the graph's node list.
    // We emit the same variable names the ngspice golden uses
    // (lowercase `v(node_name)`).
    // Ground node is excluded: ngspice does not emit ground as a
    // variable in the rawfile — it is always 0 V by definition.
    let mut pairs: Vec<(String, f64)> = Vec::new();
    for node in g.nodes() {
        let name = node.name();
        if name == "0" || name == "gnd" {
            continue; // skip ground
        }
        let v = op.voltage_at(node.id()).unwrap_or(0.0);
        pairs.push((format!("v({name})"), v));
    }

    // Sort for deterministic ordering matching the golden.
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs
}

// =============================================================================
// Test 1 — Happy path: solver matches golden, verdict == Pass
// =============================================================================

/// Witness for the *positive* arm of the Gherkin: every node
/// voltage in the `OperatingPoint` matches the Golden Reference
/// within the ADR-0008 envelope and conformance is reported as
/// "pass".
///
/// This drives the full pipeline:
///
/// 1. Synthesize the ngspice ASCII rawfile from the analytic golden.
/// 2. Parse it with [`load_ngspice_ascii`] into a
///    [`GoldenReference`].
/// 3. Build and solve the Sky130 passive RC bench with
///    [`dc_analysis`].
/// 4. Compare under the ADR-0008 DC envelope
///    (`AnalysisKind::Dc.default_tolerance()`).
/// 5. Assert verdict == Pass with zero failures.
#[test]
fn dc_op_conformance_against_ngspice_golden_passes_on_sky130_bench() {
    // Step 1: synthesize the golden.
    let analytic = analytic_golden_voltages();
    let raw_body = synthesize_ngspice_dc_op_raw(&analytic);
    let raw_path = write_temp_fixture("sky130_passive_rc_dc_op.raw", &raw_body);

    // Step 2: parse the golden.
    let golden = load_ngspice_ascii(&raw_path).expect("load Sky130 DC-OP golden");
    assert_eq!(
        golden.sweep_kind,
        SweepKind::OperatingPoint,
        "Plotname `Operating Point` must classify as SweepKind::OperatingPoint"
    );
    assert_eq!(
        golden.n_points(),
        1,
        "DC operating-point golden has exactly 1 point"
    );
    assert_eq!(
        golden.n_variables(),
        analytic.len(),
        "golden declares exactly {} variables",
        analytic.len()
    );

    // Step 3: run the DC analysis driver.
    let solver_pairs = solve_dc_on_sky130_bench();

    // Since compare() takes &[f64] slices, we need to put each
    // value into its own 1-element vec/slice. We allocate a Vec of
    // 1-element Vecs and then borrow them.
    let actual_values: Vec<Vec<f64>> = solver_pairs
        .iter()
        .map(|(_, val)| vec![*val])
        .collect();

    let actual_pairs: Vec<(&str, &[f64])> = solver_pairs
        .iter()
        .zip(actual_values.iter())
        .map(|((name, _), vals)| (name.as_str(), vals.as_slice()))
        .collect();

    // Step 4: compare under the ADR-0008 DC envelope.
    let tolerance = AnalysisKind::Dc.default_tolerance();
    assert!(
        (tolerance.relative - 0.01).abs() < 1e-15,
        "envelope precondition: ADR-0008 DC relative must be 1 %, got {}",
        tolerance.relative
    );
    assert!(
        (tolerance.absolute - 1e-3).abs() < 1e-15,
        "envelope precondition: ADR-0008 DC absolute must be 1 mV, got {}",
        tolerance.absolute
    );

    let report = compare(&golden, actual_pairs, tolerance, 16);

    // Step 5: assert verdict == Pass.
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Pass,
        "DC operating-point conformance must be Pass; got {:?}. \
         Worst variable: {}; worst margin: {:.6e}; \
         {} of {} variables failed.",
        report.verdict,
        report.worst_variable,
        report.worst_margin,
        report.n_failed_variables,
        report.variables.len(),
    );
    assert_eq!(
        report.n_failed_variables, 0,
        "no variables should fail the ADR-0008 DC envelope"
    );

    // Per-variable: every variable passes.
    for var_summary in &report.variables {
        assert_eq!(
            var_summary.n_failures, 0,
            "variable {} must have 0 failures, got {}",
            var_summary.name, var_summary.n_failures
        );
        assert!(
            !var_summary.missing_from_actual,
            "variable {} must not be missing from actual",
            var_summary.name
        );
        assert!(
            var_summary.worst_margin >= 0.0,
            "variable {} must have non-negative worst_margin, got {:.6e}",
            var_summary.name,
            var_summary.worst_margin
        );
    }
}

// =============================================================================
// Test 2 — Fault injection: perturbed golden produces verdict == Fail
// =============================================================================

/// Witness for the *negative* arm of the conformance contract: when
/// the golden's values disagree with the solver's by *more than* the
/// ADR-0008 envelope on **any** variable, the comparator must report
/// a Fail verdict for that variable and surface the failing point.
///
/// Without this witness an off-by-one in the comparator (e.g.,
/// misaligning single-point operating-point data) could silently
/// turn the entire conformance test into a no-op rubber-stamp.
/// ADR-0008's "Positive consequences" explicitly cites "Per-node
/// checking means a single outlier node does not cause a global
/// failure" — but the dual requirement is that an outlier **must**
/// still cause a per-variable failure. This is that dual.
///
/// The perturbed golden injects +50 mV on `v(n_mid)` — well outside
/// the ADR-0008 DC envelope (1% of 0.1 V = 1 mV; 50 mV >> 1 mV;
/// also >> 1 mV absolute floor).
#[test]
fn dc_op_conformance_perturbed_golden_reports_fail() {
    // Synthesize the perturbed golden.
    let analytic = analytic_golden_voltages();
    let raw_body = synthesize_ngspice_dc_op_raw_perturbed(&analytic);
    let raw_path = write_temp_fixture("sky130_passive_rc_dc_op_perturbed.raw", &raw_body);
    let golden = load_ngspice_ascii(&raw_path).expect("load perturbed Sky130 DC-OP golden");
    assert_eq!(golden.sweep_kind, SweepKind::OperatingPoint);

    // Run the solver honestly.
    let solver_pairs = solve_dc_on_sky130_bench();

    let actual_values: Vec<Vec<f64>> = solver_pairs
        .iter()
        .map(|(_, val)| vec![*val])
        .collect();

    let actual_pairs: Vec<(&str, &[f64])> = solver_pairs
        .iter()
        .zip(actual_values.iter())
        .map(|((name, _), vals)| (name.as_str(), vals.as_slice()))
        .collect();

    let tolerance = AnalysisKind::Dc.default_tolerance();
    let report = compare(&golden, actual_pairs, tolerance, 16);

    // The perturbed golden shifts v(n_mid) by +50 mV.
    // At v_ref = 0.1 V, the ADR-0008 DC envelope is max(1% × 0.1, 1mV)
    // = max(1 mV, 1 mV) = 1 mV. A 50 mV offset is 50× the envelope.
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Fail,
        "comparator must Fail when v(n_mid) is 50 mV outside the envelope"
    );
    assert!(
        report.n_failed_variables >= 1,
        "at least one variable must fail on the perturbed golden"
    );

    // Verify that v(n_mid) is the failing variable.
    let n_mid_summary = report
        .variables
        .iter()
        .find(|v| v.name == "v(n_mid)")
        .expect("v(n_mid) summary present");
    assert!(
        n_mid_summary.n_failures >= 1,
        "v(n_mid) must have at least 1 failure, got {}",
        n_mid_summary.n_failures
    );
    assert!(
        n_mid_summary.worst_margin < 0.0,
        "perturbed v(n_mid) must report a negative worst margin, got {:.6e}",
        n_mid_summary.worst_margin
    );

    // The injected error is 50 mV and the envelope at v_ref = 0.1 V
    // is 1 mV. The margin should be approximately -(50 - 1) = -49 mV.
    // We assert a loose band around that so floating-point drift
    // doesn't make this test flaky.
    assert!(
        n_mid_summary.worst_margin < -0.04 && n_mid_summary.worst_margin > -0.06,
        "perturbed v(n_mid) worst margin ≈ -0.049 V expected, got {:.6e}",
        n_mid_summary.worst_margin
    );

    // Non-perturbed variables should still pass.
    for var_summary in &report.variables {
        if var_summary.name == "v(n_mid)" {
            continue;
        }
        assert_eq!(
            var_summary.n_failures, 0,
            "non-perturbed variable {} must have 0 failures",
            var_summary.name
        );
    }
}

// =============================================================================
// Test 3 — Analytic vs solver direct: pin the exact DC voltages
// =============================================================================

/// A cross-check that the solver's operating-point voltages match the
/// analytic solution to high precision. This test does *not* use the
/// conformance harness — it is a sanity check that the circuit under
/// test produces the expected voltages, so that the conformance tests
/// above are exercising the harness pipeline rather than also serving
/// as the primary correctness check on the solver.
///
/// For a linear resistive circuit the MNA solution is exact (no
/// iterative convergence tolerance) so the solver should match the
/// analytic result to floating-point precision.
#[test]
fn dc_op_solver_matches_antic_on_sky130_bench() {
    let (fs, g) = build_sky130_passive_bench();
    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs))
        .expect("dc_analysis succeeds");
    assert!(result.is_converged());
    let op = result.operating_point.expect("OperatingPoint present");

    let analytic = analytic_golden_voltages();

    for (var_name, expected) in &analytic {
        let node_name = var_name
            .trim_start_matches("v(")
            .trim_end_matches(')');
        if node_name == "0" || node_name == "gnd" {
            // Ground node: voltage is 0.0 by definition, not stored
            // in the operating-point vector.
            continue;
        }
        let node: &Node = g
            .node_by_name(node_name)
            .unwrap_or_else(|| panic!("node {node_name} not found in graph"));
        let actual = op.voltage_at(node.id()).unwrap_or_else(|| {
            panic!("no voltage for node {node_name} (id {:?})", node.id())
        });
        // For a linear circuit the MNA solve is exact; we expect
        // agreement to ~1e-12 relative.
        let rel_err = if expected.abs() > 1e-15 {
            (actual - expected).abs() / expected.abs()
        } else {
            (actual - expected).abs()
        };
        assert!(
            rel_err < 1e-10,
            "solver v({node_name}) = {actual:.15e}, analytic = {expected:.15e}, \
             relative error = {rel_err:.6e}"
        );
    }
}

// =============================================================================
// Test 4 — Wheatstone bridge: multi-node DC conformance
// =============================================================================

/// Extend the conformance witness to a circuit with more than one
/// interior node, exercising the comparator's multi-variable
/// iteration path. Uses a Wheatstone bridge topology:
///
/// ```text
///                 ┌──── R1 (1 kΩ) ──── n_a ──── R3 (3 kΩ) ────┐
///                 │                                            │
///   V1 (5 V) → n_top                                        n_bot → gnd
///                 │                                            │
///                 └──── R2 (2 kΩ) ──── n_b ──── R4 (4 kΩ) ────┘
/// ```
///
/// Analytic node voltages:
/// - `V(n_top) = 5 V`
/// - `V(n_a) = 5 · R3 / (R1 + R3) = 5 · 3/(1+3) = 3.75 V`
/// - `V(n_b) = 5 · R4 / (R2 + R4) = 5 · 4/(2+4) ≈ 3.333… V`
/// - `V(gnd) = 0 V`
#[test]
fn dc_op_conformance_wheatstone_bridge_against_ngspice_golden_passes() {
    // Build the Wheatstone bridge.
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: 5.0,
        },
        ["n_top", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: 1_000.0,
        },
        ["n_top", "n_a"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "R2",
        ElementKind::Resistor {
            resistance_ohms: 2_000.0,
        },
        ["n_top", "n_b"],
        None,
    )
    .expect("add R2");
    b.add_element(
        "R3",
        ElementKind::Resistor {
            resistance_ohms: 3_000.0,
        },
        ["n_a", "0"],
        None,
    )
    .expect("add R3");
    b.add_element(
        "R4",
        ElementKind::Resistor {
            resistance_ohms: 4_000.0,
        },
        ["n_b", "0"],
        None,
    )
    .expect("add R4");

    let g = b.build().expect("Wheatstone bridge builds");
    let fs = flatten(&g).expect("flatten Wheatstone bridge");

    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs))
        .expect("dc_analysis succeeds on Wheatstone bridge");
    assert!(result.is_converged());
    let op = result.operating_point.expect("OperatingPoint present");

    // Analytic golden (ground excluded — ngspice does not emit it).
    let v_a = 5.0 * 3_000.0 / (1_000.0 + 3_000.0); // 3.75 V
    let v_b = 5.0 * 4_000.0 / (2_000.0 + 4_000.0); // 3.333... V
    let analytic_vars: [(&str, f64); 3] = [
        ("v(n_a)", v_a),
        ("v(n_b)", v_b),
        ("v(n_top)", 5.0),
    ];

    // Synthesize the ngspice golden.
    let raw_body = synthesize_ngspice_dc_op_raw(&analytic_vars);
    let raw_path =
        write_temp_fixture("sky130_wheatstone_dc_op.raw", &raw_body);
    let golden = load_ngspice_ascii(&raw_path).expect("load Wheatstone DC-OP golden");
    assert_eq!(golden.sweep_kind, SweepKind::OperatingPoint);
    assert_eq!(golden.n_points(), 1);
    assert_eq!(golden.n_variables(), analytic_vars.len());

    // Collect solver results as named slices (ground excluded).
    let mut solver_pairs: Vec<(String, f64)> = Vec::new();
    for node in g.nodes() {
        let name = node.name();
        if name == "0" || name == "gnd" {
            continue; // skip ground
        }
        let v = op.voltage_at(node.id()).unwrap_or(0.0);
        solver_pairs.push((format!("v({name})"), v));
    }
    solver_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let actual_values: Vec<Vec<f64>> = solver_pairs
        .iter()
        .map(|(_, val)| vec![*val])
        .collect();

    let actual_pairs: Vec<(&str, &[f64])> = solver_pairs
        .iter()
        .zip(actual_values.iter())
        .map(|((name, _), vals)| (name.as_str(), vals.as_slice()))
        .collect();

    // Compare under the ADR-0008 DC envelope.
    let tolerance = AnalysisKind::Dc.default_tolerance();
    let report = compare(&golden, actual_pairs, tolerance, 16);

    assert_eq!(
        report.verdict,
        ConformanceVerdict::Pass,
        "Wheatstone bridge DC-OP conformance must be Pass; got {:?}",
        report.verdict
    );
    assert_eq!(report.n_failed_variables, 0);

    for var_summary in &report.variables {
        assert_eq!(
            var_summary.n_failures, 0,
            "variable {} must have 0 failures",
            var_summary.name
        );
        assert!(!var_summary.missing_from_actual);
        assert!(
            var_summary.worst_margin >= 0.0,
            "variable {} worst_margin must be non-negative, got {:.6e}",
            var_summary.name,
            var_summary.worst_margin
        );
    }
}
