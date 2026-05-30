//! Scenario-level integration test for
//! `dc-operating-point#dc-sweep-over-a-voltage-source`.
//!
//! This file is the executable witness for the Gherkin scenario
//! inlined into kanban task `t_6d57797f`. It exercises the **public**
//! API of `analysis-orchestration` (and its transitive
//! `numeric-solver` and `netlist-graph` dependencies) end-to-end by
//! running an 11-step DC sweep on a voltage-divider circuit and
//! asserting the spec's observable promises:
//!
//! 1. *The Simulator returns a `Result` containing 11
//!    `OperatingPoints`.*
//! 2. *Each `OperatingPoint` matches the corresponding Golden
//!    Reference within the tolerance envelope.*
//! 3. *The `Result` is addressable by sweep index.*
//!
//! Sibling unit tests inside
//! `crates/analysis-orchestration/src/dc_sweep.rs` cover the broader
//! API contracts (error surface, builder overrides, empty / single
//! sweep degeneracies, graph-isolation between substitute graphs).
//! This integration test is intentionally narrower and load-bearing
//! for **this** scenario only: it consumes solely the public crate
//! exports, so a future refactor that breaks the v1 surface fails
//! here loudly.
//!
//! # Gherkin (verbatim, from
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md`)
//!
//! ```text
//! Given CircuitDesigner has constructed a Circuit with a swept
//!       voltage source "V1"
//! And the sweep range is 0 V to 5 V in 11 steps
//! When CircuitDesigner submits a DC Sweep Analysis request
//! Then the Simulator returns a Result containing 11 OperatingPoints
//! And each OperatingPoint matches the corresponding Golden Reference
//!     within the tolerance envelope
//! And the Result is addressable by sweep index
//! ```
//!
//! # Golden-reference choice
//!
//! Per the inlined glossary, *"Golden Reference — a trusted external
//! simulator against which results are compared."* The cross-cutting
//! ngspice conformance harness lives at tasks.md item #62 and is
//! gated on the full v1 stack; that harness is tracked on its own
//! kanban thread. For a **linear resistive** voltage divider the
//! analytic DC operating point at each sweep value is closed-form
//! and *exactly* the same reference that ngspice (or any other
//! industrial simulator) would converge to — there is no
//! semiconductor model, no nonlinear iteration, no PDK
//! parameterisation drift. We therefore use the analytic solution
//! as the golden reference for this scenario witness. This mirrors
//! the pattern established by sibling `analysis-orchestration`
//! integration tests
//! (`tests/scenario_linear_resistive_dc_operating_point.rs`).
//!
//! # Tolerance envelope (ADR-0008 row "DC")
//!
//! Per ADR-0008 *Per-Node max(Relative, Absolute) Tolerance Envelope*
//! the DC defaults are:
//!
//! | quantity          | relative | absolute |
//! |-------------------|---------:|---------:|
//! | DC node voltage   | 1 %      | 1 mV     |
//! | DC branch current | 1 %      | 1 µA     |
//!
//! and the pass criterion is `|err| ≤ max(rel · |ref|, abs)`. Against
//! an analytic reference (no ngspice rounding) we typically beat the
//! absolute floor by many decades, so passing this envelope is a
//! strong correctness signal.

// Numerical-test pragmas: this file is a Gherkin-mirroring witness
// that pins exact f64 endpoints from the spec ("0 V to 5 V in 11
// steps") and indexes into parallel `values`/`points` vectors. The
// strict float-cmp lint trips on the `endpoint == endpoint`
// assertions but those are the *right* test (we're checking that
// the linspace builder did not drift); the needless-range-loop lint
// trips on the parallel-index sweep but the readable form mirrors
// the Gherkin "each OperatingPoint ... at the corresponding sweep
// index" phrasing. The wildcard-match lint trips on a single-variant
// error match where we want the catch-all message to evolve as the
// error enum grows. The doc-markdown lint trips on the inlined
// Gherkin scenario quoted verbatim from spec.md — we preserve the
// spec wording exactly rather than back-tick-quoting domain nouns.
#![allow(
    clippy::float_cmp,
    clippy::needless_range_loop,
    clippy::match_wildcard_for_single_variants,
    clippy::doc_markdown
)]

use analysis_orchestration::{dc_sweep, DcSweepError, DcSweepPoint, DcSweepRequest, DcSweepResult};
use circuit_solver_types::NodeId;
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{flatten, FlattenedStructure};

// --- Constants from ADR-0008 (DC defaults) ----------------------------------

/// Relative tolerance for DC node voltages, per ADR-0008.
const DC_V_REL: f64 = 0.01;
/// Absolute floor for DC node voltages, per ADR-0008.
const DC_V_ABS: f64 = 1e-3;
/// Relative tolerance for DC branch currents (treated identically
/// to voltages per ADR-0008's "DC" row covering both node voltages
/// and branch currents under a shared envelope).
const DC_I_REL: f64 = 0.01;
/// Absolute floor for DC branch currents (1 µA — micro-scale by
/// analogy with the 1 mV voltage floor; the ADR-0008 table folds
/// branch currents into the same "DC" row, leaving the absolute
/// floor to be chosen per circuit scale. For these milliampere-scale
/// test circuits 1 µA is the conservative analogue of 1 mV at the
/// millivolt scale; matches the sibling
/// `scenario_linear_resistive_dc_operating_point.rs` choice and the
/// aggregator-facing note carried forward from reviewer t_d1dcd7ed).
const DC_I_ABS: f64 = 1e-6;

/// True iff `actual` is within the
/// `max(relative · |reference|, absolute)` band of `reference`. This
/// is the ADR-0008 envelope operator applied to a scalar quantity.
fn within_envelope(actual: f64, reference: f64, rel: f64, abs: f64) -> bool {
    let bound = (rel * reference.abs()).max(abs);
    (actual - reference).abs() <= bound
}

// --- Builder helpers --------------------------------------------------------

fn add_resistor(b: &mut CircuitBuilder, name: &str, n1: &str, n2: &str, ohms: f64) {
    b.add_element(
        name,
        ElementKind::Resistor {
            resistance_ohms: ohms,
        },
        [n1, n2],
        None,
    )
    .expect("add resistor");
}

fn add_voltage_source(b: &mut CircuitBuilder, name: &str, plus: &str, minus: &str, volts: f64) {
    b.add_element(
        name,
        ElementKind::VoltageSource {
            voltage_volts: volts,
        },
        [plus, minus],
        None,
    )
    .expect("add voltage source");
}

fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
    g.nodes()
        .iter()
        .find(|n| n.name() == name)
        .expect("node present")
        .id()
}

// --- Test circuit -----------------------------------------------------------

/// The witnessing circuit: a 1:1 voltage divider whose top source
/// `V1` is the swept parameter.
///
/// ```text
///   V1 (swept, n_in) → R1 (1 kΩ) → n_mid → R2 (1 kΩ) → gnd
/// ```
///
/// Analytic golden reference at sweep value `v`:
///
/// - `V(n_in) = v`
/// - `V(n_mid) = v · R2 / (R1 + R2) = v / 2`
/// - `V(gnd) = 0`
/// - `|i_V1| = v / (R1 + R2) = v / 2 kΩ` (sign per assembler stamping
///   convention; magnitude is the testable invariant — see
///   aggregator-facing note carried forward from reviewer t_d1dcd7ed
///   noting that the spec does not pin the sign convention).
fn swept_divider(initial_v: f64) -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_in", "0", initial_v);
    add_resistor(&mut b, "R1", "n_in", "n_mid", 1_000.0);
    add_resistor(&mut b, "R2", "n_mid", "0", 1_000.0);
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    (fs, g)
}

/// Generate the 11-step sweep schedule 0 V → 5 V (inclusive endpoints,
/// 0.5 V step). Mirrors the spec phrasing "0 V to 5 V in 11 steps".
fn zero_to_five_in_eleven_steps() -> Vec<f64> {
    (0..=10).map(|i| 0.5 * f64::from(i)).collect()
}

// --- Scenario assertions ----------------------------------------------------

fn assert_voltage_within_envelope(label: &str, actual: f64, reference: f64) {
    assert!(
        within_envelope(actual, reference, DC_V_REL, DC_V_ABS),
        "DC voltage at {label} = {actual} V violates the ADR-0008 envelope around \
         reference {reference} V (rel={DC_V_REL}, abs={DC_V_ABS} V)"
    );
}

fn assert_current_magnitude_within_envelope(label: &str, actual: f64, reference: f64) {
    assert!(
        within_envelope(actual.abs(), reference.abs(), DC_I_REL, DC_I_ABS),
        "DC branch-current magnitude at {label} = {} A violates the ADR-0008 envelope \
         around reference magnitude {} A (rel={DC_I_REL}, abs={DC_I_ABS} A)",
        actual.abs(),
        reference.abs()
    );
}

// --- Tests ------------------------------------------------------------------

/// **Headline scenario witness.**
///
/// > Given CircuitDesigner has constructed a Circuit with a swept
/// > voltage source "V1"
/// > And the sweep range is 0 V to 5 V in 11 steps
/// > When CircuitDesigner submits a DC Sweep Analysis request
/// > Then the Simulator returns a Result containing 11 OperatingPoints
/// > And each OperatingPoint matches the corresponding Golden Reference
/// > within the tolerance envelope
/// > And the Result is addressable by sweep index
#[test]
fn dc_sweep_over_a_voltage_source_zero_to_five_volts_eleven_steps() {
    // "Given CircuitDesigner has constructed a Circuit with a swept
    // voltage source 'V1'"
    let (fs, g) = swept_divider(0.0);

    // "And the sweep range is 0 V to 5 V in 11 steps"
    let values = zero_to_five_in_eleven_steps();
    assert_eq!(
        values.len(),
        11,
        "11 sweep points expected from the schedule"
    );
    assert!((values[0] - 0.0).abs() < 1e-15);
    assert!((values[10] - 5.0).abs() < 1e-15);

    // "When CircuitDesigner submits a DC Sweep Analysis request"
    let result: DcSweepResult =
        dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &values)).expect("dc sweep ok");

    // "Then the Simulator returns a Result containing 11
    // OperatingPoints"
    assert_eq!(result.len(), 11, "expected 11 sweep points in the result");
    assert_eq!(result.source_name, "V1");
    assert!(
        result.all_converged(),
        "every sweep point should converge on this linear divider"
    );

    let n_in = node_id(&g, "n_in");
    let n_mid = node_id(&g, "n_mid");

    // "And each OperatingPoint matches the corresponding Golden
    // Reference within the tolerance envelope"
    //
    // Golden reference per sweep value v:
    //   V(n_in) = v
    //   V(n_mid) = v / 2
    //   V(gnd) = 0
    //   |i_V1| = v / 2 kΩ
    for (i, v) in values.iter().copied().enumerate() {
        // "And the Result is addressable by sweep index" — exercised
        // here directly via the accessor.
        let pt: &DcSweepPoint = result.point(i).expect("sweep index in range");
        assert_eq!(pt.source_value, v);

        let op = pt
            .analysis
            .operating_point
            .as_ref()
            .expect("OperatingPoint present at every sweep point");

        assert_voltage_within_envelope(
            &format!("n_in @ i={i}, v={v}"),
            op.voltage_at(n_in).unwrap(),
            v,
        );
        assert_voltage_within_envelope(
            &format!("n_mid @ i={i}, v={v}"),
            op.voltage_at(n_mid).unwrap(),
            v / 2.0,
        );
        assert_voltage_within_envelope(
            &format!("gnd @ i={i}, v={v}"),
            op.voltage_at(NodeId::GROUND).unwrap(),
            0.0,
        );

        // Branch current: |i_V1| = v / 2 kΩ.
        assert_eq!(
            op.branch_currents.len(),
            1,
            "the only branch unknown is the voltage source's MNA branch"
        );
        assert_current_magnitude_within_envelope(
            &format!("i_V1 @ i={i}, v={v}"),
            op.branch_currents[0].current_amperes,
            v / 2_000.0,
        );
    }
}

/// Sweep index addressability — second-tier witness that exercises
/// the bounded accessor contract independently of the convergence
/// assertion above. The spec's *"addressable by sweep index"*
/// criterion implies bounded access; this test pins the boundary
/// behavior.
#[test]
fn sweep_result_is_addressable_by_index_and_returns_none_out_of_range() {
    let (fs, g) = swept_divider(0.0);
    let values = zero_to_five_in_eleven_steps();
    let result = dc_sweep(DcSweepRequest::new(&g, &fs, "V1", &values)).expect("ok");

    // In-range indices resolve.
    for i in 0..values.len() {
        let pt = result.point(i).expect("in range");
        assert_eq!(pt.source_value, values[i]);
    }
    // Out-of-range indices return None (the spec phrasing is silent
    // on negative or past-the-end behavior; bounded `Option` is the
    // idiomatic Rust choice and matches `slice::get`).
    assert!(result.point(values.len()).is_none());
    assert!(result.point(usize::MAX).is_none());
}

/// The user-supplied graph is left logically untouched by the
/// sweep — a behavioural counterpart to ADR-0001's *Immutable
/// `CircuitGraph` Handle*. The sweep substitutes via
/// `with_voltage_source_value` which returns a *new* graph; the
/// caller's graph must still report its original `V1` value after
/// the sweep returns.
#[test]
fn sweep_preserves_user_graph_immutability() {
    let (fs, g) = swept_divider(7.5);
    let original = match g.element_by_name("V1").unwrap().kind() {
        ElementKind::VoltageSource { voltage_volts } => *voltage_volts,
        other => panic!("expected V1 to be a voltage source, got {other:?}"),
    };
    let _ = dc_sweep(DcSweepRequest::new(
        &g,
        &fs,
        "V1",
        &zero_to_five_in_eleven_steps(),
    ))
    .expect("ok");
    let after = match g.element_by_name("V1").unwrap().kind() {
        ElementKind::VoltageSource { voltage_volts } => *voltage_volts,
        other => panic!("expected V1 to be a voltage source, got {other:?}"),
    };
    assert!(
        (original - after).abs() < 1e-15,
        "user graph's V1 changed across sweep: before={original}, after={after}"
    );
}

/// Negative-path witness: requesting a sweep against a non-existent
/// source name surfaces a structural error at sweep index 0, *before*
/// any per-point analysis runs. This is the v1 contract for "the
/// caller asked for something we cannot do".
#[test]
fn sweep_against_unknown_source_errors_structurally() {
    let (fs, g) = swept_divider(0.0);
    let err = dc_sweep(DcSweepRequest::new(&g, &fs, "VBOGUS", &[1.0, 2.0]))
        .expect_err("expected SourceOverrideFailed");
    match err {
        DcSweepError::SourceOverrideFailed { sweep_index, .. } => {
            assert_eq!(sweep_index, 0);
        }
        other => panic!("expected SourceOverrideFailed, got {other:?}"),
    }
}
