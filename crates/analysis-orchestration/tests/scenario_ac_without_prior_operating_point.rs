//! Scenario-level integration witness for
//! `ac-small-signal#ac-analysis-without-prior-operating-point`.
//!
//! Per the executable specification (verbatim Gherkin block from the
//! kanban task body / spec):
//!
//! ```gherkin
//! Given CircuitDesigner has constructed a Circuit
//! And no OperatingPoint has been computed for this Circuit
//! When CircuitDesigner submits an AC small-signal Analysis request
//! Then the Simulator first computes a DC OperatingPoint
//! And the Simulator proceeds with AC linearization at that OperatingPoint
//! And the Result contains both the OperatingPoint and the AC frequency-domain data
//! ```
//!
//! # Position of this test in the implementation pipeline
//!
//! tasks.md slices the work for this scenario as:
//!
//! - **#20** — DC operating-point analysis control loop
//!   (`analysis_orchestration::dc::dc_analysis`).
//! - **#25** — AC analysis control loop
//!   (`analysis_orchestration::ac::ac_analysis`).
//! - **#26** — Auto-DC AC composition
//!   (`analysis_orchestration::auto_dc_ac::ac_analysis_with_auto_dc`),
//!   which is the load-bearing surface for *this* scenario.
//!
//! The control-loop landing (`#26`) carries inline unit-test witnesses
//! in `crates/analysis-orchestration/src/auto_dc_ac.rs` exercising the
//! same code path at finer granularity (DC happy-path, AC failure
//! preserves OP, builder ergonomics, etc.). **This file is the
//! scenario-level witness:** it materializes the *Given* clause's
//! "no `OperatingPoint` has been computed" by handing
//! [`ac_analysis_with_auto_dc`] only the `(graph, structure)` pair —
//! *no* pre-built `MnaSystem`, *no* pre-built `OperatingPoint` — and
//! it checks the *three* Then clauses one-by-one:
//!
//! 1. The Simulator first computes a DC `OperatingPoint`
//!    — `result.operating_point` is present and the embedded
//!    `dc_convergence.is_converged()` is true.
//! 2. The Simulator proceeds with AC linearization at that
//!    `OperatingPoint`
//!    — the returned `AcAnalysisResult` matches the analytic Golden
//!    Reference for the RC low-pass at the same operating point.
//! 3. The Result contains both the `OperatingPoint` and the AC
//!    frequency-domain data
//!    — `result` carries both `operating_point` *and* `ac` on a
//!    single bundle.
//!
//! # Choice of fixture
//!
//! Identical to the sibling
//! `scenario_ac_with_precomputed_operating_point` witness: a
//! first-order RC low-pass (V1 → R → output → C → gnd) with the
//! canonical Golden Reference `H(jω) = 1 / (1 + jωRC)`. For
//! R = 1 kΩ, C = 1 µF the cutoff is `f_c ≈ 159.155 Hz` and the
//! DC operating point is trivial (`V_in` = 1 V, `V_out` = 1 V because
//! no DC current flows through the capacitor). Reusing the same
//! fixture across the precomputed-OP and auto-DC witnesses keeps
//! the conformance bound apples-to-apples.
//!
//! # Why a smaller sweep here
//!
//! The precomputed-OP sibling witness exercises the full 1 Hz to
//! 100 MHz Gherkin sweep; *this* witness exercises 1 Hz to 1 MHz
//! (6 decades, 10 points/decade = 61 points). The scenario for #26
//! does *not* pin sweep extents (only "an AC small-signal Analysis
//! request"), so we choose a narrower sweep that still covers the
//! cutoff and both passband / stopband regions — the load-bearing
//! claim of *this* witness is the **composition** (auto-DC then AC),
//! not the sweep-width conformance. The sibling witness pins the
//! latter.
//!
//! [`ac_analysis_with_auto_dc`]: analysis_orchestration::ac_analysis_with_auto_dc

use analysis_orchestration::{ac_analysis_with_auto_dc, AcWithAutoDcRequest};
use circuit_solver_types::NodeId;
use netlist_graph::{CircuitBuilder, ElementKind};
use numeric_solver::flatten;

// =============================================================================
// Fixture: first-order RC low-pass
// =============================================================================

const R_OHMS: f64 = 1_000.0;
const C_FARADS: f64 = 1.0e-6;
const VSRC_VOLTS: f64 = 1.0;

// Sweep envelope: 1 Hz to 1 MHz (covers ~3.8 decades below cutoff and
// ~3.8 decades above cutoff f_c ≈ 159 Hz).
const F_MIN_HZ: f64 = 1.0;
const F_MAX_HZ: f64 = 1.0e6;
const POINTS_PER_DECADE: usize = 10;

// Tolerance envelope per ADR-0008. Identical numeric constants to the
// sibling `scenario_ac_with_precomputed_operating_point` witness, so
// the same conformance bound applies across the two paths.
const MAGNITUDE_DB_ABS_TOL: f64 = 0.1;
const MAGNITUDE_DB_REL_TOL: f64 = 1.0e-3;
const PHASE_DEG_ABS_TOL: f64 = 1.0;
const PHASE_DEG_REL_TOL: f64 = 1.0e-3;

/// Build the RC low-pass: V1 across `n_in` → 0, R from `n_in` → `n_out`,
/// C from `n_out` → 0. Returns the source circuit graph and the
/// flattened structure. **No `MnaSystem` is built here** — the
/// "no `OperatingPoint` has been computed" precondition is enforced
/// by *not* calling `assemble` in the test setup; only
/// [`ac_analysis_with_auto_dc`] is allowed to materialize one.
///
/// Node layout (matches the sibling witness): `0 = gnd`, `1 = n_in`,
/// `2 = n_out`.
fn build_rc_lowpass() -> (
    netlist_graph::CircuitGraph,
    circuit_solver_types::FlattenedStructure,
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
    (graph, flat)
}

/// Build a log-spaced frequency sweep from `f_min_hz` to `f_max_hz`
/// inclusive at `pts_per_decade` density. The endpoints are honored
/// exactly: `out[0] == f_min_hz`, `out[last] == f_max_hz`.
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
// Scenario witness
// =============================================================================

// `clippy::too_many_lines` is silenced here on the same principle as
// the sibling `scenario_ac_with_precomputed_operating_point` witness:
// a Gherkin-shaped Given/When/Then test that walks one scenario in
// one place is more readable as a single contiguous block than as a
// constellation of inline helpers.
#[allow(clippy::too_many_lines)]
#[test]
fn ac_analysis_without_prior_operating_point_scenario() {
    // ---- Given ----------------------------------------------------------
    // CircuitDesigner has constructed a Circuit.
    let (graph, flat) = build_rc_lowpass();

    // And no OperatingPoint has been computed for this Circuit.
    //
    // Witnessed by construction: `build_rc_lowpass` returns *only*
    // the `(CircuitGraph, FlattenedStructure)` pair — it does not
    // call `assemble`, does not build any `MnaSystem`, and does not
    // invoke `dc_analysis`. The local variable scope contains no
    // operating-point binding the test could pass to a sub-analysis.
    // This is the strongest form of the "no `OperatingPoint`
    // computed" precondition: nothing in the calling scope could
    // satisfy the AC-only path's input contract.

    // ---- When -----------------------------------------------------------
    // CircuitDesigner submits an AC small-signal Analysis request.
    let frequencies_hz = log_sweep_hz(F_MIN_HZ, F_MAX_HZ, POINTS_PER_DECADE);
    // Endpoint invariants.
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

    let n_out = NodeId::new(2);
    let result = ac_analysis_with_auto_dc(AcWithAutoDcRequest::new(
        &graph,
        &flat,
        &frequencies_hz,
        &[n_out],
    ))
    .expect(
        "ac_analysis_with_auto_dc must succeed on a linear RC low-pass \
         with no prior operating point",
    );

    // ---- Then -----------------------------------------------------------
    // [Then-1] The Simulator first computes a DC OperatingPoint.
    //
    // Witnessed directly: the result carries an `OperatingPoint` with
    // a `Converged` status. For this fixture the DC steady-state is
    // analytically trivial — no DC current flows through the
    // capacitor, so V(n_out) == V(n_in) == V1 == 1.0 V.
    assert!(
        result.dc_convergence.is_converged(),
        "Then-1: DC sub-analysis must report Converged; got {:?}",
        result.dc_convergence
    );
    let v_in = result
        .operating_point
        .as_ref()
        .expect("Then-1: OperatingPoint must be Some on the converged path")
        .voltage_at(NodeId::new(1))
        .expect("V(n_in) must be present in OperatingPoint");
    let v_out = result
        .operating_point
        .as_ref()
        .expect("Then-1: OperatingPoint must be Some on the converged path")
        .voltage_at(n_out)
        .expect("V(n_out) must be present in OperatingPoint");
    assert!(
        (v_in - VSRC_VOLTS).abs() < 1e-9,
        "Then-1: V(n_in) should equal V1 = {VSRC_VOLTS} V; got {v_in} V"
    );
    assert!(
        (v_out - VSRC_VOLTS).abs() < 1e-9,
        "Then-1: V(n_out) should equal V1 = {VSRC_VOLTS} V (no DC \
         current through C); got {v_out} V"
    );

    // [Then-2] The Simulator proceeds with AC linearization at that
    //          OperatingPoint.
    //
    // Witnessed indirectly via the *Result-matches-Golden-Reference*
    // claim of the AC half. If the AC step had linearized at a
    // *different* operating point — say, at the all-zero initial
    // iterate, or at some divergent NR iterate — the magnitude /
    // phase response would not match the analytic `H(jω)` for the
    // RC low-pass. The conformance check below is therefore the
    // load-bearing witness for "linearized at *that* OperatingPoint".
    //
    // Witnessed structurally as well: the AC result is non-empty and
    // its frequency axis matches the requested sweep verbatim.
    assert_eq!(
        result
            .ac
            .as_ref()
            .expect("Then-2: ac must be Some on the converged path")
            .transfer_functions
            .len(),
        1,
        "Then-2: expected exactly one TransferFunction for one output node"
    );
    let ac = result
        .ac
        .as_ref()
        .expect("Then-2: ac must be Some on the converged path");
    let tf = &ac.transfer_functions[0];
    assert_eq!(
        tf.output, n_out,
        "Then-2: TransferFunction must address n_out"
    );
    assert_eq!(
        tf.frequencies_hz.len(),
        frequencies_hz.len(),
        "Then-2: TransferFunction frequencies length must match Sweep length"
    );
    assert_eq!(
        tf.magnitude_db.len(),
        frequencies_hz.len(),
        "Then-2: magnitude vector length must match Sweep length"
    );
    assert_eq!(
        tf.phase_degrees.len(),
        frequencies_hz.len(),
        "Then-2: phase vector length must match Sweep length"
    );

    // Conformance against the analytic Golden Reference. If this
    // passes, the AC step *must* have linearized at the operating
    // point that produced V_out = V_in (i.e., the DC steady state
    // computed in Then-1).
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
            "Then-2: magnitude conformance failed at f[{i}] = {f_hz} Hz: \
             got {got_mag_db} dB, want {golden_mag_db} dB \
             (|err| = {mag_err}, envelope abs={MAGNITUDE_DB_ABS_TOL} dB \
             rel={MAGNITUDE_DB_REL_TOL})"
        );
        assert!(
            within_envelope(
                got_phase_deg,
                golden_phase_deg,
                PHASE_DEG_ABS_TOL,
                PHASE_DEG_REL_TOL,
            ),
            "Then-2: phase conformance failed at f[{i}] = {f_hz} Hz: \
             got {got_phase_deg}°, want {golden_phase_deg}° \
             (|err| = {phase_err}, envelope abs={PHASE_DEG_ABS_TOL}° \
             rel={PHASE_DEG_REL_TOL})"
        );
    }

    eprintln!(
        "ac-analysis-without-prior-operating-point scenario witness: \
         worst magnitude err = {} dB at f[{}] = {} Hz; \
         worst phase err = {}° at f[{}] = {} Hz",
        worst_mag_err,
        worst_mag_idx,
        tf.frequencies_hz[worst_mag_idx],
        worst_phase_err,
        worst_phase_idx,
        tf.frequencies_hz[worst_phase_idx],
    );

    // [Then-3] The Result contains both the OperatingPoint and the
    //          AC frequency-domain data.
    //
    // Witnessed structurally: a single binding (`result`) gives the
    // caller access to *both* halves. The DC half (operating_point,
    // dc_convergence, dc_topology_warnings) and the AC half (ac,
    // with one TransferFunction per requested output) are present on
    // the same value. We assert both halves are non-trivial here so
    // a future refactor that elides either one would fail this
    // witness.
    assert!(
        !result
            .operating_point
            .as_ref()
            .expect("Then-3: OperatingPoint must be Some on the converged path")
            .node_voltages
            .is_empty(),
        "Then-3: Result's OperatingPoint must carry node voltages"
    );
    assert!(
        !result
            .ac
            .as_ref()
            .expect("Then-3: ac must be Some on the converged path")
            .transfer_functions
            .is_empty(),
        "Then-3: Result's AC half must carry TransferFunctions"
    );
    assert!(
        result.dc_convergence.is_converged(),
        "Then-3: Result's DC half must carry a converged status"
    );

    // Boundary-point spot checks (defense in depth):
    //
    // At f = 1 Hz (deep passband, ≈2.2 decades below f_c ≈ 159 Hz)
    // magnitude must be essentially 0 dB. At f = 1 MHz (≈3.8 decades
    // above cutoff) the first-order low-pass has rolled off by ~75
    // dB and the phase is approaching -90°.
    assert!(
        tf.magnitude_db[0].abs() < 0.01,
        "1 Hz magnitude should be ≈0 dB (passband); got {} dB",
        tf.magnitude_db[0]
    );
    assert!(
        tf.phase_degrees[0].abs() < 1.0,
        "1 Hz phase should sit inside the passband (|phase| < 1°); got {}°",
        tf.phase_degrees[0]
    );
    let last = tf.magnitude_db.len() - 1;
    assert!(
        tf.magnitude_db[last] < -70.0,
        "1 MHz magnitude should be ≪-70 dB; got {} dB",
        tf.magnitude_db[last]
    );
    assert!(
        (tf.phase_degrees[last] - (-90.0)).abs() < 1.0,
        "1 MHz phase should approach -90°; got {}°",
        tf.phase_degrees[last]
    );
}
