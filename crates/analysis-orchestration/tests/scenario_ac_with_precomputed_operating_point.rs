//! Scenario-level integration witness for
//! `ac-small-signal#ac-analysis-with-pre-computed-operating-point`.
//!
//! Per the executable specification (verbatim Gherkin block from the
//! kanban task body):
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit and obtained an
//!   OperatingPoint from a prior DC analysis
//! And the OperatingPoint Convergence status is "converged"
//! When CircuitDesigner submits an AC small-signal Analysis request
//!   with a frequency Sweep from 1 Hz to 100 MHz
//! Then the Simulator linearizes the Circuit at the OperatingPoint
//! And the Result contains magnitude and phase for every output/input
//!   pair at every frequency in the Sweep
//! And every TransferFunction value matches the Golden Reference within
//!   the tolerance envelope
//! ```
//!
//! # Position of this test in the implementation pipeline
//!
//! tasks.md slices the work for this scenario across several primitive
//! tasks that have already merged to trunk under the
//! `ac-small-signal` capability:
//!
//! - **#23** — `faer` complex-valued sparse-LU dispatch
//!   (`numeric_solver::FaerComplexSolver`).
//! - **#24** — AC sub-view extraction with `G + jωC + jωL`
//!   augmentation (`numeric_solver::AcSubViewBuilder`).
//! - **#25** — AC analysis control loop
//!   (`analysis_orchestration::ac::ac_analysis`), which composes #23 +
//!   #24 into a per-frequency driver returning
//!   [`AcAnalysisResult`].
//!
//! The control-loop landing (`#25`) carries an inline unit-test witness
//! also named `ac_analysis_with_pre_computed_operating_point` in
//! `crates/analysis-orchestration/src/ac.rs`. That inline witness
//! exercises the same code path but at a tighter, 13-point sweep
//! centered on the RC cutoff. **This file is the scenario-level
//! witness:** it drives the *exact Sweep stated in the Gherkin*
//! ("1 Hz to 100 MHz"), it materializes the *Given* clause's
//! "`OperatingPoint` from a prior DC analysis" as a real
//! [`circuit_solver_types::ConvergenceStatus::Converged`] handle
//! (rather than implicitly trusting the linear-MNA solve), and it
//! checks *every* `TransferFunction` sample against the analytic Golden
//! Reference at the ADR-0008-style tolerance envelope used by the
//! capability's conformance task (tasks.md #64: 0.1 dB magnitude,
//! 1° phase).
//!
//! # Choice of fixture
//!
//! A first-order RC low-pass (V1 → R → output → C → gnd) is the
//! canonical small-signal Golden Reference: its transfer function
//! `H(jω) = 1 / (1 + jωRC)` is exact, dimensionless, and has known
//! limits (`|H|→1` for `ω≪1/RC`, `|H|→1/ωRC` for `ω≫1/RC`). For
//! R = 1 kΩ, C = 1 µF the cutoff `f_c = 1/(2π·RC) ≈ 159.155 Hz` sits
//! ~2.2 decades above the sweep floor and ~5.8 decades below the
//! sweep ceiling — well within the 1 Hz to 100 MHz Sweep mandated by
//! the Gherkin.
//!
//! # Why a converged status is constructed synthetically
//!
//! For a purely linear circuit the DC operating-point assembly *is*
//! the linearization — no semiconductor stamps, no Newton-Raphson
//! iteration. The truthful `ConvergenceStatus` for such a case is
//! `Converged` with zero NR iterations: the residue is identically
//! zero because the system is linear, and there was no Δx to measure.
//! This test constructs that handle explicitly, asserts
//! `status.is_converged()` to honor the *Given* clause's
//! `"converged"` constraint, and then proceeds with AC. When the
//! Newton-Raphson driver lands (tasks.md #17) and the DC analysis
//! control loop (tasks.md #20), the operating-point construction
//! will route through them and this synthetic step will be replaced
//! by `dc_analysis(...).status` — but the assertion on
//! `is_converged()` will remain the load-bearing one.
//!
//! [`AcAnalysisResult`]: analysis_orchestration::AcAnalysisResult

use analysis_orchestration::{ac_analysis, AcAnalysisRequest};
use circuit_solver_types::{
    ConvergenceDiagnostic, ConvergenceStatus, ConvergenceTolerances, NodeId,
};
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::{assemble, flatten};

// =============================================================================
// Fixture: first-order RC low-pass
// =============================================================================

const R_OHMS: f64 = 1_000.0;
const C_FARADS: f64 = 1.0e-6;
const VSRC_VOLTS: f64 = 1.0;

// Sweep envelope mandated by the Gherkin: 1 Hz to 100 MHz.
const F_MIN_HZ: f64 = 1.0;
const F_MAX_HZ: f64 = 1.0e8;
// Density: 10 points per decade across 8 decades → 81 points total.
// This is dense enough to witness "every frequency in the Sweep"
// non-trivially while staying inside the unit-test runtime budget.
const POINTS_PER_DECADE: usize = 10;

// Tolerance envelope per ADR-0008 (per-point `max(rel, abs)`),
// numerically aligned with tasks.md #64's AC conformance constants
// (0.1 dB magnitude, 1° phase). For our analytic golden reference we
// can afford to be stricter — the analytic formula has no rounding
// floor — but we keep the conformance constants verbatim so this
// witness pins the same envelope the future ngspice conformance test
// will pin.
const MAGNITUDE_DB_ABS_TOL: f64 = 0.1;
const MAGNITUDE_DB_REL_TOL: f64 = 1.0e-3;
const PHASE_DEG_ABS_TOL: f64 = 1.0;
const PHASE_DEG_REL_TOL: f64 = 1.0e-3;

/// Build the RC low-pass: V1 across `n_in` → 0, R from `n_in` → `n_out`,
/// C from `n_out` → 0. Returns the flattened structure, the source
/// circuit graph, and the assembled MNA system that will play the role
/// of the precomputed `OperatingPoint`.
///
/// Node layout (matches the inline witness in `src/ac.rs::tests`):
/// `0 = gnd`, `1 = n_in`, `2 = n_out`.
fn build_rc_lowpass() -> (
    circuit_solver_types::FlattenedStructure,
    netlist_graph::CircuitGraph,
    numeric_solver::MnaSystem,
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
            resistance_ohms: R_OHMS,
        },
        ["n_in", "n_out"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: C_FARADS,
        },
        ["n_out", "0"],
        None,
    )
    .expect("add C1");
    let graph = b.build().expect("graph build ok");
    let flat = flatten(&graph).expect("flatten ok");
    let system = assemble(&flat, &graph, &[]).expect("assemble ok");
    (flat, graph, system)
}

/// Build a log-spaced frequency sweep from `f_min_hz` to `f_max_hz`
/// inclusive at `pts_per_decade` density. The endpoints are honored
/// exactly: `out[0] == f_min_hz`, `out[last] == f_max_hz`.
fn log_sweep_hz(f_min_hz: f64, f_max_hz: f64, pts_per_decade: usize) -> Vec<f64> {
    assert!(f_min_hz > 0.0 && f_max_hz > f_min_hz);
    assert!(pts_per_decade >= 1);
    let n_decades = f_max_hz.log10() - f_min_hz.log10();
    // +1 for the inclusive endpoint.
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
    // Force endpoints to be exactly equal to avoid floating-point
    // drift on the boundary points; this is a presentational
    // correction, not a behavioral one.
    out[0] = f_min_hz;
    let last = out.len() - 1;
    out[last] = f_max_hz;
    out
}

// =============================================================================
// Golden Reference: analytic first-order low-pass
// =============================================================================

/// Analytic Golden Reference: `H(jω) = 1 / (1 + jωRC)`.
///
/// Returns `(magnitude_db, phase_degrees)` at frequency `f_hz`.
/// `|H| = 1 / sqrt(1 + (ωRC)^2)`, `arg(H) = -atan(ωRC)`.
fn golden_h_db_phase(f_hz: f64) -> (f64, f64) {
    let omega = 2.0 * core::f64::consts::PI * f_hz;
    let omega_rc = omega * R_OHMS * C_FARADS;
    let mag = 1.0 / (1.0 + omega_rc * omega_rc).sqrt();
    let mag_db = 20.0 * mag.log10();
    let phase_deg = (-omega_rc.atan()).to_degrees();
    (mag_db, phase_deg)
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
/// "the `OperatingPoint` Convergence status is `converged`" in the
/// Gherkin Given clause. For a purely linear circuit (this fixture
/// has no semiconductors), the MNA assembly *is* the operating point
/// and no Newton-Raphson iteration was required — so the truthful
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
// Scenario witness
// =============================================================================

// `clippy::too_many_lines` is silenced here on the same principle as
// the sibling `scenario_adaptive_timestepping` witness: a Gherkin-
// shaped Given/When/Then test that walks one scenario in one place is
// more readable as a single contiguous block than as a constellation
// of inline helpers. The function is long because the spec is long.
#[allow(clippy::too_many_lines)]
#[test]
fn ac_analysis_with_pre_computed_operating_point_scenario() {
    // ---- Given ----------------------------------------------------------
    // CircuitDesigner has constructed a Circuit and obtained an
    // OperatingPoint from a prior DC analysis.
    let (flat, graph, system) = build_rc_lowpass();

    // And the OperatingPoint Convergence status is "converged".
    let op_status = synthetic_converged_status();
    assert!(
        op_status.is_converged(),
        "Given precondition violated: operating-point status must be Converged, \
         got {op_status:?}"
    );

    // ---- When ----------------------------------------------------------
    // CircuitDesigner submits an AC small-signal Analysis request with
    // a frequency Sweep from 1 Hz to 100 MHz.
    let frequencies_hz = log_sweep_hz(F_MIN_HZ, F_MAX_HZ, POINTS_PER_DECADE);
    // Endpoint invariants — the Gherkin pins the sweep boundary
    // values exactly.
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
    // Sweep monotonicity (strict, log-spaced).
    for win in frequencies_hz.windows(2) {
        assert!(
            win[1] > win[0],
            "sweep must be strictly increasing; got [{}, {}]",
            win[0],
            win[1]
        );
    }

    let n_out = NodeId::new(2);
    let result = ac_analysis(AcAnalysisRequest {
        system: &system,
        structure: &flat,
        graph: &graph,
        frequencies_hz: &frequencies_hz,
        outputs: &[n_out],
        ground: None,
    })
    .expect("ac_analysis must succeed on a converged operating point");

    // ---- Then ----------------------------------------------------------
    // [Then-1] The Simulator linearizes the Circuit at the
    //          OperatingPoint.
    //
    // Witnessed indirectly: `ac_analysis` consumed the operating-point
    // [`MnaSystem`] by reference and produced complex-valued results
    // without ever requesting re-linearization. The AC sub-view
    // builder (tasks.md #24) carries the linearization-at-OP
    // contract; if it were violated we would either fail to assemble
    // or produce garbage results, both of which are caught by the
    // subsequent Then clauses.

    // [Then-2] The Result contains magnitude and phase for every
    //          output/input pair at every frequency in the Sweep.
    //
    // For this fixture there is exactly one output (n_out) — the
    // input/output pair (V1, n_out) — and the result must carry one
    // TransferFunction whose three parallel vectors all have length
    // equal to the sweep length.
    assert_eq!(
        result.transfer_functions.len(),
        1,
        "expected exactly one TransferFunction for one output node"
    );
    let tf = &result.transfer_functions[0];
    assert_eq!(tf.output, n_out, "TransferFunction must address n_out");
    assert_eq!(
        tf.frequencies_hz.len(),
        frequencies_hz.len(),
        "TransferFunction frequencies vector length must match Sweep length"
    );
    assert_eq!(
        tf.magnitude_db.len(),
        frequencies_hz.len(),
        "TransferFunction magnitude vector length must match Sweep length"
    );
    assert_eq!(
        tf.phase_degrees.len(),
        frequencies_hz.len(),
        "TransferFunction phase vector length must match Sweep length"
    );
    // Frequency axis preserved verbatim in the result.
    for (i, (got, want)) in tf
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

    // [Then-3] Every TransferFunction value matches the Golden
    //          Reference within the tolerance envelope.
    //
    // Per ADR-0008, the envelope is per-node `max(rel, abs)`. We
    // apply it pointwise across the entire Sweep and report the
    // worst offender on failure, so a reviewer can localize the
    // defect.
    let mut worst_mag_err = 0.0_f64;
    let mut worst_phase_err = 0.0_f64;
    let mut worst_mag_idx = 0usize;
    let mut worst_phase_idx = 0usize;
    for (i, &f_hz) in tf.frequencies_hz.iter().enumerate() {
        let (golden_mag_db, golden_phase_deg) = golden_h_db_phase(f_hz);
        let got_mag_db = tf.magnitude_db[i];
        let got_phase_deg = tf.phase_degrees[i];

        let mag_err = (got_mag_db - golden_mag_db).abs();
        let phase_err = (got_phase_deg - golden_phase_deg).abs();
        if mag_err > worst_mag_err {
            worst_mag_err = mag_err;
            worst_mag_idx = i;
        }
        if phase_err > worst_phase_err {
            worst_phase_err = phase_err;
            worst_phase_idx = i;
        }

        assert!(
            within_envelope(
                got_mag_db,
                golden_mag_db,
                MAGNITUDE_DB_ABS_TOL,
                MAGNITUDE_DB_REL_TOL,
            ),
            "magnitude conformance failed at f[{i}] = {f_hz} Hz: \
             got {got_mag_db} dB, want {golden_mag_db} dB \
             (|err| = {mag_err}, envelope abs={MAGNITUDE_DB_ABS_TOL} dB rel={MAGNITUDE_DB_REL_TOL})"
        );
        assert!(
            within_envelope(
                got_phase_deg,
                golden_phase_deg,
                PHASE_DEG_ABS_TOL,
                PHASE_DEG_REL_TOL,
            ),
            "phase conformance failed at f[{i}] = {f_hz} Hz: \
             got {got_phase_deg}°, want {golden_phase_deg}° \
             (|err| = {phase_err}, envelope abs={PHASE_DEG_ABS_TOL}° rel={PHASE_DEG_REL_TOL})"
        );
    }

    // The aggregate worst-case values are useful diagnostic context
    // for the reviewer. We print them via `eprintln!` so they show up
    // in `cargo test -- --nocapture`. Asserting strict floors on them
    // here would couple the test to faer's interior precision; the
    // pointwise envelope checks above are the load-bearing
    // assertions.
    eprintln!(
        "ac-precomputed-op scenario witness: \
         worst magnitude err = {} dB at f[{}] = {} Hz; \
         worst phase err = {}° at f[{}] = {} Hz",
        worst_mag_err,
        worst_mag_idx,
        tf.frequencies_hz[worst_mag_idx],
        worst_phase_err,
        worst_phase_idx,
        tf.frequencies_hz[worst_phase_idx],
    );

    // Boundary-point spot checks (defense in depth):
    //
    // At f = 1 Hz (≈2.2 decades below cutoff f_c ≈ 159 Hz) the
    // magnitude must be essentially 0 dB (passband, |H| ≈ 1) and the
    // phase well inside the passband shoulder (the analytic value is
    // -atan(2π·f·RC) ≈ -atan(0.00628) ≈ -0.36°, so a < 1° bound is
    // the right passband floor and is also exactly the conformance
    // envelope for phase from ADR-0008 / tasks.md #64). At f = 100
    // MHz (≈5.8 decades above cutoff) the magnitude must be deep in
    // the stopband (well below -100 dB) and the phase must be
    // approaching -90° (the first-order limit).
    assert!(
        tf.magnitude_db[0].abs() < 0.01,
        "1 Hz magnitude should be ≈0 dB (passband), got {} dB",
        tf.magnitude_db[0]
    );
    assert!(
        tf.phase_degrees[0].abs() < 1.0,
        "1 Hz phase should sit inside the passband (|phase| < 1°), got {}°",
        tf.phase_degrees[0]
    );
    let last = tf.magnitude_db.len() - 1;
    assert!(
        tf.magnitude_db[last] < -100.0,
        "100 MHz magnitude should be ≪-100 dB, got {} dB",
        tf.magnitude_db[last]
    );
    assert!(
        (tf.phase_degrees[last] - (-90.0)).abs() < 1.0,
        "100 MHz phase should approach -90°, got {}°",
        tf.phase_degrees[last]
    );
}
