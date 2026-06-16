//! [`MosfetLevel1`] — `traits::DeviceModel` implementor for the SPICE Level-1 MOSFET.
//!
//! This module wraps the closed-enum [`linearize_mosfet_level1`] stamp in the
//! open [`traits::DeviceModel`] trait so the Newton-Raphson engine can hold a
//! `Vec<Box<dyn DeviceModel>>` containing Level-1 MOSFETs alongside other
//! device types.
//!
//! # DC operating point
//!
//! `stamp_nonlinear` evaluates the Shichman-Hodges square-law I-V and stamps
//! the 4×4 Jacobian plus companion current into the MNA system.  See
//! [`linearize_mosfet_level1`] for the equation details.
//!
//! # Meyer gate capacitance (transient)
//!
//! When [`MosfetLevel1::timestep`] is non-zero, `stamp_nonlinear` also stamps
//! the bias-dependent Meyer gate capacitances as trapezoidal companion
//! conductances `G_eq = 2·C/h` between the relevant terminal pairs.
//!
//! The Meyer model partitions the total intrinsic gate capacitance
//! `C_ox = cox_wl` (= Cox·W·L, in Farads) as follows:
//!
//! ```text
//! Cutoff (V_ov ≤ 0):
//!   Cgs = 0,         Cgd = 0,         Cgb = C_ox
//!
//! Triode (0 < V_ds < V_ov):
//!   Cgs = C_ox/2,    Cgd = C_ox/2,    Cgb = 0
//!
//! Saturation (V_ds ≥ V_ov):
//!   Cgs = 2/3·C_ox,  Cgd = 0,         Cgb = 0
//! ```
//!
//! Each capacitance `C` is converted to a trapezoidal companion conductance
//! `G_eq = 2·C/h` and stamped between the respective terminal pair (G-S,
//! G-D, or G-B).  No companion current is added (zero-initial-state
//! trapezoidal step), which is the standard first-step assumption used by
//! SPICE when the previous charge state is unknown.
//!
//! # Terminal order
//!
//! The four terminals are stored and expected in `[drain, gate, source, bulk]`
//! order, consistent with [`MOSFET_TERMINALS`] and [`stamp::MOSFET_TERMINALS`].

use circuit_solver_types::NodeId;

use crate::mna_matrix::MnaMatrix;
use crate::params::{MosLevel1Params, MosPolarity};
use crate::stamp::{linearize_mosfet_level1, MOSFET_TERMINALS};
use crate::traits::DeviceModel;
use crate::var_map::VarMap;

/// Drain terminal slot.
const D: usize = 0;
/// Gate terminal slot.
const G: usize = 1;
/// Source terminal slot.
const S: usize = 2;
/// Bulk terminal slot.
const B: usize = 3;

/// SPICE Level-1 MOSFET implementing [`DeviceModel`].
///
/// Provides square-law DC stamps and (optionally) Meyer gate capacitance
/// transient stamps to the Newton-Raphson MNA engine.
///
/// # Example
///
/// ```rust
/// use device_modeling::mosfet_level1_device::MosfetLevel1;
/// use device_modeling::params::{MosLevel1Params, MosPolarity};
/// use circuit_solver_types::{ModelName, NodeId};
///
/// let params = MosLevel1Params {
///     name: ModelName::new("nmos"),
///     polarity: MosPolarity::Nmos,
///     vto: 1.0,
///     kp: 50e-6,
///     lambda: 0.0,
///     gamma: 0.0,
///     phi: 0.6,
///     kf: 0.0,
///     af: 1.0,
/// };
/// let nmos = MosfetLevel1::new(
///     params,
///     [NodeId::new(1), NodeId::new(2), NodeId::GROUND, NodeId::GROUND],
///     0.0,   // cox_wl = 0 disables Meyer caps
///     0.0,   // timestep = 0 disables transient stamps
/// );
/// ```
#[derive(Debug, Clone)]
pub struct MosfetLevel1 {
    /// Level-1 model card parameters.
    pub params: MosLevel1Params,

    /// Node identifiers in `[drain, gate, source, bulk]` order.
    terminals: [NodeId; MOSFET_TERMINALS],

    /// Total intrinsic gate oxide capacitance Cox·W·L (Farads).
    ///
    /// Set to `0.0` to disable the Meyer capacitance stamp.  A positive
    /// value enables trapezoidal companion conductances in `stamp_nonlinear`
    /// when [`timestep`](MosfetLevel1::timestep) is also positive.
    pub cox_wl: f64,

    /// Transient timestep `h` (seconds).
    ///
    /// When positive (transient analysis), `stamp_nonlinear` stamps Meyer
    /// capacitances as `G_eq = 2·C/h` conductances.  Set to `0.0` for DC
    /// analysis (no capacitive stamps).
    pub timestep: f64,
}

impl MosfetLevel1 {
    /// Construct a new Level-1 MOSFET device model.
    ///
    /// # Arguments
    ///
    /// - `params`    — the Level-1 model card (`VTO`, `KP`, …, `polarity`).
    /// - `terminals` — node ids `[drain, gate, source, bulk]`.
    /// - `cox_wl`    — total intrinsic gate capacitance Cox·W·L (F).
    ///   Pass `0.0` to disable Meyer stamps.
    /// - `timestep`  — transient timestep `h` (s).
    ///   Pass `0.0` for a DC-only device.
    #[must_use]
    pub fn new(
        params: MosLevel1Params,
        terminals: [NodeId; MOSFET_TERMINALS],
        cox_wl: f64,
        timestep: f64,
    ) -> Self {
        Self {
            params,
            terminals,
            cox_wl,
            timestep,
        }
    }

    /// Update the timestep for the next transient step.
    ///
    /// The transient control loop calls this before each time step to keep
    /// the companion conductance `G_eq = 2·C/h` current.
    pub fn set_timestep(&mut self, h: f64) {
        self.timestep = h;
    }

    // -----------------------------------------------------------------------
    // Internal: stamp one two-terminal trapezoidal companion conductance.
    // -----------------------------------------------------------------------

    /// Stamp a two-terminal trapezoidal companion conductance between `na`
    /// and `nb` (MNA indices) with capacitance `c` and timestep `h`.
    ///
    /// Uses the Trapezoidal rule: `G_eq = 2·C/h`, stamped as the standard
    /// 2×2 conductance stamp (no companion current — zero initial state).
    fn stamp_cap_conductance(
        matrix: &mut MnaMatrix<'_>,
        na: usize,
        nb: usize,
        c: f64,
        h: f64,
    ) {
        debug_assert!(h > 0.0, "timestep must be positive to stamp capacitor");
        let g_eq = 2.0 * c / h;
        matrix.add_element(na, na, g_eq);
        matrix.add_element(nb, nb, g_eq);
        matrix.add_element(na, nb, -g_eq);
        matrix.add_element(nb, na, -g_eq);
    }

    // -----------------------------------------------------------------------
    // Internal: compute Meyer capacitances at the given operating voltages.
    // -----------------------------------------------------------------------

    /// Compute (Cgs, Cgd, Cgb) under the Meyer model from the operating-point
    /// voltages and return them as a tuple.
    ///
    /// Voltages are in `[drain, gate, source, bulk]` true-terminal order.
    fn meyer_caps(&self, v: &[f64; MOSFET_TERMINALS]) -> (f64, f64, f64) {
        let cox = self.cox_wl;
        if cox <= 0.0 {
            return (0.0, 0.0, 0.0);
        }

        // Source-referenced operating voltages (NMOS-equivalent).
        let polarity_sign: f64 = match self.params.polarity {
            MosPolarity::Nmos => 1.0,
            MosPolarity::Pmos => -1.0,
        };
        let vgs = polarity_sign * (v[G] - v[S]);
        let vds = polarity_sign * (v[D] - v[S]);
        let vbs = polarity_sign * (v[B] - v[S]);

        // Threshold with body effect (same formula as linearize_mosfet_level1).
        let vto_mag = self.params.vto.abs();
        let phi = self.params.phi;
        let gamma = self.params.gamma;
        let body_sqrt_arg = (phi - vbs).max(0.0);
        let phi_sqrt = phi.max(0.0).sqrt();
        let vth = vto_mag + gamma * (body_sqrt_arg.sqrt() - phi_sqrt);

        let v_ov = vgs - vth;

        if v_ov <= 0.0 {
            // Cutoff: full gate cap sits between gate and bulk.
            (0.0, 0.0, cox)
        } else if vds < v_ov {
            // Triode: cap splits equally between Cgs and Cgd.
            (0.5 * cox, 0.5 * cox, 0.0)
        } else {
            // Saturation: 2/3 sits between gate and source, none to drain.
            (2.0 / 3.0 * cox, 0.0, 0.0)
        }
    }
}

impl DeviceModel for MosfetLevel1 {
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    /// No bias-independent DC contribution; all stamps go through
    /// `stamp_nonlinear`.  For transient analysis the Meyer caps are
    /// also operating-point dependent, so they live in `stamp_nonlinear`
    /// as well.
    fn stamp_linear(&self, _matrix: &mut MnaMatrix<'_>, _var_map: &VarMap) {
        // Purely nonlinear device; no linear contribution at this level.
    }

    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, x: &[f64]) {
        // Resolve terminal MNA indices.
        let Some(id_d) = var_map.node_index(self.terminals[D]) else {
            return;
        };
        let Some(id_g) = var_map.node_index(self.terminals[G]) else {
            return;
        };
        let Some(id_s) = var_map.node_index(self.terminals[S]) else {
            return;
        };
        let Some(id_b) = var_map.node_index(self.terminals[B]) else {
            return;
        };

        // Read current terminal voltages from the solution vector.
        let v: [f64; MOSFET_TERMINALS] = [x[id_d], x[id_g], x[id_s], x[id_b]];

        // --- DC stamp (Shichman-Hodges Jacobian + companion current) ---
        let lin = linearize_mosfet_level1(&self.params, &v);

        // Map our [D, G, S, B] slots to MNA indices.
        let mna_idx = [id_d, id_g, id_s, id_b];

        for (k, &row) in mna_idx.iter().enumerate() {
            // Jacobian entries for row k.
            for (j, &col) in mna_idx.iter().enumerate() {
                let g = lin.jacobian[k][j];
                if g != 0.0 {
                    matrix.add_element(row, col, g);
                }
            }
            // Companion current (RHS contribution).
            let ieq = lin.companion_current[k];
            if ieq != 0.0 {
                matrix.add_rhs(row, ieq);
            }
        }

        // --- Meyer gate capacitance transient stamp ---
        // Only active when both cox_wl > 0 and timestep > 0.
        if self.cox_wl > 0.0 && self.timestep > 0.0 {
            let (cgs, cgd, cgb) = self.meyer_caps(&v);

            // Cgs: between gate (id_g) and source (id_s).
            if cgs > 0.0 {
                Self::stamp_cap_conductance(matrix, id_g, id_s, cgs, self.timestep);
            }
            // Cgd: between gate (id_g) and drain (id_d).
            if cgd > 0.0 {
                Self::stamp_cap_conductance(matrix, id_g, id_d, cgd, self.timestep);
            }
            // Cgb: between gate (id_g) and bulk (id_b).
            if cgb > 0.0 {
                Self::stamp_cap_conductance(matrix, id_g, id_b, cgb, self.timestep);
            }
        }
    }

    /// Level-1 is piecewise-smooth (C¹ except at the cutoff/triode and
    /// triode/saturation boundaries, where the first derivative is
    /// continuous but the second is not).  The NR engine treats `false`
    /// here as a hint to use smaller, more conservative updates near
    /// region boundaries.
    fn is_smooth(&self) -> bool {
        // The Level-1 model has two kink points (Vov=0, Vds=Vov); it is C0
        // but not C1 across those boundaries (the derivatives are continuous
        // but the model is piecewise — not globally analytic).  Return false
        // to allow the convergence heuristics to apply smaller steps.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mna_matrix::MnaMatrix;
    use crate::params::MosLevel1Params;
    use crate::var_map::VarMap;
    use circuit_solver_types::{ModelName, NodeId};

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Standard NMOS test device: VTO=1V, KP=50µA/V², LAMBDA=0, no body effect.
    fn nmos_device() -> MosfetLevel1 {
        let params = MosLevel1Params {
            name: ModelName::new("nmos_test"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,
            kp: 50.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        };
        // terminals: [D=n1, G=n2, S=GND, B=GND]
        MosfetLevel1::new(
            params,
            [NodeId::new(1), NodeId::new(2), NodeId::GROUND, NodeId::GROUND],
            0.0,
            0.0,
        )
    }

    /// Helper: build a 4-node VarMap for [n_d, n_g, n_s, n_b] and a 4×4 MNA.
    fn make_mna(
        nd: NodeId,
        ng: NodeId,
        ns: NodeId,
        nb: NodeId,
    ) -> (VarMap, Vec<f64>, Vec<f64>) {
        let nodes = [nd, ng, ns, nb];
        let vm = VarMap::from_nodes(&nodes);
        let a = vec![0.0_f64; 16]; // 4×4
        let b = vec![0.0_f64; 4];
        (vm, a, b)
    }

    // -----------------------------------------------------------------------
    // US-014 acceptance test: saturation I_D within 0.1 %
    // -----------------------------------------------------------------------

    /// Verify that `stamp_nonlinear` recovers `I_D = (KP/2)·V_ov^2`
    /// in saturation to within 0.1 % of the analytical Level-1 formula.
    ///
    /// Chosen operating point:
    ///   VTO=1V, KP=50µA/V², Vgs=3V → V_ov=2V, Vds=5V (>> V_ov → saturation)
    ///   Expected I_D = 0.5 · 50e-6 · 4 = 100 µA
    #[test]
    fn saturation_id_matches_level1_formula_within_0_1_pct() {
        let device = nmos_device();

        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::GROUND;

        let (vm, mut a, mut b) = make_mna(nd, ng, ns, nb);

        // Solution vector: V_d=5V, V_g=3V, V_s=0, V_b=0
        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();
        let id_b = vm.node_index(nb).unwrap();

        let mut x = vec![0.0_f64; 4];
        x[id_d] = 5.0;
        x[id_g] = 3.0;
        x[id_s] = 0.0;
        x[id_b] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        // Recover I_D from the stamp: I_D(V*) = Σ_j J[D,j]·V*_j + I_eq_D
        // which equals the sum over the drain row in `a` times `x`, plus `b[id_d]`.
        let id_reconstructed: f64 = (0..4)
            .map(|j| a[id_d * 4 + j] * x[j])
            .sum::<f64>()
            + b[id_d];

        // Analytical Level-1 saturation: I_D = (KP/2)·(Vgs - VTO)^2
        let kp = device.params.kp;
        let vgs = x[id_g] - x[id_s]; // 3.0 V
        let vto = device.params.vto; // 1.0 V
        let v_ov = vgs - vto; // 2.0 V
        let id_expected = 0.5 * kp * v_ov * v_ov; // 100 µA

        let rel_error = (id_reconstructed - id_expected).abs() / id_expected;
        assert!(
            rel_error < 1.0e-3, // 0.1 %
            "saturation I_D: expected {id_expected:.6e} A, got {id_reconstructed:.6e} A \
             (rel error = {rel_error:.2e})"
        );
    }

    // -----------------------------------------------------------------------
    // traits::DeviceModel interface
    // -----------------------------------------------------------------------

    #[test]
    fn terminals_returns_four_nodes_in_d_g_s_b_order() {
        let device = nmos_device();
        let t = device.terminals();
        assert_eq!(t.len(), 4);
        assert_eq!(t[D], NodeId::new(1)); // drain
        assert_eq!(t[G], NodeId::new(2)); // gate
        assert_eq!(t[S], NodeId::GROUND); // source
        assert_eq!(t[B], NodeId::GROUND); // bulk
    }

    #[test]
    fn stamp_linear_is_noop() {
        let device = nmos_device();
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::GROUND;
        let (vm, mut a, mut b) = make_mna(nd, ng, ns, nb);
        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_linear(&mut matrix, &vm);
        }
        assert!(a.iter().all(|&v| v == 0.0), "stamp_linear must be a no-op");
        assert!(b.iter().all(|&v| v == 0.0), "stamp_linear RHS must be zero");
    }

    #[test]
    fn is_smooth_returns_false() {
        let device = nmos_device();
        assert!(!device.is_smooth(), "Level-1 has region-boundary kinks");
    }

    // -----------------------------------------------------------------------
    // Cutoff / triode regions through the DeviceModel interface
    // -----------------------------------------------------------------------

    #[test]
    fn cutoff_stamps_zero_current_and_zero_jacobian() {
        let device = nmos_device();
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::GROUND;
        let (vm, mut a, mut b) = make_mna(nd, ng, ns, nb);

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();
        let id_b = vm.node_index(nb).unwrap();

        // Vgs = 0.5 V < VTO = 1 V → cutoff
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 3.0;
        x[id_g] = 0.5;
        x[id_s] = 0.0;
        x[id_b] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        // All Jacobian entries and companion currents must be zero in cutoff.
        for &v in a.iter() {
            assert!(v.abs() <= f64::EPSILON, "cutoff: non-zero Jacobian entry {v}");
        }
        for &v in b.iter() {
            assert!(v.abs() <= f64::EPSILON, "cutoff: non-zero companion current {v}");
        }
    }

    #[test]
    fn triode_stamps_nonzero_current() {
        let device = nmos_device();
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::GROUND;
        let (vm, mut a, mut b) = make_mna(nd, ng, ns, nb);

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();
        let id_b = vm.node_index(nb).unwrap();

        // Vgs=3V → Vov=2V, Vds=0.5V < Vov → triode
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 0.5;
        x[id_g] = 3.0;
        x[id_s] = 0.0;
        x[id_b] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        // I_D = KP·(Vov·Vds - Vds^2/2) = 50e-6·(2·0.5 - 0.125) = 50e-6·0.875 = 43.75 µA
        let id_reconstructed: f64 = (0..4)
            .map(|j| a[id_d * 4 + j] * x[j])
            .sum::<f64>()
            + b[id_d];
        let expected = device.params.kp * (2.0 * 0.5 - 0.5 * 0.5 * 0.5);
        let rel = (id_reconstructed - expected).abs() / expected;
        assert!(rel < 1.0e-12, "triode I_D mismatch: {id_reconstructed} vs {expected}");
    }

    // -----------------------------------------------------------------------
    // PMOS polarity
    // -----------------------------------------------------------------------

    #[test]
    fn pmos_saturation_drain_current_is_negative() {
        let params = MosLevel1Params {
            name: ModelName::new("pmos_test"),
            polarity: MosPolarity::Pmos,
            vto: -1.0,
            kp: 25.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        };
        let vdd = NodeId::new(3);
        let ng = NodeId::new(2);
        let nd = NodeId::new(1);
        let nb = NodeId::new(3); // bulk tied to VDD
        let device = MosfetLevel1::new(params, [nd, ng, vdd, nb], 0.0, 0.0);

        let (vm, mut a, mut b) = make_mna(nd, ng, vdd, nb);

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(vdd).unwrap();
        let _id_b = vm.node_index(nb).unwrap();

        // V_s=3.3V, V_g=1.3V → V_sg=2V, |VTP|=1V → Vov_eq=1V
        // V_sd=3.0V > Vov_eq → saturation
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 0.3;
        x[id_g] = 1.3;
        x[id_s] = 3.3;
        // id_b same as id_s since nb==vdd
        // x[id_b] = 3.3 set implicitly below
        x[vm.node_index(nb).unwrap()] = 3.3;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        let id_reconstructed: f64 = (0..4)
            .map(|j| a[id_d * 4 + j] * x[j])
            .sum::<f64>()
            + b[id_d];

        assert!(
            id_reconstructed < 0.0,
            "PMOS drain current must be negative, got {id_reconstructed}"
        );
    }

    // -----------------------------------------------------------------------
    // Meyer capacitance stamps
    // -----------------------------------------------------------------------

    #[test]
    fn meyer_caps_cutoff_stamps_cgb_only() {
        let params = MosLevel1Params {
            name: ModelName::new("nmos_cap"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,
            kp: 50.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        };
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::new(3);
        let cox_wl = 1.0e-14; // 10 fF
        let h = 1.0e-9; // 1 ns
        let device = MosfetLevel1::new(params, [nd, ng, ns, nb], cox_wl, h);

        let nodes = [nd, ng, ns, nb];
        let vm = VarMap::from_nodes(&nodes);
        let mut a = vec![0.0_f64; 16];
        let mut b = vec![0.0_f64; 4];

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();
        let id_b = vm.node_index(nb).unwrap();

        // Vgs=0 < VTO=1 → cutoff → Cgb = cox_wl, Cgs=Cgd=0
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 1.0;
        x[id_g] = 0.0;
        x[id_s] = 0.0;
        x[id_b] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        let g_eq = 2.0 * cox_wl / h; // G_eq for Cgb

        // G-B conductance should be stamped.
        let g_gb = a[id_g * 4 + id_b];
        let g_bg = a[id_b * 4 + id_g];
        assert!(
            (g_gb + g_eq).abs() < 1.0e-20,
            "Cgb off-diagonal G-B: expected {}, got {}",
            -g_eq,
            g_gb,
        );
        assert!(
            (g_bg + g_eq).abs() < 1.0e-20,
            "Cgb off-diagonal B-G: expected {}, got {}",
            -g_eq,
            g_bg,
        );

        // No G-S or G-D conductance from caps in cutoff.
        // (We check only the off-diagonal coupling from caps; the diagonal
        //  includes both Jacobian and cap entries, so we skip it.)
        let g_gs_cap = a[id_g * 4 + id_s];
        // In cutoff the DC Jacobian for G-S is zero, so the whole entry is
        // from caps; but here Cgs=0, so it should be zero.
        assert!(
            g_gs_cap.abs() < 1.0e-30,
            "Cgs should be 0 in cutoff; got G-S coupling {g_gs_cap}"
        );
        let g_gd_cap = a[id_g * 4 + id_d];
        assert!(
            g_gd_cap.abs() < 1.0e-30,
            "Cgd should be 0 in cutoff; got G-D coupling {g_gd_cap}"
        );
    }

    #[test]
    fn meyer_caps_saturation_stamps_cgs_only() {
        let params = MosLevel1Params {
            name: ModelName::new("nmos_cap"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,
            kp: 50.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        };
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::new(3);
        let cox_wl = 1.0e-14; // 10 fF
        let h = 1.0e-9; // 1 ns
        let device = MosfetLevel1::new(params, [nd, ng, ns, nb], cox_wl, h);

        let nodes = [nd, ng, ns, nb];
        let vm = VarMap::from_nodes(&nodes);
        let mut a = vec![0.0_f64; 16];
        let mut b = vec![0.0_f64; 4];

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();
        let id_b = vm.node_index(nb).unwrap();

        // Vgs=3V > VTO=1V → Vov=2V; Vds=5V >> Vov → saturation
        // → Cgs = 2/3·cox_wl, Cgd=0, Cgb=0
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 5.0;
        x[id_g] = 3.0;
        x[id_s] = 0.0;
        x[id_b] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        let cgs_expected = 2.0 / 3.0 * cox_wl;
        let g_eq_cgs = 2.0 * cgs_expected / h;

        // G-S off-diagonal from Cgs.
        let g_gs = a[id_g * 4 + id_s];
        assert!(
            (g_gs + g_eq_cgs).abs() < 1.0e-20,
            "Cgs G-S off-diagonal: expected {}, got {}",
            -g_eq_cgs,
            g_gs,
        );

        // No G-D or G-B coupling from caps.
        let g_gd = a[id_g * 4 + id_d];
        assert!(
            g_gd.abs() < 1.0e-30,
            "Cgd should be 0 in saturation; got {g_gd}"
        );
        let g_gb = a[id_g * 4 + id_b];
        assert!(
            g_gb.abs() < 1.0e-30,
            "Cgb should be 0 in saturation; got {g_gb}"
        );
    }

    #[test]
    fn meyer_caps_triode_stamps_equal_cgs_and_cgd() {
        let params = MosLevel1Params {
            name: ModelName::new("nmos_cap"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,
            kp: 50.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        };
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::new(3);
        let cox_wl = 1.0e-14; // 10 fF
        let h = 1.0e-9; // 1 ns
        let device = MosfetLevel1::new(params, [nd, ng, ns, nb], cox_wl, h);

        let nodes = [nd, ng, ns, nb];
        let vm = VarMap::from_nodes(&nodes);
        let mut a = vec![0.0_f64; 16];
        let mut b = vec![0.0_f64; 4];

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();
        let id_b = vm.node_index(nb).unwrap();

        // Vgs=3V → Vov=2V; Vds=0.5V < Vov → triode
        // → Cgs=Cgd=cox_wl/2, Cgb=0
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 0.5;
        x[id_g] = 3.0;
        x[id_s] = 0.0;
        x[id_b] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        let c_half = 0.5 * cox_wl;
        let g_eq_half = 2.0 * c_half / h;

        // G-S and G-D off-diagonal entries should both be -G_eq.
        let g_gs = a[id_g * 4 + id_s];
        let g_gd = a[id_g * 4 + id_d];
        assert!(
            (g_gs + g_eq_half).abs() < 1.0e-20,
            "triode Cgs G-S: expected {}, got {}",
            -g_eq_half,
            g_gs,
        );
        assert!(
            (g_gd + g_eq_half).abs() < 1.0e-20,
            "triode Cgd G-D: expected {}, got {}",
            -g_eq_half,
            g_gd,
        );
        // No G-B coupling.
        let g_gb = a[id_g * 4 + id_b];
        assert!(
            g_gb.abs() < 1.0e-30,
            "triode Cgb should be 0; got {g_gb}"
        );
    }

    #[test]
    fn no_cap_stamp_when_timestep_is_zero() {
        let params = MosLevel1Params {
            name: ModelName::new("nmos_cap"),
            polarity: MosPolarity::Nmos,
            vto: 1.0,
            kp: 50.0e-6,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            kf: 0.0,
            af: 1.0,
        };
        let nd = NodeId::new(1);
        let ng = NodeId::new(2);
        let ns = NodeId::GROUND;
        let nb = NodeId::new(3);
        // cox_wl is non-zero but timestep=0 → no cap stamps.
        let device = MosfetLevel1::new(params, [nd, ng, ns, nb], 1.0e-14, 0.0);

        let nodes = [nd, ng, ns, nb];
        let vm = VarMap::from_nodes(&nodes);
        let mut a = vec![0.0_f64; 16];
        let mut b = vec![0.0_f64; 4];

        let id_d = vm.node_index(nd).unwrap();
        let id_g = vm.node_index(ng).unwrap();
        let id_s = vm.node_index(ns).unwrap();

        // Saturation operating point — would stamp Cgs if timestep > 0.
        let mut x = vec![0.0_f64; 4];
        x[id_d] = 5.0;
        x[id_g] = 3.0;
        x[id_s] = 0.0;
        x[vm.node_index(nb).unwrap()] = 0.0;

        {
            let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);
            device.stamp_nonlinear(&mut matrix, &vm, &x);
        }

        // G-S off-diagonal should come only from the DC Jacobian (not caps).
        // In saturation the DC Jacobian has J[D,G]=gm, J[D,D]=0, J[D,S]=-gm.
        // The G row is all zeros in Level-1 DC (no gate current).
        let g_gs_off = a[id_g * 4 + id_s];
        assert!(
            g_gs_off.abs() < 1.0e-30,
            "no cap stamp expected when timestep=0; G-S coupling = {g_gs_off}"
        );
    }

    // -----------------------------------------------------------------------
    // dyn DeviceModel usability
    // -----------------------------------------------------------------------

    #[test]
    fn dyn_mosfet_level1_usable_as_trait_object() {
        let device: Box<dyn DeviceModel> = Box::new(nmos_device());
        assert_eq!(device.terminals().len(), 4);
        assert!(!device.is_smooth());
    }
}
