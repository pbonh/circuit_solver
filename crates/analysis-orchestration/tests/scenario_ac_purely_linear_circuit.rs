//! Scenario-level integration test for
//! `ac-small-signal#ac-analysis-on-purely-linear-circuit`.
//!
//! This file is the executable witness for the Gherkin scenario inlined
//! into kanban task `t_5fbe4aaa`. It exercises the **public** API of
//! `analysis-orchestration` (and its transitive `numeric-solver` and
//! `netlist-graph` dependencies) end-to-end on two canonical purely
//! linear topologies, pinning the v1 surface per ADR-0010 and asserting
//! the spec's three observable promises:
//!
//! 1. *The Simulator returns a Result with `TransferFunction` data.*
//! 2. *The magnitude response is flat or monotonic as expected by
//!    circuit topology.*
//! 3. *The Result matches the Golden Reference within the tolerance
//!    envelope.*
//!
//! Sibling unit tests inside `crates/analysis-orchestration/src/ac.rs`
//! already cover the broader API contracts (empty-sweep rejection,
//! non-finite-frequency rejection, output-order preservation, RC
//! low-pass roll-off, RLC bandpass resonance). This integration test
//! is intentionally narrower and load-bearing for **this** scenario
//! only: it consumes solely the public crate exports, so a future
//! refactor that breaks the v1 surface fails here loudly.
//!
//! # Golden-reference choice
//!
//! Per the scenario's Gherkin
//!
//! > And the Result matches the Golden Reference within the tolerance
//! > envelope
//!
//! and the inlined glossary entry "*Golden Reference — a trusted
//! external simulator against which results are compared.*" The
//! tasks.md cross-cutting harness (item #64, *AC conformance test
//! against ngspice*) ties to a Sky130 PDK fixture comparison. That
//! harness is gated on items #62 and #25 and tracked on a separate
//! kanban thread.
//!
//! For a **purely linear** RC low-pass and series-RLC bandpass, the
//! analytic transfer function is closed-form and *exactly* the same
//! reference ngspice (or any other industrial simulator) would
//! converge to — there is no semiconductor model, no numerical
//! integration error, no Newton-Raphson tolerance, no PDK
//! parameterisation drift. We therefore use the analytic transfer
//! functions as the golden reference for this scenario witness. This
//! mirrors the pattern established by
//! `crates/numeric-solver/tests/ac_lowpass_sweep.rs`, which the
//! ADR-0002 dispatch layer pinned in trunk well before any ngspice
//! harness existed.
//!
//! # Tolerance envelope (ADR-0008 row "AC magnitude / phase")
//!
//! Per ADR-0008 *Per-Node max(Relative, Absolute) Tolerance Envelope*
//! the AC defaults are:
//!
//! | quantity      | relative | absolute |
//! |---------------|---------:|---------:|
//! | AC magnitude  | 0.1 dB   | 0.01 dB  |
//! | AC phase      | 1°       | 0.1°     |
//!
//! and the pass criterion is `|err| ≤ max(rel · |ref|, abs)`. We
//! adopt the same envelope here. Against an analytic reference (no
//! ngspice rounding) we typically beat the absolute floor by 6+
//! decades, so passing this envelope is a strong correctness signal.

use analysis_orchestration::{ac_analysis, AcAnalysisRequest, AcAnalysisResult, TransferFunction};
use circuit_solver_types::NodeId;
use netlist_graph::{CircuitBuilder, ElementKind};
use num_complex::Complex;
use numeric_solver::{assemble, flatten};

// --- Constants from ADR-0008 (AC defaults) ----------------------------------

/// `max(relative, absolute)` envelope for AC magnitude (decibels)
/// per ADR-0008. The first column applies for "large" magnitudes
/// where the relative bound dominates; the second column floors near
/// 0 dB where the relative bound would collapse.
const AC_MAG_REL_DB: f64 = 0.1;
const AC_MAG_ABS_DB: f64 = 0.01;

/// `max(relative, absolute)` envelope for AC phase (degrees) per
/// ADR-0008.
const AC_PHASE_REL_DEG: f64 = 1.0;
const AC_PHASE_ABS_DEG: f64 = 0.1;

/// True iff `actual` is within the
/// `max(relative · |reference|, absolute)` band of `reference`. This
/// is the ADR-0008 envelope operator applied to a scalar quantity.
fn within_envelope(actual: f64, reference: f64, rel: f64, abs: f64) -> bool {
    let bound = (rel * reference.abs()).max(abs);
    (actual - reference).abs() <= bound
}

// --- Circuit builders ------------------------------------------------------

/// Build the canonical RC low-pass network:
///
/// ```text
///   V1 (1 V, n_in) → R1 → n_out → C1 → gnd
/// ```
///
/// Analytic transfer function `H_RC(jω) = 1 / (1 + jωRC)`.
fn build_rc_lowpass(r_ohms: f64, c_farads: f64) -> (CircuitBuilder, &'static str, &'static str) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource { voltage_volts: 1.0 },
        ["n_in", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: r_ohms,
        },
        ["n_in", "n_out"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: c_farads,
        },
        ["n_out", "0"],
        None,
    )
    .expect("add C1");
    (b, "n_in", "n_out")
}

/// Build the canonical series-RLC network with output tapped at the
/// capacitor (cap-tap), which is the textbook resonant-peak low-pass:
///
/// ```text
///   V1 (1 V, n_in) → R1 → n_a → L1 → n_b → C1 → gnd
/// ```
///
/// Analytic transfer function for `V(n_b) / V(n_in)`:
///
/// ```text
///   H_RLC(jω) = (1/(jωC)) / (R + jωL + 1/(jωC))
/// ```
fn build_rlc_cap_tap(
    r_ohms: f64,
    l_henries: f64,
    c_farads: f64,
) -> (CircuitBuilder, &'static str, &'static str) {
    let mut b = CircuitBuilder::default();
    b.add_element(
        "V1",
        ElementKind::VoltageSource { voltage_volts: 1.0 },
        ["n_in", "0"],
        None,
    )
    .expect("add V1");
    b.add_element(
        "R1",
        ElementKind::Resistor {
            resistance_ohms: r_ohms,
        },
        ["n_in", "n_a"],
        None,
    )
    .expect("add R1");
    b.add_element(
        "L1",
        ElementKind::Inductor {
            inductance_henries: l_henries,
        },
        ["n_a", "n_b"],
        None,
    )
    .expect("add L1");
    b.add_element(
        "C1",
        ElementKind::Capacitor {
            capacitance_farads: c_farads,
        },
        ["n_b", "0"],
        None,
    )
    .expect("add C1");
    (b, "n_in", "n_b")
}

// --- Analytic golden references --------------------------------------------

/// Analytic RC low-pass `H(jω) = 1 / (1 + jωRC)`.
fn analytic_rc(omega: f64, r: f64, c: f64) -> Complex<f64> {
    Complex::new(1.0, 0.0) / Complex::new(1.0, omega * r * c)
}

/// Analytic series-RLC cap-tap `H(jω) = (1/jωC) / (R + jωL + 1/jωC)`.
fn analytic_rlc(omega: f64, r: f64, l: f64, c: f64) -> Complex<f64> {
    let zc = Complex::new(0.0, -1.0 / (omega * c));
    let zl = Complex::new(0.0, omega * l);
    zc / (Complex::new(r, 0.0) + zl + zc)
}

/// Run the public AC analysis driver against a freshly-built linear
/// circuit and return the result plus the resolved output `NodeId`.
fn run_ac_on_linear_circuit(
    mut b: CircuitBuilder,
    in_name: &str,
    out_name: &str,
    frequencies_hz: &[f64],
) -> (AcAnalysisResult, NodeId, NodeId) {
    let g = b.build().expect("build ok");
    let fs = flatten(&g).expect("flatten ok");
    let sys = assemble(&fs, &g, &[]).expect("assemble ok");

    let n_in = g
        .node_by_name(in_name)
        .map(netlist_graph::Node::id)
        .expect("input node resolves");
    let n_out = g
        .node_by_name(out_name)
        .map(netlist_graph::Node::id)
        .expect("output node resolves");

    let result = ac_analysis(AcAnalysisRequest {
        system: &sys,
        structure: &fs,
        graph: &g,
        frequencies_hz,
        outputs: &[n_out],
        ground: None,
    })
    .expect("ac_analysis ok on purely linear circuit");

    (result, n_in, n_out)
}

/// Assert the parallel-length invariant across the
/// [`TransferFunction`] axes (Then 1 of the Gherkin).
fn assert_result_shape(tf: &TransferFunction, expected_output: NodeId, n_freq: usize) {
    assert_eq!(tf.output, expected_output, "transfer-function output node");
    assert_eq!(tf.frequencies_hz.len(), n_freq, "frequencies_hz length");
    assert_eq!(tf.magnitude_db.len(), n_freq, "magnitude_db length");
    assert_eq!(tf.phase_degrees.len(), n_freq, "phase_degrees length");
    assert_eq!(tf.len(), n_freq);
    assert!(!tf.is_empty());
}

/// Compare a solver-produced transfer function against the analytic
/// reference in dB / degrees using the ADR-0008 envelope. Returns the
/// (worst-magnitude-err, worst-phase-err) pair for diagnostic logging.
fn assert_matches_golden(
    tf: &TransferFunction,
    omega_of: impl Fn(usize) -> f64,
    ref_of: impl Fn(f64) -> Complex<f64>,
    label: &str,
) -> (f64, f64) {
    let mut worst_mag = 0.0_f64;
    let mut worst_phase = 0.0_f64;
    for (i, &f_hz) in tf.frequencies_hz.iter().enumerate() {
        let omega = omega_of(i);
        let h_ref = ref_of(omega);
        let mag_ref_db = 20.0 * h_ref.norm().log10();
        let phase_ref_deg = h_ref.arg().to_degrees();
        let mag_actual_db = tf.magnitude_db[i];
        let phase_actual_deg = tf.phase_degrees[i];

        assert!(
            within_envelope(mag_actual_db, mag_ref_db, AC_MAG_REL_DB, AC_MAG_ABS_DB),
            "{label}: magnitude at f[{i}]={f_hz} Hz outside ADR-0008 envelope: \
             actual={mag_actual_db} dB, ref={mag_ref_db} dB \
             (max(rel={AC_MAG_REL_DB} dB · |ref|, abs={AC_MAG_ABS_DB} dB))"
        );
        assert!(
            within_envelope(
                phase_actual_deg,
                phase_ref_deg,
                AC_PHASE_REL_DEG,
                AC_PHASE_ABS_DEG
            ),
            "{label}: phase at f[{i}]={f_hz} Hz outside ADR-0008 envelope: \
             actual={phase_actual_deg}°, ref={phase_ref_deg}° \
             (max(rel={AC_PHASE_REL_DEG}° · |ref|, abs={AC_PHASE_ABS_DEG}°))"
        );

        let mag_err = (mag_actual_db - mag_ref_db).abs();
        let phase_err = (phase_actual_deg - phase_ref_deg).abs();
        if mag_err > worst_mag {
            worst_mag = mag_err;
        }
        if phase_err > worst_phase {
            worst_phase = phase_err;
        }
    }
    (worst_mag, worst_phase)
}

// --- Then 1 : Result shape pinned across both topologies -------------------

#[test]
fn ac_purely_linear_returns_transfer_function_data() {
    // RC low-pass.
    let r = 1_000.0_f64;
    let c = 1.0e-6_f64;
    let f_cutoff_hz = 1.0 / (2.0 * core::f64::consts::PI * r * c);
    let frequencies_hz = vec![
        f_cutoff_hz * 0.01,
        f_cutoff_hz * 0.1,
        f_cutoff_hz,
        f_cutoff_hz * 10.0,
        f_cutoff_hz * 100.0,
    ];

    let (b, _, out_name) = build_rc_lowpass(r, c);
    let (result, _, n_out) = run_ac_on_linear_circuit(b, "n_in", out_name, &frequencies_hz);

    // "Then the Simulator returns a Result with TransferFunction data"
    assert_eq!(
        result.transfer_functions.len(),
        1,
        "exactly one transfer function for the one requested output"
    );
    let tf = result
        .transfer_for(n_out)
        .expect("transfer_for the requested output");
    assert_result_shape(tf, n_out, frequencies_hz.len());

    // The frequency axis copied through verbatim (bit-exact, since
    // [`ac_analysis`] should `.to_vec()` the input slice without any
    // arithmetic).
    for (i, &f_hz) in frequencies_hz.iter().enumerate() {
        assert_eq!(
            tf.frequencies_hz[i].to_bits(),
            f_hz.to_bits(),
            "frequencies_hz preserved at i={i}"
        );
    }

    // Every reported sample is finite.
    for (i, (m, p)) in tf
        .magnitude_db
        .iter()
        .zip(tf.phase_degrees.iter())
        .enumerate()
    {
        assert!(m.is_finite(), "magnitude_db[{i}]={m} is non-finite");
        assert!(p.is_finite(), "phase_degrees[{i}]={p} is non-finite");
    }
}

// --- Then 2 : Magnitude monotonicity on a low-pass --------------------------

#[test]
fn ac_purely_linear_rc_magnitude_is_monotone_non_increasing() {
    // 6-decade log sweep around the cutoff of a 1 kΩ × 1 µF
    // low-pass. The textbook result has |H| monotonically
    // non-increasing in ω, with no resonance, no zero, and no
    // overshoot. This is the "flat or monotonic" arm of the
    // scenario's Then-clause for a topology that is unambiguously a
    // low-pass.
    let r = 1_000.0_f64;
    let c = 1.0e-6_f64;
    let f_cutoff_hz = 1.0 / (2.0 * core::f64::consts::PI * r * c);
    let points_per_decade = 11;
    let decades = [-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0];
    let mut frequencies_hz = Vec::new();
    for d in 0..(decades.len() - 1) {
        let start = decades[d];
        let end = decades[d + 1];
        for k in 0..points_per_decade {
            let frac = f64::from(k) / f64::from(points_per_decade);
            let exp = start + frac * (end - start);
            frequencies_hz.push(f_cutoff_hz * 10.0_f64.powf(exp));
        }
    }
    frequencies_hz.push(f_cutoff_hz * 10.0_f64.powf(decades[decades.len() - 1]));

    let (b, _, out_name) = build_rc_lowpass(r, c);
    let (result, _, n_out) = run_ac_on_linear_circuit(b, "n_in", out_name, &frequencies_hz);
    let tf = result.transfer_for(n_out).expect("transfer_for n_out");

    // Per-pair check: each next magnitude is no greater than the
    // previous, within a tiny floating-point slack that swamps the
    // 0.1 dB envelope at the inflection point.
    for (i, win) in tf.magnitude_db.windows(2).enumerate() {
        assert!(
            win[1] <= win[0] + 1e-9,
            "RC low-pass magnitude must be monotone non-increasing across \
             the sweep; failed at i={i}: f[{}]={} Hz → {} dB, f[{}]={} Hz → {} dB",
            i,
            frequencies_hz[i],
            win[0],
            i + 1,
            frequencies_hz[i + 1],
            win[1]
        );
    }

    // Three sanity bands the Gherkin "flat or monotonic" clause
    // implies. At f ≪ f_cutoff the response is ≈ 0 dB; at f_cutoff
    // it is ≈ -3 dB; at f ≫ f_cutoff it is deep in the rolloff.
    let low_dc = tf.magnitude_db[0];
    let cutoff_idx = tf
        .frequencies_hz
        .iter()
        .position(|&f| (f - f_cutoff_hz).abs() < f_cutoff_hz * 0.001)
        .expect("cutoff frequency is one of the sweep points");
    let high = *tf.magnitude_db.last().unwrap();
    assert!(
        (low_dc - 0.0).abs() < 0.1,
        "low-freq passband should be ≈ 0 dB, got {low_dc}"
    );
    assert!(
        (tf.magnitude_db[cutoff_idx] - (-3.0103)).abs() < 0.1,
        "cutoff magnitude should be ≈ -3.0103 dB, got {}",
        tf.magnitude_db[cutoff_idx]
    );
    assert!(
        high < -50.0,
        "high-freq stopband should be deep (< -50 dB), got {high}"
    );
}

// --- Then 3 : Conformance against an analytic golden reference --------------

#[test]
fn ac_purely_linear_rc_matches_analytic_golden_reference() {
    // Log sweep around the cutoff. The analytic reference H(jω) is
    // exact, so we expect the solver to beat the ADR-0008 envelope
    // by many decades on every sample.
    let r = 1_000.0_f64;
    let c = 1.0e-6_f64;
    let f_cutoff_hz = 1.0 / (2.0 * core::f64::consts::PI * r * c);
    let frequencies_hz: Vec<f64> = (-3..=3).map(|d| f_cutoff_hz * 10.0_f64.powi(d)).collect();

    let (b, _, out_name) = build_rc_lowpass(r, c);
    let (result, _, n_out) = run_ac_on_linear_circuit(b, "n_in", out_name, &frequencies_hz);
    let tf = result.transfer_for(n_out).expect("transfer_for n_out");

    let omega_of = |i: usize| 2.0 * core::f64::consts::PI * frequencies_hz[i];
    let ref_of = |omega: f64| analytic_rc(omega, r, c);
    let (worst_mag, worst_phase) =
        assert_matches_golden(tf, omega_of, ref_of, "ac_rc_lowpass_golden");

    // Diagnostic floor: against the analytic reference we expect
    // numerical errors to be much smaller than the ADR-0008 floor.
    // If this ever fails it is a strong signal that a regression
    // in the AC sub-view stamping or the LU dispatch is masking a
    // larger error behind the envelope.
    assert!(
        worst_mag < 1e-6,
        "RC golden mag error {worst_mag} dB looks suspiciously large \
         vs analytic reference (expected << 1e-6 dB)"
    );
    assert!(
        worst_phase < 1e-6,
        "RC golden phase error {worst_phase}° looks suspiciously large \
         vs analytic reference (expected << 1e-6°)"
    );
}

#[test]
fn ac_purely_linear_rlc_matches_analytic_golden_reference() {
    // Lightly-damped series RLC with cap-tap. ω0 = 1/√(LC) ≈ 31_622
    // rad/s, f0 ≈ 5_032.92 Hz, Q ≈ 31.6. We sample below, at, and
    // far above resonance. We deliberately avoid landing a sweep
    // point *exactly* at ω0 — the undamped pole would give a
    // singular complex MNA, which the spec acknowledges in the
    // `AcAnalysisError::SolverFailed` doc.
    let r = 1.0_f64;
    let l = 1.0e-3_f64;
    let c = 1.0e-6_f64;
    let f0_hz = 1.0 / (2.0 * core::f64::consts::PI * (l * c).sqrt());
    let frequencies_hz = vec![
        f0_hz * 0.001,
        f0_hz * 0.01,
        f0_hz * 0.1,
        f0_hz * 0.5,
        // shift slightly off resonance to keep the matrix well-
        // conditioned at Q≈31.6
        f0_hz * 1.05,
        f0_hz * 2.0,
        f0_hz * 10.0,
        f0_hz * 100.0,
    ];

    let (b, _, out_name) = build_rlc_cap_tap(r, l, c);
    let (result, _, n_out) = run_ac_on_linear_circuit(b, "n_in", out_name, &frequencies_hz);
    let tf = result.transfer_for(n_out).expect("transfer_for n_out");
    assert_result_shape(tf, n_out, frequencies_hz.len());

    let omega_of = |i: usize| 2.0 * core::f64::consts::PI * frequencies_hz[i];
    let ref_of = |omega: f64| analytic_rlc(omega, r, l, c);
    let (worst_mag, worst_phase) =
        assert_matches_golden(tf, omega_of, ref_of, "ac_rlc_cap_tap_golden");

    // Same diagnostic floor as the RC case.
    assert!(
        worst_mag < 1e-6,
        "RLC golden mag error {worst_mag} dB looks suspiciously large \
         vs analytic reference (expected << 1e-6 dB)"
    );
    assert!(
        worst_phase < 1e-6,
        "RLC golden phase error {worst_phase}° looks suspiciously large \
         vs analytic reference (expected << 1e-6°)"
    );
}

// --- Cross-cutting : the scenario's Given/When/Then transcript -------------

/// Mirrors the full Gherkin scenario in one test for traceability:
///
/// > Given CircuitDesigner has constructed a Circuit containing only
/// > linear elements (R, L, C, independent sources)
/// > When CircuitDesigner submits an AC small-signal Analysis request
/// > Then the Simulator returns a Result with TransferFunction data
/// > And the magnitude response is flat or monotonic as expected by
/// > circuit topology
/// > And the Result matches the Golden Reference within the tolerance
/// > envelope
#[test]
fn scenario_ac_analysis_on_purely_linear_circuit() {
    // GIVEN: a Circuit containing only linear elements (R, C,
    // independent voltage source). We use the RC low-pass as the
    // canonical witness; the RLC variant is exercised in the
    // dedicated test above. No semiconductors, no nonlinear
    // models — purely linear.
    let r = 1_000.0_f64;
    let c = 1.0e-6_f64;
    let f_cutoff_hz = 1.0 / (2.0 * core::f64::consts::PI * r * c);
    let frequencies_hz: Vec<f64> = (-3..=3).map(|d| f_cutoff_hz * 10.0_f64.powi(d)).collect();
    let (b, in_name, out_name) = build_rc_lowpass(r, c);

    // WHEN: an AC small-signal Analysis request is submitted. We do
    // not stage a separate DC operating-point computation; on a
    // purely linear circuit, `assemble(&fs, &g, &[])` already *is*
    // the linearization (no device companion stamps required), so
    // the spec's clause "linearizes the Circuit around a previously
    // computed OperatingPoint" reduces to identity.
    let (result, _, n_out) = run_ac_on_linear_circuit(b, in_name, out_name, &frequencies_hz);

    // THEN (1): the Simulator returns a Result with TransferFunction
    // data — one TF per requested output, parallel axes, all finite.
    let tf = result
        .transfer_for(n_out)
        .expect("Result contains TransferFunction data");
    assert_result_shape(tf, n_out, frequencies_hz.len());

    // THEN (2): the magnitude response is flat or monotonic as
    // expected by circuit topology. RC low-pass ⇒ monotone
    // non-increasing.
    for win in tf.magnitude_db.windows(2) {
        assert!(
            win[1] <= win[0] + 1e-9,
            "RC low-pass magnitude must be monotone non-increasing: \
             {win:?}"
        );
    }

    // THEN (3): the Result matches the Golden Reference within the
    // ADR-0008 tolerance envelope.
    let omega_of = |i: usize| 2.0 * core::f64::consts::PI * frequencies_hz[i];
    let ref_of = |omega: f64| analytic_rc(omega, r, c);
    let (_worst_mag, _worst_phase) =
        assert_matches_golden(tf, omega_of, ref_of, "scenario_rc_lowpass");
}
