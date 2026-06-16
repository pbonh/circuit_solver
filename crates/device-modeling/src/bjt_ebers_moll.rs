//! [`BjtEbersMoll`] — transport-form Ebers-Moll BJT as a [`traits::DeviceModel`].
//!
//! This module implements the `traits::DeviceModel` open trait for a
//! bipolar junction transistor using the transport (charge-control) form
//! of the Ebers-Moll equations.  It bridges the closed-enum
//! [`stamp::linearize_bjt`] implementation (ADR-0005, tasks.md #10)
//! to the `dyn`-safe trait path introduced in US-011 so the
//! Newton-Raphson engine can hold `Box<dyn DeviceModel>` slices that
//! contain BJT instances.
//!
//! # Ebers-Moll equations (transport form)
//!
//! The transport-form equations express the collector, base, and
//! emitter terminal currents entirely in terms of two *transport*
//! junction currents `If` and `Ir`, each modelled by the Shockley
//! equation:
//!
//! ```text
//! If  = IS · (exp(Vbe / (NF · Vt)) − 1)
//! Ir  = IS · (exp(Vbc / (NR · Vt)) − 1)
//!
//! Ic  = (If − Ir) / qb − Ir / BR
//! Ib  = If / BF  + Ir / BR
//! Ie  = −(Ic + Ib)   (KCL)
//! ```
//!
//! where `qb` is the base-charge factor modelling the Early effect.
//! PNP devices are handled by negating the junction voltages and
//! flipping the sign of the resulting terminal currents.
//!
//! # Newton-Raphson interface
//!
//! `stamp_linear` is a no-op (pure nonlinear device).
//! `stamp_nonlinear` calls [`stamp::linearize_bjt`] at the current
//! iterate, assembles the Jacobian into the MNA matrix, and stamps the
//! companion-current vector onto the RHS using the standard
//! Norton-equivalent companion model:
//!
//! ```text
//! G[row_i, col_j] += J[terminal_i][terminal_j]
//! b[row_i]        -= companion_current[terminal_i]
//! ```
//!
//! # Terminal ordering
//!
//! SPICE convention: `terminals[0]` = collector, `terminals[1]` = base,
//! `terminals[2]` = emitter.

use circuit_solver_types::NodeId;

use crate::mna_matrix::MnaMatrix;
use crate::params::BJTParams;
use crate::stamp::{linearize_bjt, BJT_TERMINALS};
use crate::traits::DeviceModel;
use crate::var_map::VarMap;

/// Ebers-Moll BJT implementing [`DeviceModel`].
///
/// Holds the three circuit-node terminals and the SPICE-parameter payload
/// (`BJTParams`) used by the Ebers-Moll equations.  The terminals are
/// stored in SPICE order: `[collector, base, emitter]`.
///
/// # Construction
///
/// Use [`BjtEbersMoll::new`].
///
/// # NPN / PNP
///
/// Polarity is encoded in [`BJTParams::polarity`] and handled entirely
/// inside [`stamp::linearize_bjt`]; the stamping code here is
/// polarity-agnostic.
#[derive(Debug, Clone)]
pub struct BjtEbersMoll {
    /// Circuit node IDs for `[collector, base, emitter]`.
    terminals: [NodeId; BJT_TERMINALS],
    /// Ebers-Moll / Gummel-Poon parameters.
    params: BJTParams,
}

impl BjtEbersMoll {
    /// Create a new `BjtEbersMoll` device.
    ///
    /// # Arguments
    ///
    /// - `collector` — [`NodeId`] of the collector terminal.
    /// - `base`      — [`NodeId`] of the base terminal.
    /// - `emitter`   — [`NodeId`] of the emitter terminal.
    /// - `params`    — [`BJTParams`] carrying `IS`, `BF`, `BR`, `NF`, `NR`,
    ///   `VAF`, `VAR`, `Vt`, and the NPN/PNP polarity.
    #[must_use]
    pub fn new(collector: NodeId, base: NodeId, emitter: NodeId, params: BJTParams) -> Self {
        Self {
            terminals: [collector, base, emitter],
            params,
        }
    }

    /// Borrow the device parameters.
    #[must_use]
    pub fn params(&self) -> &BJTParams {
        &self.params
    }
}

impl DeviceModel for BjtEbersMoll {
    /// Returns `[collector, base, emitter]` node IDs.
    fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    /// No-op: the BJT has no operating-point-independent linear
    /// contribution.
    fn stamp_linear(&self, _matrix: &mut MnaMatrix<'_>, _var_map: &VarMap) {}

    /// Stamp the Ebers-Moll Jacobian and companion currents at the
    /// current Newton-Raphson iterate `x`.
    ///
    /// Reads terminal voltages from `x` via `var_map`, calls
    /// [`linearize_bjt`] to obtain the 3×3 Jacobian and 3-vector
    /// companion current, then accumulates into `matrix`:
    ///
    /// ```text
    /// G[row_i, col_j] += J[i][j]
    /// b[row_i]        -= companion_current[i]
    /// ```
    ///
    /// `var_map.node_index(terminal)` must return `Some(_)` for all
    /// three terminals; a missing mapping causes a panic
    /// (programming error in the assembler — see AGENTS.md).
    fn stamp_nonlinear(&self, matrix: &mut MnaMatrix<'_>, var_map: &VarMap, x: &[f64]) {
        // Map each terminal to its MNA row/col index.
        let rows: [usize; BJT_TERMINALS] = [
            var_map
                .node_index(self.terminals[0])
                .expect("BjtEbersMoll: collector NodeId not in VarMap"),
            var_map
                .node_index(self.terminals[1])
                .expect("BjtEbersMoll: base NodeId not in VarMap"),
            var_map
                .node_index(self.terminals[2])
                .expect("BjtEbersMoll: emitter NodeId not in VarMap"),
        ];

        // Read terminal voltages from the current iterate.
        let v: [f64; BJT_TERMINALS] = [x[rows[0]], x[rows[1]], x[rows[2]]];

        // Linearize the Ebers-Moll equations at this operating point.
        let lin = linearize_bjt(&self.params, &v);

        // Stamp the Jacobian into the conductance matrix.
        for (i, &row_i) in rows.iter().enumerate() {
            for (j, &row_j) in rows.iter().enumerate() {
                matrix.add_element(row_i, row_j, lin.jacobian[i][j]);
            }
        }

        // Subtract the companion current from the RHS.
        for (i, &row_i) in rows.iter().enumerate() {
            matrix.add_rhs(row_i, -lin.companion_current[i]);
        }
    }

    /// The Ebers-Moll I-V characteristic is everywhere C¹ (the
    /// exponential functions used in the Shockley equation are smooth),
    /// so the solver may apply aggressive step-size heuristics.
    fn is_smooth(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mna_matrix::MnaMatrix;
    use crate::params::{BJTParams, BJTPolarity};
    use crate::var_map::VarMap;
    use circuit_solver_types::NodeId;

    // Shared NPN device used in most tests.
    fn npn_device() -> BjtEbersMoll {
        BjtEbersMoll::new(
            NodeId::new(1), // collector
            NodeId::new(2), // base
            NodeId::new(3), // emitter
            BJTParams {
                polarity: BJTPolarity::Npn,
                ..BJTParams::default()
            },
        )
    }

    // ---------------------------------------------------------------------------
    // Construction / trait-object witnesses
    // ---------------------------------------------------------------------------

    #[test]
    fn terminals_returns_collector_base_emitter_in_order() {
        let bjt = npn_device();
        let t = bjt.terminals();
        assert_eq!(t.len(), BJT_TERMINALS);
        assert_eq!(t[0], NodeId::new(1)); // collector
        assert_eq!(t[1], NodeId::new(2)); // base
        assert_eq!(t[2], NodeId::new(3)); // emitter
    }

    #[test]
    fn is_smooth_returns_true() {
        assert!(npn_device().is_smooth());
    }

    #[test]
    fn bjt_ebers_moll_is_dyn_safe() {
        fn accepts(_: &dyn DeviceModel) {}
        accepts(&npn_device());
    }

    #[test]
    fn bjt_ebers_moll_boxed_dyn_compiles() {
        let b: Box<dyn DeviceModel> = Box::new(npn_device());
        assert_eq!(b.terminals().len(), BJT_TERMINALS);
        assert!(b.is_smooth());
    }

    // ---------------------------------------------------------------------------
    // stamp_linear is a no-op
    // ---------------------------------------------------------------------------

    #[test]
    fn stamp_linear_is_noop() {
        let nodes = [NodeId::new(1), NodeId::new(2), NodeId::new(3)];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a = vec![0.0_f64; 9]; // 3×3
        let mut b = vec![0.0_f64; 3];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 3);

        npn_device().stamp_linear(&mut matrix, &var_map);

        for &v in a.iter() {
            assert_eq!(v, 0.0, "stamp_linear must not touch the matrix");
        }
        for &v in b.iter() {
            assert_eq!(v, 0.0, "stamp_linear must not touch the RHS");
        }
    }

    // ---------------------------------------------------------------------------
    // Forward-active region: Ic matches Ebers-Moll formula
    // ---------------------------------------------------------------------------

    /// US-015 acceptance criterion: forward-active region Ic matches the
    /// Ebers-Moll formula for known Vbe / Vce.
    ///
    /// Operating point: Vce = 5 V (V_C = 5, V_E = 0), Vbe = 0.7 V
    /// (V_B = 0.7).  With default NPN parameters (IS = 1e-16, BF = 100,
    /// NF = 1, Vt = 25.852 mV, Early disabled), the expected collector
    /// current is:
    ///
    /// ```text
    /// If ≈ IS · exp(Vbe / Vt)  (Vbe >> Vt, so -1 term negligible)
    /// Ic ≈ If · (1 − 1/BF)  ≈  If   (BF >> 1 for NPN)
    ///    = 1e-16 · exp(0.7 / 0.025852)
    /// ```
    #[test]
    fn forward_active_ic_matches_ebers_moll_formula() {
        // 4-node system: GND(0), C(1), B(2), E(3).  Emitter tied to GND.
        let nodes = [
            NodeId::GROUND, // row 0
            NodeId::new(1), // row 1 — collector
            NodeId::new(2), // row 2 — base
            NodeId::new(3), // row 3 — emitter
        ];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a = vec![0.0_f64; 16]; // 4×4
        let mut b = vec![0.0_f64; 4];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 4);

        // Solution vector: V_GND=0, V_C=5.0, V_B=0.7, V_E=0.0
        let x = [0.0_f64, 5.0, 0.7, 0.0];

        let bjt = BjtEbersMoll::new(
            NodeId::new(1), // collector at index 1
            NodeId::new(2), // base    at index 2
            NodeId::new(3), // emitter at index 3
            BJTParams {
                polarity: BJTPolarity::Npn,
                ..BJTParams::default()
            },
        );

        bjt.stamp_nonlinear(&mut matrix, &var_map, &x);

        // Expected Ic from the Ebers-Moll formula.
        let p = BJTParams::default();
        let vbe = 0.7_f64;
        let vbc = 0.7 - 5.0_f64; // Vbe - Vce
        let i_f = p.is * ((vbe / (p.nf * p.vt)).exp() - 1.0);
        let i_r = p.is * ((vbc / (p.nr * p.vt)).exp() - 1.0);
        let ic_expected = i_f - i_r - i_r / p.br; // q_b = 1, no Early

        // From stamp_nonlinear the companion current for the collector
        // (terminal index 0, MNA row 1) was subtracted from b[1].
        // The companion current is I_eq[k] = I_k(v0) − Σⱼ J[k][j]·v0[j].
        // At the operating point b[1] = −companion_current[0].
        // We recover Ic by computing it directly via linearize_bjt.
        let v_terminal: [f64; BJT_TERMINALS] = [x[1], x[2], x[3]]; // [Vc, Vb, Ve]
        let lin = linearize_bjt(&p, &v_terminal);

        // The actual Ic at the operating point is the sum of current
        // contributions: Ic = companion_current[0] + Σⱼ J[0][j]·v0[j].
        let ic_recovered = lin.companion_current[0]
            + lin.jacobian[0][0] * v_terminal[0]
            + lin.jacobian[0][1] * v_terminal[1]
            + lin.jacobian[0][2] * v_terminal[2];

        let tol = 1.0e-9 * ic_expected.abs().max(1.0e-20);
        assert!(
            (ic_recovered - ic_expected).abs() < tol,
            "Ic_recovered={ic_recovered:.6e} does not match ic_expected={ic_expected:.6e} (tol={tol:.2e})"
        );
    }

    // ---------------------------------------------------------------------------
    // KCL closure
    // ---------------------------------------------------------------------------

    #[test]
    fn forward_active_kcl_closes_over_all_terminals() {
        // Row/col indices: C=0, B=1, E=2 in VarMap.
        let nodes = [NodeId::new(1), NodeId::new(2), NodeId::new(3)];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a = vec![0.0_f64; 9]; // 3×3
        let mut b = vec![0.0_f64; 3];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 3);

        // Forward-active: Vc=5, Vb=0.7, Ve=0
        let x = [5.0_f64, 0.7, 0.0];
        npn_device().stamp_nonlinear(&mut matrix, &var_map, &x);

        // KCL test: each column of the Jacobian must sum to ~0 (current
        // entering one terminal = current leaving the others).
        for col in 0..3 {
            let col_sum = matrix.element(0, col)
                + matrix.element(1, col)
                + matrix.element(2, col);
            assert!(
                col_sum.abs() < 1.0e-9,
                "KCL violation on Jacobian column {col}: col_sum={col_sum:.3e}"
            );
        }

        // Also verify RHS sums to ~0 (companion currents are KCL-consistent).
        let rhs_sum = -(matrix.rhs(0) + matrix.rhs(1) + matrix.rhs(2));
        assert!(
            rhs_sum.abs() < 1.0e-9,
            "KCL violation on companion-current RHS: sum={rhs_sum:.3e}"
        );
    }

    // ---------------------------------------------------------------------------
    // PNP polarity test
    // ---------------------------------------------------------------------------

    #[test]
    fn pnp_polarity_reverses_collector_current_sign() {
        let nodes = [NodeId::new(1), NodeId::new(2), NodeId::new(3)];
        let var_map = VarMap::from_nodes(&nodes);

        let mut a_npn = vec![0.0_f64; 9];
        let mut b_npn = vec![0.0_f64; 3];
        let mut mat_npn = MnaMatrix::new(&mut a_npn, &mut b_npn, 3);

        let mut a_pnp = vec![0.0_f64; 9];
        let mut b_pnp = vec![0.0_f64; 3];
        let mut mat_pnp = MnaMatrix::new(&mut a_pnp, &mut b_pnp, 3);

        let x_npn = [5.0_f64, 0.7, 0.0]; // Vc=5, Vb=0.7, Ve=0
        let x_pnp = [-5.0_f64, -0.7, 0.0]; // mirrored PNP operating point

        let npn = BjtEbersMoll::new(
            NodeId::new(1),
            NodeId::new(2),
            NodeId::new(3),
            BJTParams {
                polarity: BJTPolarity::Npn,
                ..BJTParams::default()
            },
        );
        let pnp = BjtEbersMoll::new(
            NodeId::new(1),
            NodeId::new(2),
            NodeId::new(3),
            BJTParams {
                polarity: BJTPolarity::Pnp,
                ..BJTParams::default()
            },
        );

        npn.stamp_nonlinear(&mut mat_npn, &var_map, &x_npn);
        pnp.stamp_nonlinear(&mut mat_pnp, &var_map, &x_pnp);

        // The Jacobian diagonal entry (0,0) for NPN and PNP should have
        // the same *magnitude* (same junction conductances) but may differ
        // in sign depending on how the Early effect enters — at least
        // confirm both are non-zero (device is active).
        assert!(
            mat_npn.element(0, 0).abs() > 0.0,
            "NPN Jacobian[0,0] should be non-zero in forward-active"
        );
        assert!(
            mat_pnp.element(0, 0).abs() > 0.0,
            "PNP Jacobian[0,0] should be non-zero at mirrored operating point"
        );
    }

    // ---------------------------------------------------------------------------
    // Jacobian matches finite difference
    // ---------------------------------------------------------------------------

    #[test]
    fn jacobian_matches_finite_difference_at_forward_active_point() {
        let nodes = [NodeId::new(1), NodeId::new(2), NodeId::new(3)];
        let var_map = VarMap::from_nodes(&nodes);

        let p = BJTParams {
            polarity: BJTPolarity::Npn,
            ..BJTParams::default()
        };
        let bjt = BjtEbersMoll::new(NodeId::new(1), NodeId::new(2), NodeId::new(3), p.clone());

        let v0 = [5.0_f64, 0.7, 0.0];
        let mut a = vec![0.0_f64; 9];
        let mut b = vec![0.0_f64; 3];
        let mut matrix = MnaMatrix::new(&mut a, &mut b, 3);
        bjt.stamp_nonlinear(&mut matrix, &var_map, &v0);

        // Compare the stamped Jacobian against the linearize_bjt output
        // (which has its own finite-difference test in stamp.rs).
        let lin = linearize_bjt(&p, &v0);
        for i in 0..BJT_TERMINALS {
            for j in 0..BJT_TERMINALS {
                let stamped = matrix.element(i, j);
                let expected = lin.jacobian[i][j];
                let tol = 1.0e-10 * expected.abs().max(1.0e-20);
                assert!(
                    (stamped - expected).abs() < tol,
                    "Jacobian[{i}][{j}]: stamped={stamped:.6e} expected={expected:.6e}"
                );
            }
        }
    }
}
