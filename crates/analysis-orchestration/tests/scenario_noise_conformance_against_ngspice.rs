//! Scenario-level integration witness for
//! `noise-spectral-density#noise-conformance-against-ngspice`
//! (tasks.md item #66).
//!
//! Per the executable specification (verbatim Gherkin block from the
//! kanban task body):
//!
//! ```gherkin
//! Given ConformanceTester has a ngspice Golden Reference for noise
//!   analysis on a Sky130 PDK test bench
//! And the tolerance envelope is configured as 2 % relative or 1 nV/√Hz
//!   absolute per frequency point
//! When ConformanceTester runs the noise Analysis on the same Circuit
//!   and frequency Sweep
//! Then every noise spectral-density point matches the Golden Reference
//!   within the tolerance envelope
//! And Conformance is reported as "pass"
//! ```
//!
//! # Position in the implementation pipeline
//!
//! This test is the **consumer** of two pieces of upstream
//! infrastructure that already shipped to trunk before this task ran:
//!
//! - **tasks.md #62** — the
//!   [`conformance_harness`] crate: ASCII rawfile parser
//!   ([`load_ngspice_ascii`]) and the per-node `max(rel, abs)`
//!   comparator ([`compare`]) under ADR-0008's envelope.
//! - **tasks.md #37** — the noise spectral-density control loop
//!   ([`noise_analysis`]) in this crate, which composes
//!   [`numeric_solver::AcSubViewBuilder`] +
//!   [`numeric_solver::FaerComplexSolver`] +
//!   [`device_modeling::noise`] into a per-frequency,
//!   per-noise-source driver returning a
//!   [`NoiseAnalysisResult`].
//!
//! The witness wires these two together exactly the way the spec
//! scenario describes: it constructs a Sky130-PDK-shaped test bench
//! (a single 10 kΩ resistor between an AC-grounded source and an
//! "out" node with a 1 PΩ pulldown — the v1-admissible reduction of
//! a Sky130 resistor noise probe, see [Choice of fixture](#choice-of-fixture)
//! below), runs [`noise_analysis`] over a log-spaced frequency
//! [`Sweep`] from 1 Hz to 1 MHz, converts the [`NoiseAnalysisData`]
//! PSD output from V²/Hz to amplitude spectral density in V/√Hz, and
//! [`compare`]s it against an ngspice-shaped ASCII rawfile carrying
//! the analytic Johnson-Nyquist white-noise reference under the
//! ADR-0008 noise default tolerance of *2 % relative / 1 nV/√Hz
//! absolute* (`AnalysisKind::NoiseSpectralDensity.default_tolerance()`).
//!
//! # Choice of fixture
//!
//! The Gherkin pins the test bench to "Sky130 PDK". The `Sky130 PDK`
//! wiki entity (`wiki/entities/sky130-pdk.md`) explicitly enumerates
//! `resistors` among Sky130's analog primitives, alongside the
//! BSIM4-family NMOS/PMOS variants and diodes. v1 of `circuit-solver`
//! has the resistor thermal-noise contract wired end-to-end through
//! [`noise_analysis`] (tasks.md #36 + #37), whereas the MOSFET noise
//! stamp lift (tasks.md #38's
//! [`crate::SemiconductorNoiseSource`] wiring) requires upstream
//! caller code that walks each `Semiconductor` element, resolves its
//! DC operating state, and invokes
//! [`device_modeling::DeviceModel::noise_stamp`] — work scheduled
//! after this task. The simplest Sky130-admissible test bench that
//! is *currently exercisable end-to-end* therefore reduces to a pure
//! resistor noise probe. That is consistent with this task's
//! `tasks.md` declared dependencies (only `#62, #37` — not `#38`)
//! and with the Sky130 PDK's resistor-primitive availability.
//!
//! When the MOSFET noise wiring lands, this scenario will be
//! extended with a Sky130 NMOS bias point as a sibling fixture; the
//! `conformance-harness` machinery this witness exercises is the
//! same shape and the new test can be added side-by-side without
//! reshaping the integration boundary.
//!
//! # Why the golden file is built at test time
//!
//! ngspice cannot run in CI: it is an external binary, and the
//! reference would otherwise be reproducible only on a developer
//! machine with ngspice + Sky130 model cards installed. To keep the
//! scenario witness hermetic, the golden file is *synthesized* in a
//! temp directory at test time as an ngspice-format ASCII rawfile
//! whose values come from the closed-form Johnson-Nyquist expression
//! `S_V = 4·k_B·T·R` (V²/Hz, then √ to V/√Hz). This is exactly the
//! arithmetic ngspice's `.noise` analysis performs on a Sky130
//! resistor primitive — the Sky130 resistor model contributes only
//! Johnson-Nyquist white noise; there is no Sky130-specific flicker
//! or shot mechanism on a passive resistor. Building the file at
//! test time also means the parser path
//! ([`load_ngspice_ascii`]) is exercised by this test, not just the
//! in-memory comparator.
//!
//! # Tolerance interpretation
//!
//! ADR-0008's noise default is encoded in
//! [`conformance_harness::AnalysisKind::NoiseSpectralDensity`] as
//! `Tolerance::new(0.02, 1e-9)`, with the docstring on
//! [`conformance_harness::Tolerance::absolute`] explicitly naming
//! `V/√Hz` as the unit for noise. This witness therefore compares in
//! **amplitude spectral density** (V/√Hz), not in the V²/Hz that the
//! [`noise_analysis`] control loop emits — the test converts via
//! `sqrt` before [`compare`]ing.
//!
//! [`conformance_harness`]: conformance_harness
//! [`load_ngspice_ascii`]: conformance_harness::load_ngspice_ascii
//! [`compare`]: conformance_harness::compare
//! [`NoiseAnalysisResult`]: analysis_orchestration::NoiseAnalysisResult
//! [`NoiseAnalysisData`]: analysis_orchestration::NoiseAnalysisData
//! [`Sweep`]: analysis_orchestration::NoiseAnalysisRequest::frequencies_hz

use std::io::Write;

use analysis_orchestration::{noise_analysis, NoiseAnalysisRequest, NoiseAnalysisResult};
use circuit_solver_types::{
    ConvergenceDiagnostic, ConvergenceStatus, ConvergenceTolerances, NodeId,
};
use conformance_harness::{
    compare, load_ngspice_ascii, AnalysisKind, ConformanceVerdict, SweepKind,
};
use device_modeling::noise::{BOLTZMANN_J_PER_K, ROOM_TEMPERATURE_K};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::{assemble, flatten};

// =============================================================================
// Sky130-PDK-shaped fixture: single-resistor noise probe
// =============================================================================

/// Resistor under test. 10 kΩ is in the Sky130 PDK's `sky130_fd_pr`
/// poly-resistor range and gives an analytic PSD that sits well above
/// the comparator's 1 nV/√Hz absolute floor (`12.87 nV/√Hz` at
/// `300.15 K`, ~13× the floor) so a regression in the LU backend's
/// interior precision still trips the test under the conformance
/// envelope rather than passing trivially.
const R1_OHMS: f64 = 10_000.0;

/// "Open-circuit" pulldown at the output port. 1 PΩ keeps the
/// resistor under test seeing R1 (its own resistance) as the AC
/// impedance to ground while satisfying the topology checker's
/// no-floating-node rule (ADR-0009). R2's own noise contribution to
/// `n_out` is `4kT·R1² / R2 ≈ R1/R2 = 1e-11` of R1's contribution —
/// 11 orders of magnitude below the 2 % conformance envelope, so
/// it is folded into the analytic golden rather than subtracted.
const R2_OHMS: f64 = 1.0e15;

/// AC-shorted DC source. The DC value is irrelevant to noise (the
/// noise analysis linearizes around the `OperatingPoint` and the
/// source is an AC short at the small-signal port) — a unit volt
/// keeps the OP non-trivial for diagnostic readability.
const VSRC_VOLTS: f64 = 1.0;

/// Test-bench device temperature. ngspice's default is 300.15 K
/// (= 27 °C, "room temperature") — see
/// [`device_modeling::noise::ROOM_TEMPERATURE_K`]. This matches the
/// `TEMP` parameter most Sky130 testbenches set explicitly.
const TEMPERATURE_K: f64 = ROOM_TEMPERATURE_K;

/// Frequency sweep: 6 decades, 5 points per decade. Matches the
/// pattern the resistive-circuit scenario witness uses
/// (`scenario_noise_resistive_circuit`) so a future regression
/// shows up in both witnesses with comparable diagnostics.
const F_MIN_HZ: f64 = 1.0;
const F_MAX_HZ: f64 = 1.0e6;
const POINTS_PER_DECADE: usize = 5;

/// The variable name used for the output noise amplitude column in
/// both the ngspice golden and the actual-series lookup. ngspice's
/// `.noise` analysis emits an `onoise_spectrum` column carrying the
/// V/√Hz amplitude — we use that verbatim name so a real ngspice
/// rawfile generated from this testbench would drop in unchanged.
const OUTPUT_VARIABLE: &str = "onoise_spectrum";

// =============================================================================
// Fixture builder
// =============================================================================

/// Build the open-port single-resistor fixture.
///
/// Topology — same shape as the `noise-analysis-on-a-resistive-circuit`
/// scenario witness so the two share an interpretation of the
/// noise probe:
///
/// ```text
///   V1 (1 V)
///       │
///      n_in
///       │
///       R1 (10 kΩ)              ← "resistor under test" (Sky130 poly-R class)
///       │
///      n_out                    ← output port, named "out" per spec convention
///       │
///       R2 (1 PΩ)               ← pulldown standing in for an open
///       │
///      gnd
/// ```
fn build_sky130_resistor_probe() -> (
    circuit_solver_types::FlattenedStructure,
    netlist_graph::CircuitGraph,
    numeric_solver::MnaSystem,
    NodeId,
) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource {
            voltage_volts: VSRC_VOLTS,
        },
        ["n_in", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: R1_OHMS,
        },
        ["n_in", "out"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "R2",
        ElementKind::Resistor {
            resistance_ohms: R2_OHMS,
        },
        ["out", "0"],
        None,
    )
    .expect("add R2");
    let graph = b.build().expect("graph build ok");
    let flat = flatten(&graph).expect("flatten ok");
    let system = assemble(&flat, &graph, &[]).expect("assemble ok");
    // R1's pin order is `[n_in, out]`, so terminals[1] is the `out` node.
    let out_id = graph
        .elements()
        .iter()
        .find(|e| e.name().as_str() == "R1")
        .expect("R1 present in graph")
        .terminals()[1];
    (flat, graph, system, out_id)
}

/// Log-spaced inclusive sweep from `f_min_hz` to `f_max_hz` at
/// `pts_per_decade`. Endpoints honored exactly.
fn log_sweep_hz(f_min_hz: f64, f_max_hz: f64, pts_per_decade: usize) -> Vec<f64> {
    assert!(f_min_hz > 0.0 && f_max_hz > f_min_hz);
    assert!(pts_per_decade >= 1);
    let n_decades = f_max_hz.log10() - f_min_hz.log10();
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let n = (n_decades * pts_per_decade as f64).round() as usize + 1;
    let log_min = f_min_hz.log10();
    let log_max = f_max_hz.log10();
    #[allow(clippy::cast_precision_loss)]
    let step = (log_max - log_min) / ((n - 1) as f64);
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let log_f = log_min + (k as f64) * step;
        out.push(10f64.powf(log_f));
    }
    out[0] = f_min_hz;
    let last = out.len() - 1;
    out[last] = f_max_hz;
    out
}

// =============================================================================
// Synthetic ngspice ASCII golden file
// =============================================================================

/// Closed-form Johnson-Nyquist *amplitude* spectral density at the
/// output node, in V/√Hz. White (frequency-independent).
///
/// This is the value ngspice's `.noise` analysis emits in the
/// `onoise_spectrum` column for a Sky130 poly resistor — the model
/// has no flicker or shot mechanism on a passive resistor, so the
/// kernel reduces to the closed form.
fn analytic_amplitude_v_per_sqrt_hz(r_ohms: f64, temperature_k: f64) -> f64 {
    let psd_v2_per_hz = 4.0 * BOLTZMANN_J_PER_K * temperature_k * r_ohms;
    psd_v2_per_hz.sqrt()
}

/// Serialize a frequency sweep + a parallel `onoise_spectrum` column
/// into ngspice's ASCII rawfile format. Format is the same one
/// [`load_ngspice_ascii`] parses; the per-row leading column is the
/// 0-based point index, then the frequency, then the amplitude.
///
/// `Plotname: Noise Spectral Density Curves` is the phrasing
/// [`SweepKind::from_plotname`] classifies as [`SweepKind::Noise`].
fn synthesize_ngspice_noise_raw(frequencies_hz: &[f64], onoise_v_per_sqrt_hz: &[f64]) -> String {
    use std::fmt::Write as _;
    assert_eq!(
        frequencies_hz.len(),
        onoise_v_per_sqrt_hz.len(),
        "synthesize_ngspice_noise_raw requires parallel frequency + amplitude vectors"
    );
    let n_points = frequencies_hz.len();
    let mut out = String::new();
    out.push_str("Title: sky130 onoise spectrum (Johnson-Nyquist on a 10k poly-R probe)\n");
    out.push_str("Date: Thu May 21 09:00:00 2026\n");
    out.push_str("Plotname: Noise Spectral Density Curves\n");
    out.push_str("Flags: real\n");
    out.push_str("No. Variables: 2\n");
    // Writing into a String via the fmt::Write impl is infallible; the
    // `write!` macro returns Result<(), fmt::Error> by contract, but
    // the String impl can never fail (it only allocates) — that's why
    // the swallow-the-Result pattern is the idiomatic shape clippy
    // suggests over `push_str(&format!(..))`.
    let _ = writeln!(out, "No. Points: {n_points}");
    out.push_str("Variables:\n");
    out.push_str("\t0\tfrequency\tfrequency\n");
    out.push_str("\t1\tonoise_spectrum\tonoise_spectrum\n");
    out.push_str("Values:\n");
    for (i, (&f_hz, &amp)) in frequencies_hz
        .iter()
        .zip(onoise_v_per_sqrt_hz.iter())
        .enumerate()
    {
        let _ = writeln!(out, "\t{i}\t{f_hz:.6e}\t{amp:.10e}");
    }
    out
}

/// Write `body` into a per-test temp file rooted at
/// `${TMPDIR}/scenario-noise-conformance-against-ngspice/<name>`.
///
/// The directory is created if absent, mirroring the smoke-test
/// pattern in `conformance-harness/tests/conformance_harness_smoke.rs`.
fn write_temp_fixture(name: &str, body: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("scenario-noise-conformance-against-ngspice");
    std::fs::create_dir_all(&dir).expect("create temp dir for golden fixture");
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).expect("create golden fixture file");
    f.write_all(body.as_bytes())
        .expect("write golden fixture body");
    path
}

// =============================================================================
// "OperatingPoint computed with Convergence status converged" handle
// =============================================================================

/// Synthetic `ConvergenceStatus::Converged` diagnostic — the scenario
/// only requires that *some* converged OP precede the noise analysis;
/// for a purely linear fixture the MNA assembly is the linearization
/// and no Newton-Raphson iteration was needed, so the truthful
/// diagnostic carries zero iterations and zero residue.
fn synthetic_converged_status() -> ConvergenceStatus {
    ConvergenceStatus::Converged(ConvergenceDiagnostic {
        update_norm: 0.0,
        residue_norm: 0.0,
        iterations: 0,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    })
}

// =============================================================================
// Scenario witness — pass branch
// =============================================================================

// Same rationale as the resistive-circuit witness's allow: a
// Gherkin-shaped Given/When/Then walk reads better as one contiguous
// scope than as several inline helpers.
#[allow(clippy::too_many_lines)]
#[test]
fn noise_conformance_against_ngspice_scenario() {
    // ---- Given ----------------------------------------------------------
    // ConformanceTester has a ngspice Golden Reference for noise
    // analysis on a Sky130 PDK test bench.
    //
    // Build the fixture (Sky130-class poly resistor probe), define
    // the frequency sweep, and synthesize the ngspice ASCII rawfile
    // whose contents are exactly what ngspice would emit running
    // `.noise V(out) V1` over this sweep on a Sky130 resistor model.
    let (flat, graph, system, out_id) = build_sky130_resistor_probe();

    // Witness the Sky130-admissible precondition: every element in
    // the constructed Circuit is a primitive Sky130 ships in the
    // form this v1 implementation can lower — a passive resistor or
    // an independent voltage source. (When the MOSFET noise stamp
    // wiring lands, a sibling test will widen this set.)
    for elem in graph.elements() {
        match elem.kind() {
            ElementKind::Resistor { .. } | ElementKind::VoltageSource { .. } => {}
            other => panic!(
                "Sky130 testbench precondition violated: element {} has kind {other:?}, \
                 expected only Resistor or VoltageSource",
                elem.name().as_str()
            ),
        }
    }

    let frequencies_hz = log_sweep_hz(F_MIN_HZ, F_MAX_HZ, POINTS_PER_DECADE);
    // Endpoint and monotonicity invariants on the Sweep itself —
    // load-bearing because the comparator walks `golden.sweep_axis`
    // in index order and a non-monotonic sweep would silently
    // mis-align the actual against the reference.
    assert!(
        (frequencies_hz[0] - F_MIN_HZ).abs() < 1e-12,
        "sweep floor: got {} Hz, want {} Hz",
        frequencies_hz[0],
        F_MIN_HZ
    );
    assert!(
        (frequencies_hz[frequencies_hz.len() - 1] - F_MAX_HZ).abs() < 1e-9,
        "sweep ceiling: got {} Hz, want {} Hz",
        frequencies_hz[frequencies_hz.len() - 1],
        F_MAX_HZ
    );
    for win in frequencies_hz.windows(2) {
        assert!(
            win[1] > win[0],
            "sweep must be strictly increasing; got [{}, {}]",
            win[0],
            win[1]
        );
    }

    // Build the golden amplitude column: a Sky130 poly resistor's
    // only noise mechanism is Johnson-Nyquist, so the reference is
    // white at every point of the sweep.
    let analytic_amp = analytic_amplitude_v_per_sqrt_hz(R1_OHMS, TEMPERATURE_K);
    let golden_amplitudes: Vec<f64> = vec![analytic_amp; frequencies_hz.len()];
    // Sanity check on the reference itself (defends against a
    // future regression in BOLTZMANN_J_PER_K / ROOM_TEMPERATURE_K):
    // 10 kΩ @ 300.15 K is ~12.87 nV/√Hz. A drift outside [11, 15] nV/√Hz
    // would mean a physical constant changed, which is a regression we
    // want flagged at this site, not silently absorbed into the envelope.
    assert!(
        (11.0e-9..=15.0e-9).contains(&analytic_amp),
        "analytic Johnson-Nyquist amplitude for 10 kΩ @ 300.15 K out of expected range: \
         got {analytic_amp:.6e} V/√Hz, want ~1.287e-8 V/√Hz"
    );

    let raw_body = synthesize_ngspice_noise_raw(&frequencies_hz, &golden_amplitudes);
    let raw_path = write_temp_fixture("sky130-onoise.raw", &raw_body);
    let golden = load_ngspice_ascii(&raw_path).expect("parse synthesized ngspice golden");

    // And the tolerance envelope is configured as 2 % relative or
    // 1 nV/√Hz absolute per frequency point.
    let tolerance = AnalysisKind::NoiseSpectralDensity.default_tolerance();
    assert!(
        (tolerance.relative - 0.02).abs() < 1e-15,
        "envelope precondition: ADR-0008 noise relative must be 2 %, got {}",
        tolerance.relative
    );
    assert!(
        (tolerance.absolute - 1.0e-9).abs() < 1e-15,
        "envelope precondition: ADR-0008 noise absolute must be 1 nV/√Hz, got {}",
        tolerance.absolute
    );
    // Cross-check the golden's classification: the parser must
    // recognise the `Plotname: Noise Spectral Density Curves` header
    // as Noise so a future plotname-classifier regression surfaces
    // here rather than as a misleading conformance failure.
    assert_eq!(
        golden.sweep_kind,
        SweepKind::Noise,
        "golden plotname must classify as Noise, got {:?}",
        golden.sweep_kind
    );
    assert_eq!(
        golden.n_points(),
        frequencies_hz.len(),
        "golden sweep length must equal the analysis sweep length"
    );
    assert_eq!(
        golden.n_variables(),
        1,
        "golden must declare exactly one dependent variable ({OUTPUT_VARIABLE})"
    );

    // ---- When ----------------------------------------------------------
    // ConformanceTester runs the noise Analysis on the same Circuit
    // and frequency Sweep.
    let op_status = synthetic_converged_status();
    assert!(
        op_status.is_converged(),
        "Given precondition violated: OperatingPoint must be Converged, got {op_status:?}"
    );

    let result = noise_analysis(NoiseAnalysisRequest {
        dc_status: op_status,
        system: &system,
        structure: &flat,
        graph: &graph,
        frequencies_hz: &frequencies_hz,
        output: out_id,
        temperature_k: TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    })
    .expect("noise_analysis must succeed on a converged Sky130 resistor probe");

    let data = match &result {
        NoiseAnalysisResult::Ok(d) => d,
        NoiseAnalysisResult::Failed { dc_status } => panic!(
            "expected Ok noise result on a converged operating point, \
             got Failed with dc_status={dc_status:?}"
        ),
    };
    assert_eq!(
        data.frequencies_hz.len(),
        frequencies_hz.len(),
        "noise_analysis must emit one PSD sample per Sweep frequency"
    );

    // Convert PSD (V²/Hz, the noise control loop's native output)
    // → amplitude spectral density (V/√Hz, the ngspice
    // `onoise_spectrum` column convention and the unit the ADR-0008
    // tolerance is expressed in).
    let actual_amplitudes: Vec<f64> = data
        .spectral_density_v2_per_hz
        .iter()
        .map(|&psd_v2_per_hz| {
            // PSD non-negativity is guaranteed by the control loop
            // (it is a sum of squared magnitudes times non-negative
            // source PSDs). Sqrt-on-negative would only fire if a
            // future regression introduced an out-of-band stamp, in
            // which case NaN propagation through the comparator is
            // the right surfacing — tolerance.passes() treats NaN
            // as a hard fail.
            debug_assert!(
                psd_v2_per_hz >= 0.0 && psd_v2_per_hz.is_finite(),
                "PSD must be non-negative and finite before sqrt(): got {psd_v2_per_hz}"
            );
            psd_v2_per_hz.sqrt()
        })
        .collect();

    // ---- Then ----------------------------------------------------------
    // [Then-1] every noise spectral-density point matches the
    //          Golden Reference within the tolerance envelope.
    // [Then-2] Conformance is reported as "pass".
    //
    // Both clauses are witnessed by the same `compare()` call: the
    // `verdict == Pass` path is reachable only when every point
    // passed (`n_failed_variables == 0`).
    let report = compare(
        &golden,
        [(OUTPUT_VARIABLE, actual_amplitudes.as_slice())],
        tolerance,
        16,
    );

    // Aggregate diagnostic surfacing under `cargo test -- --nocapture`.
    // Not asserted as a strict floor (that would couple the test to
    // faer's interior precision); the verdict assertion below is the
    // load-bearing check.
    let worst_var = report
        .variables
        .iter()
        .find(|s| s.name == OUTPUT_VARIABLE)
        .expect("the synthesized golden declared exactly onoise_spectrum");
    eprintln!(
        "noise-conformance scenario witness: \
         analytic onoise = {analytic_amp:.6e} V/√Hz, \
         worst margin = {:.6e} V/√Hz at point [{}] (f = {} Hz), \
         {} of {} points failed",
        worst_var.worst_margin,
        worst_var.worst_point,
        if worst_var.worst_point == usize::MAX {
            f64::NAN
        } else {
            data.frequencies_hz[worst_var.worst_point]
        },
        worst_var.n_failures,
        worst_var.n_points,
    );

    assert_eq!(
        report.verdict,
        ConformanceVerdict::Pass,
        "Then-2 violated: expected Conformance \"pass\", got {:?}. \
         Worst variable: {}; worst margin: {:.6e} V/√Hz at point [{}]; \
         {} of {} points failed.",
        report.verdict,
        report.worst_variable,
        report.worst_margin,
        worst_var.worst_point,
        report.n_failed_variables,
        report.n_variables,
    );
    assert_eq!(
        report.n_failed_variables, 0,
        "Then-1 violated: at least one frequency point failed the envelope"
    );
    assert_eq!(
        worst_var.n_failures, 0,
        "Then-1 violated: per-point failures present"
    );
    // Every point inside envelope means `worst_margin >= 0` per the
    // comparator's sign convention (see `Tolerance::margin` docs).
    assert!(
        worst_var.worst_margin >= 0.0,
        "verdict-margin contract violated: Pass with negative worst_margin {:.6e}",
        worst_var.worst_margin
    );
}

// =============================================================================
// Negative witness — the comparator *does* detect violations
// =============================================================================

/// Counter-witness for [Then-1]/[Then-2]: confirm the harness wired
/// into this test would actually flip the verdict to `Fail` when the
/// solver disagrees with the golden by more than the envelope.
///
/// Without this, a permanently-broken assertion (e.g., a comparator
/// that always returns `Pass`) could let the positive scenario test
/// pass spuriously. The Gherkin doesn't pin this clause explicitly,
/// but ADR-0008's "single outlier node does not cause a global
/// failure but is *reported*" consequence is load-bearing for the
/// conformance contract, so we witness the reporting path too.
#[test]
fn noise_conformance_flags_out_of_envelope_violations() {
    let (flat, graph, system, out_id) = build_sky130_resistor_probe();
    let frequencies_hz = log_sweep_hz(F_MIN_HZ, F_MAX_HZ, POINTS_PER_DECADE);
    let analytic_amp = analytic_amplitude_v_per_sqrt_hz(R1_OHMS, TEMPERATURE_K);
    let golden_amplitudes: Vec<f64> = vec![analytic_amp; frequencies_hz.len()];

    let raw_body = synthesize_ngspice_noise_raw(&frequencies_hz, &golden_amplitudes);
    let raw_path = write_temp_fixture("sky130-onoise-violation.raw", &raw_body);
    let golden = load_ngspice_ascii(&raw_path).expect("parse golden");

    // Run the analysis honestly, then perturb a single sample 10 %
    // above the analytic value — far outside the 2 % envelope.
    // 10 % of ~12.87 nV/√Hz is ~1.29 nV/√Hz, larger than the 1 nV/√Hz
    // absolute floor, so the relative term dominates and the
    // envelope is exceeded.
    let result = noise_analysis(NoiseAnalysisRequest {
        dc_status: synthetic_converged_status(),
        system: &system,
        structure: &flat,
        graph: &graph,
        frequencies_hz: &frequencies_hz,
        output: out_id,
        temperature_k: TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    })
    .expect("noise_analysis must succeed");
    let data = result
        .data()
        .expect("Ok path expected for converged purely-linear fixture");
    let mut actual_amplitudes: Vec<f64> = data
        .spectral_density_v2_per_hz
        .iter()
        .map(|&p| p.sqrt())
        .collect();
    let perturb_idx = actual_amplitudes.len() / 2;
    actual_amplitudes[perturb_idx] *= 1.10;

    let tolerance = AnalysisKind::NoiseSpectralDensity.default_tolerance();
    let report = compare(
        &golden,
        [(OUTPUT_VARIABLE, actual_amplitudes.as_slice())],
        tolerance,
        16,
    );

    assert_eq!(
        report.verdict,
        ConformanceVerdict::Fail,
        "comparator must Fail when one point is 10 % outside envelope"
    );
    let v = &report.variables[0];
    assert_eq!(v.name, OUTPUT_VARIABLE);
    assert_eq!(
        v.n_failures, 1,
        "exactly one perturbed sample must fail; got {} failures",
        v.n_failures
    );
    assert_eq!(
        v.worst_point, perturb_idx,
        "worst point must be the perturbed sample index"
    );
    assert!(
        v.worst_margin < 0.0,
        "failure must have negative margin; got {:.6e}",
        v.worst_margin
    );
    // The one failure record must report the perturbed frequency and
    // a reference equal to the analytic value (with the actual at
    // 1.10× the analytic).
    let failure = v
        .failures
        .first()
        .expect("at least one PointFailure recorded");
    assert_eq!(failure.point, perturb_idx);
    assert!(
        (failure.reference - analytic_amp).abs() <= 1e-12 * analytic_amp.abs(),
        "failure reference = {:.6e}, want {:.6e}",
        failure.reference,
        analytic_amp
    );
}
