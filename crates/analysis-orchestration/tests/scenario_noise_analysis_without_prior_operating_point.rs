//! Scenario-level integration witness for
//! `noise-spectral-density#noise-analysis-without-prior-operating-point`.
//!
//! Per the executable specification (verbatim Gherkin block from the
//! kanban task body for tasks.md #40):
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit
//! And no OperatingPoint has been computed for this Circuit
//! When CircuitDesigner submits a noise spectral-density Analysis request
//! Then the Simulator first computes a DC OperatingPoint
//! And the Simulator proceeds with noise linearization at that OperatingPoint
//! And the Result contains both the OperatingPoint and the noise spectral-density data
//! ```
//!
//! # Position in the pipeline
//!
//! tasks.md slices the noise capability across:
//!
//! - **#36** — intrinsic device noise source modeling (`4kTG`, `2qI`,
//!   `KF/I^AF / f`).
//! - **#37** — noise analysis control loop with pre-computed
//!   `OperatingPoint`
//!   (`analysis_orchestration::noise::noise_analysis`).
//! - **#40** — auto-DC entry point (this scenario), which composes
//!   `dc_analysis` (tasks.md #20) and `noise_analysis` (#37) into a
//!   single
//!   [`analysis_orchestration::noise::noise_analysis_with_auto_dc`]
//!   call. The "same pattern as AC" called out in tasks.md #40
//!   refers to the architectural shape — internal DC dispatch
//!   feeding a frequency-domain loop — that AC will adopt at
//!   tasks.md #26.
//!
//! `noise_analysis_with_auto_dc` carries its own in-crate unit tests
//! covering the API contract (`auto_dc_resistor_only_returns_op_and_4ktr_psd`,
//! `auto_dc_failure_short_circuits_without_running_noise_loop`,
//! and the four error / accessor witnesses). **This file is the
//! scenario-level witness:** it drives the exact `(graph, structure)`
//! pair from the spec's Given clause without any out-of-band setup,
//! asserts both halves of the Then clause (operating point present
//! *and* spectral-density data present), and pins the PSD to its
//! Johnson-Nyquist Golden Reference (`4·k_B·T·R`) on an analytic
//! single-resistor witness.
//!
//! # Choice of fixture
//!
//! We re-use the canonical noise-witness topology from
//! `noise::tests::single_resistor_to_ground`:
//!
//! ```text
//!     ┌──── V1 (1 V) ──── n_in
//!     │                      │
//!    GND                     R1 = 1 kΩ
//!     │                      │
//!     │                    n_out
//!     │                      │
//!     │                     R2 = 1 PΩ   (effective open at noise port)
//!     │                      │
//!     └────────────────── GND
//! ```
//!
//! - The DC operating point is `V(n_in) = 1 V`, `V(n_out) ≈ 0 V`
//!   (R2 → ∞), `V(GND) = 0 V`. Linear and trivially convergent —
//!   exactly the case the v1 linear DC path handles.
//! - The noise at `n_out` is dominated by R1's thermal source seen
//!   through R1 itself (R2 is an open at the noise port, V1 is an
//!   AC short). The output PSD is therefore the classical
//!   Johnson-Nyquist voltage PSD: `S_V(f) = 4·k_B·T·R1`, white in
//!   frequency.
//!
//! The fixture is intentionally aligned with the in-crate unit-test
//! witness so a regression in the auto-DC composition shows up here
//! and in `noise::tests::auto_dc_resistor_only_returns_op_and_4ktr_psd`
//! simultaneously — the two tests will be in agreement at every
//! release tip.

use analysis_orchestration::{
    noise_analysis_with_auto_dc, NoiseAnalysisWithAutoDcRequest, NoiseAnalysisWithAutoDcResult,
};
use circuit_solver_types::NodeId;
use device_modeling::noise::{BOLTZMANN_J_PER_K, ROOM_TEMPERATURE_K};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::flatten;

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

/// Single-resistor noise witness topology (see module docs).
///
/// Returns `(graph, n_in_id, n_out_id, r1_ohms)`. The caller flattens
/// inside the test body so the scenario witness shows the *exact
/// pipeline a real caller would run*: build → flatten → submit
/// noise-with-auto-DC request — no MNA assembly handled by the test
/// fixture, since the spec's "no `OperatingPoint` has been computed"
/// precondition demands the caller arrive with only the graph +
/// structure in hand.
fn noise_witness_graph(r1_ohms: f64) -> (CircuitGraph, NodeId, NodeId, f64) {
    let mut b = CircuitBuilder::default();
    add_voltage_source(&mut b, "V1", "n_in", "0", 1.0);
    add_resistor(&mut b, "R1", "n_in", "n_out", r1_ohms);
    add_resistor(&mut b, "R2", "n_out", "0", 1.0e15);
    let g = b.build().expect("build ok");

    // Resolve `n_in` / `n_out` NodeIds through the graph (rather than
    // through an MNA system) so the test exercises the
    // pre-assembly handle-resolution path callers will use.
    let n_in = g
        .nodes()
        .iter()
        .find(|n| n.name() == "n_in")
        .expect("n_in present")
        .id();
    let n_out = g
        .nodes()
        .iter()
        .find(|n| n.name() == "n_out")
        .expect("n_out present")
        .id();

    (g, n_in, n_out, r1_ohms)
}

fn approx(a: f64, b: f64, rel: f64) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0e-300);
    (a - b).abs() / scale <= rel
}

/// Scenario witness for
/// `noise-spectral-density#noise-analysis-without-prior-operating-point`.
///
/// Given the witness graph (no `OperatingPoint` pre-computed), when
/// the user submits a noise request via the auto-DC entry point,
/// the result contains both:
///
/// 1. an `OperatingPoint` matching the analytic DC solution (`V(n_in) = 1 V`),
/// 2. a noise spectral-density curve matching the Johnson-Nyquist
///    Golden Reference (`4·k_B·T·R1`) at every requested frequency.
#[test]
fn auto_dc_noise_returns_both_operating_point_and_spectral_density() {
    let r1 = 1.0e3;
    let (graph, n_in_id, n_out_id, _r) = noise_witness_graph(r1);
    let structure = flatten(&graph).expect("flatten ok");

    // Multi-decade sweep. Pure resistive noise is white, so every
    // sample equals `4·k_B·T·R1` to within the LU solver's tight
    // determinism on a 3×3 conductance matrix.
    let frequencies_hz: Vec<f64> = vec![1.0, 10.0, 100.0, 1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7];

    let request = NoiseAnalysisWithAutoDcRequest::new(
        &graph,
        &structure,
        &frequencies_hz,
        n_out_id,
        ROOM_TEMPERATURE_K,
    );

    let result = noise_analysis_with_auto_dc(request)
        .expect("auto-DC + noise analysis must complete on the linear witness");

    // ── First half of the Then clause ──────────────────────────────
    // "the Result contains both the OperatingPoint and the noise
    //  spectral-density data"
    //
    // We assert the OperatingPoint half by name (no Option-unwrap
    // gymnastics) so a future refactor that drops the OperatingPoint
    // from the Ok variant fails this witness loudly.
    let (operating_point, spectral_density) = match &result {
        NoiseAnalysisWithAutoDcResult::Ok {
            operating_point,
            data,
        } => (operating_point, data),
        NoiseAnalysisWithAutoDcResult::Failed {
            dc_status,
            operating_point: _,
        } => panic!(
            "expected auto-DC + noise to succeed on the linear witness; \
             got Failed with dc_status={dc_status:?}"
        ),
    };

    // ── DC operating-point check ──────────────────────────────────
    //
    // The witness places V1 = 1 V from n_in to ground, so V(n_in)
    // is exactly 1 V. V(n_out) is set by the R1 / R2 divider with
    // R2 = 1 PΩ; that voltage is *approximately* zero
    // (R1 / (R1 + R2) ≈ 1e-12) but the precise value depends on
    // numeric solver rounding. We pin only V(n_in) for the witness
    // — the noise-curve check below transitively verifies the
    // linearization at the operating point.
    assert!(
        approx(
            operating_point.voltage_at(n_in_id).expect("n_in present"),
            1.0,
            1.0e-9,
        ),
        "auto-DC operating point: V(n_in) should be 1 V, got {}",
        operating_point.voltage_at(n_in_id).unwrap()
    );
    assert!(
        operating_point.voltage_at(NodeId::GROUND).unwrap().abs() < 1.0e-12,
        "auto-DC operating point: V(GND) must pin to 0 V"
    );

    // ── Second half of the Then clause ─────────────────────────────
    // "the Result contains … the noise spectral-density data"
    //
    // The parallel-vector contract from `NoiseAnalysisData`: one
    // PSD sample per requested frequency, in the same order, every
    // sample non-negative.
    assert_eq!(
        spectral_density.len(),
        frequencies_hz.len(),
        "auto-DC noise: one PSD sample per requested frequency"
    );
    assert_eq!(
        &spectral_density.frequencies_hz, &frequencies_hz,
        "auto-DC noise: frequency axis echoes request verbatim"
    );

    // ── Golden Reference comparison ────────────────────────────────
    //
    // Johnson-Nyquist: `S_V(f) = 4·k_B·T·R1`, white in `f`. Tolerance
    // 1e-6 mirrors the in-crate witness's relative bound; the LU
    // solver is well within that on a 3×3 conductance matrix. We
    // pick the relative tolerance rather than the per-node
    // ADR-0008 envelope because the absolute floor for V²/Hz is
    // not pinned in the spec at v1 and ADR-0008 names a voltage
    // floor; the relative bound is the tightest defensible
    // golden-reference comparison until ngspice-conformance work
    // (tasks.md #66) lands an explicit PSD tolerance row.
    let expected = 4.0 * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K * r1;
    for (i, &s_v) in spectral_density
        .spectral_density_v2_per_hz
        .iter()
        .enumerate()
    {
        assert!(
            s_v >= 0.0,
            "auto-DC noise: PSD must be non-negative; f[{i}]={} Hz, S_V={s_v}",
            frequencies_hz[i]
        );
        assert!(
            approx(s_v, expected, 1.0e-6),
            "auto-DC noise: f[{i}]={} Hz: expected ~{expected:.6e} V²/Hz, got {s_v:.6e}",
            frequencies_hz[i]
        );
    }
}

/// Companion witness pinning the *invariant* implied by the Then
/// clause: the curve is **white** (frequency-independent). A future
/// regression that, e.g., accidentally accumulated only the
/// `node_pos` contribution and not the `node_neg` contribution would
/// still produce the right magnitude at one frequency by luck but
/// would not produce a white curve across the full sweep.
#[test]
fn auto_dc_noise_curve_is_white_across_eight_decades() {
    let r1 = 2.2e3;
    let (graph, _n_in_id, n_out_id, _r) = noise_witness_graph(r1);
    let structure = flatten(&graph).expect("flatten ok");

    let frequencies_hz: Vec<f64> =
        vec![1.0e-1, 1.0, 1.0e1, 1.0e2, 1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7];

    let request = NoiseAnalysisWithAutoDcRequest::new(
        &graph,
        &structure,
        &frequencies_hz,
        n_out_id,
        ROOM_TEMPERATURE_K,
    );
    let result =
        noise_analysis_with_auto_dc(request).expect("auto-DC must complete on linear witness");

    let data = result.data().expect("Ok result has data");
    let first = data.spectral_density_v2_per_hz[0];
    assert!(first > 0.0, "first PSD sample must be strictly positive");
    for &s in &data.spectral_density_v2_per_hz[1..] {
        assert!(
            approx(s, first, 1.0e-9),
            "auto-DC noise: white-noise invariant — every sample equals the \
             first ({first:.6e}), got {s:.6e}"
        );
    }
}
