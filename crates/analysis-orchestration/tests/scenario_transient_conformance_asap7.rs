//! Scenario test: `transient-time-domain#transient-conformance-against-ngspice`
//! — **ASAP7 PDK variant** (tasks.md item #68).
//!
//! This file transposes the item #65 Sky130 transient conformance
//! scaffolding onto ASAP7-relevant parameter values. The test-bench
//! shape (passive RC discharge, linear resampling onto the golden
//! grid, ADR-0008 transient envelope) is identical to the Sky130
//! sibling; only the PDK identity and component values change.
//!
//! ## Gherkin (verbatim from the spec, ASAP7-adapted)
//!
//! ```gherkin
//! Given ConformanceTester has a ngspice Golden Reference for a transient analysis on an ASAP7 PDK test bench
//! And the tolerance envelope is configured as 1 % relative or 1 mV absolute per time point per node
//! When ConformanceTester runs the transient Analysis on the same Circuit with the same time interval and method
//! Then every Waveform matches the Golden Reference within the tolerance envelope at every reported time point
//! And Conformance is reported as "pass"
//! ```
//!
//! ## Glossary terms used here (inlined verbatim from the spec)
//!
//! - **Golden Reference** — "a trusted external simulator against
//!   which results are compared." Here: an ngspice ASCII rawfile
//!   committed at
//!   `crates/analysis-orchestration/tests/fixtures/asap7_rc_discharge_transient.raw`.
//! - **Conformance** — "passing the tolerance-bounded comparison
//!   against a golden reference."
//! - **`ConformanceTester`** — "an automated agent or engineer who
//!   compares solver results against golden references and reports
//!   pass/fail." (Here: the test harness driven by this test.)
//! - **Tolerance envelope** — the `max(relative, absolute)` per-point
//!   per-node envelope from ADR-0008; for transient analysis the
//!   default is `1 %` relative or `1 mV` absolute.
//! - **Waveform** — "a time-domain voltage or current signal."
//!
//! ## v1 scope: passive ASAP7 RC bench with PDK-relevant values
//!
//! The Gherkin says "ASAP7 PDK test bench." At v1 the cleanest
//! faithful witness is a *passive* ASAP7-derived RC discharge. The
//! ASAP7 PDK is a 7 nm `FinFET` process whose analog primitives use
//! BSIM-CMG — out of scope for v1 per ADR-0005. So a passive bench
//! mirrors the v1-scope-deferral pattern the Sky130 sibling documents
//! for MOSFET coupling.
//!
//! Parameters are chosen to reflect ASAP7 on-chip parasitics:
//! - `R = 5 kΩ` — a representative on-chip poly resistor at 7 nm,
//!   about half the 10 kΩ Sky130 metal-layer value.
//! - `C = 1 pF` — same small-cell load capacitance; the R change
//!   gives `τ = 5 ns` and exercises the LTE controller on a
//!   different timescale.
//! - `V₀ = 0.7 V` — the ASAP7 core supply voltage, replacing the
//!   Sky130 sibling's 1.0 V UIC initial condition.
//!
//! The analytic solution `v_C(t) = V₀ · exp(−t/τ)` is what ngspice
//! would emit at LTE-tight settings; the committed rawfile encodes
//! these values.
//!
//! ## Residual risk: same LTE controller pin as the Sky130 sibling
//!
//! The same `max_grow_factor = 1.0` pin from the Sky130 sibling
//! applies here — the adaptive LTE controller's non-uniform-grid
//! overestimation defect (tracked in the sibling file header) is
//! upstream of this conformance test. See the sibling
//! `scenario_transient_conformance_against_ngspice.rs` for the
//! full defect tracker entry.

#![allow(clippy::float_cmp)]

use std::collections::HashMap;

use analysis_orchestration::{
    transient_analysis, InitialState, IntegrationMethod, TransientAnalysisRequest,
};
use circuit_solver_types::{NodeId, SimulationTime, Waveform};
use conformance_harness::{
    compare, load_ngspice_ascii, AnalysisKind, ConformanceVerdict, GoldenReference, SweepKind,
};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{flatten, StepSizeBounds};

// -----------------------------------------------------------------------------
// Bench wiring
// -----------------------------------------------------------------------------

/// The ASAP7 RC discharge bench documented in the file header.
///
/// Returns `(graph, flattened_structure, n_cap_id, v0)`:
/// - `n_cap` carries the discharging capacitor's voltage.
/// - `v0 = 0.7 V` is the UIC initial voltage on the cap (ASAP7 core).
///
/// The topology is a single-pole RC tied directly to ground: both R
/// and C share the `(n_cap, "0")` node pair, so MNA introduces no
/// voltage-source branch-current state.
///
/// Component values:
/// - R = 5 kΩ (on-chip poly resistor at 7 nm)
/// - C = 1 pF (small-cell load capacitance)
/// - τ = 5 ns
fn build_asap7_rc_discharge_bench() -> (
    CircuitGraph,
    numeric_solver::FlattenedStructure,
    NodeId,
    f64,
) {
    let mut b = CircuitBuilder::default();
    // R = 5 kΩ between n_cap and ground.
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: 5.0e3,
        },
        ["n_cap", "0"],
        None,
    )
    .expect("add R1");
    // C = 1 pF between n_cap and ground.
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: 1.0e-12,
        },
        ["n_cap", "0"],
        None,
    )
    .expect("add C1");

    let g = b.build().expect("build");
    let fs = flatten(&g).expect("flatten");

    let n_cap = node_id_of(&g, "n_cap");
    (g, fs, n_cap, 0.7)
}

fn node_id_of(g: &CircuitGraph, name: &str) -> NodeId {
    g.nodes()
        .iter()
        .find(|n| n.name() == name)
        .unwrap_or_else(|| panic!("node {name} present"))
        .id()
}

// -----------------------------------------------------------------------------
// Resampling
// -----------------------------------------------------------------------------

/// Resample `wf` onto a fixed time grid `target_times_sec` by
/// piecewise-linear interpolation between adjacent simulated
/// samples.
///
/// This is the same resampler used by the Sky130 sibling; see its
/// docstring for the full contract and rationale (linear interpolation
/// is sound because trapezoidal integration's per-step reconstruction
/// *is* linear between adjacent accepted samples).
fn resample_linear(wf: &Waveform, target_times_sec: &[f64]) -> Vec<f64> {
    let sim_t: Vec<f64> = wf.times.iter().map(|t| t.as_seconds_f64()).collect();
    assert_eq!(sim_t.len(), wf.values.len(), "waveform shape invariant");
    assert!(sim_t.len() >= 2, "need ≥2 simulated points to interpolate");

    let mut out = Vec::with_capacity(target_times_sec.len());
    let mut hint = 0usize;
    for &t_target in target_times_sec {
        while hint + 1 < sim_t.len() && sim_t[hint + 1] < t_target {
            hint += 1;
        }
        if hint + 1 >= sim_t.len() {
            out.push(wf.values[sim_t.len() - 1]);
            continue;
        }
        let t0 = sim_t[hint];
        let t1 = sim_t[hint + 1];
        let v0 = wf.values[hint];
        let v1 = wf.values[hint + 1];
        if t1 == t0 {
            out.push(v0);
        } else {
            let alpha = (t_target - t0) / (t1 - t0);
            out.push(v0 + alpha * (v1 - v0));
        }
    }
    out
}

// -----------------------------------------------------------------------------
// Headline scenario witness
// -----------------------------------------------------------------------------

/// **The headline witness (ASAP7 variant).** Runs the entire transient
/// pipeline against the committed ASAP7 RC golden and asserts the
/// harness reports Pass at the ADR-0008 transient envelope.
///
/// This single test exercises every observable Then of the adapted
/// Gherkin scenario:
///
/// - **Given 1 / Given 2** (golden + envelope present): the golden
///   loads cleanly as `SweepKind::Transient` and the tolerance pair
///   is the ADR-0008 transient default `(0.01, 1e-3)`.
/// - **When** (analysis runs on the same circuit, interval, method):
///   `transient_analysis` is invoked on the *same* RC bench with
///   `t_start = 0 s`, `t_stop = 15 ns` (matching the golden's last
///   time point), and `IntegrationMethod::Trapezoidal`.
/// - **Then 1** (per-point match within envelope): after resampling
///   onto the golden grid, every point passes.
/// - **Then 2** (conformance reported as "pass"): the report's
///   `verdict` is `ConformanceVerdict::Pass`.
#[test]
fn headline_scenario_transient_conformance_against_asap7_rc_golden() {
    // ---- Given: the ngspice golden reference loads ----
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/asap7_rc_discharge_transient.raw");
    let golden: GoldenReference =
        load_ngspice_ascii(&fixture).expect("load ASAP7 RC transient golden");
    assert_eq!(
        golden.sweep_kind,
        SweepKind::Transient,
        "golden must be a transient sweep"
    );
    assert_eq!(golden.n_points(), 7, "golden has 7 fixed-grid time points");
    assert_eq!(
        golden.n_variables(),
        1,
        "golden declares v(n_cap) — the single observed state-bearing node"
    );

    // ---- Given: the tolerance envelope is the ADR-0008 transient default ----
    let tol = AnalysisKind::Transient.default_tolerance();
    assert_eq!(
        tol.relative, 0.01,
        "1 % relative per ADR-0008 transient row"
    );
    assert_eq!(
        tol.absolute, 1.0e-3,
        "1 mV absolute per ADR-0008 transient row"
    );

    // ---- When: transient_analysis runs on the same circuit and interval ----
    let (g, fs, n_cap, v0) = build_asap7_rc_discharge_bench();
    let mut uic: HashMap<NodeId, f64> = HashMap::new();
    uic.insert(n_cap, v0);

    // Same `max_grow_factor = 1.0` pin as the Sky130 sibling — the
    // LTE controller's non-uniform-grid defect is upstream; see the
    // file-header residual-risk note.
    let bounds = StepSizeBounds {
        h_min: 1.0e-12,
        h_max: 100.0e-12,
        safety_factor: 0.9,
        max_grow_factor: 1.0,
        min_shrink_factor: 0.5,
    };

    let req = TransientAnalysisRequest::new(
        &g,
        &fs,
        SimulationTime::ZERO,
        // 15 ns exactly = 3 τ for the ASAP7 bench (τ = 5 ns).
        SimulationTime::from_nanoseconds(15),
        // Fixed step size of 50 ps (1/100 of τ).
        50.0e-12,
    )
    .with_initial_state(InitialState::UseInitialConditions { node_voltages: uic })
    .with_integration_method(IntegrationMethod::Trapezoidal)
    .with_step_bounds(bounds);

    let result = transient_analysis(req).expect("transient analysis succeeds");
    assert!(
        result.is_converged(),
        "final per-step NR must be Converged, got {:?}",
        result.final_convergence
    );

    // ---- Then 1: resample onto the golden grid ----
    let golden_times_sec: Vec<f64> = golden.sweep_axis.clone();

    let wf_cap = result
        .transient
        .waveforms
        .iter()
        .find(|w| w.node == n_cap)
        .expect("n_cap waveform present in result");

    let actual_cap = resample_linear(wf_cap, &golden_times_sec);
    assert_eq!(actual_cap.len(), golden.n_points());

    // ---- Then 1 (cont.) / Then 2: harness reports Pass ----
    let report = compare(&golden, [("v(n_cap)", actual_cap.as_slice())], tol, 16);

    assert_eq!(
        report.verdict,
        ConformanceVerdict::Pass,
        "conformance must report Pass; worst margin = {:.6e} at variable {:?}; full report: {:#?}",
        report.worst_margin,
        report.worst_variable,
        report
    );
    assert!(report.is_pass(), "is_pass() must agree with verdict");
    assert_eq!(report.n_failed_variables, 0);
    assert_eq!(report.n_variables, 1);

    assert!(
        report.worst_margin >= 0.0,
        "Pass verdict implies worst_margin >= 0, got {}",
        report.worst_margin
    );
}

// -----------------------------------------------------------------------------
// Companion negative witness
// -----------------------------------------------------------------------------

/// **Negative companion (ASAP7 variant).** Confirms the same
/// comparison would *fail* if the simulated trace were off by more
/// than the envelope at every point. This is the regression guard
/// for the "Pass" verdict in the headline test.
///
/// A 100 mV shift at every sample is far outside the 1 % / 1 mV
/// envelope for an ASAP7 trace (envelope @ 0.7 V ref = 7 mV, envelope
/// @ 35 mV ref = 1 mV; 100 mV fails both).
#[test]
fn diverged_trace_fails_against_asap7_rc_golden() {
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/asap7_rc_discharge_transient.raw");
    let golden = load_ngspice_ascii(&fixture).expect("load golden");
    let tol = AnalysisKind::Transient.default_tolerance();

    // Construct a fake "actual" v(n_cap) trace that is the golden
    // shifted upward by 100 mV at every point.
    let bogus_cap: Vec<f64> = golden
        .variables
        .iter()
        .find(|v| v.name == "v(n_cap)")
        .unwrap()
        .values
        .iter()
        .map(|v| v + 0.100)
        .collect();

    let report = compare(&golden, [("v(n_cap)", bogus_cap.as_slice())], tol, 16);
    assert_eq!(
        report.verdict,
        ConformanceVerdict::Fail,
        "a +100 mV shift must fail the 1 %/1 mV envelope"
    );
    assert_eq!(report.worst_variable, "v(n_cap)");
    assert!(report.worst_margin < 0.0);
    assert_eq!(report.n_failed_variables, 1);
}
