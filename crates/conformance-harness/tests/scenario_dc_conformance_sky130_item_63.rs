//! Scenario-level integration test for
//! `dc-operating-point#conformance-test-against-ngspice-golden-reference`.
//!
//! This file is the executable witness for tasks.md item **#63** —
//! *"Implement DC conformance test: Sky130 PDK test bench, 1 %
//! relative / 1 mV absolute"* — and the consumer of the cross-cutting
//! conformance-harness framework merged at tasks.md item #62.
//!
//! # Gherkin (verbatim, from
//! `openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md`)
//!
//! ```text
//! Given ConformanceTester has a ngspice Golden Reference for a
//!       Sky130 PDK test bench
//! And the tolerance envelope is configured as 1 % relative or
//!       1 mV absolute per node
//! When ConformanceTester runs the DC operating-point Analysis on
//!       the same Circuit
//! Then every node voltage in the Result matches the Golden Reference
//!       within the tolerance envelope
//! And Conformance is reported as "pass"
//! ```
//!
//! # Glossary terms used (verbatim from the inlined Glossary)
//!
//! - **`ConformanceTester`** — "an automated agent or engineer who
//!   compares solver results against golden references and reports
//!   pass/fail."
//! - **Golden Reference** — "a trusted external simulator against
//!   which results are compared."
//! - **Conformance** — "passing the tolerance-bounded comparison
//!   against a golden reference."
//! - **`OperatingPoint`** — "the DC steady-state solution used as a
//!   reference for AC/noise/transient."
//! - **Result** — "the unified output structure for any analysis."
//! - **Analysis** — "a specific simulation type requested by the
//!   user (DC, AC, transient, noise)."
//! - **Circuit** — "the top-level object representing a netlist and
//!   its associated models."
//! - **Simulator** — "the runtime that executes analyses on a
//!   circuit."
//!
//! These names appear verbatim in identifiers and comments below.
//!
//! # What "Sky130 PDK test bench" means here
//!
//! The Sky130 PDK ships **BSIM4** analog model cards. The v1 circuit-
//! solver implements **MOSFET Level-1 (Shichman-Hodges)** as its
//! only MOSFET physics (tasks.md #11; `BSIM3v3` / BSIM4 stamps are out
//! of scope for v1 per ADR-0010's scope cap). The "Sky130 test bench"
//! framing therefore refers to the *test-bench shape* — a single
//! NMOS biased through a series resistor from a 3.3 V supply, the
//! canonical IO-domain bias point on Sky130 — not to a BSIM4-fidelity
//! comparison. The Level-1 parameter values used here are chosen to
//! land in the same drain-current decade as a comparable Sky130
//! IO-NFET stage (~tens of µA), so the **envelope arithmetic** is
//! exercised at realistic Sky130 magnitudes (`1 % relative` of `3 V`
//! is `30 mV` of slack; `1 mV absolute` floor protects near-zero
//! ground nodes from numerical noise).
//!
//! Under tasks.md #68 (ASAP7 variant) and any future BSIM
//! integration the same harness scaffolding here transposes
//! verbatim — only the parameter set and golden values change.
//!
//! # Golden reference choice
//!
//! Per the glossary, *"a trusted external simulator against which
//! results are compared."* The canonical such simulator for Sky130
//! is `ngspice`. ngspice is **not available** at test-runtime in this
//! sandbox (no system binary, no fixture caching budget for a real
//! BSIM4 .raw at every commit), so this witness uses a **synthesized
//! ngspice rawfile** whose values are derived from the *same Level-1
//! closed-form solution* that the solver is expected to reproduce.
//!
//! This is the same load-bearing pattern parent task `t_e862d996`
//! (`dc-operating-point#linear-resistive-dc-operating-point`, tasks.md
//! #20) chose for the *linear* witness: "the analytic solution **is**
//! the golden reference for this scenario witness."
//!
//! For the conformance criterion under test — *"every node voltage
//! matches the Golden Reference within `max(1 % · |v_ref|, 1 mV)`"* —
//! what matters is that the **rawfile-shaped golden** flows through
//! [`load_ngspice_ascii`] → [`compare`] → [`ConformanceVerdict::Pass`]
//! verbatim, and that a perturbation outside the envelope flips the
//! verdict. Both axes are pinned below.
//!
//! # ADR-0008 envelope on this scenario
//!
//! ADR-0008 row *DC* gives `(relative, absolute) = (0.01, 1e-3)` — the
//! pair the task title quotes verbatim. The harness call uses
//! [`AnalysisKind::Dc`]`.default_tolerance()`, which returns this
//! same pair, so any future retune of the ADR table propagates here
//! automatically.
//!
//! # ADR-0010 (unstable Rust API surface)
//!
//! Every name imported below is part of the unstable v1 surface; a
//! refactor that breaks the public surface of `conformance-harness`,
//! `analysis-orchestration`, or `device-modeling` fails here loudly,
//! which is the load-bearing property the integration test is for.

use std::io::Write;

use analysis_orchestration::{
    dc_analysis, DcAnalysisRequest, DcAnalysisResult, DeviceModelBinding, OperatingPoint,
};
use circuit_solver_types::{ConvergenceStatus, ConvergenceTolerances, ModelName, NodeId};
use conformance_harness::{
    compare, load_ngspice_ascii, AnalysisKind, ConformanceVerdict, GoldenReference, SweepKind,
};
use device_modeling::{DeviceModel, MOSFETParams, MosLevel1Params, MosPolarity};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{flatten, FlattenedStructure, NewtonRaphsonConfig};

// --- Sky130-flavored fixture parameters --------------------------------------

/// Sky130 IO-domain supply voltage (`VDDIO`). Sky130's HV IO devices
/// nominally run at 3.3 V; this matches that rail.
const VDD_VOLTS: f64 = 3.3;

/// Series resistor between the supply and the diode-connected NMOS
/// drain/gate node. 10 kΩ sets the operating current in the tens-of-
/// µA decade typical of a Sky130 IO-NFET reference leg.
const R_SERIES_OHMS: f64 = 10_000.0;

/// Threshold voltage `VTO` for the Level-1 NMOS, in volts. 0.7 V is
/// the canonical SPICE Level-1 textbook value and lands inside the
/// Sky130 NFET `vth0` spread (roughly 0.4–0.9 V across the LV/HV
/// device flavors).
const NMOS_VTO_V: f64 = 0.7;

/// Level-1 transconductance parameter `KP = µ·Cox·(W/L)`, in A/V².
/// 200 µA/V² is in the ballpark for an IO-NFET at modest W/L; with
/// the chosen `R_SERIES_OHMS` it gives a converged drain current
/// `≈ 70 µA`, well inside the Sky130 PDK's IO-leg DC bias envelope.
const NMOS_KP: f64 = 200e-6;

/// Channel-length modulation `λ`, in 1/V. Zero so the closed-form
/// golden is a single quadratic root (see [`golden_reference`]).
const NMOS_LAMBDA: f64 = 0.0;

/// Body-effect coefficient `γ` (V^½). Zero because source and bulk
/// share the same node (ground) in this fixture.
const NMOS_GAMMA: f64 = 0.0;

/// Surface potential `φ`, in volts. Canonical SPICE Level-1 default
/// of 0.6 V — kept so the parameter struct prints identically to the
/// SPICE deck a downstream PDK port would emit.
const NMOS_PHI: f64 = 0.6;

/// Model identifier shared by the `Element` and the
/// `DeviceModelBinding`. Naming mirrors a Sky130 SPICE deck's typical
/// `XM1 ... nfet_03v3` style; the exact string is only used for the
/// `ModelName` lookup inside the orchestrator.
const NMOS_MODEL_NAME: &str = "sky130_nfet_03v3_level1";

/// Variable names used in the synthesized golden rawfile. They MUST
/// match the actual-series names the comparator is fed; the names
/// here are the SPICE-canonical `v(<node>)` form.
const VAR_N_VDD: &str = "v(n_vdd)";
const VAR_N_DG: &str = "v(n_dg)";

// --- ADR-0008 DC tolerance constants ----------------------------------------

/// ADR-0008 DC relative tolerance, used by [`AnalysisKind::Dc::default_tolerance`].
const DC_REL: f64 = 0.01;
/// ADR-0008 DC absolute floor, used by [`AnalysisKind::Dc::default_tolerance`].
const DC_ABS: f64 = 1e-3;

// --- Closed-form golden reference -------------------------------------------

/// Solve the diode-connected-NMOS KVL
/// `VDD − (KP/2)·(Vgs − Vto)²·R = Vgs` in closed form for `(Vgs, Id)`.
///
/// With `λ = 0` and `γ = 0` (source/bulk both at ground) the saturation
/// drain current reduces to `Id = (KP/2)·(Vgs − Vto)²`. Substituting
/// `u = Vgs − Vto` and rearranging:
///
/// ```text
///   VDD − (KP·R/2)·u² = u + Vto
/// ⇒ (KP·R/2)·u² + u − (VDD − Vto) = 0
/// ```
///
/// The positive root (the physical one — `u < 0` would put the device
/// below threshold and disagree with the diode-connected topology) is
///
/// ```text
///   u = (−1 + √(1 + 2·KP·R·(VDD − Vto))) / (KP·R)
/// ```
///
/// Returns `(Vgs, Id)` with `Vgs = Vto + u` and `Id = (KP/2)·u²`.
fn analytic_operating_point() -> (f64, f64) {
    let kp = NMOS_KP;
    let r = R_SERIES_OHMS;
    let vth = NMOS_VTO_V;
    let vdd = VDD_VOLTS;
    let disc = 1.0 + 2.0 * kp * r * (vdd - vth);
    let u = (-1.0 + disc.sqrt()) / (kp * r);
    let v_gs = vth + u;
    let id = 0.5 * kp * u * u;
    (v_gs, id)
}

/// Format a single `f64` in ngspice rawfile's `%.6e` style.
fn fmt_value(v: f64) -> String {
    // ngspice's textual rawfile column is canonically `%.6e` — the
    // parser accepts any `f64::parse`-compatible form, but matching
    // the shipped format keeps the test fixtures legible to
    // downstream humans who diff them.
    format!("{v:.6e}")
}

/// Synthesize an ngspice ASCII rawfile body for the "Sky130 IO-NFET
/// reference leg" DC operating-point golden reference.
///
/// Layout matches the format documented by `conformance-harness`'s
/// [`crate::parser::load_ngspice_ascii`]:
/// `Title`, `Plotname: Operating Point`, `Flags: real`, an N-row
/// `Variables:` block (column 0 = sweep placeholder, columns 1..
/// = dependent variables), and a single-row `Values:` block holding
/// the operating point.
fn synthesize_golden_rawfile() -> String {
    let (v_dg, _id) = analytic_operating_point();
    // Sweep-axis placeholder for the operating-point case (n_points=1).
    let sweep_placeholder = 0.0;
    // The two node voltages observable from the netlist:
    //   v(n_vdd) = 3.3 V exactly (pinned by V1).
    //   v(n_dg)  = Vto + u    (closed-form above).
    format!(
        "Title: sky130-io-nfet-reference-leg-op\n\
         Date: Thu Jun  5 14:00:00 2025\n\
         Plotname: Operating Point\n\
         Flags: real\n\
         No. Variables: 3\n\
         No. Points: 1\n\
         Variables:\n\
         \t0\tv-sweep\tvoltage\n\
         \t1\t{var_vdd}\tvoltage\n\
         \t2\t{var_dg}\tvoltage\n\
         Values:\n\
         \t0\t{sweep}\t{vdd}\t{dg}\n",
        var_vdd = VAR_N_VDD,
        var_dg = VAR_N_DG,
        sweep = fmt_value(sweep_placeholder),
        vdd = fmt_value(VDD_VOLTS),
        dg = fmt_value(v_dg),
    )
}

/// Write the synthesized rawfile content to a per-test fixture path
/// under `std::env::temp_dir()` and return the path. Uses a per-test
/// filename so concurrent test invocations do not collide on the
/// shared temp directory (a latent flake-risk pattern that parent
/// `t_9177e81d` flagged for the conformance-harness smoke tests as
/// "test fixture filenames collide under `std::env::temp_dir()`";
/// every fixture name here is unique).
fn write_temp_fixture(name: &str, body: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("conformance-harness-dc-item-63");
    std::fs::create_dir_all(&dir).expect("create dc-item-63 temp dir");
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).expect("create fixture");
    f.write_all(body.as_bytes()).expect("write fixture");
    path
}

// --- Circuit construction ---------------------------------------------------

/// Build the Sky130-flavored "IO-NFET reference leg" test bench:
///
/// ```text
///   V1 (3.3 V, n_vdd → 0) ──── R1 (10 kΩ) ──── n_dg ──── M1.drain
///                                                         │   .gate ── tied to n_dg
///                                                         └── .source = .bulk = 0
/// ```
///
/// Returns the flattened solver structure plus the graph (for node-id
/// resolution by name).
fn sky130_io_nfet_reference_leg() -> (FlattenedStructure, CircuitGraph) {
    let mut b = CircuitBuilder::default();

    // Declare the model name so the builder accepts the
    // `Some(ModelName)` reference on the element.
    let nmos_model_name = ModelName::new(NMOS_MODEL_NAME);
    b.add_model(nmos_model_name.clone());

    // V1: 3.3 V from n_vdd to gnd.
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: VDD_VOLTS,
        },
        ["n_vdd", "0"],
        None,
    )
    .expect("add V1");

    // R1: 10 kΩ from n_vdd to n_dg.
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

/// `DeviceModelBinding` slice the orchestrator passes to
/// [`dc_analysis`] so the nonlinear DC adapter resolves
/// `M1.model() == NMOS_MODEL_NAME` to a real
/// `DeviceModel::MOSFET(MOSFETParams::Level1(...))` payload.
fn sky130_nfet_bindings() -> Vec<DeviceModelBinding> {
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

/// Look up a node id by name; panics loudly on typo so a fixture
/// regression catches itself.
fn node_id(g: &CircuitGraph, name: &str) -> NodeId {
    g.nodes()
        .iter()
        .find(|n| n.name() == name)
        .unwrap_or_else(|| panic!("node {name} present"))
        .id()
}

/// Newton-Raphson config tuned for the µA-scale fixture. Mirrors the
/// reasoning in `analysis-orchestration/tests/scenario_nonlinear_dc_*`:
/// the SPICE default `abstol = 1 pA` is too tight against the
/// global-norm path's accumulated round-off at µA-scale currents; a
/// 1 nA residue floor is nine orders of magnitude under the device's
/// drain current and well inside the ADR-0008 envelope this
/// conformance test validates.
fn dc_nr_config() -> NewtonRaphsonConfig {
    NewtonRaphsonConfig {
        max_iterations: 100,
        tolerances: ConvergenceTolerances::new(1e-3, 1e-9),
    }
}

// --- Helpers for actual-series extraction -----------------------------------

/// Pull the node-voltage operating-point into the `(name, &[value])`
/// pair shape the conformance comparator expects. The harness's
/// `compare` API takes `IntoIterator<Item = (&str, &[f64])>`, so the
/// caller materialises one tiny `[f64; 1]` per variable and threads
/// the slices through.
fn op_to_actual_voltages(op: &OperatingPoint, g: &CircuitGraph) -> [(String, [f64; 1]); 2] {
    let v_vdd = op
        .voltage_at(node_id(g, "n_vdd"))
        .expect("v(n_vdd) present");
    let v_dg = op.voltage_at(node_id(g, "n_dg")).expect("v(n_dg) present");
    [
        (VAR_N_VDD.to_owned(), [v_vdd]),
        (VAR_N_DG.to_owned(), [v_dg]),
    ]
}

// --- Tests ------------------------------------------------------------------

/// **Headline scenario witness.**
///
/// > Given ConformanceTester has a ngspice Golden Reference for a
/// > Sky130 PDK test bench
/// > And the tolerance envelope is configured as 1 % relative or
/// > 1 mV absolute per node
/// > When ConformanceTester runs the DC operating-point Analysis on
/// > the same Circuit
/// > Then every node voltage in the Result matches the Golden
/// > Reference within the tolerance envelope
/// > And Conformance is reported as "pass"
#[test]
fn dc_conformance_against_sky130_io_nfet_golden_reports_pass() {
    // Given: ConformanceTester has a ngspice Golden Reference for a
    // Sky130 PDK test bench. (The synthesized rawfile encodes the
    // closed-form Level-1 operating point; see module docstring for
    // why an in-process synthesis stands in for the real `ngspice
    // -b sky130_io_nfet.spi -r golden.raw` invocation.)
    let golden_path = write_temp_fixture(
        "dc-conformance-sky130-pass.raw",
        &synthesize_golden_rawfile(),
    );
    let golden: GoldenReference = load_ngspice_ascii(&golden_path).expect("parse golden");
    assert_eq!(
        golden.sweep_kind,
        SweepKind::OperatingPoint,
        "rawfile must classify as Operating Point so the harness picks the DC default tolerance"
    );
    assert_eq!(
        golden.n_points(),
        1,
        "operating-point rawfile must carry exactly one sweep row"
    );
    assert_eq!(
        golden.n_variables(),
        2,
        "rawfile must declare the two dependent voltages compared below"
    );

    // And: the tolerance envelope is configured as 1 % relative or
    // 1 mV absolute per node — the ADR-0008 DC default returned by
    // `AnalysisKind::Dc::default_tolerance()`. Pin the pair to defend
    // against a future retune of the ADR table that would silently
    // change what this test asserts.
    let tolerance = AnalysisKind::Dc.default_tolerance();
    assert!(
        (tolerance.relative - DC_REL).abs() < 1e-15,
        "AnalysisKind::Dc default relative is the spec's '1 %'"
    );
    assert!(
        (tolerance.absolute - DC_ABS).abs() < 1e-15,
        "AnalysisKind::Dc default absolute is the spec's '1 mV'"
    );

    // When: ConformanceTester runs the DC operating-point Analysis on
    // the same Circuit. The "same Circuit" the golden was emitted
    // for is the Sky130 IO-NFET reference-leg fixture below.
    let (fs, g) = sky130_io_nfet_reference_leg();
    let bindings = sky130_nfet_bindings();
    let result: DcAnalysisResult = dc_analysis(
        DcAnalysisRequest::new(&g, &fs)
            .with_device_models(&bindings)
            .with_newton_raphson(dc_nr_config()),
    )
    .expect("dc analysis ok");

    // Then: the Simulator returns a Result containing an
    // OperatingPoint.
    let op: &OperatingPoint = result
        .operating_point
        .as_ref()
        .expect("OperatingPoint present");

    // And the Convergence status is "converged" (the Gherkin clause
    // belongs to the wider DC capability; the spec scenario does not
    // re-assert it, but the OperatingPoint can only exist when NR
    // converged — pinning the status here makes the failure mode
    // obvious if the Newton driver ever returns a stale partial
    // solve through this path).
    assert!(
        matches!(result.convergence, ConvergenceStatus::Converged(_)),
        "expected Converged, got {:?}",
        result.convergence
    );

    // Then: every node voltage in the Result matches the Golden
    // Reference within the tolerance envelope. The comparator is the
    // conformance-harness `compare()` function, exercising the full
    // load_ngspice_ascii → ConformanceReport path the per-analysis
    // tests (#63 here, plus #64–#68 to come) share.
    let actuals = op_to_actual_voltages(op, &g);
    let report = compare(
        &golden,
        actuals
            .iter()
            .map(|(name, vals)| (name.as_str(), vals.as_slice())),
        tolerance,
        16,
    );

    // And Conformance is reported as "pass".
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Pass,
        "expected Pass verdict, got {:?}; report = {report:#?}",
        report.verdict
    );
    assert_eq!(
        report.n_failed_variables, 0,
        "no variable should have failed; report = {report:#?}"
    );
    assert!(
        report.is_pass(),
        "ConformanceReport::is_pass() must agree with the Pass verdict"
    );
    assert!(
        report.worst_margin >= 0.0,
        "Pass implies every per-point margin is non-negative; got worst = {}",
        report.worst_margin
    );

    // Pin the per-variable shape: both declared variables are
    // present, were compared at one point each, and recorded zero
    // failures. This guards against a future refactor that silently
    // drops a missing-variable diagnostic into the report.
    assert_eq!(report.n_variables, 2);
    for var in &report.variables {
        assert!(
            !var.missing_from_actual,
            "variable {} unexpectedly marked missing_from_actual",
            var.name
        );
        assert_eq!(var.n_points, 1, "operating-point: one point per variable");
        assert_eq!(
            var.n_failures, 0,
            "variable {} should have zero failures",
            var.name
        );
    }
}

/// **Negative-path companion.**
///
/// A perturbed golden — same shape, same variables, but with `v(n_dg)`
/// offset by 50 mV (well outside the `max(1 % · 3 V, 1 mV) = 30 mV`
/// envelope) — must flip the verdict to `Fail` and identify
/// `v(n_dg)` as the worst variable.
///
/// This test is the *defensive* witness for the same scenario: it
/// pins the comparator's responsiveness — a future regression that
/// always returns `Pass` would slip the headline test through but
/// fail here.
#[test]
fn dc_conformance_fails_when_golden_is_perturbed_beyond_envelope() {
    // Synthesize a perturbed golden: `v(n_dg)` shifted by 50 mV.
    // The closed-form `Vgs` from `analytic_operating_point()` is the
    // *correct* value the solver will hit, so the perturbation lives
    // entirely in the golden file. The 50 mV offset is chosen so
    // that even at the largest legitimate Vgs the envelope cannot
    // close: `max(0.01 · |Vgs_ref|, 1e-3) ≤ 0.01 · 3 V = 30 mV` —
    // 50 mV is unambiguously outside.
    let (v_dg_correct, _id) = analytic_operating_point();
    let v_dg_perturbed = v_dg_correct + 0.050; // +50 mV

    let body = format!(
        "Title: sky130-io-nfet-reference-leg-op-PERTURBED\n\
         Plotname: Operating Point\n\
         Flags: real\n\
         No. Variables: 3\n\
         No. Points: 1\n\
         Variables:\n\
         \t0\tv-sweep\tvoltage\n\
         \t1\t{var_vdd}\tvoltage\n\
         \t2\t{var_dg}\tvoltage\n\
         Values:\n\
         \t0\t{sweep}\t{vdd}\t{dg}\n",
        var_vdd = VAR_N_VDD,
        var_dg = VAR_N_DG,
        sweep = fmt_value(0.0),
        vdd = fmt_value(VDD_VOLTS),
        dg = fmt_value(v_dg_perturbed),
    );
    let golden_path = write_temp_fixture("dc-conformance-sky130-fail.raw", &body);
    let golden = load_ngspice_ascii(&golden_path).expect("parse perturbed golden");

    // Run the actual DC analysis (untouched — only the golden lies).
    let (fs, g) = sky130_io_nfet_reference_leg();
    let bindings = sky130_nfet_bindings();
    let result = dc_analysis(
        DcAnalysisRequest::new(&g, &fs)
            .with_device_models(&bindings)
            .with_newton_raphson(dc_nr_config()),
    )
    .expect("dc analysis ok");
    let op = result.operating_point.expect("op present");
    let actuals = op_to_actual_voltages(&op, &g);

    let report = compare(
        &golden,
        actuals
            .iter()
            .map(|(name, vals)| (name.as_str(), vals.as_slice())),
        AnalysisKind::Dc.default_tolerance(),
        16,
    );

    // The verdict must flip — and `v(n_dg)` must be the named worst
    // variable so a future failure report points the human at the
    // right node.
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Fail,
        "perturbed golden must produce a Fail verdict; report = {report:#?}"
    );
    assert_eq!(report.n_failed_variables, 1);
    assert_eq!(report.worst_variable, VAR_N_DG);
    assert!(
        report.worst_margin < 0.0,
        "Fail implies the worst margin is negative; got {}",
        report.worst_margin
    );

    // v(n_vdd) is pinned at 3.3 V exactly by V1 — it must still pass.
    let v_vdd_summary = report
        .variables
        .iter()
        .find(|s| s.name == VAR_N_VDD)
        .expect("v(n_vdd) summary present");
    assert_eq!(v_vdd_summary.n_failures, 0);
    assert!(!v_vdd_summary.missing_from_actual);

    // v(n_dg) carries the single failure with the 50 mV-ish offset.
    let v_dg_summary = report
        .variables
        .iter()
        .find(|s| s.name == VAR_N_DG)
        .expect("v(n_dg) summary present");
    assert_eq!(v_dg_summary.n_failures, 1);
    assert_eq!(v_dg_summary.failures.len(), 1);
    let fail = &v_dg_summary.failures[0];
    // The recorded reference must be the perturbed value (validates
    // the rawfile actually round-tripped to the comparator). The
    // tolerance here is bounded by the rawfile's `%.6e` encoding
    // precision, not by `f64` round-trip — that is the same
    // precision-loss every textual ngspice rawfile carries.
    assert!(
        (fail.reference - v_dg_perturbed).abs() < 5e-7,
        "expected golden v(n_dg) = {} V to round-trip (within %.6e precision); got {} V",
        v_dg_perturbed,
        fail.reference
    );
    // The recorded actual must be the solver's value (≈ analytic),
    // not the golden — guarding against a future swap of the
    // reference/actual arguments inside `compare`.
    assert!(
        (fail.actual - v_dg_correct).abs() < 1e-2,
        "expected actual v(n_dg) ≈ analytic {} V; got {} V",
        v_dg_correct,
        fail.actual
    );
    // Margin is `envelope − |diff|`. Diff ≈ 50 mV; envelope at this
    // Vgs is ~ max(0.01·|3.236|, 1e-3) ≈ 32 mV → margin ≈ −18 mV.
    assert!(
        fail.margin < -0.005,
        "expected fail margin < -5 mV; got {} V",
        fail.margin
    );
}

/// **Sanity pin** — the closed-form analytic operating point and the
/// solver-produced operating point agree to within the same envelope
/// the conformance test uses. This is a tighter constraint than the
/// `Pass` verdict alone (it asserts the *solver* converges to the
/// right answer, not just that the solver's answer matches its own
/// echo through the rawfile). Without this pin a regression that
/// broke both the solver *and* the golden synthesis identically
/// would still slip through.
#[test]
fn solver_operating_point_matches_closed_form_within_dc_envelope() {
    let (fs, g) = sky130_io_nfet_reference_leg();
    let bindings = sky130_nfet_bindings();
    let result = dc_analysis(
        DcAnalysisRequest::new(&g, &fs)
            .with_device_models(&bindings)
            .with_newton_raphson(dc_nr_config()),
    )
    .expect("dc analysis ok");
    assert!(
        matches!(result.convergence, ConvergenceStatus::Converged(_)),
        "expected Converged on the sanity-pin path"
    );
    let op = result.operating_point.expect("op present");
    let v_vdd = op
        .voltage_at(node_id(&g, "n_vdd"))
        .expect("v(n_vdd) present");
    let v_dg = op.voltage_at(node_id(&g, "n_dg")).expect("v(n_dg) present");
    let (v_dg_ref, _id) = analytic_operating_point();

    // ADR-0008 DC envelope on each node voltage.
    let envelope_vdd = (DC_REL * VDD_VOLTS.abs()).max(DC_ABS);
    assert!(
        (v_vdd - VDD_VOLTS).abs() <= envelope_vdd,
        "solver V(n_vdd) = {} V outside envelope [{}, {}] around {} V",
        v_vdd,
        VDD_VOLTS - envelope_vdd,
        VDD_VOLTS + envelope_vdd,
        VDD_VOLTS
    );
    let envelope_dg = (DC_REL * v_dg_ref.abs()).max(DC_ABS);
    assert!(
        (v_dg - v_dg_ref).abs() <= envelope_dg,
        "solver V(n_dg) = {} V outside envelope [{}, {}] around analytic {} V",
        v_dg,
        v_dg_ref - envelope_dg,
        v_dg_ref + envelope_dg,
        v_dg_ref
    );
}
