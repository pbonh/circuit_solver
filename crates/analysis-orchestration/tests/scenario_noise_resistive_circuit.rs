//! Scenario-level integration witness for
//! `noise-spectral-density#noise-analysis-on-a-resistive-circuit`.
//!
//! Per the executable specification (verbatim Gherkin block from the
//! kanban task body):
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit containing only
//!   resistors and independent sources
//! And an OperatingPoint has been computed with Convergence status
//!   "converged"
//! When SimulationEngineer submits a noise spectral-density Analysis
//!   request for output node "out"
//! Then the Result contains thermal noise spectral density at every
//!   frequency in the Sweep
//! And the total output noise density at each frequency matches the
//!   theoretical 4kTR value within the tolerance envelope
//! ```
//!
//! # Position of this test in the implementation pipeline
//!
//! tasks.md slices the work for this scenario across several primitive
//! tasks that have already merged to trunk under the
//! `noise-spectral-density` capability:
//!
//! - **#23** — `faer` complex-valued sparse-LU dispatch
//!   (`numeric_solver::FaerComplexSolver`).
//! - **#24** — AC sub-view extraction with `G + jωC + jωL`
//!   augmentation (`numeric_solver::AcSubViewBuilder`).
//! - **#36** — intrinsic device noise source modeling, including
//!   resistor Johnson-Nyquist thermal noise
//!   (`device_modeling::noise::resistor_thermal_noise`).
//! - **#37** — noise analysis control loop
//!   (`analysis_orchestration::noise::noise_analysis`), which composes
//!   #23 + #24 + #36 into a per-frequency, per-source driver returning
//!   [`NoiseAnalysisResult`].
//!
//! The control-loop landing (`#37`) carries inline unit-test witnesses
//! in `crates/analysis-orchestration/src/noise.rs`, including
//! `spec_scenario_resistor_only_thermal_noise_matches_4ktr` (a 7-point
//! sweep) and `output_psd_is_white_across_decades_for_purely_resistive`
//! (a 7-point sweep spanning 10 decades).
//!
//! **This file is the scenario-level witness:** it walks the *exact*
//! Given / When / Then steps from the Gherkin block, with the
//! operating-point status materialized as an explicit
//! [`circuit_solver_types::ConvergenceStatus::Converged`] handle
//! (rather than implicitly trusting the linear-MNA solve), it submits
//! the noise analysis "for output node `out`" by name (the Gherkin
//! pins the node name verbatim, so the witness uses a builder node
//! named `"out"`), and it checks *every* PSD sample against the
//! analytic `4·k_B·T·R` Golden Reference at the ADR-0008-style tolerance
//! envelope used by the capability's conformance task (tasks.md #66:
//! 2 % relative / 1 nV/√Hz absolute, squared to V²/Hz space here).
//!
//! # Choice of fixture
//!
//! The canonical Johnson-Nyquist witness is *one resistor* whose
//! thermal-noise voltage PSD `S_V = 4·k_B·T·R` is read out across an
//! open-circuit port. The control-loop's inline witness selected the
//! topology `V1 → R1 → n_out → R2 (1 PΩ) → gnd` so the assembler
//! accepts the netlist (it requires at least one independent source
//! and rejects floating nodes per ADR-0009) while the open-circuit
//! AC behavior is preserved: V1 is an AC short, R2 → ∞ approximates
//! the open, and the noise voltage at `n_out` is dominated by R1's
//! thermal source driving R1 itself (the AC impedance from R1's port
//! to ground). The result is `S_V(f) ≈ 4·k_B·T·R1` at every f.
//!
//! This witness reuses that topology because (a) it is the only
//! v1-admissible netlist that exposes the bare 4kTR formula at a
//! named output node, and (b) the inline witness pinned a 1e-6
//! relative tolerance on the topology already — we tighten the spec
//! envelope to ngspice-conformance constants here so a regression
//! in the LU backend's interior precision still trips this test.
//!
//! # Why a converged status is constructed synthetically
//!
//! For a purely linear circuit (this fixture has no semiconductors)
//! the DC operating-point assembly *is* the linearization — no
//! Newton-Raphson iteration. The truthful `ConvergenceStatus` for
//! such a case is `Converged` with zero NR iterations: the residue
//! is identically zero because the system is linear, and there was
//! no Δx to measure. This test constructs that handle explicitly
//! and asserts `is_converged()` to honor the Gherkin *Given* clause's
//! `"converged"` constraint, mirroring the AC scenario witness for
//! `ac-analysis-with-pre-computed-operating-point`.
//!
//! [`NoiseAnalysisResult`]: analysis_orchestration::NoiseAnalysisResult

use analysis_orchestration::{noise_analysis, NoiseAnalysisRequest, NoiseAnalysisResult};
use circuit_solver_types::{
    ConvergenceDiagnostic, ConvergenceStatus, ConvergenceTolerances, NodeId,
};
use device_modeling::noise::{BOLTZMANN_J_PER_K, ROOM_TEMPERATURE_K};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::{assemble, flatten};

// =============================================================================
// Fixture: single resistor at an open-circuit port
// =============================================================================

/// The "resistor under test" whose thermal noise dominates the
/// output. 10 kΩ is a textbook choice for a Johnson-Nyquist witness:
/// large enough that the resulting PSD is comfortably above any
/// numerical floor, small enough that the integrated noise over a
/// reasonable bandwidth is in the µV RMS regime.
const R1_OHMS: f64 = 10_000.0;
/// The "infinite resistor" pulldown standing in for an open circuit
/// at `n_out`. 1 PΩ makes `R1 / R2 = 10⁻¹¹`, which puts R2's
/// contribution (which is `4kT R1² / R2`) 11 orders of magnitude
/// below R1's contribution (`4kT R1`) — well below any tolerance the
/// scenario pins.
const R2_OHMS: f64 = 1.0e15;
const VSRC_VOLTS: f64 = 1.0;

// Sweep envelope: 6 decades from 1 Hz to 1 MHz at 5 points per
// decade. The exact endpoints are not pinned by the Gherkin (it
// just says "every frequency in the Sweep"), so we choose a range
// that is wide enough to demonstrate frequency-independence of the
// white noise convincingly.
const F_MIN_HZ: f64 = 1.0;
const F_MAX_HZ: f64 = 1.0e6;
const POINTS_PER_DECADE: usize = 5;

// Tolerance envelope per ADR-0008 (per-point `max(rel, abs)`),
// numerically aligned with tasks.md #66's noise conformance constants
// (2 % relative / 1 nV/√Hz absolute). The conformance task targets
// voltage-noise *amplitude* density in nV/√Hz, whereas the control
// loop returns *power* spectral density in V²/Hz. Squaring the
// amplitude tolerance:
//   abs:    (1 nV/√Hz)² = 1e-18 V²/Hz
//   rel:    2 % amplitude → ≈4 % power (because (1 ± 0.02)² ≈ 1 ± 0.04)
// We use the squared values here so this witness pins the same
// envelope the future ngspice conformance test will pin.
const PSD_ABS_TOL_V2_PER_HZ: f64 = 1.0e-18;
const PSD_REL_TOL: f64 = 0.04;

/// Build the open-port resistive fixture.
///
/// Topology (matches `single_resistor_to_ground` in
/// `src/noise.rs::tests` so this scenario witness and the inline
/// unit witnesses agree on what they're measuring):
///
/// ```text
///   V1 (1 V)
///       │
///      n_in
///       │
///       R1 (10 kΩ)              ← "resistor under test"
///       │
///      n_out                    ← output port, named "out" per Gherkin
///       │
///       R2 (1 PΩ)               ← pulldown standing in for an open
///       │
///      gnd
/// ```
///
/// Returns the flattened structure, the source `CircuitGraph`, the
/// assembled MNA system that plays the role of the precomputed
/// operating point, and the `NodeId` of the `"out"` node.
fn build_open_port_resistor() -> (
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
    // Look up the `"out"` node by walking R1's terminals.
    // R1's pin order is `[n_in, out]`, so terminals[1] is `out`.
    let out_id = graph
        .elements()
        .iter()
        .find(|e| e.name().as_str() == "R1")
        .expect("R1 present in graph")
        .terminals()[1];
    (flat, graph, system, out_id)
}

/// Log-spaced inclusive sweep from `f_min_hz` to `f_max_hz` at
/// `pts_per_decade` density. Endpoints are honored exactly.
///
/// (Duplicated from `scenario_ac_with_precomputed_operating_point.rs`
/// because the two tests' tolerances and assertions are different
/// enough that hoisting the helper into a shared module would obscure
/// rather than clarify — each scenario witness wants to be a single
/// readable file with its Gherkin Given/When/Then walk in one place.)
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
// Golden Reference: textbook Johnson-Nyquist white noise
// =============================================================================

/// Analytic Golden Reference: `S_V = 4·k_B·T·R` (V²/Hz).
///
/// White noise — frequency-independent — for a single resistor seen
/// across its terminals at temperature `temperature_k`.
fn golden_4ktr(r_ohms: f64, temperature_k: f64) -> f64 {
    4.0 * BOLTZMANN_J_PER_K * temperature_k * r_ohms
}

/// Per-point `max(rel, abs)` tolerance envelope (ADR-0008 shape).
///
/// `|got - want| <= max(abs_tol, rel_tol * max(|got|, |want|))`.
fn within_envelope(got: f64, want: f64, abs_tol: f64, rel_tol: f64) -> bool {
    let err = (got - want).abs();
    let scale = got.abs().max(want.abs());
    err <= abs_tol.max(rel_tol * scale)
}

// =============================================================================
// "OperatingPoint" construction for the Given clause
// =============================================================================

/// Build the `ConvergenceStatus::Converged` handle that stands in for
/// "an `OperatingPoint` has been computed with Convergence status
/// `\"converged\"`" in the Gherkin Given clause. For a purely linear
/// circuit (this fixture has no semiconductors) the MNA assembly *is*
/// the operating point and no Newton-Raphson iteration was required
/// — the truthful diagnostic carries zero iterations and zero
/// residue.
fn synthetic_converged_status() -> ConvergenceStatus {
    ConvergenceStatus::Converged(ConvergenceDiagnostic {
        update_norm: 0.0,
        residue_norm: 0.0,
        iterations: 0,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    })
}

// =============================================================================
// Scenario witness
// =============================================================================

// `clippy::too_many_lines` is silenced here on the same principle as
// the sibling `scenario_ac_with_precomputed_operating_point` witness:
// a Gherkin-shaped Given/When/Then test that walks one scenario in
// one place is more readable as a single contiguous block than as a
// constellation of inline helpers.
#[allow(clippy::too_many_lines)]
#[test]
fn noise_analysis_on_a_resistive_circuit_scenario() {
    // ---- Given ----------------------------------------------------------
    // CircuitDesigner has constructed a Circuit containing only
    // resistors and independent sources.
    let (flat, graph, system, out_id) = build_open_port_resistor();

    // Witness the Given: every element in the constructed Circuit is
    // either a resistor or an independent source (no semiconductors,
    // no reactives) — the spec's "only resistors and independent
    // sources" precondition.
    for elem in graph.elements() {
        match elem.kind() {
            ElementKind::Resistor { .. } | ElementKind::VoltageSource { .. } => {}
            other => panic!(
                "Given precondition violated: element {} has kind {other:?}, \
                 expected only Resistor or VoltageSource",
                elem.name().as_str()
            ),
        }
    }

    // And an OperatingPoint has been computed with Convergence status
    // "converged".
    let op_status = synthetic_converged_status();
    assert!(
        op_status.is_converged(),
        "Given precondition violated: operating-point status must be Converged, \
         got {op_status:?}"
    );

    // ---- When ----------------------------------------------------------
    // SimulationEngineer submits a noise spectral-density Analysis
    // request for output node "out".
    let frequencies_hz = log_sweep_hz(F_MIN_HZ, F_MAX_HZ, POINTS_PER_DECADE);
    // Endpoint and monotonicity invariants on the Sweep itself.
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

    let result = noise_analysis(NoiseAnalysisRequest {
        dc_status: op_status,
        system: &system,
        structure: &flat,
        graph: &graph,
        frequencies_hz: &frequencies_hz,
        output: out_id,
        temperature_k: ROOM_TEMPERATURE_K,
        ground: None,
        semiconductor_noise: &[],
    })
    .expect("noise_analysis must succeed on a converged purely-resistive circuit");

    // ---- Then ----------------------------------------------------------
    // [Then-1] The Result contains thermal noise spectral density at
    //          every frequency in the Sweep.
    //
    // The control loop returns either `Ok(data)` with samples or
    // `Failed { dc_status }`. The Gherkin's Then clause pins the
    // success branch; assert it explicitly so a future regression
    // that returns `Failed` is caught at the witness rather than
    // silently producing an empty data vector.
    let data = match &result {
        NoiseAnalysisResult::Ok(d) => d,
        NoiseAnalysisResult::Failed { dc_status } => panic!(
            "expected Ok result on a converged operating point, \
             got Failed with dc_status={dc_status:?}"
        ),
    };
    assert_eq!(
        data.frequencies_hz.len(),
        frequencies_hz.len(),
        "Result must carry one PSD sample per Sweep frequency \
         (got {} samples, want {})",
        data.frequencies_hz.len(),
        frequencies_hz.len()
    );
    assert_eq!(
        data.spectral_density_v2_per_hz.len(),
        frequencies_hz.len(),
        "PSD vector length must match Sweep length \
         (got {} samples, want {})",
        data.spectral_density_v2_per_hz.len(),
        frequencies_hz.len()
    );
    // Frequency axis preserved verbatim in the result.
    for (i, (got, want)) in data
        .frequencies_hz
        .iter()
        .zip(frequencies_hz.iter())
        .enumerate()
    {
        assert!(
            (got - want).abs() <= 1e-12 * want.abs().max(1.0),
            "Sweep frequency mismatch at index {i}: got {got}, want {want}"
        );
    }
    // PSD non-negativity is a load-bearing invariant of the noise
    // analysis (sum of squared magnitudes times non-negative source
    // PSDs).
    for (i, &s_v) in data.spectral_density_v2_per_hz.iter().enumerate() {
        assert!(
            s_v >= 0.0 && s_v.is_finite(),
            "PSD must be non-negative and finite; got {s_v} at index {i} (f = {} Hz)",
            data.frequencies_hz[i]
        );
        assert!(
            s_v > 0.0,
            "thermal noise from a finite resistor must be strictly positive; \
             got {s_v} at index {i} (f = {} Hz)",
            data.frequencies_hz[i]
        );
    }

    // [Then-2] The total output noise density at each frequency
    //          matches the theoretical 4kTR value within the tolerance
    //          envelope.
    //
    // Golden Reference: S_V = 4·k_B·T·R1. R2's contribution (which
    // is ≈ 4·k_B·T·R1² / R2 ≈ R1 / R2 of R1's, i.e. ~1e-11 relative)
    // is far below the conformance envelope and is therefore folded
    // into the "≈ 4kTR1" Golden Reference rather than subtracted.
    let expected = golden_4ktr(R1_OHMS, ROOM_TEMPERATURE_K);
    let mut worst_err = 0.0_f64;
    let mut worst_idx = 0usize;
    for (i, &s_v) in data.spectral_density_v2_per_hz.iter().enumerate() {
        let err = (s_v - expected).abs();
        if err > worst_err {
            worst_err = err;
            worst_idx = i;
        }
        assert!(
            within_envelope(s_v, expected, PSD_ABS_TOL_V2_PER_HZ, PSD_REL_TOL),
            "PSD conformance failed at f[{i}] = {} Hz: got {s_v:.6e} V²/Hz, \
             want {expected:.6e} V²/Hz (|err| = {err:.6e}, envelope abs={PSD_ABS_TOL_V2_PER_HZ:.1e} V²/Hz rel={PSD_REL_TOL})",
            data.frequencies_hz[i]
        );
    }

    // Aggregate diagnostic context for the reviewer. Printed via
    // `eprintln!` so it surfaces under `cargo test -- --nocapture`.
    // Not asserted as a strict floor: that would couple the test to
    // faer's interior precision; the pointwise envelope checks above
    // are the load-bearing assertions.
    eprintln!(
        "noise-resistive scenario witness: \
         expected 4kTR = {expected:.6e} V²/Hz, \
         worst PSD err = {worst_err:.6e} V²/Hz at f[{worst_idx}] = {} Hz",
        data.frequencies_hz[worst_idx]
    );

    // Defense in depth: thermal noise must be *white* — every PSD
    // sample agrees with every other to high precision. We anchor
    // on the first sample and check the rest. The Gherkin doesn't
    // pin this explicitly, but "thermal noise spectral density at
    // every frequency" plus "the theoretical 4kTR value" implies
    // frequency-independence, since 4kTR has no `f` in it. If a
    // future regression introduces an `f`-dependent stamp into a
    // resistor's noise contribution, this is the assertion that
    // would catch it.
    let first = data.spectral_density_v2_per_hz[0];
    for (i, &s) in data.spectral_density_v2_per_hz.iter().enumerate().skip(1) {
        let drift = (s - first).abs();
        let scale = s.abs().max(first.abs()).max(1e-300);
        assert!(
            drift / scale <= 1.0e-9,
            "white-noise invariant violated at f[{i}] = {} Hz: \
             sample {s:.6e} drifts from first sample {first:.6e} by {drift:.6e} \
             ({:.2e} relative)",
            data.frequencies_hz[i],
            drift / scale
        );
    }
}
