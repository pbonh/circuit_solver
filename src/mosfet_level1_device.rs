//! SPICE Level 1 MOSFET device model (square-law).
//!
//! Square-law I-V equations (NMOS; PMOS uses negated terminal voltages):
//!
//!   Cutoff   (Vgs < Vth):                  Id = 0
//!   Linear   (Vgs >= Vth, Vds < Vgs-Vth):  Id = k*(Vgs-Vth)*Vds - k/2*Vds²
//!   Saturation (Vgs >= Vth, Vds >= Vgs-Vth): Id = k/2*(Vgs-Vth)²
//!
//! where k = Kp * W/L  (process transconductance × aspect ratio).
//!
//! Default parameters (NMOS):
//!   Kp  = 50e-6 A/V²  (process transconductance)
//!   Vth = 0.7 V       (threshold voltage)
//!   W   = 1.0 µm
//!   L   = 1.0 µm
//!
//! Newton-Raphson linearization
//! ----------------------------
//! At operating point (Vgs0, Vds0) the device is linearized as:
//!
//!   Id(Vgs, Vds) ≈ Id0 + gm*(Vgs-Vgs0) + gds*(Vds-Vds0)
//!
//! where:
//!   gm  = dId/dVgs (transconductance)
//!   gds = dId/dVds (output conductance)
//!
//! The companion Norton model stamps gm and gds as conductances plus a
//! constant current source.

use crate::{traits::DeviceModel, MnaMatrix, VarMap};

/// MOSFET polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosType {
    /// N-channel MOSFET (conventional, positive Vgs and Vds in saturation).
    Nmos,
    /// P-channel MOSFET (voltage signs are negated internally).
    Pmos,
}

/// SPICE Level 1 MOSFET (square-law model).
#[derive(Debug, Clone)]
pub struct MosfetLevel1 {
    /// Drain net name.
    pub drain: String,
    /// Gate net name.
    pub gate: String,
    /// Source net name.
    pub source: String,
    /// MOSFET type (NMOS / PMOS).
    pub mos_type: MosType,
    /// Process transconductance Kp in A/V².
    pub kp: f64,
    /// Threshold voltage Vth in volts (positive for NMOS, negative for PMOS).
    pub vth: f64,
    /// Channel width W in metres.
    pub w: f64,
    /// Channel length L in metres.
    pub l: f64,
}

impl MosfetLevel1 {
    /// Create an NMOS with default SPICE Level 1 parameters.
    pub fn new_nmos(
        drain: impl Into<String>,
        gate: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        MosfetLevel1 {
            drain: drain.into(),
            gate: gate.into(),
            source: source.into(),
            mos_type: MosType::Nmos,
            kp: 50e-6,
            vth: 0.7,
            w: 1e-6,
            l: 1e-6,
        }
    }

    /// Create a PMOS with default parameters.
    pub fn new_pmos(
        drain: impl Into<String>,
        gate: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        MosfetLevel1 {
            drain: drain.into(),
            gate: gate.into(),
            source: source.into(),
            mos_type: MosType::Pmos,
            kp: 25e-6,
            vth: -0.7,
            w: 1e-6,
            l: 1e-6,
        }
    }

    /// Effective transconductance parameter k = Kp * W/L.
    pub fn k(&self) -> f64 {
        self.kp * self.w / self.l
    }

    /// Drain current Id for given Vgs and Vds (NMOS convention).
    ///
    /// For PMOS the caller should pass Vsg and Vsd (negated terminal voltages)
    /// and then negate the result, but this function always uses NMOS equations.
    pub fn drain_current_nmos(&self, vgs: f64, vds: f64) -> f64 {
        let vov = vgs - self.vth.abs();
        if vov <= 0.0 {
            return 0.0; // cutoff
        }
        if vds < vov {
            // Linear (triode) region
            self.k() * (vov * vds - 0.5 * vds * vds)
        } else {
            // Saturation region
            0.5 * self.k() * vov * vov
        }
    }

    /// Drain current Id at the given terminal voltages (handles NMOS/PMOS).
    pub fn drain_current(&self, vgs: f64, vds: f64) -> f64 {
        match self.mos_type {
            MosType::Nmos => self.drain_current_nmos(vgs, vds),
            MosType::Pmos => -self.drain_current_nmos(-vgs, -vds),
        }
    }

    /// Transconductance gm = dId/dVgs at operating point.
    pub fn gm(&self, vgs: f64, vds: f64) -> f64 {
        let sign = if self.mos_type == MosType::Pmos { -1.0 } else { 1.0 };
        let (vgs_eff, vds_eff) = if self.mos_type == MosType::Pmos {
            (-vgs, -vds)
        } else {
            (vgs, vds)
        };
        let vov = vgs_eff - self.vth.abs();
        if vov <= 0.0 {
            return 0.0;
        }
        sign * if vds_eff < vov {
            self.k() * vds_eff
        } else {
            self.k() * vov
        }
    }

    /// Output conductance gds = dId/dVds at operating point.
    pub fn gds(&self, vgs: f64, vds: f64) -> f64 {
        let sign = if self.mos_type == MosType::Pmos { -1.0 } else { 1.0 };
        let (vgs_eff, vds_eff) = if self.mos_type == MosType::Pmos {
            (-vgs, -vds)
        } else {
            (vgs, vds)
        };
        let vov = vgs_eff - self.vth.abs();
        if vov <= 0.0 {
            return 0.0;
        }
        sign * if vds_eff < vov {
            self.k() * (vov - vds_eff)
        } else {
            0.0 // ideal saturation: gds = 0
        }
    }
}

impl DeviceModel for MosfetLevel1 {
    fn terminals(&self) -> Vec<String> {
        vec![self.drain.clone(), self.gate.clone(), self.source.clone()]
    }

    fn stamp_linear(&self, _matrix: &mut MnaMatrix, _var_map: &VarMap) {
        // No linear stamp; all contribution is operating-point dependent.
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix, var_map: &VarMap, solution: &[f64]) {
        let to_row = |name: &str| -> Option<usize> {
            match var_map.node_index(name) {
                Some(0) | None => None,
                Some(i) => Some(i - 1),
            }
        };

        let row_d = to_row(&self.drain);
        let row_g = to_row(&self.gate);
        let row_s = to_row(&self.source);

        let v = |row: Option<usize>| row.map(|r| solution[r]).unwrap_or(0.0);
        let vd = v(row_d);
        let vg = v(row_g);
        let vs = v(row_s);

        let vgs = vg - vs;
        let vds = vd - vs;

        let id = self.drain_current(vgs, vds);
        let gm = self.gm(vgs, vds);
        let gds = self.gds(vgs, vds);

        // Norton companion: I_eq = id - gm*vgs - gds*vds
        let i_eq = id - gm * vgs - gds * vds;

        // Stamp gm as VCCS: current = gm*(Vg - Vs) flows from source to drain.
        // Stamp entries:
        //   gm: (drain, gate) += gm; (drain, source) += -gm
        //       (source, gate) += -gm; (source, source) += gm
        if let Some(d) = row_d {
            if let Some(g) = row_g {
                matrix.stamp(d, g, gm);
            }
            if let Some(s) = row_s {
                matrix.stamp(d, s, -gm);
                matrix.stamp(s, s, gm);
                if let Some(g) = row_g {
                    matrix.stamp(s, g, -gm);
                }
            }
        }

        // Stamp gds: (drain, drain) += gds; (drain, source) += -gds
        //            (source, drain) += -gds; (source, source) += gds
        if let Some(d) = row_d {
            if let Some(s) = row_s {
                matrix.stamp(d, d, gds);
                matrix.stamp(d, s, -gds);
                matrix.stamp(s, d, -gds);
                matrix.stamp(s, s, gds);
            } else {
                matrix.stamp(d, d, gds);
            }
        } else if let Some(s) = row_s {
            matrix.stamp(s, s, gds);
        }

        // Constant current source I_eq (Norton companion):
        // flows into drain, out of source.
        if let Some(d) = row_d {
            matrix.stamp_rhs(d, -i_eq);
        }
        if let Some(s) = row_s {
            matrix.stamp_rhs(s, i_eq);
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

    /// MosfetLevel1 is not smooth (piecewise square-law).
    #[test]
    fn mosfet_is_not_smooth() {
        let m = MosfetLevel1::new_nmos("D", "G", "S");
        assert!(!m.is_smooth());
    }

    /// Terminals are [drain, gate, source].
    #[test]
    fn mosfet_terminals() {
        let m = MosfetLevel1::new_nmos("drain", "gate", "source");
        assert_eq!(m.terminals(), vec!["drain", "gate", "source"]);
    }

    /// Saturation region: Id at Vgs=1.5V, Vds=2V matches Level 1 formula.
    ///
    /// With default parameters: k = Kp * W/L = 50e-6 * 1.0 = 50e-6 A/V²
    ///   Vov = Vgs - Vth = 1.5 - 0.7 = 0.8 V
    ///   Vov < Vds  →  saturation
    ///   Id = k/2 * Vov² = 50e-6/2 * 0.64 = 16 µA
    #[test]
    fn mosfet_saturation_id_matches_level1_formula() {
        let m = MosfetLevel1::new_nmos("D", "G", "S");
        let vgs = 1.5_f64;
        let vds = 2.0_f64;
        let vov = vgs - m.vth;
        // Must be in saturation: Vds >= Vov
        assert!(vds >= vov, "test point must be in saturation");
        let id_expected = 0.5 * m.k() * vov * vov;
        let id_computed = m.drain_current(vgs, vds);
        let rel_err = (id_computed - id_expected).abs() / id_expected;
        assert!(
            rel_err < 0.001,
            "Id={id_computed:.6e}, expected={id_expected:.6e}, rel_err={rel_err:.4}"
        );
    }

    /// Cutoff: Id = 0 when Vgs < Vth.
    #[test]
    fn mosfet_cutoff() {
        let m = MosfetLevel1::new_nmos("D", "G", "S");
        let id = m.drain_current(0.3, 1.0); // Vgs < Vth = 0.7
        assert!(id.abs() < 1e-30, "cutoff: Id should be 0, got {id:.3e}");
    }

    /// Linear region: Id at Vds < Vov.
    #[test]
    fn mosfet_linear_region() {
        let m = MosfetLevel1::new_nmos("D", "G", "S");
        let vgs = 1.5_f64;
        let vds = 0.3_f64; // Vov = 0.8, Vds < Vov → linear
        let vov = vgs - m.vth;
        let id_expected = m.k() * (vov * vds - 0.5 * vds * vds);
        let id_computed = m.drain_current(vgs, vds);
        let rel_err = (id_computed - id_expected).abs() / id_expected;
        assert!(
            rel_err < 0.001,
            "linear Id={id_computed:.6e}, expected={id_expected:.6e}"
        );
    }
}
