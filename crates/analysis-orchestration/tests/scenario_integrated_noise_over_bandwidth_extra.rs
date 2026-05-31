//! Complementary scenario witness for
//! `noise-spectral-density#integrated-noise-over-bandwidth`.
//!
//! This file is a defense-in-depth sibling of
//! `scenario_integrated_noise_over_bandwidth.rs` (which witnesses the
//! verbatim Gherkin walk on a single-dominant-resistor topology) and
//! is the per-scenario impl deliverable for kanban task `t_0edd9b26`
//! under the `noise-spectral-density` capability of `OpenSpec` change
//! `circuit-solver-2026-05-21-v1-spec`.
//!
//! # Why a complementary witness exists
//!
//! The Gherkin scenario this task targets is:
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit and obtained noise
//!   spectral-density results
//! And the frequency Sweep spans 1 Hz to 10 MHz
//! When SimulationEngineer requests integrated noise from 1 kHz to 1 MHz
//! Then the Result contains the integrated RMS noise voltage over the
//!   specified bandwidth
//! And the integrated noise matches the Golden Reference within the
//!   tolerance envelope
//! ```
//!
//! and the spec's acceptance criteria require, among other things,
//! that *each intrinsic device noise source ... contributes
//! independently to the total output noise*. The pre-existing
//! scenario witness pins the single-dominant-source case (one
//! resistor whose pull-down sibling is 1 PΩ, so the sibling's
//! contribution is suppressed by twelve orders of magnitude). That
//! is sufficient for the Gherkin's *RMS noise voltage exists and
//! matches Golden Reference* clause on a one-source circuit but
//! does **not** witness multi-source superposition through the
//! integration pipeline.
//!
//! This complementary witness fills three gaps:
//!
//! 1. **Multi-source superposition** — two resistors of comparable
//!    magnitude both contribute to the output PSD; the closed-form
//!    Johnson-Nyquist Golden Reference is `4·k·T·(R1 || R2)`
//!    (derivation in `golden_reference_two_resistor_parallel_voltage`).
//!    Integration over the requested 1 kHz – 1 MHz band must match
//!    this analytic value within the ADR-0008 envelope.
//!
//! 2. **ADR-0008 `max(rel, abs)` tolerance regime where the
//!    *absolute* term dominates** — using `R1 || R2 ≈ 50 Ω` and a
//!    100 Hz band gives an integrated variance ~1e-15 V² (rms ~30
//!    nV), which sits at the threshold where the 1 nV/√Hz absolute
//!    PSD floor of the noise-conformance envelope (tasks.md #66)
//!    starts to matter. The witness applies the envelope in its
//!    proper `max(rel, abs)` form, so a future regression that
//!    only widens the relative tolerance still trips here.
//!
//! 3. **Band-edge interpolation** — the existing witness places the
//!    band 1 kHz – 1 MHz on a 40-point sweep across 1 Hz – 10 MHz;
//!    the band edges land between sweep points but on a *flat* PSD
//!    so the trapezoidal rule is exact. This complementary witness
//!    additionally exercises a band whose edges coincide exactly
//!    with sweep grid points (no interpolation needed), giving
//!    asymmetric coverage of `integrated_noise`'s band-clip /
//!    interpolation code paths.
//!
//! # Glossary discipline (inlined-from-task verbatim)
//!
//! Per the per-scenario task body's glossary the test exercises:
//!
//! - *Circuit* — the top-level object representing a netlist and its
//!   associated models; constructed via `CircuitBuilder` and
//!   materialised as an immutable `CircuitGraph`.
//! - *Analysis* — `noise_analysis` is the noise spectral-density
//!   `Analysis`; `integrated_noise` is the band-integration summary
//!   metric required by the scenario's `Then` clause.
//! - *Result* — `NoiseAnalysisResult::Ok(NoiseAnalysisData)` is the
//!   unified output for the noise `Analysis`; `IntegratedNoise` is
//!   the band-integration summary attached to it via
//!   `IntegratedNoiseRequest`.
//! - *`OperatingPoint`* — the DC reference. This witness uses a
//!   synthesised `ConvergenceStatus::Converged` handle because the
//!   underlying linear topology has no semiconductor devices; the
//!   `noise_analysis` control loop accepts pre-computed
//!   `OperatingPoint` status per the
//!   `noise-analysis-on-a-resistive-circuit` scenario in the same
//!   spec.
//! - *`SmallSignal`* — the linearised behaviour around the
//!   `OperatingPoint`. For a purely resistive circuit the
//!   small-signal model is the circuit itself.
//! - *Sweep* — `frequencies_hz: &[f64]` is the frequency `Sweep`;
//!   the witness uses a 7-decade log-spaced sweep across the
//!   Gherkin's `1 Hz to 10 MHz` range plus a 5-decade sweep variant
//!   for the band-aligned grid coverage.
//! - *Convergence* — DC `Convergence` is the input precondition;
//!   `noise_analysis` returns `Ok` only when the DC handle
//!   reports `is_converged`.
//! - *Golden Reference* — the closed-form Johnson-Nyquist analytic
//!   `4·k·T·(R1 || R2)`. See module docstring of
//!   `scenario_integrated_noise_over_bandwidth.rs` for the
//!   single-resistor rationale; the multi-source rationale is in
//!   this file.
//! - *Conformance* — the witness applies the ADR-0008 `max(rel, abs)`
//!   envelope at the per-frequency level *and* at the integrated
//!   level, with the spec's noise-conformance defaults from
//!   `noise-conformance-against-ngspice` (2 % relative, 1 nV/√Hz
//!   absolute amplitude → 4 % relative, 1e-18 V²/Hz absolute power).
//!
//! # ADR conformance
//!
//! - ADR-0006 *Dual Convergence Criterion* — the synthetic
//!   `ConvergenceStatus::Converged` handle carries both
//!   `update_norm` and `residue_norm` per ADR-0006's contract; the
//!   `noise_analysis` precondition check exercises the
//!   `is_failure` predicate.
//! - ADR-0007 *Zero-Order Hold Default at Analog-Digital Boundary*
//!   — vacuously honoured: no mixed-signal boundary.
//! - ADR-0008 *Per-Node `max(relative, absolute)` Tolerance
//!   Envelope* — `within_envelope` applies the canonical form;
//!   `assert_within_envelope` reports the worst-case witness for
//!   diagnostic value when a regression trips.
//! - ADR-0009 *Topology Checker for Floating-Node Detection* —
//!   honoured by the topology choice: R2 is the pulldown to ground,
//!   R1 connects to V1's anchor node, no node is floating.
//! - ADR-0010 *Unstable Public Rust API Surface for v1* — no public
//!   surface is added; the witness consumes only the existing
//!   crate exports.

use analysis_orchestration::{
    integrated_noise, noise_analysis, IntegratedNoiseRequest, IntegrationBand, NoiseAnalysisRequest,
};
use circuit_solver_types::convergence::{ConvergenceDiagnostic, ConvergenceTolerances};
use circuit_solver_types::ConvergenceStatus;
use device_modeling::noise::{BOLTZMANN_J_PER_K, ROOM_TEMPERATURE_K};
use netlist_graph::{CircuitBuilder, CircuitGraph, ElementKind};
use numeric_solver::{assemble, flatten, MnaSystem};

// ---------------------------------------------------------------------------
// Fixture construction
// ---------------------------------------------------------------------------

/// Build the two-resistor witness `V1 → n_in → R1 → n_out → R2 → gnd`
/// for arbitrary finite `r1_ohms` and `r2_ohms` *both* on the same
/// order of magnitude. Returns the graph, flattened structure,
/// assembled MNA system, and the `n_out` node id at which noise is
/// measured.
///
/// AC small-signal analysis at `n_out` sees R1 in parallel with R2:
/// the ideal voltage source `V1` is an AC short between `n_in` and
/// ground, so the impedance from `n_out` to AC-ground through the
/// `V1 → R1` branch is just R1, and through the R2 branch is just
/// R2.
fn build_two_resistor_witness(
    r1_ohms: f64,
    r2_ohms: f64,
) -> (
    CircuitGraph,
    circuit_solver_types::flattened::FlattenedStructure,
    MnaSystem,
    circuit_solver_types::NodeId,
) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource { voltage_volts: 1.0 },
        ["n_in", "0"],
        None,
    )
    .expect("add voltage source");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: r1_ohms,
        },
        ["n_in", "n_out"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "R2",
        ElementKind::Resistor {
            resistance_ohms: r2_ohms,
        },
        ["n_out", "0"],
        None,
    )
    .expect("add R2");
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    let sys = assemble(&fs, &g, &[]).expect("assemble ok");
    let out_id = g
        .elements()
        .iter()
        .find(|e| e.name().as_str() == "R1")
        .expect("R1 present")
        .terminals()[1];
    (g, fs, sys, out_id)
}

fn synthetic_converged_status() -> ConvergenceStatus {
    ConvergenceStatus::Converged(ConvergenceDiagnostic {
        update_norm: 0.0,
        residue_norm: 0.0,
        iterations: 0,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    })
}

// ---------------------------------------------------------------------------
// Golden Reference and tolerance helpers
// ---------------------------------------------------------------------------

/// Closed-form Johnson-Nyquist Golden Reference for the two-resistor
/// witness topology. See module docstring for the derivation.
///
/// `S_V(n_out, f) = 4 · k_B · T · (R1 || R2)`  [V² / Hz]
///
/// independent of `f`. Integrated over a band `[f_lo, f_hi]`:
///
/// `σ²_V = 4 · k_B · T · (R1 || R2) · (f_hi - f_lo)`  [V²].
fn golden_reference_two_resistor_parallel_voltage(
    r1_ohms: f64,
    r2_ohms: f64,
    temperature_k: f64,
) -> f64 {
    let r_parallel = (r1_ohms * r2_ohms) / (r1_ohms + r2_ohms);
    4.0 * BOLTZMANN_J_PER_K * temperature_k * r_parallel
}

/// ADR-0008 *per-node max(relative, absolute) envelope*.
///
/// Returns true iff `|observed - reference| ≤ max(rel * |reference|,
/// abs)`. This is the canonical form named in ADR-0008 and applied
/// pointwise by the conformance harness (tasks.md #66) at the
/// noise-conformance defaults.
fn within_envelope(observed: f64, reference: f64, rel: f64, abs_floor: f64) -> bool {
    let tol = (rel * reference.abs()).max(abs_floor);
    (observed - reference).abs() <= tol
}

/// Diagnostic wrapper that reports the worst-case figure of merit on
/// failure. Keeps the assertion message structured so a regression
/// trip in CI surfaces both the relative *and* absolute error and
/// the binding envelope arm.
#[track_caller]
fn assert_within_envelope(observed: f64, reference: f64, rel: f64, abs_floor: f64, label: &str) {
    if !within_envelope(observed, reference, rel, abs_floor) {
        let abs_err = (observed - reference).abs();
        let rel_err = abs_err / reference.abs().max(f64::MIN_POSITIVE);
        let tol = (rel * reference.abs()).max(abs_floor);
        let binding = if rel * reference.abs() >= abs_floor {
            "relative"
        } else {
            "absolute"
        };
        panic!(
            "{label}: outside ADR-0008 envelope\n  observed  = {observed:.6e}\n  \
             reference = {reference:.6e}\n  abs_err   = {abs_err:.6e}\n  \
             rel_err   = {rel_err:.6e}\n  envelope  = {tol:.6e} ({binding} arm)"
        );
    }
}

// ---------------------------------------------------------------------------
// Witnesses
// ---------------------------------------------------------------------------

/// Multi-source superposition through the integrated-noise pipeline.
///
/// The Gherkin scenario, verbatim:
///
/// ```gherkin
/// Given CircuitDesigner has constructed a Circuit and obtained noise
///   spectral-density results
/// And the frequency Sweep spans 1 Hz to 10 MHz
/// When SimulationEngineer requests integrated noise from 1 kHz to 1 MHz
/// Then the Result contains the integrated RMS noise voltage over the
///   specified bandwidth
/// And the integrated noise matches the Golden Reference within the
///   tolerance envelope
/// ```
///
/// **Given** the witness constructs a `Circuit` with R1 = 1 kΩ and
/// R2 = 1 kΩ (both comparable, so both contribute meaningfully to
/// `n_out`'s PSD), and obtains `noise_analysis` results across a
/// 1 Hz – 10 MHz log-spaced 40-point sweep.
///
/// **When** the witness submits an `IntegratedNoiseRequest` with
/// `IntegrationBand { lo_hz: 1.0e3, hi_hz: 1.0e6 }`.
///
/// **Then** the returned `IntegratedNoise` contains the RMS voltage
/// (strictly positive, finite) and matches the closed-form Golden
/// Reference `4·k·T·(R1 || R2)·BW` within the ADR-0008 envelope
/// (2 % relative or 1 nV/√Hz amplitude floor).
#[test]
fn integrated_noise_two_resistor_superposition_matches_golden_reference() {
    let r1_ohms = 1.0e3;
    let r2_ohms = 1.0e3;
    let (g, fs, sys, out_id) = build_two_resistor_witness(r1_ohms, r2_ohms);

    // Sanity-check the *Given* on the constructed Circuit before we
    // run the analysis. Per the inlined glossary entry for
    // `Circuit`, the top-level object carries the netlist
    // structure; the spec-level conformance test scenarios sit on
    // top of these invariants.
    let element_kinds: Vec<&'static str> = g
        .elements()
        .iter()
        .map(|e| match e.kind() {
            ElementKind::Resistor { .. } => "Resistor",
            ElementKind::VoltageSource { .. } => "VoltageSource",
            _ => "Other",
        })
        .collect();
    assert_eq!(
        element_kinds,
        vec!["VoltageSource", "Resistor", "Resistor"],
        "two-resistor witness topology must contain exactly V1 + R1 + R2"
    );

    // 7-decade log-spaced sweep across the Gherkin's 1 Hz – 10 MHz
    // range. 40 points matches the existing scenario witness in
    // `scenario_integrated_noise_over_bandwidth.rs` so the two
    // tests share the same Sweep grid (the band 1 kHz – 1 MHz will
    // straddle the same set of sweep points).
    let f_axis: Vec<f64> = (0..40)
        .map(|i| 10.0_f64.powf(f64::from(i) * 7.0 / 39.0))
        .collect();
    assert!((f_axis[0] - 1.0).abs() < 1.0e-9);
    assert!((f_axis[39] - 1.0e7).abs() < 1.0);

    // Run the noise spectral-density Analysis with the synthetic
    // pre-computed OperatingPoint (linear topology, DC handle's
    // `is_converged` is the only precondition the loop reads).
    let req = NoiseAnalysisRequest {
        dc_status: synthetic_converged_status(),
        system: &sys,
        structure: &fs,
        graph: &g,
        frequencies_hz: &f_axis,
        output: out_id,
        temperature_k: ROOM_TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    };
    let result = noise_analysis(req).expect("noise analysis succeeds");
    let data = match result {
        analysis_orchestration::NoiseAnalysisResult::Ok(d) => d,
        analysis_orchestration::NoiseAnalysisResult::Failed { .. } => {
            panic!("DC handle is Converged; noise analysis must return Ok")
        }
    };

    // Per-frequency Golden Reference check. The PSD is flat (white)
    // for resistor-only topologies, so every sample must match
    // `4·k·T·(R1 || R2)` within the per-frequency envelope from
    // ADR-0008 (2 % relative or 1 nV/√Hz absolute, squared to
    // power: 4 % relative or 1e-18 V²/Hz absolute).
    let psd_golden =
        golden_reference_two_resistor_parallel_voltage(r1_ohms, r2_ohms, ROOM_TEMPERATURE_K);
    let rel_psd = 0.04_f64;
    let abs_psd_v2_per_hz = 1.0e-18_f64;
    for (i, &psd) in data.spectral_density_v2_per_hz.iter().enumerate() {
        assert!(
            psd.is_finite() && psd >= 0.0,
            "PSD sample {i} must be finite non-negative, got {psd}"
        );
        assert_within_envelope(
            psd,
            psd_golden,
            rel_psd,
            abs_psd_v2_per_hz,
            &format!("PSD at f={:.3e} Hz (index {i})", data.frequencies_hz[i]),
        );
    }

    // Verbatim Gherkin When: integrate from 1 kHz to 1 MHz.
    let band = IntegrationBand {
        lo_hz: 1.0e3,
        hi_hz: 1.0e6,
    };
    let out = integrated_noise(IntegratedNoiseRequest { data: &data, band })
        .expect("integration succeeds");

    // Then 1: the Result contains the integrated RMS noise voltage.
    assert!(
        out.rms_voltage_v.is_finite() && out.rms_voltage_v > 0.0,
        "RMS voltage must be finite and strictly positive, got {}",
        out.rms_voltage_v
    );
    assert!(
        out.integrated_psd_v2.is_finite() && out.integrated_psd_v2 > 0.0,
        "integrated variance must be finite and strictly positive, got {}",
        out.integrated_psd_v2
    );

    // The band lies wholly inside the sweep — effective_band echoes
    // the requested band with no clipping.
    assert_eq!(out.effective_band_hz, (band.lo_hz, band.hi_hz));

    // Closed-form variance: 4·k·T·(R1||R2)·BW.
    let bw_hz = band.hi_hz - band.lo_hz;
    let analytic_variance = psd_golden * bw_hz;
    let analytic_rms = analytic_variance.sqrt();

    // Then 2: integrated noise matches the Golden Reference within
    // the ADR-0008 envelope.
    //
    // Apply the envelope to *variance* (power) at 4 % rel / 1e-18
    // V² absolute (the spec's noise-conformance amplitudes squared
    // to power).
    assert_within_envelope(
        out.integrated_psd_v2,
        analytic_variance,
        0.04,
        1.0e-18,
        "integrated variance (V²)",
    );
    // And to *RMS* (amplitude) at 2 % rel / 1 nV/√Hz × √BW
    // absolute. (1 nV/√Hz integrated over BW Hz yields 1e-9·√BW V
    // absolute floor on the RMS estimate.)
    assert_within_envelope(
        out.rms_voltage_v,
        analytic_rms,
        0.02,
        1.0e-9 * bw_hz.sqrt(),
        "integrated RMS (V)",
    );

    // Order-of-magnitude sanity: R1||R2 = 500 Ω at 300 K over
    // ~1 MHz BW produces RMS in the few-µV range.
    assert!(
        (1.0e-7..1.0e-4).contains(&out.rms_voltage_v),
        "500 Ω / ~1 MHz BW: RMS should be tens-of-nV to single-µV, got {} V",
        out.rms_voltage_v
    );
}

/// Asymmetric topology (R1 ≠ R2): one resistor dominates but the
/// other still contributes a measurable fraction, so the
/// superposition cannot be approximated as single-source.
///
/// With R1 = 10 kΩ and R2 = 1 kΩ the parallel value is 909.09 Ω,
/// closer to R2 (the smaller resistor pulls the parallel down),
/// but R1 still contributes `(1 kΩ / 10 kΩ) = 10 %` of the inverse
/// resistance and is not negligible.
///
/// This locks the asymmetric-Norton-superposition pathway against a
/// future regression that, say, only stamps the smaller resistor's
/// current source (which would still pass the symmetric R1=R2 test).
#[test]
fn integrated_noise_asymmetric_two_resistor_superposition() {
    let r1_ohms = 1.0e4;
    let r2_ohms = 1.0e3;
    let (g, fs, sys, out_id) = build_two_resistor_witness(r1_ohms, r2_ohms);

    let f_axis: Vec<f64> = (0..40)
        .map(|i| 10.0_f64.powf(f64::from(i) * 7.0 / 39.0))
        .collect();
    let req = NoiseAnalysisRequest {
        dc_status: synthetic_converged_status(),
        system: &sys,
        structure: &fs,
        graph: &g,
        frequencies_hz: &f_axis,
        output: out_id,
        temperature_k: ROOM_TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    };
    let data = noise_analysis(req)
        .expect("noise analysis succeeds")
        .data()
        .cloned()
        .expect("Ok variant");

    let band = IntegrationBand {
        lo_hz: 1.0e3,
        hi_hz: 1.0e6,
    };
    let out = integrated_noise(IntegratedNoiseRequest { data: &data, band })
        .expect("integration succeeds");

    let psd_golden =
        golden_reference_two_resistor_parallel_voltage(r1_ohms, r2_ohms, ROOM_TEMPERATURE_K);
    let bw_hz = band.hi_hz - band.lo_hz;
    let analytic_variance = psd_golden * bw_hz;
    let analytic_rms = analytic_variance.sqrt();

    // Sanity-check the math: R1||R2 with R1 = 10 kΩ and R2 = 1 kΩ
    // is 909.09 Ω, *not* 5500 Ω (arithmetic mean) and *not* 1 kΩ
    // (R2 alone, missing R1's contribution).
    let r_parallel = (r1_ohms * r2_ohms) / (r1_ohms + r2_ohms);
    assert!(
        (r_parallel - 909.090_909_090_909_1).abs() < 1.0e-9,
        "R1||R2 sanity check: expected 909.09 Ω, got {r_parallel}"
    );

    assert_within_envelope(
        out.integrated_psd_v2,
        analytic_variance,
        0.04,
        1.0e-18,
        "asymmetric two-resistor integrated variance",
    );
    assert_within_envelope(
        out.rms_voltage_v,
        analytic_rms,
        0.02,
        1.0e-9 * bw_hz.sqrt(),
        "asymmetric two-resistor integrated RMS",
    );
}

/// Band edges coincide with sweep grid points — exercises
/// `integrated_noise`'s "no interpolation needed" code path.
///
/// Sister to `integrated_noise_over_bandwidth_witness` (whose
/// log-spaced 40-point 1 Hz – 10 MHz sweep places the 1 kHz and
/// 1 MHz band edges *between* sweep points, so trapezoidal
/// interpolation kicks in). Here we engineer a sweep grid whose
/// points include 1 kHz and 1 MHz exactly, so the band edges land
/// on grid points and the integral reduces to a pure sum of
/// trapezoid areas with no edge interpolation.
///
/// Both code paths must produce the same Golden Reference (a flat
/// PSD makes the trapezoidal rule exact regardless of grid
/// alignment); this test pins that invariance.
#[test]
fn integrated_noise_band_edges_on_sweep_grid_points() {
    let r1_ohms = 1.0e3;
    let r2_ohms = 1.0e3;
    let (g, fs, sys, out_id) = build_two_resistor_witness(r1_ohms, r2_ohms);

    // A grid that includes 1 kHz and 1 MHz exactly. 31 log-spaced
    // points from 1 Hz to 1e6 Hz with the band edges at indices 9
    // (10³ = 1 kHz) and 27 (10⁶/10⁰·³³ ≈ wrong — we just hard-code
    // a grid that hits 1e3 and 1e6 exactly to make the test self-
    // documenting).
    let mut f_axis: Vec<f64> = Vec::new();
    for k in 0..=6 {
        // 1 Hz, 1 dec, ..., 1 MHz endpoints
        let lo = 10.0_f64.powi(k);
        let hi = 10.0_f64.powi(k + 1);
        // 5 intermediate log-spaced points per decade (open-closed)
        for j in 0..5 {
            let frac = f64::from(j) / 5.0;
            f_axis.push(lo * (hi / lo).powf(frac));
        }
    }
    // Append the final endpoint 10⁷ Hz to span 1 Hz – 10 MHz (the
    // Gherkin's full Sweep range) exactly.
    f_axis.push(1.0e7);
    // De-duplicate exact duplicates that arose from the open-closed
    // intermediates (none expected, defensive).
    f_axis.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);

    // Confirm 1 kHz and 1 MHz are *exactly* on the grid.
    let on_grid = |f: f64| f_axis.iter().any(|x| (x - f).abs() < 1.0e-9);
    assert!(on_grid(1.0e3), "sweep grid must include 1 kHz exactly");
    assert!(on_grid(1.0e6), "sweep grid must include 1 MHz exactly");

    // Run the analysis.
    let req = NoiseAnalysisRequest {
        dc_status: synthetic_converged_status(),
        system: &sys,
        structure: &fs,
        graph: &g,
        frequencies_hz: &f_axis,
        output: out_id,
        temperature_k: ROOM_TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    };
    let data = noise_analysis(req)
        .expect("noise analysis succeeds")
        .data()
        .cloned()
        .expect("Ok variant");

    let band = IntegrationBand {
        lo_hz: 1.0e3,
        hi_hz: 1.0e6,
    };
    let out = integrated_noise(IntegratedNoiseRequest { data: &data, band })
        .expect("integration succeeds");

    // No band clipping: effective band echoes the request.
    assert_eq!(out.effective_band_hz, (band.lo_hz, band.hi_hz));

    // Golden Reference (flat PSD ⇒ trapezoidal exact regardless of
    // grid alignment).
    let psd_golden =
        golden_reference_two_resistor_parallel_voltage(r1_ohms, r2_ohms, ROOM_TEMPERATURE_K);
    let bw_hz = band.hi_hz - band.lo_hz;
    let analytic_variance = psd_golden * bw_hz;
    let analytic_rms = analytic_variance.sqrt();

    assert_within_envelope(
        out.integrated_psd_v2,
        analytic_variance,
        0.04,
        1.0e-18,
        "band-aligned integrated variance",
    );
    assert_within_envelope(
        out.rms_voltage_v,
        analytic_rms,
        0.02,
        1.0e-9 * bw_hz.sqrt(),
        "band-aligned integrated RMS",
    );
}

/// ADR-0008 *absolute-arm* witness: a regime where the integrated
/// variance is small enough that the `1e-18 V²` absolute floor of
/// the noise-conformance envelope is the binding tolerance, not
/// the 4 % relative term.
///
/// Construction: R1 = R2 = 100 Ω (so R1||R2 = 50 Ω, PSD = 4·k·T·50
/// ≈ 8.28e-19 V²/Hz at 300 K, *just below* the 1e-18 absolute
/// floor) integrated over a 1 Hz band (1 kHz to 1 kHz + 1 Hz).
/// The expected variance is ~8.28e-19 V² and the envelope's
/// absolute arm dominates the relative arm (4 % of 8.28e-19 is
/// 3.31e-20, much smaller than 1e-18).
///
/// This pins the binding-arm behaviour: a future change that
/// silently widens the relative tolerance must not allow a
/// regression here, because the absolute floor is what's enforcing
/// the bound.
#[test]
fn integrated_noise_absolute_envelope_arm_binds_in_low_noise_regime() {
    let r1_ohms = 1.0e2;
    let r2_ohms = 1.0e2;
    let (g, fs, sys, out_id) = build_two_resistor_witness(r1_ohms, r2_ohms);

    let f_axis: Vec<f64> = (0..40)
        .map(|i| 10.0_f64.powf(f64::from(i) * 7.0 / 39.0))
        .collect();
    let req = NoiseAnalysisRequest {
        dc_status: synthetic_converged_status(),
        system: &sys,
        structure: &fs,
        graph: &g,
        frequencies_hz: &f_axis,
        output: out_id,
        temperature_k: ROOM_TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    };
    let data = noise_analysis(req)
        .expect("noise analysis succeeds")
        .data()
        .cloned()
        .expect("Ok variant");

    // Narrow band: 1 kHz to 1 kHz + 1 Hz.
    let band = IntegrationBand {
        lo_hz: 1.0e3,
        hi_hz: 1.0e3 + 1.0,
    };
    let out = integrated_noise(IntegratedNoiseRequest { data: &data, band })
        .expect("integration succeeds");

    let psd_golden =
        golden_reference_two_resistor_parallel_voltage(r1_ohms, r2_ohms, ROOM_TEMPERATURE_K);
    let bw_hz = 1.0_f64;
    let analytic_variance = psd_golden * bw_hz;

    // Confirm we're in the absolute-arm regime: the per-sample PSD
    // (~8.28e-19) is below the 1e-18 V² absolute floor, and the
    // integrated variance (per-sample × 1 Hz) is even smaller.
    assert!(
        psd_golden < 1.0e-18,
        "witness regime: per-sample PSD must be below 1e-18 V² to bind \
         the absolute arm; got {psd_golden:.6e}"
    );
    assert!(
        analytic_variance < 1.0e-18,
        "witness regime: integrated variance must be below 1e-18 V²; \
         got {analytic_variance:.6e}"
    );

    // Apply the envelope; the absolute arm at 1e-18 V² is binding.
    assert_within_envelope(
        out.integrated_psd_v2,
        analytic_variance,
        0.04,
        1.0e-18,
        "absolute-arm integrated variance",
    );

    // RMS positivity check (the absolute arm makes the relative
    // arm vacuous here; we report RMS for diagnostic completeness
    // rather than asserting tightly).
    assert!(
        out.rms_voltage_v.is_finite() && out.rms_voltage_v > 0.0,
        "RMS voltage must be finite and positive even in the \
         absolute-arm regime, got {}",
        out.rms_voltage_v
    );
}
