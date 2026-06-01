//! Scenario-level integration test for
//! `noise-spectral-density#integrated-noise-over-bandwidth`.
//!
//! This file is the executable witness for the Gherkin scenario inlined
//! into kanban task `t_0b0817c4` (tasks.md item #39). It exercises the
//! **public** API of `analysis-orchestration` (and its transitive
//! `numeric-solver`, `netlist-graph`, `device-modeling` dependencies)
//! end-to-end on the canonical Johnson-Nyquist witness topology, pinning
//! the v1 surface per ADR-0010 and asserting the spec's two observable
//! promises:
//!
//! 1. *The Result contains the integrated RMS noise voltage over the
//!    specified bandwidth.*
//! 2. *The integrated noise matches the Golden Reference within the
//!    tolerance envelope.*
//!
//! Sibling unit tests inside `crates/analysis-orchestration/src/noise.rs`
//! cover the broader API contracts (empty / reversed / out-of-sweep
//! band rejection, flat PSD analytic, linear PSD analytic exactness,
//! band-edge interpolation, band clipping, zero-PSD short-circuit,
//! band-inside-one-interval). This integration test is intentionally
//! narrower and load-bearing for **this** scenario only: it consumes
//! solely the public crate exports, so a future refactor that breaks
//! the v1 surface fails here loudly.
//!
//! # Golden-reference choice
//!
//! Per the scenario's Gherkin
//!
//! > And the integrated noise matches the Golden Reference within the
//! > tolerance envelope
//!
//! and the inlined glossary entry *"Golden Reference — a trusted
//! external simulator against which results are compared."* The
//! tasks.md cross-cutting harness (item #66, *Noise conformance test
//! against ngspice on Sky130*) is gated on items #37 / #38 / #56 and
//! tracked on a separate kanban thread.
//!
//! For a **purely resistive** circuit (the witness topology used by
//! the noise-analysis scenario in this same capability), the
//! Johnson-Nyquist white-noise PSD is `S_V(f) = 4·k·T·R`,
//! independent of frequency. Integrated over a band `[f_lo, f_hi]`
//! the variance is exactly `4·k·T·R·(f_hi - f_lo)` V² and the RMS is
//! `sqrt(4·k·T·R·(f_hi - f_lo))` V. This is the closed-form analytic
//! reference any industrial simulator (ngspice included) would
//! converge to — there is no semiconductor model, no numerical
//! integration error from the trapezoidal rule (the integrand is
//! constant, so trapezoidal is exact), no PDK parameterisation
//! drift. We therefore use the analytic expression as the golden
//! reference for this scenario witness. This mirrors the pattern
//! established by `scenario_ac_purely_linear_circuit.rs` and
//! `scenario_ac_with_precomputed_operating_point.rs`.
//!
//! # Tolerance envelope (ADR-0008 row "Noise PSD / RMS")
//!
//! Per ADR-0008 *Per-Node max(Relative, Absolute) Tolerance Envelope*
//! the noise defaults are 2 % relative or 1 nV/√Hz absolute per
//! frequency point (per the spec's #noise-conformance-against-ngspice
//! scenario). For the integrated metric on a *resistive* circuit
//! the only error sources are:
//!
//! - the AC sub-view's complex LU round-off (already well below 1e-8
//!   per ac.rs's empirical bound),
//! - the trapezoidal rule's edge interpolation (zero error for flat
//!   PSDs).
//!
//! We assert a tighter 1e-4 relative tolerance here as a regression
//! gate, well within the spec's 2 % envelope.

use analysis_orchestration::{
    integrated_noise, noise_analysis, IntegratedNoiseRequest, IntegrationBand, NoiseAnalysisRequest,
};
use circuit_solver_types::convergence::{ConvergenceDiagnostic, ConvergenceTolerances};
use circuit_solver_types::ConvergenceStatus;
use device_modeling::noise::{BOLTZMANN_J_PER_K, ROOM_TEMPERATURE_K};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::{assemble, flatten};

/// Construct the witness fixture: a 1 V DC source feeding R1 in
/// series with a 1 PΩ pulldown R2. The noise output node is the
/// midpoint between R1 and R2. See the in-source rationale in
/// `noise.rs::tests::single_resistor_to_ground` for the topology
/// choice (ADR-0009 floating-node-checker workaround).
fn build_single_resistor_witness(
    r1_ohms: f64,
) -> (
    netlist_graph::CircuitGraph,
    circuit_solver_types::flattened::FlattenedStructure,
    numeric_solver::MnaSystem,
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
            resistance_ohms: 1.0e15,
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

fn approx(a: f64, b: f64, rel: f64) -> bool {
    let scale = a.abs().max(b.abs()).max(1.0e-300);
    (a - b).abs() / scale <= rel
}

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
/// **Given:** a circuit dominated by a 1 kΩ resistor and its
/// noise-analysis result computed across a 1 Hz – 10 MHz log-spaced
/// sweep (7-decade, 40-point sweep).
///
/// **When:** `integrated_noise` is invoked with band [1 kHz, 1 MHz].
///
/// **Then:** the output's `rms_voltage_v` is strictly positive, the
/// effective band echoes [1 kHz, 1 MHz] (no clipping), and both the
/// variance and the RMS match the closed-form Johnson-Nyquist value
/// `4·k·T·R·(f_hi - f_lo)` within 1e-4 relative (well inside the
/// spec's 2 % envelope).
#[test]
fn integrated_noise_over_bandwidth_witness() {
    let r1_ohms = 1.0e3;
    let (g, fs, sys, out_id) = build_single_resistor_witness(r1_ohms);

    // 7 decades from 1 Hz to 10 MHz, 40 log-spaced points.
    let f_axis: Vec<f64> = (0..40)
        .map(|i| 10.0_f64.powf(f64::from(i) * 7.0 / 39.0))
        .collect();
    assert!((f_axis[0] - 1.0).abs() < 1.0e-9);
    assert!((f_axis[39] - 1.0e7).abs() < 1.0);

    // Run the underlying noise spectral-density analysis.
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
    let data = result.data().cloned().expect("Ok variant");
    assert_eq!(data.frequencies_hz.len(), 40);
    assert_eq!(data.spectral_density_v2_per_hz.len(), 40);

    // Integrate from 1 kHz to 1 MHz.
    let band = IntegrationBand {
        lo_hz: 1.0e3,
        hi_hz: 1.0e6,
    };
    let int_req = IntegratedNoiseRequest { data: &data, band };
    let out = integrated_noise(int_req).expect("integration succeeds");

    // 1. The Result contains the integrated RMS noise voltage.
    assert!(
        out.rms_voltage_v > 0.0,
        "RMS voltage must be strictly positive for a thermal-noisy resistor"
    );
    assert!(
        out.integrated_psd_v2 > 0.0,
        "integrated variance must be strictly positive"
    );
    // The effective band echoes the request — band lies wholly
    // inside the sweep so no clipping occurs.
    assert_eq!(out.effective_band_hz, (band.lo_hz, band.hi_hz));

    // 2. Matches the Golden Reference (4·k·T·R·BW) within the
    //    tolerance envelope.
    let analytic_psd = 4.0 * BOLTZMANN_J_PER_K * ROOM_TEMPERATURE_K * r1_ohms;
    let analytic_variance = analytic_psd * (band.hi_hz - band.lo_hz);
    let analytic_rms = analytic_variance.sqrt();

    assert!(
        approx(out.integrated_psd_v2, analytic_variance, 1.0e-4),
        "integrated variance: golden {analytic_variance:.6e} V², got {:.6e} V² (rel err > 1e-4)",
        out.integrated_psd_v2
    );
    assert!(
        approx(out.rms_voltage_v, analytic_rms, 1.0e-4),
        "integrated RMS: golden {analytic_rms:.6e} V, got {:.6e} V (rel err > 1e-4)",
        out.rms_voltage_v
    );

    // Order-of-magnitude sanity: 1 kΩ @ 300 K over ~1 MHz BW
    // produces RMS in the single-digit microvolt range.
    assert!(
        (1.0e-6..1.0e-5).contains(&out.rms_voltage_v),
        "1 kΩ / 1 MHz integration BW: RMS should be a few µV, got {} V",
        out.rms_voltage_v
    );
}

/// Companion check: doubling the resistance doubles the variance (so
/// the RMS grows by √2). This locks the linear-in-R Johnson-Nyquist
/// scaling through the integrated-noise pipeline, defending the
/// composition against future refactors of either `noise_analysis` or
/// `integrated_noise`.
#[test]
fn integrated_noise_scales_linearly_with_resistance() {
    let band = IntegrationBand {
        lo_hz: 1.0e3,
        hi_hz: 1.0e5,
    };
    let f_axis: Vec<f64> = (0..30)
        .map(|i| 10.0_f64.powf(f64::from(i) * 5.0 / 29.0))
        .collect();

    let run = |r1_ohms: f64| -> f64 {
        let (g, fs, sys, out_id) = build_single_resistor_witness(r1_ohms);
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
        integrated_noise(IntegratedNoiseRequest { data: &data, band })
            .expect("integration succeeds")
            .integrated_psd_v2
    };
    let v1 = run(1.0e3);
    let v2 = run(2.0e3);
    assert!(
        approx(v2, 2.0 * v1, 1.0e-4),
        "variance should double with R: 2·{v1:.6e} ≈ {v2:.6e}"
    );
}
