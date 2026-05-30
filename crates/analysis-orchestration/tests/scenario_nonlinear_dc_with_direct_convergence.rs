//! Scenario-level integration witness for
//! `dc-operating-point#nonlinear-dc-operating-point-with-direct-convergence`.
//!
//! This file is the executable witness for the Gherkin scenario
//! inlined into kanban task `t_ec4d3f07`. It exercises the **public**
//! API of `analysis-orchestration` (and its transitive
//! `numeric-solver`, `device-modeling`, and `netlist-graph`
//! dependencies) end-to-end on a topology that contains a real
//! MOSFET, pinning the v1 surface per ADR-0010 and asserting the
//! Gherkin's observable promises:
//!
//! 1. The MOSFET dispatches through the closed-enum
//!    [`device_modeling::DeviceModel`] surface (ADR-0005).
//! 2. The Simulator returns a Result containing an `OperatingPoint`.
//! 3. Every node voltage matches the Golden Reference within the
//!    tolerance envelope (ADR-0008).
//! 4. The `Convergence` status is `"converged"`.
//! 5. The Newton-Raphson iteration count is reported in the Result.
//!
//! # Gherkin (verbatim, from
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md`)
//!
//! ```text
//! Given CircuitDesigner has constructed a Circuit from a netlist
//!       containing MOSFET devices
//! And the MOSFET devices use closed-enum DeviceModel dispatch
//! When CircuitDesigner submits a DC operating-point Analysis request
//! Then the Simulator returns a Result containing an OperatingPoint
//! And every node voltage matches the Golden Reference within the
//!     tolerance envelope
//! And the Convergence status is "converged"
//! And the Newton-Raphson iteration count is reported in the Result
//! ```
//!
//! # Position of this test in the implementation pipeline
//!
//! tasks.md slices the work for this scenario across the
//! `dc-operating-point` and shared-infrastructure capabilities; the
//! prerequisites have already merged to trunk:
//!
//! - **#2** — shared `NodeId` / `BranchId` / `ConvergenceStatus`
//!   types.
//! - **#3** — `FlattenedStructure` (Pass-1 incidence).
//! - **#5** — `CircuitGraph` builder.
//! - **#6** — Pass-1 structure flattening.
//! - **#7** — closed-enum `DeviceModel` (ADR-0005).
//! - **#8** — `LinearizedModel` Jacobian + companion-current dispatch.
//! - **#11** — MOSFET Level-1 (Shichman-Hodges) stamp.
//! - **#14** — Pass-2 MNA matrix assembly stamping linearized models.
//! - **#15** — sub-view extractor (ADR-0003).
//! - **#16** — `russell_sparse` real-valued LU dispatch.
//! - **#17** — `NewtonRaphsonDriver` with dual convergence criterion
//!   (ADR-0006).
//! - **#20** — DC analysis control loop returning
//!   [`OperatingPoint`][analysis_orchestration::OperatingPoint] +
//!   [`ConvergenceStatus`][circuit_solver_types::ConvergenceStatus].
//!
//! The nonlinear DC adapter
//! ([`analysis_orchestration::dc::NonlinearDcSystem`][_nonlinear]) and
//! the [`DeviceModelBinding`] surface that drives it ship in **this**
//! task slice — they are the composition glue between the closed-enum
//! `DeviceModel::linearize` callback and the existing Pass-2 / sub-view
//! / NR-driver / sparse-LU pipeline. The test below is the executable
//! demonstration that the composition lands the converged operating
//! point inside the ADR-0008 envelope on a real MOSFET circuit.
//!
//! [_nonlinear]: analysis_orchestration::dc
//!
//! # Choice of fixture
//!
//! The canonical nonlinear-DC textbook case is a **diode-connected
//! NMOS** with a series resistor:
//!
//! ```text
//!   V1 (5 V, n_vdd) → R1 (1 kΩ) → n_dg
//!                                  ┝──── (gate of M1)
//!                                  │
//!                                  └──── (drain of M1)
//!   M1: drain=n_dg, gate=n_dg, source=gnd, bulk=gnd
//! ```
//!
//! With the gate tied to the drain the device is forced into
//! saturation (`Vds ≥ Vgs - Vth ⇔ 0 ≥ -Vth`, true for an enhancement
//! NMOS). The Level-1 saturation current is
//!
//! ```text
//!   Id = (KP / 2) · (Vgs - Vth)² · (1 + λ · Vds)
//! ```
//!
//! and KVL gives `Vdd = Id · R + Vdg`. For the parameters chosen
//! below (`VTO = 0.7 V`, `KP = 200 µA/V²`, `λ = 0`, `Vdd = 5 V`,
//! `R = 1 kΩ`) the analytic golden reference solves
//!
//! ```text
//!   5 − 0.1 · (Vdg − 0.7)² = Vdg
//!     ⇒ Vdg ≈ 3.94621 V,   Id ≈ 1.05379 mA
//! ```
//!
//! (see [`golden_reference`] for the closed-form derivation).
//!
//! The diode-connected fixture is ideal for a *direct-convergence*
//! witness because the saturation regime is well-conditioned
//! (`Id` is convex in `Vgs`), the iterate starts at the all-zero
//! vector well inside the basin of attraction, and Newton-Raphson
//! converges in a single-digit number of iterations without any
//! homotopy aid. Scenarios that require Gmin- or source-stepping
//! homotopy live in their own sibling witness
//! (`dc-operating-point-with-gmin-stepping-homotopy`, tasks.md #18 /
//! #19).
//!
//! # Tolerance envelope (ADR-0008 row "DC")
//!
//! Per ADR-0008 *Per-Node max(Relative, Absolute) Tolerance Envelope*
//! the DC defaults are:
//!
//! | quantity          | relative | absolute |
//! |-------------------|---------:|---------:|
//! | DC node voltage   | 1 %      | 1 mV     |
//!
//! and the pass criterion is `|err| ≤ max(rel · |ref|, abs)`. Against
//! a closed-form analytic reference (no PDK / SPICE rounding) we
//! typically beat the absolute floor by many decades. Passing this
//! envelope is the load-bearing assertion for the Gherkin's
//! *"every node voltage matches the Golden Reference within the
//! tolerance envelope"* clause.

use analysis_orchestration::{
    dc_analysis, DcAnalysisRequest, DcAnalysisResult, DeviceModelBinding, OperatingPoint,
};
use circuit_solver_types::{ConvergenceStatus, ConvergenceTolerances, ModelName, NodeId};
use device_modeling::{DeviceModel, MOSFETParams, MosLevel1Params, MosPolarity};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{flatten, FlattenedStructure, NewtonRaphsonConfig};

// --- Constants from ADR-0008 (DC defaults) ----------------------------------

/// Relative tolerance for DC node voltages, per ADR-0008.
const DC_V_REL: f64 = 0.01;
/// Absolute floor for DC node voltages, per ADR-0008.
const DC_V_ABS: f64 = 1e-3;

/// True iff `actual` is within the
/// `max(relative · |reference|, absolute)` band of `reference`. This
/// is the ADR-0008 envelope operator applied to a scalar quantity.
fn within_envelope(actual: f64, reference: f64, rel: f64, abs: f64) -> bool {
    let bound = (rel * reference.abs()).max(abs);
    (actual - reference).abs() <= bound
}

/// Assert a single (actual, reference) voltage pair lies within the
/// ADR-0008 DC voltage envelope, producing a readable failure
/// message.
fn assert_voltage_within_envelope(label: &str, actual: f64, reference: f64) {
    assert!(
        within_envelope(actual, reference, DC_V_REL, DC_V_ABS),
        "DC voltage at {label} = {actual} V violates the ADR-0008 envelope around \
         reference {reference} V (rel={DC_V_REL}, abs={DC_V_ABS} V)"
    );
}

// --- Fixture parameters ------------------------------------------------------

/// Supply voltage driving the diode-connected NMOS topology.
const VDD_VOLTS: f64 = 5.0;
/// Series resistor between V1 and the drain/gate node.
const R_SERIES_OHMS: f64 = 1_000.0;
/// Threshold voltage `VTO` of the Level-1 NMOS (volts).
const NMOS_VTO_V: f64 = 0.7;
/// Transconductance parameter `KP = µ · Cox · (W/L)` of the Level-1
/// NMOS, in A/V².
const NMOS_KP: f64 = 200e-6;
/// Channel-length modulation `λ` of the Level-1 NMOS, in 1/V. Zero
/// for the simplest closed-form golden reference.
const NMOS_LAMBDA: f64 = 0.0;
/// Body-effect coefficient `γ` (V^½). Zero because the source and
/// bulk are tied to the same node (ground) in this fixture.
const NMOS_GAMMA: f64 = 0.0;
/// Surface potential `φ` of the Level-1 NMOS (volts). Used by the
/// body-effect formula even when `γ = 0` so we keep the canonical
/// SPICE value.
const NMOS_PHI: f64 = 0.6;
/// Model name shared by the [`netlist_graph::Element`] and the
/// [`DeviceModelBinding`].
const NMOS_MODEL_NAME: &str = "nmos_lvt";

// --- Golden reference -------------------------------------------------------

/// Solve the diode-connected-NMOS KVL `5 − 0.1·(Vdg − 0.7)² = Vdg`
/// in closed form and return the converged (`Vdg`, `Id`) pair.
///
/// Algebra: with `u = Vdg − Vth`, `Vdg = u + Vth` and KVL becomes
///
/// ```text
///   Vdd − (KP / 2) · u² · R = u + Vth
///       ⇒ (KP·R/2) · u² + u − (Vdd − Vth) = 0
/// ```
///
/// which is a positive-discriminant quadratic in `u`. Selecting the
/// positive root (the physical one — `u < 0` would put the device
/// below threshold) yields
///
/// ```text
///   u = (−1 + √(1 + 2 · KP · R · (Vdd − Vth))) / (KP · R)
/// ```
///
/// and `Vdg = Vth + u`, `Id = (KP/2)·u²`. We return both so the
/// caller can assert against each independently.
fn golden_reference() -> (f64, f64) {
    let kp = NMOS_KP;
    let r = R_SERIES_OHMS;
    let vth = NMOS_VTO_V;
    let vdd = VDD_VOLTS;
    // Discriminant of the quadratic `(KP·R/2) u² + u − (Vdd − Vth) = 0`,
    // pre-multiplied through to drop the `/2`. Equivalent form:
    // `u² + (2/(KP·R)) u − (2·(Vdd − Vth) / (KP·R)) = 0`.
    let disc = 1.0 + 2.0 * kp * r * (vdd - vth);
    let u = (-1.0 + disc.sqrt()) / (kp * r);
    let v_dg = vth + u;
    let id = 0.5 * kp * u * u;
    (v_dg, id)
}

// --- Circuit construction --------------------------------------------------

/// Build the diode-connected NMOS test fixture and return both the
/// flattened structure (for solver inputs) and the graph (for node-
/// lookup convenience).
fn diode_connected_nmos() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();

    // Declare the model name so the builder accepts the
    // `Some(ModelName)` reference on the element.
    let nmos_model_name = ModelName::new(NMOS_MODEL_NAME);
    b.add_model(nmos_model_name.clone());

    // V1: 5 V from n_vdd to gnd.
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: VDD_VOLTS,
        },
        ["n_vdd", "0"],
        None,
    )
    .expect("add V1");

    // R1: 1 kΩ from n_vdd to n_dg.
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: R_SERIES_OHMS,
        },
        ["n_vdd", "n_dg"],
        None,
    )
    .expect("add R1");

    // M1: diode-connected NMOS Level-1. Terminal order is
    // [drain, gate, source, bulk] per the device-modeling crate's
    // `MOSFET_TERMINALS` convention. Drain and gate both attach to
    // n_dg (the "diode-connected" topology); source and bulk are
    // tied to ground.
    b.add_element(
        "M1",
        ElementKind::Semiconductor,
        ["n_dg", "n_dg", "0", "0"],
        Some(nmos_model_name),
    )
    .expect("add M1");

    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    (fs, g)
}

/// Build the `DeviceModelBinding` slice the orchestrator passes to
/// [`dc_analysis`] so the [`NonlinearDcSystem`][adapter] adapter
/// resolves `M1.model() == "nmos_lvt"` to a real
/// [`DeviceModel::MOSFET`] payload.
///
/// [adapter]: analysis_orchestration::dc
fn nmos_bindings() -> Vec<DeviceModelBinding> {
    let nmos_params = MosLevel1Params {
        name: ModelName::new(NMOS_MODEL_NAME),
        polarity: MosPolarity::Nmos,
        vto: NMOS_VTO_V,
        kp: NMOS_KP,
        lambda: NMOS_LAMBDA,
        gamma: NMOS_GAMMA,
        phi: NMOS_PHI,
        kf: 0.0,
        af: 1.0,
    };
    vec![DeviceModelBinding::new(
        ModelName::new(NMOS_MODEL_NAME),
        DeviceModel::MOSFET(MOSFETParams::Level1(nmos_params)),
    )]
}

/// Look up a node id by name. Falls over loudly if the netlist
/// builder failed to register the net (catches typos in the fixture).
fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
    g.nodes()
        .iter()
        .find(|n| n.name() == name)
        .unwrap_or_else(|| panic!("node {name} present"))
        .id()
}

/// Newton-Raphson tuning suited to a milliamp-scale fixture.
///
/// The default SPICE tolerances (`reltol = 1e-3`, `abstol = 1e-12`)
/// pair an absolute-amperes residue floor with an update-norm
/// tolerance. For a 1 mA circuit that absolute floor lands within
/// roughly three orders of magnitude of stamp-level round-off
/// accumulation, which is too tight for a guaranteed
/// direct-convergence witness without a relative-residue term.
/// (ngspice's real check is per-node KCL with an effective
/// `reltol·|i_max| + abstol` band that runs at ~1 µA for this
/// fixture; the v1 single-global-norm path collapses that into a
/// pure absolute tolerance.)
///
/// We loosen `residue_tol` to `1e-9` (one nA) — still nine orders
/// of magnitude below the device's drain current and well within
/// the per-node max(relative, absolute) envelope ADR-0008 reports
/// against the golden reference. The update-norm tolerance stays
/// at the SPICE default `1e-3 V`, which is the stall-mode guard
/// ADR-0006 actually exists to defend.
fn nonlinear_dc_nr_config() -> NewtonRaphsonConfig {
    NewtonRaphsonConfig {
        max_iterations: 100,
        tolerances: ConvergenceTolerances::new(1e-3, 1e-9),
    }
}

// --- Tests ------------------------------------------------------------------

/// **Headline scenario witness.**
///
/// > Given CircuitDesigner has constructed a Circuit from a netlist
/// > containing MOSFET devices
/// > And the MOSFET devices use closed-enum DeviceModel dispatch
/// > When CircuitDesigner submits a DC operating-point Analysis
/// > request
/// > Then the Simulator returns a Result containing an OperatingPoint
/// > And every node voltage matches the Golden Reference within the
/// > tolerance envelope
/// > And the Convergence status is "converged"
/// > And the Newton-Raphson iteration count is reported in the Result
#[test]
fn nonlinear_dc_operating_point_with_direct_convergence_diode_connected_nmos() {
    // Given: a Circuit with a MOSFET device.
    let (fs, g) = diode_connected_nmos();

    // And: the MOSFET dispatches through the closed-enum DeviceModel
    // surface (ADR-0005). The `DeviceModelBinding` slice carries a
    // `DeviceModel::MOSFET(MOSFETParams::Level1(...))` payload; the
    // adapter resolves `M1.model() → DeviceModel` by `ModelName`
    // match, and every Newton iterate routes through the exhaustive
    // `match` in `DeviceModel::linearize` (see ADR-0005's
    // exhaustiveness guarantee). The witness that this dispatch
    // happens is the simple fact that we got *any* nonzero current
    // out the other side — a non-MOSFET stamp (or a placeholder
    // zero linearization) would leave the device disconnected and
    // the node would float at Vdd.
    let bindings = nmos_bindings();

    // When: CircuitDesigner submits a DC operating-point Analysis
    // request.
    let result: DcAnalysisResult = dc_analysis(
        DcAnalysisRequest::new(&g, &fs)
            .with_device_models(&bindings)
            .with_newton_raphson(nonlinear_dc_nr_config()),
    )
    .expect("dc analysis ok");

    // Then: the Result carries an OperatingPoint.
    let op: &OperatingPoint = result
        .operating_point
        .as_ref()
        .expect("OperatingPoint present");

    // And: the Convergence status is "converged".
    assert!(
        result.is_converged(),
        "expected Converged, got {:?}",
        result.convergence
    );
    assert!(
        matches!(result.convergence, ConvergenceStatus::Converged(_)),
        "convergence variant should be Converged, got {:?}",
        result.convergence
    );

    // And: the Newton-Raphson iteration count is reported in the
    // Result. ADR-0006 pins the iteration count on the
    // ConvergenceDiagnostic; we read it and assert it is finite (a
    // direct-convergence witness expects a single-digit count, but
    // we deliberately do not pin a tighter upper bound here because
    // future changes to the NR driver's tuning could shift it
    // within a small range without violating the Gherkin clause).
    let diag = result.convergence.diagnostic();
    assert!(
        diag.iterations >= 1,
        "expected at least one NR iteration, got {}",
        diag.iterations
    );
    assert!(
        diag.iterations < 100,
        "direct-convergence path should not approach the ITL1 ceiling; got {}",
        diag.iterations
    );

    // And: every node voltage matches the Golden Reference within
    // the ADR-0008 tolerance envelope.
    let (v_dg_ref, _id_ref) = golden_reference();
    assert_voltage_within_envelope(
        "n_vdd",
        op.voltage_at(node_id(&g, "n_vdd")).unwrap(),
        VDD_VOLTS,
    );
    assert_voltage_within_envelope(
        "n_dg",
        op.voltage_at(node_id(&g, "n_dg")).unwrap(),
        v_dg_ref,
    );
    assert_voltage_within_envelope("gnd", op.voltage_at(NodeId::GROUND).unwrap(), 0.0);
}

/// Companion test: pinning the *immutability-once-produced*
/// acceptance criterion on the nonlinear path. The criterion is the
/// same for linear and nonlinear DC (the spec's
/// `acceptance criterion` block is capability-wide), but the linear
/// witness already covers the structural side (no setters, no
/// mutating methods); this test pins the *behavioural* side under
/// the nonlinear adapter so a future refactor that accidentally
/// rebuilds the operating point in-place on a clone would fail
/// here.
#[test]
fn nonlinear_operating_point_clone_preserves_equality() {
    let (fs, g) = diode_connected_nmos();
    let bindings = nmos_bindings();
    let result = dc_analysis(
        DcAnalysisRequest::new(&g, &fs)
            .with_device_models(&bindings)
            .with_newton_raphson(nonlinear_dc_nr_config()),
    )
    .expect("dc analysis ok");
    let op = result.operating_point.expect("op present");
    let cloned = op.clone();
    assert_eq!(op, cloned);
}

/// Negative-path companion: when the orchestrator forgets to attach
/// a `DeviceModelBinding` for a semiconductor element, the assembler
/// must surface the omission via
/// [`MnaAssemblyError::MissingLinearizationForDevice`] — *not* by
/// silently treating the device as a placeholder zero stamp. This
/// pin is important because the same `Semiconductor` graph element
/// can be served by either the linear path (which errors) or the
/// nonlinear path (which errors after the `ModelName` lookup
/// misses); both surfaces must agree.
#[test]
fn nonlinear_dc_with_no_bindings_surfaces_missing_linearization() {
    let (fs, g) = diode_connected_nmos();
    // Empty bindings slice: the `Semiconductor` element exists but
    // its `ModelName` doesn't resolve to a `DeviceModel`.
    let empty_bindings: Vec<DeviceModelBinding> = Vec::new();
    let err = dc_analysis(
        DcAnalysisRequest::new(&g, &fs)
            .with_device_models(&empty_bindings)
            .with_newton_raphson(nonlinear_dc_nr_config()),
    )
    .expect_err("expected missing-linearization error");
    let msg = format!("{err}");
    let msg_lower = msg.to_lowercase();
    assert!(
        msg_lower.contains("device")
            || msg_lower.contains("linearization")
            || msg_lower.contains("linearizedmodel")
            || msg_lower.contains("semiconductor"),
        "expected diagnostic mentioning the missing-linearization condition, got: {msg}"
    );
}
