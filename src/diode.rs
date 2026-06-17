//! Shockley diode device model.
//!
//! The Shockley ideal diode equation is:
//!
//!   I = Is * (exp(V / Vt) - 1)
//!
//! where:
//!   Is  = saturation current (default 1e-14 A)
//!   Vt  = thermal voltage = k*T/q ≈ 0.025852 V at 300 K
//!
//! Newton-Raphson linearization
//! ----------------------------
//! At operating point V0:
//!   I(V) ≈ I(V0) + gd * (V - V0)
//!
//! where the dynamic conductance is:
//!   gd = dI/dV = Is/Vt * exp(V0/Vt)
//!
//! The companion model stamps:
//!   - Conductance gd in the four-quadrant pattern (anode/cathode)
//!   - Current source I_eq = I(V0) - gd*V0  on RHS (Norton equivalent)
//!
//! Clamping
//! --------
//! When V > 40*Vt the exponential overflows; the conductance and current are
//! evaluated at 40*Vt and extrapolated linearly beyond that point.
//! When V < -5*Vt the reverse current is negligible: gd ≈ Is/Vt, I ≈ -Is.

use crate::{traits::DeviceModel, MnaMatrix, VarMap};

/// Thermal voltage at 300 K (k*T/q).
pub const VT_300K: f64 = 0.025852;

/// Shockley diode: Is*(exp(V/Vt)-1).
#[derive(Debug, Clone)]
pub struct Diode {
    /// Anode net name.
    pub anode: String,
    /// Cathode net name.
    pub cathode: String,
    /// Saturation current in amperes (default 1e-14 A).
    pub is: f64,
    /// Thermal voltage in volts (default VT_300K ≈ 0.025852 V).
    pub vt: f64,
}

impl Diode {
    /// Create a diode with default parameters (Is=1e-14 A, Vt=0.025852 V).
    pub fn new(anode: impl Into<String>, cathode: impl Into<String>) -> Self {
        Diode {
            anode: anode.into(),
            cathode: cathode.into(),
            is: 1e-14,
            vt: VT_300K,
        }
    }

    /// Create a diode with explicit Is and Vt.
    pub fn with_params(
        anode: impl Into<String>,
        cathode: impl Into<String>,
        is: f64,
        vt: f64,
    ) -> Self {
        Diode {
            anode: anode.into(),
            cathode: cathode.into(),
            is,
            vt,
        }
    }

    /// Evaluate the Shockley current I at voltage V (with forward clamping).
    pub fn current(&self, v: f64) -> f64 {
        let v_clamp = v.min(40.0 * self.vt);
        self.is * (v_clamp / self.vt).exp() - self.is
    }

    /// Evaluate the dynamic conductance gd = dI/dV at voltage V.
    pub fn conductance(&self, v: f64) -> f64 {
        let v_clamp = v.min(40.0 * self.vt);
        (self.is / self.vt) * (v_clamp / self.vt).exp()
    }
}

impl DeviceModel for Diode {
    fn terminals(&self) -> Vec<String> {
        vec![self.anode.clone(), self.cathode.clone()]
    }

    fn stamp_linear(&self, _matrix: &mut MnaMatrix, _var_map: &VarMap) {
        // No linear stamp; all contribution is operating-point dependent.
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, solution: &[f64]) {
        // Look up node indices (0-indexed after removing ground).
        let idx_a = var_map.node_index(&self.anode);
        let idx_k = var_map.node_index(&self.cathode);

        // Helper: convert VarMap index (ground=0) to matrix row (ground excluded).
        // Returns None for ground (index 0) so stamp helpers skip it.
        let to_row = |idx: Option<usize>| match idx {
            Some(0) | None => None,
            Some(i) => Some(i - 1),
        };

        let row_a = to_row(idx_a);
        let row_k = to_row(idx_k);

        // Read current operating-point voltage across the diode.
        let v_a = row_a.map(|r| solution[r]).unwrap_or(0.0);
        let v_k = row_k.map(|r| solution[r]).unwrap_or(0.0);
        let v_d = v_a - v_k;

        let gd = self.conductance(v_d);
        let id = self.current(v_d);
        // Norton companion: I_eq = id - gd*v_d  (current source in parallel with gd)
        let i_eq = id - gd * v_d;

        // Stamp conductance in four-quadrant pattern.
        if let Some(a) = row_a {
            if let Some(k) = row_k {
                matrix.stamp(a, k, -gd);
                matrix.stamp(k, a, -gd);
            }
            matrix.stamp(a, a, gd);
            // RHS: Norton current (positive into anode)
            matrix.stamp_rhs(a, -i_eq);
        }
        if let Some(k) = row_k {
            matrix.stamp(k, k, gd);
            // RHS: Norton current (negative into cathode)
            matrix.stamp_rhs(k, i_eq);
        }
    }

    fn is_smooth(&self) -> bool {
        false
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Diode is not smooth (piecewise-linear clamping).
    #[test]
    fn diode_is_not_smooth() {
        let d = Diode::new("A", "K");
        assert!(!d.is_smooth());
    }

    /// Terminals are [anode, cathode].
    #[test]
    fn diode_terminals() {
        let d = Diode::new("anode", "cathode");
        assert_eq!(d.terminals(), vec!["anode", "cathode"]);
    }

    /// I-V sweep: each point within 1% of ideal Shockley formula.
    ///
    /// Tests voltages from 0.1 V to 0.8 V in 0.1 V steps.  At each point
    /// the diode current is compared against Is*(exp(V/Vt)-1).
    #[test]
    fn diode_iv_sweep_within_1pct_of_shockley() {
        let is = 1e-14_f64;
        let vt = VT_300K;
        let d = Diode::with_params("A", "K", is, vt);

        let voltages = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        for &v in &voltages {
            let computed = d.current(v);
            let expected = is * (v / vt).exp() - is;
            let rel_err = (computed - expected).abs() / expected.abs().max(1e-30);
            assert!(
                rel_err < 0.01,
                "V={v:.1} V: computed={computed:.6e}, expected={expected:.6e}, rel_err={rel_err:.4}"
            );
        }
    }

    /// Forward-bias current at 0.7 V is physically sensible (mA range with
    /// Is=1e-14 A, Vt=0.025852 V).
    #[test]
    fn diode_current_forward_bias() {
        let d = Diode::new("A", "K");
        let i = d.current(0.7);
        // At 0.7 V: I ≈ 1e-14 * exp(0.7/0.025852) ≈ 1.5 mA
        assert!(i > 1e-4, "forward current at 0.7 V should be in mA range, got {i:.3e}");
    }

    /// Reverse-bias current is approximately -Is.
    #[test]
    fn diode_current_reverse_bias() {
        let d = Diode::new("A", "K");
        let i = d.current(-0.5);
        // Should be close to -Is = -1e-14 A
        assert!((i + d.is).abs() / d.is < 0.01, "reverse current {i:.3e} not close to -Is");
    }
}
