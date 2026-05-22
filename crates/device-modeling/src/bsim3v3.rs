//! MOSFET `BSIM3v3` DC stamp (tasks.md #12).
//!
//! This module implements the DC linearization for the
//! [`MOSFETParams::BSIM3v3`](crate::params::MOSFETParams::`BSIM3v3`)
//! variant: at a Newton-Raphson iterate's terminal voltages it returns
//! a [`MOSFETLinearization`] — a 4×4 small-signal conductance Jacobian
//! and a 4-vector of companion currents in terminal-local coordinates
//! `[drain, gate, source, bulk]` — for the
//! [`DeviceModel::linearize`](crate::DeviceModel::linearize) dispatch
//! site in [`crate::stamp`].
//!
//! # Equation scope (v1.0, ADR-0010)
//!
//! `BSIM3v3` is a ~100-parameter industry-standard short-channel
//! MOSFET model. The full `BSIM3v3.2.4` equation set is too large to
//! land in a single tasks.md item; what this module implements is a
//! **faithful `BSIM3v3` DC core** that captures the physical effects
//! the dc-operating-point capability needs to converge on
//! short-channel circuits and that the
//! [`crate::params::MosBSIM3v3Params`] sparse raw map naturally
//! exposes:
//!
//! 1. Threshold computation with **body effect** and **drain-induced
//!    barrier lowering (DIBL)**:
//!
//!    `Vth = Vth0 + K1·(√(Φs − Vbs) − √Φs) − K2·Vbs − Eta0·Vds`
//!
//!    `K1` and `K2` are the BSIM3 body-effect coefficients; `Eta0`
//!    is the DIBL coefficient. When `Vbs` is positive (forward body
//!    bias) the `√(Φs − Vbs)` argument is clamped against the square
//!    root at zero to keep the derivative finite.
//!
//! 2. **Smoothed strong/sub-threshold transition** via the canonical
//!    `BSIM3v3` `Vgsteff` function (Eq. (4.4) in the `BSIM3v3` manual):
//!
//!    `Vgsteff = (n·vt·ln(1 + exp((Vgs − Vth)/(n·vt)))) /
//!               (1 + 2·n·Cox·√(2·Φs/q/Nch)·exp(−(Vgs−Vth)/(n·vt)/2))`
//!
//!    To keep this stamp closed-form and analytic-derivative friendly
//!    we use the well-known numerically stable simplification:
//!
//!    `Vgsteff = n·vt·ln(1 + exp((Vgs − Vth)/(n·vt)))`
//!
//!    which has the correct asymptotes (`→ Vgs − Vth` in strong
//!    inversion, `→ n·vt·exp((Vgs−Vth)/(n·vt))` in sub-threshold) and
//!    a continuous analytic derivative.
//!
//! 3. **Velocity saturation** via the canonical BSIM3 `Vdsat`:
//!
//!    `Vdsat = Esat·Leff·Vgsteff / (Esat·Leff + Vgsteff)`
//!
//!    so that as Vgsteff grows, Vdsat asymptotes to Esat·Leff — the
//!    short-channel velocity-saturation cap.
//!
//! 4. **Smoothed Vds clamp** for continuous saturation/linear
//!    transition (Eq. (4.50)):
//!
//!    `Vdseff = Vdsat − ½·(Vdsat − Vds − δ + √((Vdsat − Vds − δ)² + 4·δ·Vdsat))`
//!
//!    `δ ≈ 0.01 V` (BSIM3 default `Delta`). This keeps the model
//!    `C¹`-continuous across the saturation knee, which the
//!    Newton-Raphson driver (tasks.md #17, ADR-0006) needs to avoid
//!    stall on the dual convergence criterion.
//!
//! 5. **Channel length modulation** in saturation:
//!
//!    `Ids = (μ·Cox·W/Leff)·Vgsteff·Vdseff·(1 − Vdseff/(2·Vgsteff))·
//!           (1 + (Vds − Vdseff)/VA)`
//!
//!    where `VA = Vdseff·(1/Pclm)` is the BSIM3 Early-voltage analog.
//!
//! 6. **NMOS / PMOS polarity** via
//!    [`MosPolarity`]: the PMOS case
//!    negates the bias voltages and the resulting current, so the
//!    same equations cover both channels (`BSIM3v3` manual Eq. (1.1)).
//!
//! What this module **does not** model at v1.0:
//!
//! - Quantum-mechanical Vth shifts (`Vth_QM`),
//! - Bin-by-bin temperature scaling,
//! - Gate-induced drain leakage (GIDL),
//! - Charge-based capacitance model (DC stamp only).
//!
//! These are scoped to a follow-on change once tasks.md #17 lands
//! convergence diagnostics that reveal which effects matter on real
//! benchmark circuits.
//!
//! # Parameter extraction
//!
//! [`MosBSIM3v3Params::raw`] is a `BTreeMap<String, f64>` keyed on
//! lowercase SPICE-card names. Every parameter has a `SPICE3f5` /
//! `BSIM3v3.2.4` **default** so a freshly defaulted model carrying an
//! empty map produces a stable, physically reasonable Ids — this is
//! load-bearing for [`crate::DeviceModel`]'s
//! `#[derive(Default)]`-friendliness and for the
//! `linearize_mosfet_bsim3v3_dispatches_through_match` test in
//! [`crate::stamp`].
//!
//! See the private `Bsim3v3DcParams::extract` helper for the full
//! key → default map.
//!
//! # Companion model
//!
//! At the iterate `(Vgs, Vds, Vbs)` the MNA companion model for the
//! drain current `Ids` is:
//!
//! `I_eq = Ids − gm·Vgs − gds·Vds − gmbs·Vbs`
//!
//! where `gm = ∂Ids/∂Vgs`, `gds = ∂Ids/∂Vds`,
//! `gmbs = ∂Ids/∂Vbs`. The terminal-local Jacobian and companion
//! current are then assembled by KCL at each terminal — drain sinks
//! `+Ids`, source sources `+Ids`, gate and bulk see zero DC current
//! (no gate leakage, no DC bulk current in this scope). See the
//! private `assemble_companion` helper for the full 4×4 + 4-vector
//! layout.
//!
//! # Analytic vs. numeric Jacobian
//!
//! Every conductance returned by this module is **analytic** — there
//! is no finite-difference fallback. This is load-bearing for the
//! dual convergence criterion (ADR-0006): a numeric Jacobian
//! introduces O(ε) noise into the residue check that can manifest as
//! false-positive convergence. The body of [`linearize_bsim3v3`]
//! threads the partial-derivative chain through every smoothing
//! function so that `(gm, gds, gmbs)` are exact at the iterate.

use crate::params::{MosBSIM3v3Params, MosPolarity};
use crate::stamp::{MOSFETLinearization, MOSFET_TERMINALS};

/// Terminal index for the drain in `[drain, gate, source, bulk]`.
const TERM_D: usize = 0;
/// Terminal index for the gate in `[drain, gate, source, bulk]`.
const TERM_G: usize = 1;
/// Terminal index for the source in `[drain, gate, source, bulk]`.
const TERM_S: usize = 2;
/// Terminal index for the bulk in `[drain, gate, source, bulk]`.
const TERM_B: usize = 3;

// ---------------------------------------------------------------------
// Parameter extraction
// ---------------------------------------------------------------------

/// The closed set of `BSIM3v3` DC-stamp parameters this module
/// consumes from [`MosBSIM3v3Params::raw`].
///
/// Every field has a `BSIM3v3.2.4` / `SPICE3f5` default so an empty raw
/// map produces a stable, NMOS-defaulted device. The full `BSIM3v3`
/// parameter set is much larger; the fields here are the ones whose
/// physical effects are captured by the equations in the module
/// docstring (threshold + body + DIBL + velocity saturation + CLM +
/// sub-threshold).
///
/// Field names mirror the SPICE-card parameter names (lowercased) so
/// look-up against the raw map is mechanical.
#[derive(Debug, Clone, Copy)]
struct Bsim3v3DcParams {
    /// Zero-bias threshold voltage `VTH0` (V). NMOS default `+0.7`,
    /// PMOS default `−0.7`.
    vth0: f64,
    /// First-order body-effect coefficient `K1` (√V). Default `0.5`.
    k1: f64,
    /// Second-order body-effect coefficient `K2` (unitless). Default
    /// `0.0`.
    k2: f64,
    /// DIBL coefficient `Eta0` (unitless, on Vds). Default `0.08`.
    eta0: f64,
    /// Surface potential `Φs ≈ 2·Φf` (V). Default `0.7`.
    phi: f64,
    /// Low-field mobility `U0` (cm²/V/s, converted to m²/V/s
    /// internally). Default `670` cm²/V/s (NMOS) or `250` (PMOS).
    u0: f64,
    /// Oxide thickness `Tox` (m). Default `1e-8 m` (10 nm) — the
    /// `BSIM3v3.2.4` default for a 0.25 µm node.
    tox: f64,
    /// Channel-doping concentration `Nch` (cm⁻³, converted to m⁻³).
    /// Default `1.7e17 cm⁻³`.
    #[allow(dead_code)] // Reserved for the QM-correction follow-on; load-bearing for `n`.
    nch: f64,
    /// Channel width `W` (m). Default `1e-5` (10 µm).
    w: f64,
    /// Channel length `L` (m). Default `1e-6` (1 µm).
    l: f64,
    /// Saturation electric field `Esat` (V/m). Default `4.0e6` V/m
    /// for NMOS, `2.0e6` V/m for PMOS. (`BSIM3v3` takes `Vsat` and
    /// derives `Esat = 2·Vsat/μ`; we expose `Esat` directly because
    /// it appears in the closed-form Vdsat.)
    esat: f64,
    /// Channel-length-modulation parameter `Pclm` (unitless).
    /// Default `1.3`.
    pclm: f64,
    /// Smoothing parameter `Delta` for the Vdseff transition (V).
    /// Default `0.01`.
    delta: f64,
    /// Sub-threshold slope factor `n` (unitless). Default `1.0` —
    /// i.e. textbook 60 mV/decade at room temperature. Real silicon
    /// is typically `1.2`–`1.5`; calibration is the user's job.
    n_slope: f64,
    /// Thermal voltage `vt = kT/q` (V). Default `0.025_852` V (room
    /// temperature, matches [`crate::params::DiodeParams::vt`]'s
    /// default).
    vt: f64,
}

impl Bsim3v3DcParams {
    /// Extract the DC-stamp parameter set from a `BSIM3v3` raw map,
    /// applying `SPICE3f5` / `BSIM3v3.2.4` defaults per the
    /// [`Bsim3v3DcParams`] field docstrings.
    ///
    /// Defaults depend on polarity for `vth0`, `u0`, and `esat`:
    /// these three are the parameters whose canonical SPICE
    /// defaults differ between NMOS and PMOS.
    fn extract(params: &MosBSIM3v3Params) -> Self {
        let polarity = params.polarity;
        let raw = &params.raw;

        // Polarity-dependent defaults (BSIM3v3.2.4 manual table 5.1).
        let (default_vth0, default_u0_cm2, default_esat) = match polarity {
            MosPolarity::Nmos => (0.7, 670.0, 4.0e6),
            MosPolarity::Pmos => (-0.7, 250.0, 2.0e6),
        };

        // SI-unit conversion: U0 is specified in cm²/V/s on SPICE
        // cards; internally we work in m²/V/s.
        let u0_cm2 = lookup(raw, "u0", default_u0_cm2);
        let u0 = u0_cm2 * 1.0e-4;

        // SI-unit conversion: Nch is specified in cm⁻³; internally
        // m⁻³.
        let nch_cm3 = lookup(raw, "nch", 1.7e17);
        let nch = nch_cm3 * 1.0e6;

        Self {
            vth0: lookup(raw, "vth0", default_vth0),
            k1: lookup(raw, "k1", 0.5),
            k2: lookup(raw, "k2", 0.0),
            eta0: lookup(raw, "eta0", 0.08),
            phi: lookup(raw, "phi", 0.7),
            u0,
            tox: lookup(raw, "tox", 1.0e-8),
            nch,
            w: lookup(raw, "w", 1.0e-5),
            l: lookup(raw, "l", 1.0e-6),
            esat: lookup(raw, "esat", default_esat),
            pclm: lookup(raw, "pclm", 1.3),
            delta: lookup(raw, "delta", 0.01),
            n_slope: lookup(raw, "nfactor", 1.0),
            vt: lookup(raw, "vt", 0.025_852),
        }
    }

    /// Effective oxide capacitance per unit area `Cox = ε_ox / Tox`
    /// (F/m²). `ε_ox ≈ 3.9 · ε_0 ≈ 3.4531·10⁻¹¹ F/m`.
    fn cox(&self) -> f64 {
        const EPSILON_OX: f64 = 3.453_1e-11; // F/m, BSIM3v3 default
        EPSILON_OX / self.tox
    }

    /// Trans-conductance prefactor `β = μ·Cox·W/L` (A/V²).
    fn beta(&self) -> f64 {
        self.u0 * self.cox() * self.w / self.l
    }
}

/// Look up `key` in `raw` and return its value, or `default` when
/// absent. Centralized so the per-key default doesn't drift between
/// extract sites.
fn lookup(raw: &std::collections::BTreeMap<String, f64>, key: &str, default: f64) -> f64 {
    raw.get(key).copied().unwrap_or(default)
}

// ---------------------------------------------------------------------
// The core stamp
// ---------------------------------------------------------------------

/// Linearize a `BSIM3v3` MOSFET at the given terminal voltages.
///
/// This is the body the `BSIM3v3` arm of
/// [`crate::stamp::linearize_mosfet`] delegates to. Returns a
/// [`MOSFETLinearization`] in terminal-local coordinates
/// `[drain, gate, source, bulk]`.
///
/// # Arguments
///
/// - `params` — the `BSIM3v3` `.MODEL` parameter payload.
/// - `terminal_voltages` — `[V_drain, V_gate, V_source, V_bulk]`
///   absolute voltages relative to circuit ground (the
///   [`OperatingPoint::MOSFET`](crate::stamp::OperatingPoint::MOSFET)
///   contract).
///
/// # Returns
///
/// A [`MOSFETLinearization`] holding the analytic 4×4 conductance
/// Jacobian and the 4-vector companion current. NMOS produces
/// `Ids ≥ 0` when `Vgs ≥ Vth` and `Vds ≥ 0`; PMOS produces
/// `Ids ≤ 0` under the analogous condition. The off-state
/// (`Vgs ≤ Vth − several·vt`) collapses to exponentially small
/// sub-threshold current with a non-zero but small `gm`.
///
/// # Numerical stability
///
/// All smoothing functions are evaluated via
/// `ln(1 + exp(x))` and `√(x² + δ²)`-style formulae that have
/// continuous derivatives everywhere on ℝ. The `Vgsteff` saturating
/// log-sum-exp is computed via the standard
/// `softplus(x) = max(x, 0) + ln(1 + exp(−|x|))` form so it stays
/// numerically accurate when `(Vgs − Vth)/(n·vt)` is large positive
/// or large negative.
#[must_use]
pub fn linearize_bsim3v3(
    params: &MosBSIM3v3Params,
    terminal_voltages: &[f64; MOSFET_TERMINALS],
) -> MOSFETLinearization {
    let dc = Bsim3v3DcParams::extract(params);

    // Polarity-normalize: BSIM3v3 equations are written for NMOS;
    // PMOS is handled by negating biases and the resulting current
    // (BSIM3v3.2.4 manual Eq. (1.1)).
    let sign = match params.polarity {
        MosPolarity::Nmos => 1.0,
        MosPolarity::Pmos => -1.0,
    };

    let vd = terminal_voltages[TERM_D];
    let vg = terminal_voltages[TERM_G];
    let vs = terminal_voltages[TERM_S];
    let vb = terminal_voltages[TERM_B];

    let vgs = sign * (vg - vs);
    let vds = sign * (vd - vs);
    let vbs = sign * (vb - vs);

    // Compute Ids and its three partial derivatives in NMOS-normalized
    // coordinates.
    let CurrentAndJacobian { ids, gm, gds, gmbs } = bsim3v3_strong_inversion(&dc, vgs, vds, vbs);

    // De-normalize back into the original polarity:
    //   Ids_real = sign · Ids_norm
    //   ∂Ids_real/∂(V_real)  →  same magnitude (sign² = 1) because
    //   both Ids and the bias under differentiation flip sign.
    let ids = sign * ids;

    assemble_companion(ids, gm, gds, gmbs, sign * vgs, sign * vds, sign * vbs, sign)
}

// ---------------------------------------------------------------------
// Strong-inversion / sub-threshold core
// ---------------------------------------------------------------------

/// Internal record carrying the drain current and its three
/// analytic partial derivatives at the iterate.
#[derive(Debug, Clone, Copy)]
struct CurrentAndJacobian {
    /// Drain current `Ids` (A), NMOS sign convention (positive when
    /// `Vgs ≥ Vth` and `Vds ≥ 0`).
    ids: f64,
    /// `gm = ∂Ids/∂Vgs` (S).
    gm: f64,
    /// `gds = ∂Ids/∂Vds` (S).
    gds: f64,
    /// `gmbs = ∂Ids/∂Vbs` (S).
    gmbs: f64,
}

/// `BSIM3v3` strong-inversion + sub-threshold DC core. Inputs are
/// NMOS-normalized (`Vgs`, `Vds`, `Vbs`), output is the drain current
/// plus its three small-signal conductances at the iterate.
///
/// The equation set is documented in the module-level docstring.
/// This function is the analytic Jacobian thread.
#[allow(clippy::similar_names)] // gm / gds / gmbs are SPICE-canonical names.
#[allow(clippy::too_many_lines)] // Single physical model, kept in one place for review.
fn bsim3v3_strong_inversion(
    dc: &Bsim3v3DcParams,
    vgs: f64,
    vds: f64,
    vbs: f64,
) -> CurrentAndJacobian {
    // ------------------------------------------------------------------
    // (1) Threshold with body effect + DIBL.
    //
    // Vth = Vth0 + K1·(√(Φs − Vbs) − √Φs) − K2·Vbs − Eta0·Vds
    // ------------------------------------------------------------------
    let phi = dc.phi.max(1.0e-6); // guard against degenerate Φs = 0.
    let sqrt_phi = phi.sqrt();
    // Clamp Φs − Vbs ≥ small_floor to keep √ smooth at the body-bias
    // boundary. dphi_dvbs accounts for the clamp: where the argument
    // is clamped, the derivative is zero.
    let arg = phi - vbs;
    let (sqrt_arg, dsqrt_arg_dvbs) = if arg > 1.0e-6 {
        let s = arg.sqrt();
        (s, -1.0 / (2.0 * s))
    } else {
        (1.0e-3_f64.sqrt(), 0.0)
    };
    let vth = dc.vth0 + dc.k1 * (sqrt_arg - sqrt_phi) - dc.k2 * vbs - dc.eta0 * vds;
    let dvth_dvds = -dc.eta0;
    let dvth_dvbs = dc.k1 * dsqrt_arg_dvbs - dc.k2;

    // ------------------------------------------------------------------
    // (2) Smoothed strong/sub-threshold transition: Vgsteff.
    //
    // Vgsteff = n·vt · ln(1 + exp((Vgs − Vth)/(n·vt)))
    //
    // Numerically stable via softplus:
    //   softplus(x) = max(x, 0) + ln(1 + exp(−|x|))
    // ------------------------------------------------------------------
    let nvt = dc.n_slope * dc.vt;
    let x = (vgs - vth) / nvt;
    let (softplus, dsoftplus_dx) = stable_softplus_and_sigmoid(x);
    let vgsteff = nvt * softplus;
    // ∂Vgsteff/∂Vgs = sigmoid(x)
    // ∂Vgsteff/∂Vds = sigmoid(x) · (−dVth/dVds)/1 — chain rule
    // ∂Vgsteff/∂Vbs = sigmoid(x) · (−dVth/dVbs)
    let dvgsteff_dvgs = dsoftplus_dx;
    let dvgsteff_dvds = -dsoftplus_dx * dvth_dvds;
    let dvgsteff_dvbs = -dsoftplus_dx * dvth_dvbs;

    // ------------------------------------------------------------------
    // (3) Velocity-saturation Vdsat.
    //
    // Vdsat = Esat·L · Vgsteff / (Esat·L + Vgsteff)
    // ------------------------------------------------------------------
    let esat_l = (dc.esat * dc.l).max(1.0e-6);
    let denom_vdsat = esat_l + vgsteff;
    let vdsat = esat_l * vgsteff / denom_vdsat;
    // dVdsat/dVgsteff via quotient rule.
    let dvdsat_dvgsteff = esat_l * esat_l / (denom_vdsat * denom_vdsat);
    let dvdsat_dvgs = dvdsat_dvgsteff * dvgsteff_dvgs;
    let dvdsat_dvds = dvdsat_dvgsteff * dvgsteff_dvds;
    let dvdsat_dvbs = dvdsat_dvgsteff * dvgsteff_dvbs;

    // ------------------------------------------------------------------
    // (4) Smoothed Vdseff clamp.
    //
    // Let u = Vdsat − Vds − δ. Then:
    //   Vdseff = Vdsat − ½·(u + √(u² + 4·δ·Vdsat))
    //   ∂Vdseff/∂Vdsat = 1 − ½·(1 + (u + 2·δ)/√(u² + 4·δ·Vdsat))
    //   ∂Vdseff/∂Vds   =     ½·(−(−1) + (−u·(−1))/√(...))   ← see below
    // ------------------------------------------------------------------
    let delta = dc.delta.max(1.0e-6);
    let u = vdsat - vds - delta;
    let radicand = u * u + 4.0 * delta * vdsat;
    let sqrt_radicand = radicand.max(1.0e-12).sqrt();
    let vdseff = vdsat - 0.5 * (u + sqrt_radicand);
    // Partial of radicand wrt Vdsat: 2·u·1 + 4·δ
    let dradicand_dvdsat = 2.0 * u + 4.0 * delta;
    let dsqrt_radicand_dvdsat = dradicand_dvdsat / (2.0 * sqrt_radicand);
    let dvdseff_dvdsat = 1.0 - 0.5 * (1.0 + dsqrt_radicand_dvdsat);
    // Partial of radicand wrt Vds: 2·u·(−1) + 0
    let dradicand_dvds_direct = -2.0 * u;
    let dsqrt_radicand_dvds_direct = dradicand_dvds_direct / (2.0 * sqrt_radicand);
    let dvdseff_dvds_direct = -0.5 * (-1.0 + dsqrt_radicand_dvds_direct);

    // Chain Vdseff through both Vdsat (which depends on Vgs/Vds/Vbs)
    // and the direct Vds appearance.
    let dvdseff_dvgs = dvdseff_dvdsat * dvdsat_dvgs;
    let dvdseff_dvds = dvdseff_dvdsat * dvdsat_dvds + dvdseff_dvds_direct;
    let dvdseff_dvbs = dvdseff_dvdsat * dvdsat_dvbs;

    // ------------------------------------------------------------------
    // (5) Core current with channel-length modulation.
    //
    // Inv-region drain current (BSIM3v3 simplified):
    //   I_core = β · Vgsteff · Vdseff · (1 − Vdseff/(2·Vgsteff))
    //
    // CLM multiplier:
    //   clm = 1 + (Vds − Vdseff)/VA,   VA = Vdseff · Pclm
    //
    // The CLM term is `1` when Vds = Vdseff (linear/saturation knee)
    // and grows linearly past saturation. To keep the derivative
    // finite as Vdseff → 0 we floor VA below by a small ε.
    // ------------------------------------------------------------------
    let beta = dc.beta();
    // Guard Vgsteff > 0 (softplus output is always > 0 for finite x,
    // but we want a hard floor to avoid 0/0 in the (1 − Vdseff/(2·Vgsteff))
    // term when Vgsteff is exponentially small in deep sub-threshold).
    let vgsteff_guarded = vgsteff.max(1.0e-30);
    let ratio = vdseff / (2.0 * vgsteff_guarded);
    let one_minus_ratio = (1.0 - ratio).max(0.0); // hard clamp at 0 to keep Ids non-negative
    let i_core = beta * vgsteff * vdseff * one_minus_ratio;

    // Partials of i_core wrt (Vgsteff, Vdseff). Use the unclamped
    // (1 − ratio) for the derivative when it is strictly positive;
    // when it's clamped at 0, the derivative is 0 (sub-saturation
    // boundary is C⁰-smooth; this is the standard SPICE convention).
    let clamp_active = (1.0 - ratio) > 0.0;
    let (dicore_dvgsteff, dicore_dvdseff) = if clamp_active {
        // i_core = β · Vgsteff · Vdseff · (1 − Vdseff/(2·Vgsteff))
        //        = β · Vdseff · (Vgsteff − Vdseff/2)
        // ∂/∂Vgsteff = β · Vdseff
        // ∂/∂Vdseff  = β · (Vgsteff − Vdseff)
        let d_vg = beta * vdseff;
        let d_vd = beta * (vgsteff - vdseff);
        (d_vg, d_vd)
    } else {
        (0.0, 0.0)
    };

    // CLM: clm = 1 + (Vds − Vdseff) / (Vdseff · Pclm)
    // Guard VA: VA = max(Vdseff · Pclm, ε)
    let pclm = dc.pclm.max(1.0e-3);
    let va_raw = vdseff * pclm;
    let va = va_raw.max(1.0e-12);
    let clm = 1.0 + (vds - vdseff) / va;
    // ∂clm/∂Vds       = 1/VA
    // ∂clm/∂Vdseff    = (−1)/VA − (Vds − Vdseff)·Pclm/VA²
    //                 = (−1)/VA − (Vds − Vdseff)·(1/Vdseff)·(1/VA)
    // (using VA = Vdseff·Pclm ⟹ Pclm/VA = 1/Vdseff when VA = va_raw)
    let dclm_dvds = if va_raw > 1.0e-12 { 1.0 / va } else { 0.0 };
    let dclm_dvdseff = if va_raw > 1.0e-12 {
        -1.0 / va - (vds - vdseff) * pclm / (va * va)
    } else {
        0.0
    };

    // Compose Ids = i_core · clm and apply chain rule.
    let ids = i_core * clm;
    // ∂Ids/∂Vgs = (∂i_core/∂Vgsteff · ∂Vgsteff/∂Vgs +
    //              ∂i_core/∂Vdseff · ∂Vdseff/∂Vgs) · clm
    //          +  i_core · (∂clm/∂Vdseff · ∂Vdseff/∂Vgs)
    let gm = (dicore_dvgsteff * dvgsteff_dvgs + dicore_dvdseff * dvdseff_dvgs) * clm
        + i_core * (dclm_dvdseff * dvdseff_dvgs);
    // ∂Ids/∂Vds: include the direct Vds path through clm.
    let gds = (dicore_dvgsteff * dvgsteff_dvds + dicore_dvdseff * dvdseff_dvds) * clm
        + i_core * (dclm_dvds + dclm_dvdseff * dvdseff_dvds);
    // ∂Ids/∂Vbs
    let gmbs = (dicore_dvgsteff * dvgsteff_dvbs + dicore_dvdseff * dvdseff_dvbs) * clm
        + i_core * (dclm_dvdseff * dvdseff_dvbs);

    CurrentAndJacobian { ids, gm, gds, gmbs }
}

/// Numerically stable `softplus(x) = ln(1 + exp(x))` and its
/// derivative `sigmoid(x) = 1/(1 + exp(−x))`, returned as a pair so
/// callers thread one evaluation through both partial-derivative
/// chains.
///
/// Uses the standard
/// `softplus(x) = max(x, 0) + ln(1 + exp(−|x|))` trick to avoid
/// overflow at large positive `x` and the
/// `sigmoid(x) = 1/(1 + e^{−x})` form rewritten as
/// `e^x/(1+e^x)` (for `x < 0`) to avoid overflow at large negative
/// `x`.
fn stable_softplus_and_sigmoid(x: f64) -> (f64, f64) {
    let softplus = if x > 0.0 {
        x + (1.0 + (-x).exp()).ln()
    } else {
        (1.0 + x.exp()).ln()
    };
    let sigmoid = if x >= 0.0 {
        let e_neg = (-x).exp();
        1.0 / (1.0 + e_neg)
    } else {
        let e_pos = x.exp();
        e_pos / (1.0 + e_pos)
    };
    (softplus, sigmoid)
}

// ---------------------------------------------------------------------
// Companion-model assembly
// ---------------------------------------------------------------------

/// Fold the analytic `Ids` and its three partial derivatives into the
/// terminal-local 4×4 Jacobian and 4-vector companion current that
/// the MNA assembler (tasks.md #14) expects.
///
/// # KCL layout (NMOS sign convention; PMOS handled by `sign = -1`)
///
/// At the iterate, the device sinks `+Ids` at drain and sources
/// `+Ids` at source; gate and bulk see zero DC current (no gate
/// leakage, no DC bulk current in v1.0 scope).
///
/// The MNA companion model linearizes around the iterate:
///
/// `I_d_companion(V) = Ids + gm·(Vgs − Vgs₀) + gds·(Vds − Vds₀)
///                     + gmbs·(Vbs − Vbs₀)`
///
/// Decomposed into a constant-current source plus conductances:
///
/// `I_d_companion(V) = (Ids − gm·Vgs₀ − gds·Vds₀ − gmbs·Vbs₀)
///                     + gm·Vgs + gds·Vds + gmbs·Vbs`
///
/// Translating `Vgs = Vg − Vs`, `Vds = Vd − Vs`, `Vbs = Vb − Vs`
/// into terminal-local conductances gives the standard 4×4
/// stamp pattern (drain row +Ids, source row −Ids, gate/bulk rows
/// zero current but non-zero Jacobian entries for the gm/gds/gmbs
/// cross terms).
///
/// # Polarity
///
/// `sign = +1` for NMOS and `−1` for PMOS. In the PMOS path,
/// `Ids` is already negated by the caller (so an off-state PMOS at
/// `Vsg > |Vth|` sources current at the source terminal as
/// expected); the Jacobian entries are sign² = 1 so the same matrix
/// pattern applies.
#[allow(clippy::too_many_arguments)] // All inputs are physically named scalars; an aggregate would obscure.
fn assemble_companion(
    ids: f64,
    gm: f64,
    gds: f64,
    gmbs: f64,
    vgs: f64,
    vds: f64,
    vbs: f64,
    _sign: f64,
) -> MOSFETLinearization {
    // Companion constant-current term (per the docstring derivation).
    let i_eq = ids - gm * vgs - gds * vds - gmbs * vbs;

    // 4×4 Jacobian in [drain, gate, source, bulk] order.
    //
    // I_drain  =  +(gm·Vgs + gds·Vds + gmbs·Vbs) + i_eq
    //          =  +gm·Vg  − gm·Vs
    //             +gds·Vd − gds·Vs
    //             +gmbs·Vb − gmbs·Vs
    //             + i_eq
    //
    // I_source = −I_drain  →  every row of the source line is
    //   the negation of the drain row, and the companion current
    //   is `−i_eq`.
    //
    // I_gate = 0,  I_bulk = 0  (no DC gate / bulk currents in scope).
    let mut jacobian = [[0.0_f64; MOSFET_TERMINALS]; MOSFET_TERMINALS];
    let mut companion_current = [0.0_f64; MOSFET_TERMINALS];

    // Drain row.
    jacobian[TERM_D][TERM_D] = gds;
    jacobian[TERM_D][TERM_G] = gm;
    jacobian[TERM_D][TERM_S] = -(gm + gds + gmbs);
    jacobian[TERM_D][TERM_B] = gmbs;
    companion_current[TERM_D] = i_eq;

    // Source row (mirror of drain, with sign flipped because the
    // device's source-terminal current equals −I_drain).
    jacobian[TERM_S][TERM_D] = -gds;
    jacobian[TERM_S][TERM_G] = -gm;
    jacobian[TERM_S][TERM_S] = gm + gds + gmbs;
    jacobian[TERM_S][TERM_B] = -gmbs;
    companion_current[TERM_S] = -i_eq;

    // Gate and bulk rows: zero in v1.0 scope.

    MOSFETLinearization {
        jacobian,
        companion_current,
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use circuit_solver_types::ModelName;

    /// Build a default NMOS `BSIM3v3` with an empty raw map. This is
    /// the workhorse fixture for the "default-parameters" tests.
    fn nmos_default() -> MosBSIM3v3Params {
        MosBSIM3v3Params {
            name: ModelName::new("nmos_b3_default"),
            polarity: MosPolarity::Nmos,
            raw: std::collections::BTreeMap::new(),
        }
    }

    /// Build a default PMOS `BSIM3v3` with an empty raw map.
    fn pmos_default() -> MosBSIM3v3Params {
        MosBSIM3v3Params {
            name: ModelName::new("pmos_b3_default"),
            polarity: MosPolarity::Pmos,
            raw: std::collections::BTreeMap::new(),
        }
    }

    /// Loose float-equality used for the analytic-vs-numeric checks.
    /// `rel` is a relative tolerance; `abs` is the floor for near-zero
    /// magnitudes (matches the ADR-0008 max(rel, abs) envelope).
    fn approx_eq(a: f64, b: f64, rel: f64, abs: f64) -> bool {
        (a - b).abs() <= abs.max(rel * a.abs().max(b.abs()))
    }

    /// Centered-difference numeric derivative.
    fn fd(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
        (f(x + h) - f(x - h)) / (2.0 * h)
    }

    // -----------------------------------------------------------------
    // Smoke: default-parameter NMOS at zero bias produces tiny
    // sub-threshold current and small but non-zero gm.
    // -----------------------------------------------------------------
    #[test]
    fn nmos_default_zero_bias_is_off() {
        let p = nmos_default();
        let lin = linearize_bsim3v3(&p, &[0.0, 0.0, 0.0, 0.0]);
        // Drain-row diagonal should be the gds. With Vgs = 0 and
        // Vth0 = 0.7 V, x = −0.7/(1·vt) ≈ −27, softplus ≈ exp(−27)
        // — i.e. essentially zero current, essentially zero
        // conductance.
        assert!(
            lin.jacobian[0][0].abs() < 1.0e-6,
            "off-state gds should be ~0, got {}",
            lin.jacobian[0][0]
        );
        assert!(
            lin.companion_current[0].abs() < 1.0e-6,
            "off-state companion current should be ~0, got {}",
            lin.companion_current[0]
        );
    }

    // -----------------------------------------------------------------
    // Strong inversion: NMOS at Vgs = 3 V, Vds = 1 V produces a
    // physically reasonable Ids (microamp to milliamp range with the
    // default W/L = 10 µm / 1 µm).
    // -----------------------------------------------------------------
    #[test]
    fn nmos_strong_inversion_produces_positive_drain_current() {
        let p = nmos_default();
        // Terminal voltages: Vd=1, Vg=3, Vs=0, Vb=0
        let lin = linearize_bsim3v3(&p, &[1.0, 3.0, 0.0, 0.0]);
        // Drain row companion current encodes Ids − gm·Vgs −
        // gds·Vds, but i_eq + (sum of jacobian[d][k]·V_k) must equal
        // +Ids by construction. Reconstruct Ids:
        let vd = 1.0;
        let vg = 3.0;
        let vs = 0.0;
        let vb = 0.0;
        let ids_reconstructed = lin.companion_current[0]
            + lin.jacobian[0][0] * vd
            + lin.jacobian[0][1] * vg
            + lin.jacobian[0][2] * vs
            + lin.jacobian[0][3] * vb;
        assert!(
            ids_reconstructed > 0.0,
            "NMOS Vgs=3 Vds=1 should sink positive drain current, got {ids_reconstructed}"
        );
        // gm should be positive in strong inversion.
        assert!(
            lin.jacobian[0][1] > 0.0,
            "NMOS strong-inversion gm > 0, got {}",
            lin.jacobian[0][1]
        );
        // gds should be positive (channel-length modulation
        // produces nonzero output conductance).
        assert!(
            lin.jacobian[0][0] > 0.0,
            "NMOS strong-inversion gds > 0, got {}",
            lin.jacobian[0][0]
        );
    }

    // -----------------------------------------------------------------
    // PMOS polarity: at Vsg = 3 V, Vsd = 1 V (i.e. Vs = 3, Vg = 0,
    // Vd = 2, Vb = 3) the drain current is negative (source-to-drain).
    // -----------------------------------------------------------------
    #[test]
    fn pmos_strong_inversion_produces_negative_drain_current() {
        let p = pmos_default();
        let lin = linearize_bsim3v3(&p, &[2.0, 0.0, 3.0, 3.0]);
        let vd = 2.0;
        let vg = 0.0;
        let vs = 3.0;
        let vb = 3.0;
        let ids_reconstructed = lin.companion_current[0]
            + lin.jacobian[0][0] * vd
            + lin.jacobian[0][1] * vg
            + lin.jacobian[0][2] * vs
            + lin.jacobian[0][3] * vb;
        assert!(
            ids_reconstructed < 0.0,
            "PMOS Vsg=3 Vsd=1 should source positive current at source (negative drain current), got {ids_reconstructed}"
        );
    }

    // -----------------------------------------------------------------
    // PMOS affine-model self-consistency: the companion-current +
    // Jacobian·V reconstruction must agree to O(h²) curvature when
    // evaluated from two different linearization points.
    // -----------------------------------------------------------------
    #[test]
    fn pmos_companion_is_affine_self_consistent() {
        let p = pmos_default();
        let v0 = [2.0_f64, 0.0, 3.0, 3.0]; // Vsg=3, Vsd=1
        let h = 1.0e-5;

        // Perturb one terminal (drain) by ±h and re-linearize.
        let lin0 = linearize_bsim3v3(&p, &v0);
        let v1 = [v0[0] + h, v0[1], v0[2], v0[3]];
        let lin1 = linearize_bsim3v3(&p, &v1);

        // Reconstruct I_drain at v1 using lin0 extrapolated one step.
        let reconstruct = |lin: &MOSFETLinearization, v: &[f64; 4]| -> f64 {
            lin.companion_current[0]
                + (0..MOSFET_TERMINALS)
                    .map(|j| lin.jacobian[0][j] * v[j])
                    .sum::<f64>()
        };

        let from_lin0 = reconstruct(&lin0, &v1);
        let from_lin1 = reconstruct(&lin1, &v1);

        // Both should equal the true device current at v1; their
        // disagreement is O(h²) from curvature (≈ 1e-10 at h=1e-5).
        let diff = (from_lin0 - from_lin1).abs();
        assert!(
            diff < 1.0e-9,
            "PMOS affine self-consistency diff = {diff} at Vsg=3 Vsd=1 (h={h}); \
             expected < 1e-9 (O(h²) curvature). Buggy normalized-bias path \
             produces O(gm·h) ≈ 1e-8 instead.",
        );
    }

    // -----------------------------------------------------------------
    // Source-row equals negative of drain-row (KCL: device-internal
    // current that enters drain must leave source).
    // -----------------------------------------------------------------
    #[test]
    fn source_row_negates_drain_row() {
        let p = nmos_default();
        let lin = linearize_bsim3v3(&p, &[1.0, 2.0, 0.0, 0.0]);
        for k in 0..MOSFET_TERMINALS {
            assert!(
                approx_eq(lin.jacobian[2][k], -lin.jacobian[0][k], 1.0e-12, 1.0e-15),
                "source row [k={k}] must negate drain row: source={} drain={}",
                lin.jacobian[2][k],
                lin.jacobian[0][k],
            );
        }
        assert!(approx_eq(
            lin.companion_current[2],
            -lin.companion_current[0],
            1.0e-12,
            1.0e-15
        ));
    }

    // -----------------------------------------------------------------
    // Gate and bulk rows are zero (no DC gate leakage, no DC bulk
    // current in v1.0 scope).
    // -----------------------------------------------------------------
    #[test]
    fn gate_and_bulk_rows_are_zero() {
        let p = nmos_default();
        let lin = linearize_bsim3v3(&p, &[1.0, 2.0, 0.0, 0.0]);
        // Gate / bulk rows are literally initialized to 0.0 and
        // never written in `assemble_companion`. Compare bit
        // patterns to satisfy `clippy::float_cmp` while still
        // checking exact equality.
        let zero_bits = 0.0_f64.to_bits();
        for k in 0..MOSFET_TERMINALS {
            assert_eq!(
                lin.jacobian[1][k].to_bits(),
                zero_bits,
                "gate row [k={k}] must be exactly zero, got {}",
                lin.jacobian[1][k]
            );
            assert_eq!(
                lin.jacobian[3][k].to_bits(),
                zero_bits,
                "bulk row [k={k}] must be exactly zero, got {}",
                lin.jacobian[3][k]
            );
        }
        assert_eq!(
            lin.companion_current[1].to_bits(),
            zero_bits,
            "gate i_eq must be exactly zero"
        );
        assert_eq!(
            lin.companion_current[3].to_bits(),
            zero_bits,
            "bulk i_eq must be exactly zero"
        );
    }

    // -----------------------------------------------------------------
    // Drain-row diagonal layout. The drain row stamps:
    //   J[d][d] = +gds
    //   J[d][g] = +gm
    //   J[d][s] = −(gm + gds + gmbs)
    //   J[d][b] = +gmbs
    // KCL row-sum is zero (a hallmark of a correct MOSFET stamp:
    // a uniform voltage shift on every terminal must produce no
    // current).
    // -----------------------------------------------------------------
    #[test]
    fn drain_row_sums_to_zero_kcl() {
        let p = nmos_default();
        let lin = linearize_bsim3v3(&p, &[1.0, 2.0, 0.0, 0.0]);
        let row_sum: f64 = lin.jacobian[0].iter().sum();
        assert!(
            row_sum.abs() < 1.0e-12,
            "drain row must sum to zero (KCL invariance under uniform shift), got {row_sum}"
        );
    }

    // -----------------------------------------------------------------
    // Analytic Jacobian vs centered-difference numeric Jacobian.
    // The analytic gm/gds/gmbs are the load-bearing inputs to the
    // dual convergence criterion (ADR-0006); a derivative error
    // here directly causes false-positive convergence.
    // -----------------------------------------------------------------
    #[test]
    fn analytic_gm_matches_numeric_finite_difference() {
        let p = nmos_default();
        // Probe in strong inversion where gm should be largest and
        // most sensitive to derivative errors.
        let vd = 1.0;
        let vg = 2.5;
        let vs = 0.0;
        let vb = 0.0;
        let lin = linearize_bsim3v3(&p, &[vd, vg, vs, vb]);
        let analytic_gm = lin.jacobian[0][1];
        let h = 1.0e-4;
        let ids = |vg_probe: f64| -> f64 {
            let l = linearize_bsim3v3(&p, &[vd, vg_probe, vs, vb]);
            l.companion_current[0]
                + l.jacobian[0][0] * vd
                + l.jacobian[0][1] * vg_probe
                + l.jacobian[0][2] * vs
                + l.jacobian[0][3] * vb
        };
        let numeric_gm = fd(ids, vg, h);
        assert!(
            approx_eq(analytic_gm, numeric_gm, 1.0e-4, 1.0e-12),
            "analytic gm={analytic_gm} should match FD gm={numeric_gm}",
        );
    }

    #[test]
    fn analytic_gds_matches_numeric_finite_difference() {
        let p = nmos_default();
        let vd = 1.0;
        let vg = 2.5;
        let vs = 0.0;
        let vb = 0.0;
        let lin = linearize_bsim3v3(&p, &[vd, vg, vs, vb]);
        let analytic_gds = lin.jacobian[0][0];
        let h = 1.0e-4;
        let ids = |vd_probe: f64| -> f64 {
            let l = linearize_bsim3v3(&p, &[vd_probe, vg, vs, vb]);
            l.companion_current[0]
                + l.jacobian[0][0] * vd_probe
                + l.jacobian[0][1] * vg
                + l.jacobian[0][2] * vs
                + l.jacobian[0][3] * vb
        };
        let numeric_gds = fd(ids, vd, h);
        assert!(
            approx_eq(analytic_gds, numeric_gds, 1.0e-4, 1.0e-12),
            "analytic gds={analytic_gds} should match FD gds={numeric_gds}",
        );
    }

    #[test]
    fn analytic_gmbs_matches_numeric_finite_difference() {
        let p = nmos_default();
        // Put the body at a non-zero bias so K1·√(Φ−Vbs) actually
        // varies; with Vbs ≡ 0 the body-effect contribution is
        // small and numerics are dominated by DIBL.
        let vd = 1.0;
        let vg = 2.5;
        let vs = 0.0;
        let vb = -0.5;
        let lin = linearize_bsim3v3(&p, &[vd, vg, vs, vb]);
        let analytic_gmbs = lin.jacobian[0][3];
        let h = 1.0e-4;
        let ids = |vb_probe: f64| -> f64 {
            let l = linearize_bsim3v3(&p, &[vd, vg, vs, vb_probe]);
            l.companion_current[0]
                + l.jacobian[0][0] * vd
                + l.jacobian[0][1] * vg
                + l.jacobian[0][2] * vs
                + l.jacobian[0][3] * vb_probe
        };
        let numeric_gmbs = fd(ids, vb, h);
        assert!(
            approx_eq(analytic_gmbs, numeric_gmbs, 1.0e-4, 1.0e-12),
            "analytic gmbs={analytic_gmbs} should match FD gmbs={numeric_gmbs}",
        );
    }

    // -----------------------------------------------------------------
    // Raw-map override: a custom Vth0 changes the Ids predictably.
    // -----------------------------------------------------------------
    #[test]
    fn raw_map_override_of_vth0_lowers_drain_current() {
        let baseline = nmos_default();
        let mut high_vth_raw = std::collections::BTreeMap::new();
        high_vth_raw.insert("vth0".to_string(), 1.5); // higher threshold
        let high_vth = MosBSIM3v3Params {
            name: ModelName::new("nmos_b3_high_vth"),
            polarity: MosPolarity::Nmos,
            raw: high_vth_raw,
        };

        // At Vgs = 1.0 V, baseline (Vth0 = 0.7) is barely in
        // strong inversion; high_vth (Vth0 = 1.5) is below
        // threshold.
        let vd = 0.5;
        let vg = 1.0;
        let vs = 0.0;
        let vb = 0.0;

        let lin_base = linearize_bsim3v3(&baseline, &[vd, vg, vs, vb]);
        let lin_high = linearize_bsim3v3(&high_vth, &[vd, vg, vs, vb]);

        let reconstruct = |l: &MOSFETLinearization| -> f64 {
            l.companion_current[0]
                + l.jacobian[0][0] * vd
                + l.jacobian[0][1] * vg
                + l.jacobian[0][2] * vs
                + l.jacobian[0][3] * vb
        };

        let ids_base = reconstruct(&lin_base);
        let ids_high = reconstruct(&lin_high);

        assert!(
            ids_high < ids_base,
            "higher Vth0 must reduce Ids: base={ids_base}, high={ids_high}",
        );
    }

    // -----------------------------------------------------------------
    // softplus / sigmoid numerical stability at large |x|.
    // -----------------------------------------------------------------
    #[test]
    fn softplus_sigmoid_stable_at_extreme_x() {
        // Large positive: softplus(x) ≈ x, sigmoid(x) ≈ 1.
        let (sp, sig) = stable_softplus_and_sigmoid(50.0);
        assert!(sp.is_finite(), "softplus(50) must be finite, got {sp}");
        assert!(approx_eq(sp, 50.0, 1.0e-10, 1.0e-12));
        assert!(approx_eq(sig, 1.0, 1.0e-10, 1.0e-12));
        // Large negative: softplus(x) ≈ 0, sigmoid(x) ≈ 0.
        let (sp, sig) = stable_softplus_and_sigmoid(-50.0);
        assert!(sp.is_finite() && sp.abs() < 1.0e-20);
        assert!(sig.is_finite() && sig.abs() < 1.0e-20);
        // At x = 0: softplus(0) = ln 2, sigmoid(0) = 0.5.
        let (sp, sig) = stable_softplus_and_sigmoid(0.0);
        assert!(approx_eq(sp, 2.0_f64.ln(), 1.0e-12, 1.0e-15));
        assert!(approx_eq(sig, 0.5, 1.0e-12, 1.0e-15));
    }

    // -----------------------------------------------------------------
    // Cox / beta sanity check from the default parameter extraction.
    // -----------------------------------------------------------------
    #[test]
    fn default_cox_and_beta_match_textbook_orders_of_magnitude() {
        let p = nmos_default();
        let dc = Bsim3v3DcParams::extract(&p);
        // Cox = ε_ox / Tox = 3.4531e-11 / 1e-8 ≈ 3.45e-3 F/m².
        assert!(approx_eq(dc.cox(), 3.453_1e-3, 1.0e-6, 0.0));
        // β = μ·Cox·W/L for NMOS defaults:
        //   μ = 670 cm²/V/s = 6.70e-2 m²/V/s
        //   Cox ≈ 3.45e-3 F/m²
        //   W/L = 10 µm / 1 µm = 10
        //   β ≈ 6.70e-2 · 3.45e-3 · 10 ≈ 2.31e-3 A/V²
        let expected_beta = 6.70e-2 * 3.453_1e-3 * 10.0;
        assert!(approx_eq(dc.beta(), expected_beta, 1.0e-4, 0.0));
    }

    // -----------------------------------------------------------------
    // PMOS vs NMOS polarity-defaults differ exactly per BSIM3v3.2.4
    // manual table 5.1.
    // -----------------------------------------------------------------
    #[test]
    fn nmos_pmos_polarity_defaults_differ_correctly() {
        let n = Bsim3v3DcParams::extract(&nmos_default());
        let p = Bsim3v3DcParams::extract(&pmos_default());
        assert!(approx_eq(n.vth0, 0.7, 0.0, 1.0e-15));
        assert!(approx_eq(p.vth0, -0.7, 0.0, 1.0e-15));
        // u0 in m²/V/s after the cm²→m² conversion.
        assert!(approx_eq(n.u0, 6.70e-2, 1.0e-12, 0.0));
        assert!(approx_eq(p.u0, 2.50e-2, 1.0e-12, 0.0));
        assert!(approx_eq(n.esat, 4.0e6, 0.0, 1.0e-9));
        assert!(approx_eq(p.esat, 2.0e6, 0.0, 1.0e-9));
    }

    // -----------------------------------------------------------------
    // The placeholder zero-linearization is *not* what we return any
    // more — the BSIM3v3 arm now produces real numbers. This test
    // pins that intent so a regression that re-introduces the
    // placeholder is caught.
    // -----------------------------------------------------------------
    #[test]
    fn bsim3v3_no_longer_returns_zero_placeholder_in_strong_inversion() {
        let p = nmos_default();
        let lin = linearize_bsim3v3(&p, &[1.0, 3.0, 0.0, 0.0]);
        assert_ne!(
            lin,
            MOSFETLinearization::zero(),
            "tasks.md #12 must produce a real BSIM3v3 stamp, not the #8 placeholder"
        );
    }
}
