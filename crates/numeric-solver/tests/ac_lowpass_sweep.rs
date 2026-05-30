//! Integration test for [`FaerComplexSolver`]: drive an AC frequency
//! sweep on the canonical RC low-pass network and verify the
//! solver-produced transfer function matches the analytic
//! `H(jω) = 1 / (1 + jωRC)`.
//!
//! This exercises the same code path the AC analysis control loop
//! (`tasks.md` item #25) will exercise per frequency point: it
//! constructs a per-ω complex MNA system, invokes
//! [`LinearSolver::solve`], and reads a single node-voltage entry
//! out of the solution.
//!
//! The point of pushing this into `tests/` rather than `#[cfg(test)]`
//! is to verify the **public** entry points
//! ([`numeric_solver::FaerComplexSolver`], [`numeric_solver::LinearSolver`],
//! [`numeric_solver::SparseLinearSystem`]) are usable from outside
//! the crate. Per ADR-0010 those entry points are unstable; this
//! test pins their current shape.

use num_complex::Complex;
use numeric_solver::{FaerComplexSolver, LinearSolver, SparseLinearSystem, SparseTriplet};

/// `H(jω) = 1 / (1 + jωRC)` — analytic low-pass transfer function.
fn analytic_lowpass(omega: f64, r: f64, c: f64) -> Complex<f64> {
    let den = Complex::new(1.0, omega * r * c);
    Complex::new(1.0, 0.0) / den
}

/// Build a 1×1 complex MNA system that solves for the output voltage
/// of an ideal RC low-pass at frequency ω driven by a unit-voltage
/// source through R, shunted to ground through C.
///
/// After source-stamping the Vin contribution into the RHS, the
/// nodal equation at the output is:
///
///   `(G + jωC) · V_out = G · V_in`     with `V_in = 1 V` real
///
/// i.e. a single-row complex system `(G + jωC) · V = G`. The exact
/// solution is `V = G / (G + jωC) = 1 / (1 + jωRC)`.
fn ac_lowpass_system(omega: f64, r: f64, c: f64) -> SparseLinearSystem<Complex<f64>> {
    let g = 1.0 / r;
    SparseLinearSystem::new(
        1,
        1,
        0,
        vec![SparseTriplet {
            row: 0,
            col: 0,
            value: Complex::new(g, omega * c),
        }],
        vec![Complex::new(g, 0.0)],
    )
    .expect("well-formed")
}

#[test]
fn ac_lowpass_sweep_matches_analytic_transfer_function() {
    let r = 1.0e3_f64; // 1 kΩ
    let c = 1.0e-9_f64; // 1 nF, → cutoff ω ≈ 1e6 rad/s
    let solver = FaerComplexSolver::new();

    // Sweep 4 decades around the cutoff: 1e4 … 1e8 rad/s.
    let omegas: Vec<f64> = (0..=16)
        .map(|k| 10f64.powf(4.0 + f64::from(k) * 0.25))
        .collect();

    for &omega in &omegas {
        let sys = ac_lowpass_system(omega, r, c);
        let solution = solver
            .solve(&sys)
            .unwrap_or_else(|e| panic!("solve failed at ω={omega}: {e}"));
        let v = solution.unknowns()[0];
        let expected = analytic_lowpass(omega, r, c);
        let err_re = (v.re - expected.re).abs();
        let err_im = (v.im - expected.im).abs();
        assert!(
            err_re < 1e-12 && err_im < 1e-12,
            "ω={omega}: got V = {v:?}, expected H(jω) = {expected:?} \
             (|re-err|={err_re:.3e}, |im-err|={err_im:.3e})",
        );
    }
}

/// Verify the DC limit (ω = 0): the matrix becomes purely real,
/// the imaginary part of the solution must vanish exactly.
#[test]
fn ac_at_dc_limit_yields_pure_real_solution() {
    let solver = FaerComplexSolver::new();
    let sys = ac_lowpass_system(0.0, 1.0e3, 1.0e-9);
    let v = solver.solve(&sys).expect("non-singular at DC").unknowns()[0];
    assert!((v.re - 1.0).abs() < 1e-12, "Vout(DC) = {v:?}");
    // At DC the matrix has no imaginary part. faer should produce an
    // exactly-zero imaginary part bit-for-bit; clippy's float_cmp lint
    // is therefore noisy here and we silence it locally.
    #[allow(clippy::float_cmp)]
    {
        assert_eq!(v.im, 0.0, "imag part must be exact 0 at DC");
    }
}

/// Verify the high-frequency limit (ω → ∞): magnitude shrinks to zero,
/// phase approaches -π/2.
#[test]
fn ac_high_frequency_phase_approaches_minus_pi_over_two() {
    let solver = FaerComplexSolver::new();
    // ω = 1e12 rad/s, far beyond cutoff at 1e6.
    let sys = ac_lowpass_system(1.0e12, 1.0e3, 1.0e-9);
    let v = solver.solve(&sys).unwrap().unknowns()[0];
    let mag = (v.re * v.re + v.im * v.im).sqrt();
    let phase = v.im.atan2(v.re);
    assert!(mag < 1e-2, "expected attenuation, got mag={mag}");
    assert!(
        (phase + std::f64::consts::FRAC_PI_2).abs() < 1e-2,
        "expected phase ≈ -π/2, got {phase}",
    );
}
