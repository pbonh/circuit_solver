//! `Diode` — [`traits::DeviceModel`] implementor for the Shockley diode.
//!
//! This module provides [`Diode`], a struct that wraps [`DiodeParams`] and
//! two terminal [`NodeId`]s (`[anode, cathode]`), and implements the
//! [`traits::DeviceModel`] trait so the Newton-Raphson engine can iterate
//! over it via `dyn DeviceModel` dispatch.
//!
//! # Shockley equation and companion model
//!
//! The junction current is
//!
//! ```text
//! I(Vd) = IS · (exp(Vd / (N · Vt)) − 1)
//! ```
//!
//! where `Vd = V_anode − V_cathode`. Linearizing around the iterate `Vd_k`:
//!
//! ```text
//! gd    = dI/dVd |_{Vd_k} = (IS / (N · Vt)) · exp(Vd_k / (N · Vt))
//! I_eq  = I(Vd_k) − gd · Vd_k
//! ```
//!
//! so `I(Vd) ≈ gd · Vd + I_eq` is the linear surrogate stamped into the
//! MNA system on this iteration.
//!
//! # Tangent-line clamping
//!
//! Two clamps protect NR convergence against iterates that overshoot:
//!
//! - **Forward clamp** (`Vd > 40 · Vt`): the exponent argument is capped at
//!   [`DIODE_MAX_EXP_ARG`] (`40.0`), matching SPICE3/ngspice.
//! - **Reverse clamp** (`Vd < −5 · Vt`): the tangent line at the clamp
//!   point is used, keeping `gd ≈ 0` and `I_eq ≈ −IS`.
//!
//! # Series resistance (`RS`)
//!
//! When [`DiodeParams::rs`] `> 0`, [`stamp_linear`](traits::DeviceModel::stamp_linear)
//! adds a conductance `1/RS` between anode and cathode — a lumped-element
//! approximation that is valid when the junction node split performed by the
//! netlist-graph elaborator is not yet available. For the full SPICE treatment
//! (split node + internal junction), the elaborator provides a separate
//! resistor element for `RS`; in that case `RS` on the [`Diode`] instance
//! should be set to `0`.
//!
//! # Junction capacitance (`CJ`) — transient companion
//!
//! When [`DiodeParams::cj`] `> 0` **and** a timestep is supplied via
//! [`Diode::with_timestep`], [`stamp_nonlinear`](traits::DeviceModel::stamp_nonlinear)
//! adds a Backward-Euler equivalent conductance `Cj / h` to the junction's
//! Jacobian entries. This is the standard transient companion for a linear
//! (zero-bias) junction capacitance. When `cj == 0` or no timestep has been
//! set the capacitive stamp is a no-op, which is correct for DC
//! operating-point analysis.

use circuit_solver_types::NodeId;

use crate::mna_matrix::MnaMatrix;
use crate::params::DiodeParams;
use crate::stamp::DIODE_MAX_EXP_ARG;
use crate::traits::DeviceModel;
use crate::var_map::VarMap;

// Reverse-bias clamp threshold (exponent argument).
// Below `-5 · Vt` the exponential is negligible relative to `-IS`; we
// clamp the tangent here to keep NR from taking huge reverse-bias steps.
const DIODE_MIN_EXP_ARG: f64 = -5.0;

/// Diode device implementing [`DeviceModel`](crate::traits::DeviceModel).
///
/// Wraps [`DiodeParams`] and a fixed pair of terminal [`NodeId`]s.  The
/// Newton-Raphson engine can hold a `Vec<Box<dyn DeviceModel>>` containing
/// `Diode` instances mixed with other device types.
///
/// # Terminal order
///
/// `terminals[0]` is the **anode**, `terminals[1]` is the **cathode**.
/// This is the SPICE `D` card convention.
///
/// # Transient mode
///
/// For transient analysis, call [`Diode::with_timestep`] to supply the
/// current timestep `h`.  This enables the junction-capacitance companion
/// stamp in [`stamp_nonlinear`](traits::DeviceModel::stamp_nonlinear).
#[derive(Debug, Clone)]
pub struct Diode {
    /// Electrical parameters read from the `.MODEL D …` card.
    pub params: DiodeParams,
    /// Terminal node identifiers: `[anode, cathode]`.
    terminals: [NodeId; 2],
    /// Current timestep `h` (seconds) for transient Cj stamping.
    /// `None` during DC operating-point analysis.
    timestep_s: Option<f64>,
}

impl Diode {
    /// Construct a new [`Diode`] with the given parameters and terminals.
    ///
    /// The timestep is initially `None` (DC mode, no Cj stamp).  Use
    /// [`Self::with_timestep`] to switch to transient mode.
    ///
    /// # Arguments
    ///
    /// - `params` — model parameters from the `.MODEL D` card.
    /// - `anode` — the node identifier for the anode terminal.
    /// - `cathode` — the node identifier for the cathode terminal.
    #[must_use]
    pub fn new(params: DiodeParams, anode: NodeId, cathode: NodeId) -> Self {
        Self {
            params,
            terminals: [anode, cathode],
            timestep_s: None,
        }
    }

    /// Return a clone of this diode with the timestep set to `h` seconds.
    ///
    /// When `h` is set and [`DiodeParams::cj`] `> 0`, the transient
    /// junction-capacitance companion (`Cj / h`) is added in
    /// [`stamp_nonlinear`](traits::DeviceModel::stamp_nonlinear).
    #[must_use]
    pub fn with_timestep(mut self, h: f64) -> Self {
        self.timestep_s = Some(h);
        self
    }

    /// The anode node identifier (`terminals[0]`).
    #[must_use]
    pub fn anode(&self) -> NodeId {
        self.terminals[0]
    }

    /// The cathode node identifier (`terminals[1]`).
    #[must_use]
    pub fn cathode(&self) -> NodeId {
        self.terminals[1]
    }
}

impl DeviceModel for Diode {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    /// Stamp the series resistance `RS` into the MNA matrix.
    ///
    /// When [`DiodeParams::rs`] `> 0`, this adds a conductance `g_rs = 1/RS`
    /// between anode and cathode — a lumped-element approximation of the
    /// ohmic series resistance.  The stamp has the standard resistor pattern:
    ///
    /// ```text
    ///              anode  cathode
    ///   anode   [  +g_rs  −g_rs ]
    ///   cathode [  −g_rs  +g_rs ]
    /// ```
    ///
    /// When `RS == 0` this method is a no-op.
    fn stamp_linear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap) {
        if self.params.rs <= 0.0 {
            return;
        }
        let g_rs = 1.0 / self.params.rs;
        // node_index returns None only if a terminal is not in the VarMap,
        // which is a programming error (the NR engine must ensure all
        // terminals are mapped before calling stamp). Unwrap is intentional.
        let ia = var_map.node_index(self.terminals[0]).expect("anode not in VarMap");
        let ik = var_map.node_index(self.terminals[1]).expect("cathode not in VarMap");
        matrix.add_element(ia, ia, g_rs);
        matrix.add_element(ik, ik, g_rs);
        matrix.add_element(ia, ik, -g_rs);
        matrix.add_element(ik, ia, -g_rs);
    }

    /// Stamp the Shockley-equation Jacobian, companion current, and
    /// (optionally) the junction-capacitance transient conductance.
    ///
    /// # Jacobian and companion current
    ///
    /// The 2×2 Jacobian in `[anode, cathode]` order is:
    ///
    /// ```text
    ///              anode    cathode
    ///   anode   [  +gd      −gd   ]
    ///   cathode [  −gd      +gd   ]
    /// ```
    ///
    /// The companion current sources are `+I_eq` into the anode row and
    /// `−I_eq` into the cathode row of the RHS.
    ///
    /// # Junction capacitance
    ///
    /// When [`DiodeParams::cj`] `> 0` and a timestep `h` has been set via
    /// [`Diode::with_timestep`], an additional Backward-Euler conductance
    /// `g_cj = Cj / h` is added to all four Jacobian entries (same
    /// anti-symmetric pattern as the resistor stamp above).
    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, x: &[f64]) {
        let ia = var_map.node_index(self.terminals[0]).expect("anode not in VarMap");
        let ik = var_map.node_index(self.terminals[1]).expect("cathode not in VarMap");

        let v_anode = x[ia];
        let v_cathode = x[ik];
        let vd = v_anode - v_cathode;

        let n_vt = self.params.n * self.params.vt;

        // Clamp the exponent argument to [DIODE_MIN_EXP_ARG, DIODE_MAX_EXP_ARG].
        let arg = (vd / n_vt).clamp(DIODE_MIN_EXP_ARG, DIODE_MAX_EXP_ARG);
        let exp_arg = arg.exp();

        // Shockley current and small-signal conductance.
        let i_d = self.params.is * (exp_arg - 1.0);
        let gd = (self.params.is / n_vt) * exp_arg;

        // Companion current: I_eq = I(Vd) − gd · Vd.
        let i_eq = i_d - gd * vd;

        // Jacobian stamp (KCL-conserving anti-symmetric pattern).
        matrix.add_element(ia, ia, gd);
        matrix.add_element(ia, ik, -gd);
        matrix.add_element(ik, ia, -gd);
        matrix.add_element(ik, ik, gd);

        // Companion current stamp.
        matrix.add_rhs(ia, i_eq);
        matrix.add_rhs(ik, -i_eq);

        // Transient junction-capacitance companion: G_cj = Cj / h.
        if self.params.cj > 0.0 {
            if let Some(h) = self.timestep_s {
                if h > 0.0 {
                    let g_cj = self.params.cj / h;
                    matrix.add_element(ia, ia, g_cj);
                    matrix.add_element(ia, ik, -g_cj);
                    matrix.add_element(ik, ia, -g_cj);
                    matrix.add_element(ik, ik, g_cj);
                }
            }
        }
    }

    /// Returns `false`: the Shockley exponential is C¹ everywhere, but the
    /// tangent-line clamping introduces a kink at the clamp boundaries.
    /// Reporting non-smooth forces the NR engine to use conservative
    /// step sizes, which is correct for a clamped diode model.
    fn is_smooth(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mna_matrix::MnaMatrix;
    use crate::var_map::VarMap;

    // Build a 2-node system (GND=0, n1=1) for testing diode stamps.
    fn two_node_system() -> (Vec<f64>, Vec<f64>, VarMap) {
        let nodes = [NodeId::GROUND, NodeId::new(1)];
        let var_map = VarMap::from_nodes(&nodes);
        let a = vec![0.0_f64; 4]; // 2×2
        let b = vec![0.0_f64; 2];
        (a, b, var_map)
    }

    // Default diode: anode = n1, cathode = GND.
    fn default_diode() -> Diode {
        Diode::new(DiodeParams::default(), NodeId::new(1), NodeId::GROUND)
    }

    // ---------------------------------------------------------------------------
    // Shockley current: forward-bias acceptance criterion
    // ---------------------------------------------------------------------------

    /// Forward-bias I at V=0.7 V is within 1% of the hand-computed value.
    #[test]
    fn forward_bias_current_at_0v7_within_1pct_of_shockley() {
        let p = DiodeParams::default(); // IS=1e-14, N=1, Vt=0.025852
        let vd = 0.7_f64;
        let expected = p.is * ((vd / (p.n * p.vt)).exp() - 1.0);

        // Use the stamp: stamp into a 2-node system and read back I_eq.
        let diode = default_diode();
        let (mut a, mut b, var_map) = two_node_system();
        let x = [0.0_f64, vd]; // GND=0, anode=0.7 V
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }

        // Recover gd and I_eq from the stamp.
        // n_vt * exp(arg) * IS / n_vt = IS/n_vt * exp(vd/n_vt) = gd
        // I_eq = I_d - gd * vd  (stamped into b[1] = +I_eq)
        let n_vt = p.n * p.vt;
        let exp_arg = (vd / n_vt).min(DIODE_MAX_EXP_ARG).exp();
        let gd = (p.is / n_vt) * exp_arg;
        let i_eq_computed = expected - gd * vd;

        // b[1] = +I_eq (anode row), b[0] = -I_eq (cathode row)
        assert!(
            (b[1] - i_eq_computed).abs() < 1e-20,
            "I_eq mismatch: stamp={}, hand={i_eq_computed}",
            b[1]
        );

        // Reconstruct I at Vd from the stamp: I = gd * Vd + I_eq.
        let i_from_stamp = a[1 * 2 + 1] * vd + b[1]; // a[row=1,col=1] = gd
        let rel_err = (i_from_stamp - expected).abs() / expected;
        assert!(
            rel_err < 0.01,
            "forward-bias I={i_from_stamp:.6e} differs from Shockley={expected:.6e} by {:.2}%",
            rel_err * 100.0
        );
    }

    // ---------------------------------------------------------------------------
    // KCL closure: row sums of Jacobian must be zero
    // ---------------------------------------------------------------------------

    #[test]
    fn jacobian_satisfies_kcl_at_forward_bias() {
        let diode = default_diode();
        let (mut a, mut b, var_map) = two_node_system();
        let x = [0.0_f64, 0.6]; // anode=0.6 V
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }
        // Each row of the 2×2 Jacobian must sum to zero.
        let row0 = a[0] + a[1]; // a[0,0] + a[0,1]
        let row1 = a[2] + a[3]; // a[1,0] + a[1,1]
        assert!(row0.abs() < 1e-20, "row-0 KCL sum={row0}");
        assert!(row1.abs() < 1e-20, "row-1 KCL sum={row1}");
    }

    #[test]
    fn companion_currents_are_anti_symmetric() {
        let diode = default_diode();
        let (mut a, mut b, var_map) = two_node_system();
        let x = [0.0_f64, 0.6];
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }
        // b[0] = -I_eq, b[1] = +I_eq => they must sum to zero.
        assert!(
            (b[0] + b[1]).abs() < 1e-20,
            "companion currents must sum to zero; got b={b:?}"
        );
    }

    // ---------------------------------------------------------------------------
    // Tangent-line clamping
    // ---------------------------------------------------------------------------

    #[test]
    fn forward_clamp_at_40_times_vt() {
        let p = DiodeParams::default();
        let n_vt = p.n * p.vt;
        let vd_clamped = 45.0 * n_vt; // well above the 40*Vt threshold
        let vd_at_cap = DIODE_MAX_EXP_ARG * n_vt;

        let diode = default_diode();
        let var_map = VarMap::from_nodes(&[NodeId::GROUND, NodeId::new(1)]);

        // Stamp at the clamped voltage.
        let (mut a_clamped, mut b_clamped, _) = two_node_system();
        {
            let mut m = MnaMatrix::new(&mut a_clamped, &mut b_clamped, 2);
            diode.stamp_nonlinear(&mut m, &var_map, &[0.0_f64, vd_clamped]);
        }

        // Stamp at exactly the cap.
        let (mut a_cap, mut b_cap, _) = two_node_system();
        {
            let mut m = MnaMatrix::new(&mut a_cap, &mut b_cap, 2);
            diode.stamp_nonlinear(&mut m, &var_map, &[0.0_f64, vd_at_cap]);
        }

        // Both should use arg = DIODE_MAX_EXP_ARG; gd should be equal.
        // Use tolerance comparison instead of bit equality due to floating-point
        // rounding in the Vd/n_vt division.
        let gd_clamped = a_clamped[1 * 2 + 1];
        let gd_cap = a_cap[1 * 2 + 1];
        let rel = (gd_clamped - gd_cap).abs() / gd_cap;
        assert!(
            rel < 1e-12,
            "gd above 40·Vt should equal gd at 40·Vt; rel_err={rel}"
        );
        drop(b_cap);
    }

    #[test]
    fn reverse_clamp_below_minus5_vt() {
        let p = DiodeParams::default();
        let n_vt = p.n * p.vt;
        let vd_deep = -10.0 * n_vt; // well below -5*Vt
        let vd_at_clamp = DIODE_MIN_EXP_ARG * n_vt;

        let diode = default_diode();
        let var_map = VarMap::from_nodes(&[NodeId::GROUND, NodeId::new(1)]);

        let (mut a_deep, mut b_deep, _) = two_node_system();
        {
            let mut m = MnaMatrix::new(&mut a_deep, &mut b_deep, 2);
            diode.stamp_nonlinear(&mut m, &var_map, &[0.0_f64, vd_deep]);
        }

        let (mut a_clamp, mut b_clamp, _) = two_node_system();
        {
            let mut m = MnaMatrix::new(&mut a_clamp, &mut b_clamp, 2);
            diode.stamp_nonlinear(&mut m, &var_map, &[0.0_f64, vd_at_clamp]);
        }

        // Both deep and at-clamp stamps should produce the same gd
        // (both args are clamped to DIODE_MIN_EXP_ARG = -5.0).
        // Use tolerance comparison due to floating-point rounding.
        let gd_deep = a_deep[1 * 2 + 1];
        let gd_clamp = a_clamp[1 * 2 + 1];
        let rel = (gd_deep - gd_clamp).abs() / (gd_clamp.abs().max(1e-30));
        assert!(
            rel < 1e-12,
            "gd should be equal for both Vd below -5·Vt; deep={gd_deep}, at_clamp={gd_clamp}, rel={rel}"
        );
        drop(b_deep);
    }

    // ---------------------------------------------------------------------------
    // Series resistance stamp (stamp_linear)
    // ---------------------------------------------------------------------------

    #[test]
    fn rs_zero_produces_no_stamp_linear() {
        let diode = default_diode(); // RS = 0 by default
        let (mut a, mut b, var_map) = two_node_system();
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_linear(&mut matrix, &var_map);
        }
        assert!(
            a.iter().all(|&v| v == 0.0),
            "stamp_linear must be no-op for RS=0"
        );
        assert!(
            b.iter().all(|&v| v == 0.0),
            "stamp_linear must not touch RHS for RS=0"
        );
    }

    #[test]
    fn rs_nonzero_stamps_conductance_correctly() {
        let rs = 50.0_f64; // 50 Ω
        let params = DiodeParams {
            rs,
            ..DiodeParams::default()
        };
        let diode = Diode::new(params, NodeId::new(1), NodeId::GROUND);
        let (mut a, mut b, var_map) = two_node_system();
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_linear(&mut matrix, &var_map);
        }
        let g = 1.0 / rs;
        // a[1,1] = +g, a[0,0] = +g, a[0,1] = -g, a[1,0] = -g
        assert!((a[1 * 2 + 1] - g).abs() < f64::EPSILON, "a[1,1] != g_rs");
        assert!((a[0 * 2 + 0] - g).abs() < f64::EPSILON, "a[0,0] != g_rs");
        assert!((a[0 * 2 + 1] + g).abs() < f64::EPSILON, "a[0,1] != -g_rs");
        assert!((a[1 * 2 + 0] + g).abs() < f64::EPSILON, "a[1,0] != -g_rs");
        // RHS untouched by linear stamp.
        assert!(b.iter().all(|&v| v == 0.0));
    }

    #[test]
    fn rs_stamp_satisfies_kcl() {
        let params = DiodeParams {
            rs: 100.0,
            ..DiodeParams::default()
        };
        let diode = Diode::new(params, NodeId::new(1), NodeId::GROUND);
        let (mut a, mut b, var_map) = two_node_system();
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_linear(&mut matrix, &var_map);
        }
        let row0 = a[0] + a[1];
        let row1 = a[2] + a[3];
        assert!(row0.abs() < f64::EPSILON, "RS stamp row-0 KCL={row0}");
        assert!(row1.abs() < f64::EPSILON, "RS stamp row-1 KCL={row1}");
    }

    // ---------------------------------------------------------------------------
    // Junction capacitance transient companion
    // ---------------------------------------------------------------------------

    #[test]
    fn cj_zero_adds_no_extra_conductance() {
        // Default DiodeParams has cj=0 — no transient conductance even
        // when a timestep is set.
        let diode = default_diode().with_timestep(1e-9);
        let (mut a, mut b, var_map) = two_node_system();
        let x = [0.0_f64, 0.0]; // Vd = 0
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }
        // At Vd=0 the Shockley conductance gd = IS/(N·Vt); no Cj contribution.
        let p = DiodeParams::default();
        let gd_expected = p.is / (p.n * p.vt);
        assert!(
            (a[1 * 2 + 1] - gd_expected).abs() < 1e-25,
            "Cj=0 should not add conductance; a[1,1]={}, gd={}",
            a[1 * 2 + 1],
            gd_expected
        );
        drop(b);
    }

    #[test]
    fn cj_nonzero_with_timestep_adds_cj_over_h() {
        let cj = 10e-12_f64; // 10 pF
        let h = 1e-9_f64; // 1 ns
        let params = DiodeParams {
            cj,
            ..DiodeParams::default()
        };
        let diode = Diode::new(params, NodeId::new(1), NodeId::GROUND).with_timestep(h);
        let x = [0.0_f64, 0.0]; // Vd = 0

        let (mut a_with_cj, mut b, var_map) = two_node_system();
        {
            let mut matrix = MnaMatrix::new(&mut a_with_cj, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }

        // Without Cj.
        let diode_no_cj = default_diode().with_timestep(h);
        let (mut a_no_cj, mut b2, var_map2) = two_node_system();
        {
            let mut matrix = MnaMatrix::new(&mut a_no_cj, &mut b2, 2);
            diode_no_cj.stamp_nonlinear(&mut matrix, &var_map2, &x);
        }

        let g_cj = cj / h;
        let delta = a_with_cj[1 * 2 + 1] - a_no_cj[1 * 2 + 1];
        assert!(
            (delta - g_cj).abs() < 1e-12,
            "Cj/h conductance mismatch: delta={delta}, Cj/h={g_cj}"
        );
    }

    #[test]
    fn cj_nonzero_without_timestep_adds_no_conductance() {
        let cj = 10e-12_f64;
        let params = DiodeParams {
            cj,
            ..DiodeParams::default()
        };
        // No with_timestep() call — DC mode.
        let diode = Diode::new(params, NodeId::new(1), NodeId::GROUND);
        let (mut a_with_cj, mut b, var_map) = two_node_system();
        let x = [0.0_f64, 0.0];
        {
            let mut matrix = MnaMatrix::new(&mut a_with_cj, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }

        let diode_no_cj = default_diode();
        let (mut a_no_cj, mut b2, var_map2) = two_node_system();
        {
            let mut matrix = MnaMatrix::new(&mut a_no_cj, &mut b2, 2);
            diode_no_cj.stamp_nonlinear(&mut matrix, &var_map2, &x);
        }

        // Jacobians must be identical (no Cj stamp without timestep).
        assert_eq!(
            a_with_cj,
            a_no_cj,
            "Cj stamp must be disabled in DC mode (no timestep)"
        );
    }

    #[test]
    fn cj_stamp_satisfies_kcl() {
        let params = DiodeParams {
            cj: 5e-12,
            ..DiodeParams::default()
        };
        let diode = Diode::new(params, NodeId::new(1), NodeId::GROUND).with_timestep(1e-9);
        let (mut a, mut b, var_map) = two_node_system();
        let x = [0.0_f64, 0.4];
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 2);
            diode.stamp_nonlinear(&mut matrix, &var_map, &x);
        }
        let row0 = a[0] + a[1];
        let row1 = a[2] + a[3];
        assert!(row0.abs() < 1e-12, "Cj+junction row-0 KCL={row0}");
        assert!(row1.abs() < 1e-12, "Cj+junction row-1 KCL={row1}");
    }

    // ---------------------------------------------------------------------------
    // DeviceModel trait contract
    // ---------------------------------------------------------------------------

    #[test]
    fn terminals_returns_anode_then_cathode() {
        let diode = Diode::new(DiodeParams::default(), NodeId::new(3), NodeId::new(7));
        let t = diode.terminals();
        assert_eq!(t.len(), 2);
        assert_eq!(t[0], NodeId::new(3));
        assert_eq!(t[1], NodeId::new(7));
    }

    #[test]
    fn is_smooth_returns_false() {
        assert!(!default_diode().is_smooth());
    }

    #[test]
    fn dyn_device_model_is_usable() {
        fn accepts_dyn(_: &dyn DeviceModel) {}
        let diode = default_diode();
        accepts_dyn(&diode);
    }

    #[test]
    fn vec_of_boxed_dyn_compiles() {
        let d: Box<dyn DeviceModel> = Box::new(default_diode());
        assert_eq!(d.terminals().len(), 2);
        assert!(!d.is_smooth());
    }

    // ---------------------------------------------------------------------------
    // Numerical finite-difference check for the Jacobian
    // ---------------------------------------------------------------------------

    /// Jacobian entry (anode, anode) matches ∂I_anode/∂V_anode via finite diff.
    #[test]
    fn jacobian_matches_finite_difference_at_forward_bias() {
        let diode = default_diode();
        let var_map = VarMap::from_nodes(&[NodeId::GROUND, NodeId::new(1)]);
        let vd0 = 0.5_f64;
        let eps = 1e-7_f64;

        let stamp_at = |vd: f64| -> (Vec<f64>, Vec<f64>) {
            let mut a = vec![0.0_f64; 4];
            let mut b = vec![0.0_f64; 2];
            let mut m = MnaMatrix::new(&mut a, &mut b, 2);
            // anode=n1(index 1), cathode=GND(index 0)
            diode.stamp_nonlinear(&mut m, &var_map, &[0.0_f64, vd]);
            (a, b)
        };

        let (a0, _b0) = stamp_at(vd0);

        // The Jacobian a[1,1] = gd should match the analytic value.
        let n_vt = DiodeParams::default().n * DiodeParams::default().vt;
        let is = DiodeParams::default().is;
        let arg = (vd0 / n_vt).min(DIODE_MAX_EXP_ARG);
        let gd_expected = (is / n_vt) * arg.exp();

        let gd_stamp = a0[1 * 2 + 1]; // a[anode, anode]
        assert!(
            (gd_stamp - gd_expected).abs() < 1e-20,
            "gd from stamp={gd_stamp}, analytic={gd_expected}"
        );

        // Verify the Jacobian via central finite differences on the total current.
        // Total current at anode = gd * vd + I_eq = I_d(vd).
        // So ∂I_anode/∂V_anode ≈ (I_d(vd+eps) - I_d(vd-eps)) / (2*eps).
        let i_d_plus = is * (((vd0 + eps) / n_vt).min(DIODE_MAX_EXP_ARG).exp() - 1.0);
        let i_d_minus = is * (((vd0 - eps) / n_vt).min(DIODE_MAX_EXP_ARG).exp() - 1.0);
        let gd_fd = (i_d_plus - i_d_minus) / (2.0 * eps);

        let rel = (gd_stamp - gd_fd).abs() / gd_fd;
        assert!(
            rel < 1e-5,
            "Jacobian gd={gd_stamp} vs FD gd={gd_fd}, rel_err={rel}"
        );
    }
}
