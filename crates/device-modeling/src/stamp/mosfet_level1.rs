//! MOSFET Level-1 (Shichman-Hodges) companion stamp (tasks.md #11).
//!
//! This module implements the per-iterate linearization of the
//! Shichman-Hodges square-law MOSFET model. It is invoked from the
//! `MOSFETParams::Level1(_)` arm of
//! [`linearize_mosfet`](super::linearize_mosfet) and turns the
//! Level-1 device equation into a 4×4 Jacobian + 4-vector companion
//! current in the `[drain, gate, source, bulk]` terminal-local
//! coordinate system documented on
//! [`MOSFETLinearization`].
//!
//! # Equation summary (NMOS)
//!
//! With internal voltages defined relative to the source terminal:
//!
//! ```text
//! V_gs = V_g - V_s
//! V_ds = V_d - V_s
//! V_bs = V_b - V_s
//! ```
//!
//! the threshold voltage with body effect is
//!
//! ```text
//! V_th = VTO + GAMMA * (sqrt(PHI - V_bs) - sqrt(PHI))
//! ```
//!
//! clamped at the sqrt argument so the formula remains real when the
//! source-bulk junction is forward biased. The overdrive is
//! `V_ov = V_gs - V_th`. The drain current `I_d` then falls into three
//! regions:
//!
//! - **Cutoff** (`V_ov <= 0`): `I_d = 0`.
//! - **Triode** (`0 < V_ds < V_ov`):
//!   `I_d = KP * (V_ov * V_ds - V_ds^2 / 2) * (1 + LAMBDA * V_ds)`.
//! - **Saturation** (`V_ds >= V_ov`):
//!   `I_d = (KP / 2) * V_ov^2 * (1 + LAMBDA * V_ds)`.
//!
//! Channel-length modulation `(1 + LAMBDA * V_ds)` is applied in both
//! conducting regions per the SPICE Level-1 convention so that the
//! saturation curve has a non-zero `g_ds`. (Some textbooks apply CLM
//! only in saturation; SPICE applies it in both regions and we follow
//! SPICE for golden-reference conformance per ADR-0008.)
//!
//! # PMOS
//!
//! PMOS devices are handled by computing the NMOS-equivalent quantities
//! with sign-flipped operating voltages (`V_sg`, `V_sd`, `V_sb`) and a
//! threshold magnitude `|VTO|`, then negating the drain current and
//! flipping the Jacobian sign for the rows of the device-current vector
//! (drain row gets a `-`, source row gets a `+` because `I_s = -I_d`).
//! This is equivalent to running the NMOS branch on the
//! source-referenced flipped voltages and then negating the result.
//!
//! # Jacobian convention
//!
//! The 4×4 Jacobian `J` is the partial-derivative matrix of the
//! *device current entering each terminal* with respect to each
//! terminal voltage:
//!
//! ```text
//! J[i][j] = ∂ I_terminal_i / ∂ V_terminal_j
//! ```
//!
//! In Level-1, the only non-zero terminal currents are drain and
//! source (gate and bulk currents are identically zero, since there is
//! no gate-leakage or junction-diode body model at this slice). With
//! `gm = ∂Id/∂V_gs`, `gds = ∂Id/∂V_ds`, `gmb = ∂Id/∂V_bs`, and the
//! linear chain `V_gs = V_g - V_s`, `V_ds = V_d - V_s`,
//! `V_bs = V_b - V_s`, the drain row works out to:
//!
//! ```text
//! J[D, D] =  gds
//! J[D, G] =  gm
//! J[D, S] = -gm - gds - gmb
//! J[D, B] =  gmb
//! ```
//!
//! and the source row is the exact negation
//! (`I_s = -I_d`). For PMOS, the same identity holds in terms of the
//! sign-flipped operating voltages.
//!
//! # Companion-current convention
//!
//! Per [`MOSFETLinearization`], the
//! companion current at terminal `k` is added to the MNA right-hand
//! side as a constant current source. Linearizing the nonlinear
//! terminal-current function `I_k(V)` at the operating point `V*` and
//! moving the linear part into the conductance matrix leaves the
//! companion current
//!
//! ```text
//! I_eq_k(V*) = I_k(V*) - Σ_j J[k, j] * V*_j
//! ```
//!
//! so the assembler's `(G * V) + I_eq = I_external` system reproduces
//! `I_k(V*)` exactly at the iterate and the Newton update solves the
//! Taylor expansion to first order.
//!
//! # Numerical stability
//!
//! - The body-effect sqrt is clamped at zero so a forward-biased
//!   source-bulk junction does not produce NaN.
//! - The `KP` parameter is taken as-is; SPICE convention treats `KP`
//!   as positive for both NMOS and PMOS at the Level-1 model card,
//!   with polarity carried separately. We follow that convention.
//! - There is no `MIN_COND` floor at this slice; that lives in the
//!   Gmin-stepping homotopy (tasks.md #18), not in the per-device
//!   stamp.

use crate::params::{MosLevel1Params, MosPolarity};
use crate::stamp::{MOSFETLinearization, MOSFET_TERMINALS};

/// Drain terminal slot in the `[drain, gate, source, bulk]` ordering.
pub(crate) const D: usize = 0;
/// Gate terminal slot.
pub(crate) const G: usize = 1;
/// Source terminal slot.
pub(crate) const S: usize = 2;
/// Bulk terminal slot.
pub(crate) const B: usize = 3;

/// Linearize a MOSFET Level-1 device at the given terminal voltages.
///
/// Implements the Shichman-Hodges square-law model with optional
/// channel-length modulation (`LAMBDA`) and body-effect threshold
/// shift (`GAMMA`, `PHI`).
///
/// # Arguments
///
/// - `params` — the Level-1 model card (`VTO`, `KP`, `LAMBDA`,
///   `GAMMA`, `PHI`, polarity).
/// - `terminal_voltages` — `[V_drain, V_gate, V_source, V_bulk]` in
///   the canonical `[D, G, S, B]` slot order.
///
/// # Returns
///
/// A [`MOSFETLinearization`] holding the 4×4 Jacobian and 4-vector
/// companion current in terminal-local coordinates.
#[must_use]
#[allow(clippy::similar_names)] // gm/gds/gmb and vgs/vds/vbs are the canonical SPICE names; renaming would obscure the model.
pub fn linearize_mosfet_level1(
    params: &MosLevel1Params,
    terminal_voltages: &[f64; MOSFET_TERMINALS],
) -> MOSFETLinearization {
    let v = terminal_voltages;

    // Map polarity to a NMOS-equivalent sign convention. For PMOS we
    // run the same algebra on the source-referenced flipped voltages
    // and then negate the drain current and its derivatives.
    let polarity_sign: f64 = match params.polarity {
        MosPolarity::Nmos => 1.0,
        MosPolarity::Pmos => -1.0,
    };

    // Source-referenced operating voltages, in NMOS-equivalent form.
    // For NMOS:  vgs = V_g - V_s; for PMOS: vgs = V_s - V_g.
    let vgs = polarity_sign * (v[G] - v[S]);
    let vds = polarity_sign * (v[D] - v[S]);
    let vbs = polarity_sign * (v[B] - v[S]);

    // Threshold with body effect (clamped sqrt argument).
    //
    // |VTO| is the threshold magnitude. SPICE convention writes VTO
    // as a signed value (positive for enhancement NMOS, negative for
    // enhancement PMOS); the NMOS-equivalent algebra below uses the
    // magnitude.
    let vto_mag = params.vto.abs();
    let phi = params.phi;
    let gamma = params.gamma;

    let body_sqrt_arg = (phi - vbs).max(0.0);
    let phi_sqrt = phi.max(0.0).sqrt();
    let vth = vto_mag + gamma * (body_sqrt_arg.sqrt() - phi_sqrt);

    // Overdrive.
    let v_ov = vgs - vth;

    // Compute drain current and its partial derivatives w.r.t.
    // (vgs, vds, vbs) in NMOS-equivalent coordinates.
    let kp = params.kp;
    let lambda = params.lambda;

    let (id, gm, gds, gmb) = if v_ov <= 0.0 {
        // Cutoff: no channel.
        (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64)
    } else {
        let clm = 1.0 + lambda * vds;

        // dV_th / dvbs via the body-effect formula:
        //
        //   V_th = vto_mag + gamma * (sqrt(phi - vbs) - sqrt(phi))
        //
        // and so dV_th/dvbs = -gamma / (2 * sqrt(phi - vbs))
        // when the sqrt argument is positive; zero when clamped (the
        // body diode is too forward-biased to model at this slice).
        let dvth_dvbs = if body_sqrt_arg > 0.0 {
            -gamma / (2.0 * body_sqrt_arg.sqrt())
        } else {
            0.0
        };

        if vds < v_ov {
            // Triode region.
            //
            //   I_d = KP * (V_ov * vds - vds^2 / 2) * (1 + λ vds)
            let core = v_ov * vds - 0.5 * vds * vds;
            let id_val = kp * core * clm;

            // ∂core/∂vgs    = vds        (V_ov contains +vgs)
            // ∂core/∂vds    = V_ov - vds
            // ∂core/∂V_th   = -vds       (V_ov contains -V_th)
            //
            // Combine with chain rule on CLM term and body-effect
            // dependence of V_th on vbs.
            let dcore_dvgs = vds;
            let dcore_dvds_partial = v_ov - vds;
            let dcore_dvth = -vds;

            let gm_val = kp * dcore_dvgs * clm;
            let gds_val = kp * (dcore_dvds_partial * clm + core * lambda);
            // dV_th/dvbs affects V_ov (so it affects dcore_dvth
            // dependence on vbs); KP·core's bulk dependence is solely
            // through V_th.
            let gmb_val = kp * dcore_dvth * dvth_dvbs * clm;
            (id_val, gm_val, gds_val, gmb_val)
        } else {
            // Saturation region.
            //
            //   I_d = (KP / 2) * V_ov^2 * (1 + λ vds)
            let id_val = 0.5 * kp * v_ov * v_ov * clm;

            // ∂(V_ov^2/2)/∂vgs = V_ov
            // ∂(V_ov^2/2)/∂V_th = -V_ov
            // ∂(I_d)/∂vds      = (KP / 2) * V_ov^2 * λ
            // ∂(I_d)/∂vbs      = KP * V_ov * (-dV_th/dvbs) * clm
            let gm_val = kp * v_ov * clm;
            let gds_val = 0.5 * kp * v_ov * v_ov * lambda;
            let gmb_val = kp * v_ov * (-dvth_dvbs) * clm;
            (id_val, gm_val, gds_val, gmb_val)
        }
    };

    // Map NMOS-equivalent derivatives back to true-terminal Jacobian.
    //
    // For NMOS: I_D = +id, I_S = -id, and the chain rule from
    //   vgs = V_g - V_s,
    //   vds = V_d - V_s,
    //   vbs = V_b - V_s
    // yields the drain row in [D, G, S, B] order:
    //   [ +gds, +gm, -gm - gds - gmb, +gmb ].
    //
    // For PMOS we ran the NMOS-equivalent algebra on flipped
    // (V_s - V_g) / (V_s - V_d) / (V_s - V_b) voltages. By the chain
    // rule, the partials with respect to the *true* terminal voltages
    // get an extra factor of `polarity_sign`. The PMOS drain current
    // also flips sign: `I_D` (true) = `-id`. Both factors of
    // polarity_sign multiply into the Jacobian entries — and since
    // polarity_sign^2 = 1, the Jacobian entries are *the same shape*
    // as the NMOS case, just with the sign of `id` flipped in the
    // companion current. (Geometrically: PMOS in saturation still has
    // a positive small-signal conductance to the source.)
    //
    // Concretely:
    //   I_D_true = polarity_sign * id
    //   ∂I_D_true/∂V_X = polarity_sign · (∂id / ∂V_X_eq) · (∂V_X_eq / ∂V_X)
    // and (∂V_X_eq / ∂V_X) carries the second polarity_sign factor.
    let id_true = polarity_sign * id;

    let mut jacobian = [[0.0_f64; MOSFET_TERMINALS]; MOSFET_TERMINALS];

    // Drain row (I_D = polarity_sign · id).
    jacobian[D][D] = gds;
    jacobian[D][G] = gm;
    jacobian[D][S] = -gm - gds - gmb;
    jacobian[D][B] = gmb;

    // Source row (I_S = -I_D in this model).
    jacobian[S][D] = -gds;
    jacobian[S][G] = -gm;
    jacobian[S][S] = gm + gds + gmb;
    jacobian[S][B] = -gmb;

    // Gate row (zero gate current in Level-1).
    // Bulk row (zero bulk current in Level-1 without junction diodes).

    // Terminal current vector at the iterate (true coordinates).
    let mut i_terminal = [0.0_f64; MOSFET_TERMINALS];
    i_terminal[D] = id_true;
    i_terminal[S] = -id_true;

    // Companion current per terminal:
    //   I_eq_k = I_k(V*) - Σ_j J[k, j] · V*_j
    let mut companion_current = [0.0_f64; MOSFET_TERMINALS];
    for (k, ck) in companion_current.iter_mut().enumerate() {
        let mut sum = 0.0;
        for (j, &vj) in v.iter().enumerate() {
            sum += jacobian[k][j] * vj;
        }
        *ck = i_terminal[k] - sum;
    }

    MOSFETLinearization {
        jacobian,
        companion_current,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::MosLevel1Params;
    use circuit_solver_types::ModelName;

    // -----------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------

    /// Construct a long-channel NMOS Level-1 model with realistic
    /// textbook parameters. These are the values most SPICE textbooks
    /// use for a default NMOS Level-1 example device.
    fn nmos_default() -> MosLevel1Params {
        MosLevel1Params {
            name: ModelName::new("nmos_default"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,    // V
            kp: 50.0e-6, // A/V^2
            lambda: 0.0, // 1/V
            gamma: 0.0,  // sqrt(V)
            phi: 0.6,    // V
            kf: 0.0,     // flicker disabled
            af: 1.0,
        }
    }

    /// Construct a PMOS Level-1 model. SPICE convention is that VTO
    /// for an enhancement PMOS is negative; we honor that here.
    fn pmos_default() -> MosLevel1Params {
        MosLevel1Params {
            name: ModelName::new("pmos_default"),
            polarity: MosPolarity::Pmos,
            vto: -1.0,
            kp: 25.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        }
    }

    /// f64-aware "approximately equal" used by the analytic-vs-numeric
    /// derivative comparisons. We blend an absolute and a relative
    /// floor so we do not over-constrain values near zero.
    fn approx(a: f64, b: f64, rtol: f64, atol: f64) -> bool {
        let diff = (a - b).abs();
        diff <= atol || diff <= rtol * b.abs().max(a.abs())
    }

    /// Numerical central-difference derivative of `id_at(v)` w.r.t.
    /// the `which`-th component of `v`. Step `h` is chosen large
    /// enough to avoid catastrophic cancellation but small enough
    /// that the central-difference error is below 1e-9 of the
    /// derivative magnitude on the smooth regions we test here.
    fn finite_diff_id(params: &MosLevel1Params, v: [f64; MOSFET_TERMINALS], which: usize) -> f64 {
        let h = 1.0e-4;
        let mut vp = v;
        let mut vm = v;
        vp[which] += h;
        vm[which] -= h;

        // Drain current is the terminal current at the drain index;
        // pull it back out of the linearization for the central
        // operating points so we don't reimplement the model here.
        let id_plus = drain_current_from_stamp(params, &vp);
        let id_minus = drain_current_from_stamp(params, &vm);
        (id_plus - id_minus) / (2.0 * h)
    }

    /// Recover `I_D(V*)` from a linearization: by construction,
    /// `I_D(V*) = Σ_j J[D, j] · V*_j + I_eq_D`. This is the
    /// stamp-side identity we use to bootstrap finite-difference
    /// derivative tests without re-deriving the model in the test.
    fn drain_current_from_stamp(params: &MosLevel1Params, v: &[f64; MOSFET_TERMINALS]) -> f64 {
        let lin = linearize_mosfet_level1(params, v);
        let mut sum = lin.companion_current[D];
        for (j, &vj) in v.iter().enumerate() {
            sum += lin.jacobian[D][j] * vj;
        }
        sum
    }

    // -----------------------------------------------------------------
    // Region 1: cutoff (Vgs < Vth → no channel)
    // -----------------------------------------------------------------

    #[test]
    fn nmos_cutoff_returns_zero_current_and_zero_jacobian() {
        let p = nmos_default();
        // V_gs = 0.5 V, V_th = 1.0 V → cutoff
        let v = [3.0, 0.5, 0.0, 0.0];
        let lin = linearize_mosfet_level1(&p, &v);

        // I_D(V*) reconstructed from the stamp must be zero.
        assert!(approx(drain_current_from_stamp(&p, &v), 0.0, 0.0, 1.0e-15));
        // Jacobian entries must all be zero in cutoff. IEEE 754
        // subtraction can produce -0.0 from -0 - 0 - 0; we treat
        // signed zeros as equal here by checking magnitude.
        for row in &lin.jacobian {
            for &entry in row {
                assert!(entry.abs() <= 0.0, "expected zero, got {entry}");
            }
        }
        // Companion currents reduce to I(V*) - 0 = 0. IEEE 754
        // arithmetic can legitimately produce -0.0 here from a 0-0
        // subtraction; we accept both signed zeros.
        for &c in &lin.companion_current {
            assert!(c.abs() <= 0.0, "expected companion current 0, got {c}");
        }
    }

    // -----------------------------------------------------------------
    // Region 2: saturation (V_ds >= V_ov)
    // -----------------------------------------------------------------

    #[test]
    fn nmos_saturation_matches_textbook_square_law() {
        let p = nmos_default();
        // V_gs = 3 V, V_th = 1 V → V_ov = 2 V
        // V_ds = 5 V > V_ov, λ = 0 → I_d = (KP/2) · V_ov^2 = 100 µA
        let v = [5.0, 3.0, 0.0, 0.0];
        let id = drain_current_from_stamp(&p, &v);
        let expected = 0.5 * p.kp * 2.0 * 2.0;
        assert!(
            approx(id, expected, 1.0e-12, 1.0e-15),
            "expected {expected} A, got {id} A",
        );
    }

    #[test]
    fn nmos_saturation_gm_matches_square_law() {
        // gm = KP · V_ov in saturation (LAMBDA=0).
        let p = nmos_default();
        let v = [5.0, 3.0, 0.0, 0.0];
        let lin = linearize_mosfet_level1(&p, &v);
        let v_ov = 3.0 - p.vto;
        let expected_gm = p.kp * v_ov;
        // J[D, G] is +gm in our convention.
        assert!(
            approx(lin.jacobian[D][G], expected_gm, 1.0e-12, 1.0e-15),
            "expected gm = {expected_gm}, got {}",
            lin.jacobian[D][G],
        );
    }

    #[test]
    fn nmos_saturation_with_lambda_has_finite_gds() {
        // λ > 0 → output conductance KP · V_ov^2 · λ / 2 > 0.
        let mut p = nmos_default();
        p.lambda = 0.02;
        let v = [5.0, 3.0, 0.0, 0.0];
        let lin = linearize_mosfet_level1(&p, &v);
        let v_ov: f64 = 3.0 - p.vto;
        let expected_gds = 0.5 * p.kp * v_ov * v_ov * p.lambda;
        assert!(
            approx(lin.jacobian[D][D], expected_gds, 1.0e-12, 1.0e-18),
            "expected gds = {expected_gds}, got {}",
            lin.jacobian[D][D],
        );
    }

    // -----------------------------------------------------------------
    // Region 3: triode
    // -----------------------------------------------------------------

    #[test]
    fn nmos_triode_matches_textbook_law() {
        let p = nmos_default();
        // V_gs = 3 V, V_th = 1 V → V_ov = 2 V; V_ds = 0.5 V < V_ov.
        let v = [0.5, 3.0, 0.0, 0.0];
        let id = drain_current_from_stamp(&p, &v);
        let expected = p.kp * (2.0 * 0.5 - 0.5 * 0.5 * 0.5);
        assert!(
            approx(id, expected, 1.0e-12, 1.0e-15),
            "expected {expected} A, got {id} A",
        );
    }

    #[test]
    fn nmos_triode_to_saturation_is_continuous() {
        // At V_ds = V_ov the triode and saturation formulas agree;
        // sweeping V_ds across the boundary should produce a smooth
        // I_d(V_ds) curve.
        let p = nmos_default();
        let v_ov = 1.5_f64;
        let v_gs = p.vto + v_ov;

        let eps = 1.0e-6;
        let v_below = [v_ov - eps, v_gs, 0.0, 0.0];
        let v_above = [v_ov + eps, v_gs, 0.0, 0.0];
        let id_below = drain_current_from_stamp(&p, &v_below);
        let id_above = drain_current_from_stamp(&p, &v_above);
        assert!(
            approx(id_below, id_above, 1.0e-6, 1.0e-12),
            "I_d discontinuous at triode/saturation boundary: {id_below} vs {id_above}",
        );
    }

    // -----------------------------------------------------------------
    // PMOS
    // -----------------------------------------------------------------

    #[test]
    fn pmos_saturation_drain_current_is_negative_for_negative_vds() {
        // Standard PMOS: V_s = +VDD, V_g pulled below threshold, drain
        // pulled below source. I_d flows from source to drain inside
        // the device, which means terminal current entering the drain
        // is negative in our convention.
        let p = pmos_default();
        let vdd = 3.3;
        // V_sg = 3.3 - 1.3 = 2.0, |Vtp| = 1 → V_ov_eq = 1.0
        // V_sd = 3.3 - 0.3 = 3.0 > V_ov_eq → saturation
        let v = [0.3, 1.3, vdd, vdd];
        let id = drain_current_from_stamp(&p, &v);
        let expected_mag = 0.5 * p.kp * 1.0_f64 * 1.0_f64;
        assert!(id < 0.0, "PMOS drain current must be negative, got {id}");
        assert!(
            approx(id, -expected_mag, 1.0e-12, 1.0e-15),
            "expected {} A, got {id} A",
            -expected_mag,
        );
    }

    #[test]
    fn pmos_cutoff_returns_zero() {
        // V_s = 3.3, V_g = 3.0 → V_sg_eq = 0.3 V < |Vtp| = 1 V → cutoff.
        let p = pmos_default();
        let v = [0.0, 3.0, 3.3, 3.3];
        let id = drain_current_from_stamp(&p, &v);
        assert!(approx(id, 0.0, 0.0, 1.0e-15), "expected cutoff, got {id} A");
    }

    // -----------------------------------------------------------------
    // Body effect: GAMMA, PHI shift the threshold up when V_bs < 0.
    // -----------------------------------------------------------------

    #[test]
    fn body_effect_raises_threshold_when_source_above_bulk() {
        let mut p = nmos_default();
        p.gamma = 0.4; // sqrt(V)
        p.phi = 0.7; // V
                     // Source above bulk: vbs = -1 V (reverse bias). New threshold:
                     //   V_th = VTO + γ·(sqrt(φ + 1) - sqrt(φ))
                     //        = 1.0 + 0.4·(sqrt(1.7) - sqrt(0.7))
        let expected_vth = 1.0 + 0.4 * ((1.7_f64).sqrt() - (0.7_f64).sqrt());
        // Put V_gs exactly at expected_vth; device should be at the
        // cutoff/conduction boundary → I_d ≈ 0.
        let v = [2.0, expected_vth, 1.0, 0.0];
        let id = drain_current_from_stamp(&p, &v);
        assert!(
            approx(id, 0.0, 0.0, 1.0e-12),
            "expected I_d ≈ 0 at boundary, got {id}",
        );
    }

    #[test]
    fn body_effect_partial_matches_finite_difference() {
        let mut p = nmos_default();
        p.gamma = 0.4;
        p.phi = 0.7;
        // Bias deep into saturation with reverse body bias.
        let v = [3.0, 3.0, 0.5, 0.0]; // V_s = 0.5, V_b = 0 → vbs = -0.5
        let lin = linearize_mosfet_level1(&p, &v);

        // gmb (analytic) = J[D, B] in our convention.
        let gmb_analytic = lin.jacobian[D][B];

        let gmb_numeric = finite_diff_id(&p, v, B);
        assert!(
            approx(gmb_analytic, gmb_numeric, 1.0e-5, 1.0e-12),
            "gmb analytic vs numeric mismatch: {gmb_analytic} vs {gmb_numeric}",
        );
        // gmb should be > 0 because raising V_b (toward V_s) lowers
        // the threshold and increases I_d.
        assert!(
            gmb_analytic > 0.0,
            "expected positive gmb, got {gmb_analytic}"
        );
    }

    // -----------------------------------------------------------------
    // Jacobian consistency: every entry matches a central-difference
    // numerical derivative of I_D(V*).
    // -----------------------------------------------------------------

    #[test]
    fn jacobian_drain_row_matches_finite_difference_in_saturation() {
        let mut p = nmos_default();
        p.lambda = 0.02;
        let v = [4.0, 3.0, 0.0, 0.0]; // saturation
        let lin = linearize_mosfet_level1(&p, &v);

        for (j, &j_entry) in lin.jacobian[D].iter().enumerate() {
            let numeric = finite_diff_id(&p, v, j);
            assert!(
                approx(j_entry, numeric, 1.0e-5, 1.0e-12),
                "J[D, {j}] analytic vs numeric mismatch: {j_entry} vs {numeric}",
            );
        }
    }

    #[test]
    fn jacobian_drain_row_matches_finite_difference_in_triode() {
        let mut p = nmos_default();
        p.lambda = 0.02;
        let v = [0.5, 3.0, 0.0, 0.0]; // triode
        let lin = linearize_mosfet_level1(&p, &v);

        for (j, &j_entry) in lin.jacobian[D].iter().enumerate() {
            let numeric = finite_diff_id(&p, v, j);
            assert!(
                approx(j_entry, numeric, 1.0e-5, 1.0e-12),
                "J[D, {j}] analytic vs numeric mismatch in triode: {j_entry} vs {numeric}",
            );
        }
    }

    #[test]
    fn jacobian_drain_and_source_rows_are_negatives() {
        // ADR-0005 layout contract: source row mirrors drain row
        // because I_S = -I_D in Level-1.
        let p = nmos_default();
        let v = [4.0, 3.0, 0.0, 0.0];
        let lin = linearize_mosfet_level1(&p, &v);
        for (j, (&d_entry, &s_entry)) in lin.jacobian[D]
            .iter()
            .zip(lin.jacobian[S].iter())
            .enumerate()
        {
            assert!(
                approx(s_entry, -d_entry, 1.0e-15, 1.0e-15),
                "source row must be -drain row, mismatch at column {j}",
            );
        }
    }

    #[test]
    fn jacobian_gate_and_bulk_rows_are_zero() {
        // Level-1 does not model gate leakage or body junction diodes;
        // the corresponding terminal currents are identically zero,
        // and therefore so are their Jacobian rows.
        let p = nmos_default();
        let v = [4.0, 3.0, 0.0, 0.0];
        let lin = linearize_mosfet_level1(&p, &v);
        for &entry in &lin.jacobian[G] {
            assert!(entry.abs() <= 0.0, "expected zero, got {entry}");
        }
        for &entry in &lin.jacobian[B] {
            assert!(entry.abs() <= 0.0, "expected zero, got {entry}");
        }
    }

    // -----------------------------------------------------------------
    // Companion-current identity: G·V* + I_eq = I(V*).
    // -----------------------------------------------------------------

    #[test]
    fn companion_identity_reproduces_terminal_currents() {
        // Sweep a handful of operating points across all three regions
        // and assert the stamp identity holds:
        //   Σ_j J[k, j] · V*_j + I_eq_k = I_k(V*).
        let p = nmos_default();
        let points: &[[f64; MOSFET_TERMINALS]] = &[
            [0.5, 0.5, 0.0, 0.0], // cutoff
            [0.5, 3.0, 0.0, 0.0], // triode
            [4.0, 3.0, 0.0, 0.0], // saturation
        ];
        for v in points {
            let lin = linearize_mosfet_level1(&p, v);
            // Expected terminal current at drain.
            let id = drain_current_from_stamp(&p, v);
            // The same value via the stamp identity for drain.
            let mut reconstructed = lin.companion_current[D];
            for (j, &vj) in v.iter().enumerate() {
                reconstructed += lin.jacobian[D][j] * vj;
            }
            assert!(
                approx(id, reconstructed, 1.0e-12, 1.0e-15),
                "companion identity violated at v={v:?}: id={id} reconstructed={reconstructed}",
            );
            // I_S = -I_D at every operating point.
            let mut is_reconstructed = lin.companion_current[S];
            for (j, &vj) in v.iter().enumerate() {
                is_reconstructed += lin.jacobian[S][j] * vj;
            }
            assert!(
                approx(is_reconstructed, -id, 1.0e-12, 1.0e-15),
                "source companion identity violated at v={v:?}",
            );
        }
    }
}
