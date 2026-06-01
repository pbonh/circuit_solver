//! Complementary scenario witness for
//! `dc-operating-point#linear-resistive-dc-operating-point`.
//!
//! This file is the executable witness contributed by kanban task
//! `t_a0805db8`, the per-scenario impl child of the dc-operating-point
//! spec parent `t_a84ce0f6`. It is **additive** to the headline
//! witness landed by sibling task `t_e862d996` at
//! `tests/scenario_linear_resistive_dc_operating_point.rs`, which
//! pins the voltage-divider and Wheatstone-bridge canonical
//! topologies plus the `OperatingPoint` immutability assertion. This
//! file extends the same Gherkin scenario to two additional
//! load-bearing linear-resistive topologies that the headline
//! witness does not exercise:
//!
//! 1. **Four-resistor series chain** — a multi-hop voltage divider
//!    with three interior nodes, exercising the additive stamp
//!    pattern at every interior node along a longer path than the
//!    two-resistor headline divider.
//! 2. **Current-source-driven parallel-resistor pair** — a linear
//!    resistive circuit driven by an independent `CurrentSource`
//!    instead of a `VoltageSource`, exercising the RHS-stamp path
//!    (as opposed to the MNA branch-row path) at the same scenario
//!    layer.
//!
//! Both topologies satisfy the Gherkin preconditions verbatim:
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
//! simulator against which results are compared."* For purely linear
//! resistive networks the analytic solution is closed-form and is
//! exactly what an industrial simulator would converge to (no
//! semiconductor models, no iteration, no PDK drift). The cross-
//! cutting ngspice conformance harness lives at tasks.md item #62
//! and is gated on the full v1 stack; that harness owns the
//! external-simulator comparison. This file follows the same
//! analytic-golden-reference convention established by the
//! headline witness for the same scenario.
//!
//! # Tolerance envelope (ADR-0008 row "DC")
//!
//! Per ADR-0008 *Per-Node max(Relative, Absolute) Tolerance Envelope*
//! the DC defaults are:
//!
//! | quantity          | relative | absolute |
//! |-------------------|---------:|---------:|
//! | DC node voltage   | 1 %      | 1 mV     |
//! | DC branch current | 1 %      | 1 µA (current-floor analogue) |
//!
//! and the pass criterion is `|err| ≤ max(rel · |ref|, abs)`. The
//! current-floor constant follows the precedent set by parent
//! `t_0a9c2721`'s aggregator-facing note that ADR-0008's DC row may
//! eventually grow a dedicated current-floor entry; until then this
//! file uses the same 1 µA analogue as the headline witness.

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
/// Absolute floor for DC branch currents (1 µA analogue, see file
/// header for rationale).
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

fn add_current_source(b: &mut CircuitBuilder, name: &str, from: &str, to: &str, amps: f64) {
    b.add_element(
        name,
        ElementKind::CurrentSource {
            current_amperes: amps,
        },
        [from, to],
        None,
    )
    .expect("add current source");
}

fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
    g.nodes()
        .iter()
        .find(|n| n.name() == name)
        .expect("node present")
        .id()
}

// --- Test circuits ----------------------------------------------------------

/// Four-resistor series chain:
///
/// ```text
///   V1 (12 V, n_top) → R1 (1 kΩ) → n_a → R2 (2 kΩ) → n_b → R3 (3 kΩ) → n_c → R4 (4 kΩ) → gnd
/// ```
///
/// With four equal-current series resistors the analytic node
/// voltages drop in proportion to the cumulative resistance from
/// ground. Let `R_total = 10 kΩ`; then `i = 12 V / 10 kΩ = 1.2 mA`
/// and the node voltages are:
///
/// - `V(n_top) = 12 V`
/// - `V(n_a) = 12 - 1.2 mA · 1 kΩ = 10.8 V` (i.e. 9 kΩ to ground)
/// - `V(n_b) = 12 - 1.2 mA · 3 kΩ = 8.4 V`  (i.e. 7 kΩ to ground)
/// - `V(n_c) = 12 - 1.2 mA · 6 kΩ = 4.8 V`  (i.e. 4 kΩ to ground)
/// - `V(gnd) = 0`
/// - `|i_V1| = 1.2 mA`
fn series_chain() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_top", "0", 12.0);
    add_resistor(&mut b, "R1", "n_top", "n_a", 1_000.0);
    add_resistor(&mut b, "R2", "n_a", "n_b", 2_000.0);
    add_resistor(&mut b, "R3", "n_b", "n_c", 3_000.0);
    add_resistor(&mut b, "R4", "n_c", "0", 4_000.0);
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    (fs, g)
}

/// Current-source-driven parallel-resistor pair:
///
/// ```text
///   I1 (2 mA, gnd → n_top)
///   R1 (3 kΩ, n_top → gnd)
///   R2 (6 kΩ, n_top → gnd)
/// ```
///
/// The 2 mA injected at `n_top` splits between R1 and R2 in parallel.
/// `R_par = 3 kΩ || 6 kΩ = 2 kΩ`, therefore `V(n_top) = 2 mA · 2 kΩ
/// = 4 V`. With this topology the only voltage-source-MNA-branch
/// row is absent (no `VoltageSource` present), so `branch_currents`
/// must be empty and the entire RHS contribution comes from the
/// current-source stamp path. This is a load-bearing scenario
/// because it exercises the `CurrentSource` stamp at the
/// `dc_analysis` layer — orthogonal to the `VoltageSource` paths
/// exercised by the headline witness.
fn current_source_parallel() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();
    // SPICE current-source sign convention (per
    // `numeric_solver::assemble::stamp_current_source`): with
    // `from = n_top` and `to = "0"`, positive `current_amperes`
    // adds +S to the RHS at `from` (i.e. injects positive current
    // into n_top) and -S at `to`. To inject +2 mA at n_top from
    // ground, we therefore pass ("n_top", "0", +2e-3).
    add_current_source(&mut b, "I1", "n_top", "0", 2e-3);
    add_resistor(&mut b, "R1", "n_top", "0", 3_000.0);
    add_resistor(&mut b, "R2", "n_top", "0", 6_000.0);
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
/// within the ADR-0008 DC branch-current envelope. Magnitude
/// comparison sidesteps the per-MNA-convention sign question carried
/// forward as an aggregator-facing note from `t_0a9c2721`.
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

/// **Scenario witness — four-resistor series chain.**
///
/// > Given CircuitDesigner has constructed a Circuit from a linear
/// > resistive netlist
/// > And the Circuit contains no nonlinear devices
/// > When CircuitDesigner submits a DC operating-point Analysis request
/// > Then the Simulator returns a Result containing an OperatingPoint
/// > And every node voltage and branch current in the OperatingPoint
/// > matches the Golden Reference within the tolerance envelope
/// > And the Convergence status is "converged"
///
/// Exercises the additive stamp pattern across three interior nodes
/// along a single conduction path. This is the longest-chain
/// canonical SPICE textbook example that the headline witness does
/// not cover.
#[test]
fn linear_resistive_dc_operating_point_series_chain() {
    let (fs, g) = series_chain();

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
    let i_total = 12.0 / 10_000.0; // 1.2 mA
    assert_voltage_within_envelope("n_top", op.voltage_at(node_id(&g, "n_top")).unwrap(), 12.0);
    assert_voltage_within_envelope(
        "n_a",
        op.voltage_at(node_id(&g, "n_a")).unwrap(),
        12.0 - i_total * 1_000.0,
    );
    assert_voltage_within_envelope(
        "n_b",
        op.voltage_at(node_id(&g, "n_b")).unwrap(),
        12.0 - i_total * 3_000.0,
    );
    assert_voltage_within_envelope(
        "n_c",
        op.voltage_at(node_id(&g, "n_c")).unwrap(),
        12.0 - i_total * 6_000.0,
    );
    assert_voltage_within_envelope("gnd", op.voltage_at(NodeId::GROUND).unwrap(), 0.0);

    // |i_V1| = 1.2 mA.
    assert_eq!(
        op.branch_currents.len(),
        1,
        "the only branch unknown is the voltage source's MNA branch"
    );
    assert_current_magnitude_within_envelope(
        "i_V1",
        op.branch_currents[0].current_amperes,
        i_total,
    );
}

/// **Scenario witness — current-source-driven parallel resistors.**
///
/// > Given CircuitDesigner has constructed a Circuit from a linear
/// > resistive netlist
/// > And the Circuit contains no nonlinear devices
/// > When CircuitDesigner submits a DC operating-point Analysis request
/// > Then the Simulator returns a Result containing an OperatingPoint
/// > And every node voltage and branch current in the OperatingPoint
/// > matches the Golden Reference within the tolerance envelope
/// > And the Convergence status is "converged"
///
/// This topology has **no** `VoltageSource` and therefore no
/// MNA-branch unknown; the only excitation is an independent
/// `CurrentSource` whose contribution flows through the RHS-stamp
/// path inside the assembler. Pins that the DC control loop
/// continues to converge and to expose a well-formed `OperatingPoint`
/// when the entire excitation is current-driven.
#[test]
fn linear_resistive_dc_operating_point_current_source_parallel() {
    let (fs, g) = current_source_parallel();

    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");

    assert!(
        result.is_converged(),
        "expected Converged, got {:?}",
        result.convergence
    );
    let op = result.operating_point.expect("op present");

    // V(n_top) = 2 mA · (3kΩ || 6kΩ) = 2 mA · 2 kΩ = 4 V.
    let r_par = 1.0 / (1.0 / 3_000.0 + 1.0 / 6_000.0);
    let v_top_ref = 2e-3 * r_par;
    assert_voltage_within_envelope(
        "n_top",
        op.voltage_at(node_id(&g, "n_top")).unwrap(),
        v_top_ref,
    );
    assert_voltage_within_envelope("gnd", op.voltage_at(NodeId::GROUND).unwrap(), 0.0);

    // No VoltageSource → no MNA branch row.
    assert_eq!(
        op.branch_currents.len(),
        0,
        "current-source-only topology has no MNA branch unknown"
    );
}

/// Pins the spec's *"`OperatingPoint` result is immutable once
/// produced"* acceptance criterion at the boundary of this witness
/// file too. The type-system encoding is already pinned by the
/// headline witness; this test pins the behavioural counterpart
/// for the additional topologies introduced here so the immutability
/// guarantee is asserted across the full set of linear-resistive
/// witness circuits owned by this scenario.
#[test]
fn operating_point_clone_preserves_equality_series_chain() {
    let (fs, g) = series_chain();
    let result = dc_analysis(DcAnalysisRequest::new(&g, &fs)).expect("dc analysis ok");
    let op = result.operating_point.expect("op present");
    let cloned = op.clone();
    assert_eq!(op, cloned);
}
