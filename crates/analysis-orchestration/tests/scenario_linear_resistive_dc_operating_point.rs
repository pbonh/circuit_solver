//! Scenario-level integration test for
//! `dc-operating-point#linear-resistive-dc-operating-point`.
//!
//! This file is the executable witness for the Gherkin scenario
//! inlined into kanban task `t_e862d996`. It exercises the **public**
//! API of `analysis-orchestration` (and its transitive
//! `numeric-solver` and `netlist-graph` dependencies) end-to-end on a
//! canonical linear resistive topology, pinning the v1 surface per
//! ADR-0010 and asserting the spec's observable promises:
//!
//! 1. *The Simulator returns a Result containing an `OperatingPoint`.*
//! 2. *Every node voltage and branch current in the `OperatingPoint`
//!    matches the Golden Reference within the tolerance envelope.*
//! 3. *The Convergence status is "converged".*
//!
//! Sibling unit tests inside `crates/analysis-orchestration/src/dc.rs`
//! already cover the broader API contracts (error surface, builder
//! overrides, topology pre-pass, edge cases). This integration test
//! is intentionally narrower and load-bearing for **this** scenario
//! only: it consumes solely the public crate exports, so a future
//! refactor that breaks the v1 surface fails here loudly.
//!
//! # Gherkin (verbatim, from
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md`)
//!
//! ```text
//! Given CircuitDesigner has constructed a Circuit from a linear
//!       resistive netlist
//! And the Circuit contains no nonlinear devices
//! When CircuitDesigner submits a DC operating-point Analysis request
//! Then the Simulator returns a Result containing an OperatingPoint
//! And every node voltage and branch current in the OperatingPoint
//!     matches the Golden Reference within the tolerance envelope
//! And the Convergence status is "converged"
//! ```
//!
//! # Golden-reference choice
//!
//! Per the inlined glossary, *"Golden Reference — a trusted external
//! simulator against which results are compared."* The cross-cutting
//! ngspice conformance harness lives at tasks.md item #62 and is
//! gated on the full v1 stack; that harness is tracked on its own
//! kanban thread. For a **linear resistive** circuit, the analytic
//! DC operating point is closed-form and *exactly* the same reference
//! that ngspice (or any other industrial simulator) would converge to
//! — there is no semiconductor model, no nonlinear iteration, no PDK
//! parameterisation drift. We therefore use the analytic solution as
//! the golden reference for this scenario witness. This mirrors the
//! pattern established by sibling `analysis-orchestration` integration
//! tests (`tests/scenario_ac_purely_linear_circuit.rs`).
//!
//! # Tolerance envelope (ADR-0008 row "DC")
//!
//! Per ADR-0008 *Per-Node max(Relative, Absolute) Tolerance Envelope*
//! the DC defaults are:
//!
//! | quantity          | relative | absolute |
//! |-------------------|---------:|---------:|
//! | DC node voltage   | 1 %      | 1 mV     |
//! | DC branch current | 1 %      | 1 mV (read as 1 µA for currents) |
//!
//! and the pass criterion is `|err| ≤ max(rel · |ref|, abs)`. Against
//! an analytic reference (no ngspice rounding) we typically beat the
//! absolute floor by many decades, so passing this envelope is a
//! strong correctness signal.

use analysis_orchestration::{dc_analysis, DcAnalysisRequest, DcAnalysisResult, OperatingPoint};
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
/// millivolt scale).
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

// --- Test circuits ----------------------------------------------------------

/// Two-resistor voltage divider:
///
/// ```text
///   V1 (10 V, n_in) → R1 (1 kΩ) → n_mid → R2 (1 kΩ) → gnd
/// ```
///
/// Analytic golden reference:
///
/// - `V(n_in) = 10 V`
/// - `V(n_mid) = 10 · 1k / (1k + 1k) = 5 V`
/// - `V(gnd) = 0`
/// - `i_V1 = ±10 V / 2 kΩ = ±5 mA` (sign per assembler stamping
///   convention; magnitude is the testable invariant).
fn voltage_divider() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_in", "0", 10.0);
    add_resistor(&mut b, "R1", "n_in", "n_mid", 1_000.0);
    add_resistor(&mut b, "R2", "n_mid", "0", 1_000.0);
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    (fs, g)
}

/// Wheatstone bridge with no load between the two midpoints —
/// a purely linear, four-resistor network with two interior nodes.
///
/// ```text
///                 ┌──── R1 (1 kΩ) ──── n_a ──── R3 (3 kΩ) ────┐
///                 │                                            │
///   V1 (5 V) → n_top                                        n_bot → gnd
///                 │                                            │
///                 └──── R2 (2 kΩ) ──── n_b ──── R4 (4 kΩ) ────┘
/// ```
///
/// With both legs grounded at the bottom and the top tied to a 5 V
/// source, the analytic node voltages are:
///
/// - `V(n_top) = 5 V`
/// - `V(n_a) = 5 · R3 / (R1 + R3) = 5 · 3/(1+3) = 3.75 V`
/// - `V(n_b) = 5 · R4 / (R2 + R4) = 5 · 4/(2+4) ≈ 3.333… V`
/// - `V(gnd) = 0`
fn wheatstone_bridge() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_top", "0", 5.0);
    add_resistor(&mut b, "R1", "n_top", "n_a", 1_000.0);
    add_resistor(&mut b, "R2", "n_top", "n_b", 2_000.0);
    add_resistor(&mut b, "R3", "n_a", "0", 3_000.0);
    add_resistor(&mut b, "R4", "n_b", "0", 4_000.0);
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    (fs, g)
}

// --- Scenario assertions ----------------------------------------------------

/// Assert a single (actual, reference) pair lies within the ADR-0008
/// DC voltage envelope, producing a readable failure message.
fn assert_voltage_within_envelope(label: &str, actual: f64, reference: f64) {
    assert!(
        within_envelope(actual, reference, DC_V_REL, DC_V_ABS),
        "DC voltage at {label} = {actual} V violates the ADR-0008 envelope around \
         reference {reference} V (rel={DC_V_REL}, abs={DC_V_ABS} V)"
    );
}

/// Assert a single (actual, reference) current-magnitude pair lies
/// within the ADR-0008 DC branch-current envelope. The magnitude
/// comparison sidesteps the per-MNA-convention sign question.
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
/// > Given CircuitDesigner has constructed a Circuit from a linear
/// > resistive netlist
/// > And the Circuit contains no nonlinear devices
/// > When CircuitDesigner submits a DC operating-point Analysis request
/// > Then the Simulator returns a Result containing an OperatingPoint
/// > And every node voltage and branch current in the OperatingPoint
/// > matches the Golden Reference within the tolerance envelope
/// > And the Convergence status is "converged"
#[test]
fn linear_resistive_dc_operating_point_voltage_divider() {
    let (fs, g) = voltage_divider();

    // "When CircuitDesigner submits a DC operating-point Analysis
    // request"
    let result: DcAnalysisResult =
        dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");

    // "Then the Simulator returns a Result containing an
    // OperatingPoint"
    let op: &OperatingPoint = result
        .operating_point
        .as_ref()
        .expect("OperatingPoint present");

    // "And the Convergence status is converged"
    assert!(
        result.is_converged(),
        "expected Converged, got {:?}",
        result.convergence
    );

    // "And every node voltage and branch current matches the Golden
    // Reference within the tolerance envelope"
    assert_voltage_within_envelope("n_in", op.voltage_at(node_id(&g, "n_in")).unwrap(), 10.0);
    assert_voltage_within_envelope("n_mid", op.voltage_at(node_id(&g, "n_mid")).unwrap(), 5.0);
    assert_voltage_within_envelope("gnd", op.voltage_at(NodeId::GROUND).unwrap(), 0.0);

    // Branch current magnitude: |i_V1| = 5 mA.
    assert_eq!(
        op.branch_currents.len(),
        1,
        "the only branch unknown is the voltage source's MNA branch"
    );
    assert_current_magnitude_within_envelope("i_V1", op.branch_currents[0].current_amperes, 5e-3);
}

/// Same scenario, exercised against a four-resistor Wheatstone bridge
/// (no load) to verify that the analysis composes correctly when the
/// system has more than one interior node and an additive stamp
/// pattern at each node.
#[test]
fn linear_resistive_dc_operating_point_wheatstone_bridge() {
    let (fs, g) = wheatstone_bridge();

    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");
    assert!(result.is_converged());
    let op = result.operating_point.expect("op present");

    assert_voltage_within_envelope("n_top", op.voltage_at(node_id(&g, "n_top")).unwrap(), 5.0);
    assert_voltage_within_envelope("n_a", op.voltage_at(node_id(&g, "n_a")).unwrap(), 3.75);
    assert_voltage_within_envelope(
        "n_b",
        op.voltage_at(node_id(&g, "n_b")).unwrap(),
        5.0 * 4.0 / 6.0,
    );
    assert_voltage_within_envelope("gnd", op.voltage_at(NodeId::GROUND).unwrap(), 0.0);

    // |i_V1| = V_source / R_eq where R_eq is the parallel combination
    // of the two legs: (R1+R3) || (R2+R4) = 4k || 6k = 2.4 kΩ.
    // Therefore |i_V1| = 5 V / 2.4 kΩ ≈ 2.0833 mA.
    let r_eq = 1.0 / (1.0 / 4_000.0 + 1.0 / 6_000.0);
    let i_ref = 5.0 / r_eq;
    assert_eq!(op.branch_currents.len(), 1);
    assert_current_magnitude_within_envelope("i_V1", op.branch_currents[0].current_amperes, i_ref);
}

/// The spec's *"`OperatingPoint` result is immutable once produced"*
/// acceptance criterion is encoded in the type system: the public
/// `OperatingPoint` has only `pub` data fields and no mutating
/// methods, and the analysis returns it by value (so the caller can
/// either move it or clone it; there is no shared-mutation pathway).
/// This test pins the *behavioural* counterpart: cloning the result
/// preserves equality with the original.
#[test]
fn operating_point_clone_preserves_equality() {
    let (fs, g) = voltage_divider();
    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");
    let op = result.operating_point.expect("op present");
    let cloned = op.clone();
    assert_eq!(op, cloned);
}
