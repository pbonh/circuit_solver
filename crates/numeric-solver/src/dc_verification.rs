//! DC analysis integration tests — US-024.
//!
//! Integration tests that exercise [`DcAnalysis::run`] end-to-end
//! against canonical circuit fixtures, confirming convergence behaviour
//! and error reporting as required by the US-024 acceptance criteria:
//!
//! 1. **Resistor divider** (1 kΩ / 1 kΩ, V = 10 V): converges in
//!    ≤ 3 Newton-Raphson iterations; output node = 5 V within 1e-6.
//!
//! 2. **Diode + 1 kΩ resistor** (V = 1 V): DC solution within 1 % of
//!    the hand-computed iterative Shockley answer.
//!
//! 3. **Gmin stepping** recovers a current-fed diode circuit where the
//!    initial guess is 0 V (plain NR diverges because the Jacobian is
//!    near-zero at the zero-bias operating point).
//!
//! 4. **[`ConvergenceError`]** carries a `residue_norm` and an
//!    iteration count accessible via `inner_status.diagnostic()`.

#[cfg(test)]
mod tests {
    use crate::dc_analysis::DcAnalysis;
    use crate::homotopy_engine::ConvergenceError;
    use crate::linear_solver::{RussellRealSolver, SparseLinearSystem, SparseTriplet};
    use crate::newton_raphson::{NewtonRaphsonConfig, NonlinearSystem, SystemError};
    use crate::source_stepping::SourceSteppableSystem;
    use circuit_solver_types::ConvergenceTolerances;

    // -----------------------------------------------------------------------
    // Shared helpers
    // -----------------------------------------------------------------------

    /// Clamp `arg` to `[-limit, limit]` — used to guard the Shockley
    /// exponent so it doesn't overflow before NR converges.
    #[inline]
    fn clamp(arg: f64, limit: f64) -> f64 {
        arg.max(-limit).min(limit)
    }

    // -----------------------------------------------------------------------
    // Shockley diode constants (SPICE defaults, matching DiodeParams::default())
    // -----------------------------------------------------------------------

    const DIODE_IS: f64 = 1e-14;
    const DIODE_VT: f64 = 0.025_852_0;
    /// Forward-clamp on exp argument (same as DIODE_MAX_EXP_ARG in stamp.rs).
    const EXP_ARG_MAX: f64 = 40.0;

    /// Shockley diode current I_D(vd).
    #[inline]
    fn shockley_i(vd: f64) -> f64 {
        DIODE_IS * (clamp(vd / DIODE_VT, EXP_ARG_MAX).exp() - 1.0)
    }

    /// Shockley tangent conductance gd = dI_D/dVd.
    #[inline]
    fn shockley_gd(vd: f64) -> f64 {
        DIODE_IS / DIODE_VT * clamp(vd / DIODE_VT, EXP_ARG_MAX).exp()
    }

    // -----------------------------------------------------------------------
    // Fixture 1 — Resistor divider
    //
    // Circuit: V1 = 10 V (n1 → gnd), R1 = 1 kΩ (n1 → n2), R2 = 1 kΩ (n2 → gnd).
    //
    // Ground-suppressed MNA variable layout (dim = 3):
    //   index 0 — v_n1   (node)
    //   index 1 — v_n2   (node)
    //   index 2 — i_V    (branch, voltage-source current)
    //
    // node_count = 2, branch_count = 1  →  gmin is added only to rows 0–1
    // (row 0 is "ground" as seen by DcAnalysis::new() which sets
    //  ground_node_index = 0).  Only row 1 (v_n2) receives the shunt.
    //
    // Linear KCL / KVL system (same every NR iteration — converges in 1 step):
    //
    //   [ 2G   -G    1 ] [ v_n1 ]   [  0  ]
    //   [ -G   2G    0 ] [ v_n2 ] = [  0  ]
    //   [  1    0    0 ] [ i_V  ]   [ 10  ]
    //
    // where G = 1/1000 S.  Analytic solution: v_n1 = 10 V, v_n2 = 5 V.
    // -----------------------------------------------------------------------

    struct ResistorDividerSystem {
        sparse: SparseLinearSystem<f64>,
    }

    impl ResistorDividerSystem {
        fn new() -> Self {
            let g = 1e-3_f64;
            let v = 10.0_f64;
            let triplets = vec![
                SparseTriplet { row: 0, col: 0, value: 2.0 * g },
                SparseTriplet { row: 0, col: 1, value: -g },
                SparseTriplet { row: 0, col: 2, value: 1.0 },
                SparseTriplet { row: 1, col: 0, value: -g },
                SparseTriplet { row: 1, col: 1, value: 2.0 * g },
                SparseTriplet { row: 2, col: 0, value: 1.0 },
            ];
            let rhs = vec![0.0, 0.0, v];
            // node_count = 2 (n1, n2), branch_count = 1 (i_V).
            let sparse = SparseLinearSystem::new(3, 2, 1, triplets, rhs)
                .expect("valid resistor-divider system");
            Self { sparse }
        }
    }

    impl NonlinearSystem for ResistorDividerSystem {
        fn dim(&self) -> u32 {
            3
        }

        fn linearize(
            &mut self,
            _iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            Ok(self.sparse.clone())
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            let mut f = vec![0.0_f64; 3];
            for t in self.sparse.triplets() {
                f[t.row as usize] += t.value * iterate[t.col as usize];
            }
            for (i, &rhs_i) in self.sparse.rhs().iter().enumerate() {
                f[i] -= rhs_i;
            }
            Ok(f)
        }
    }

    impl SourceSteppableSystem for ResistorDividerSystem {
        // The linear divider never needs source stepping — dead path.
        fn set_source_alpha(&mut self, _alpha: f64) {}
    }

    // -----------------------------------------------------------------------
    // Fixture 2 — Diode + 1 kΩ resistor (V = 1 V)
    //
    // Circuit: V1 (1 V, n1 → gnd), R1 (1 kΩ, n1 → n2), D1 (n2 anode → gnd).
    //
    // Ground-suppressed variable layout (dim = 3):
    //   index 0 — v_n1   (node; forced to V_src by KVL)
    //   index 1 — v_n2   (node; diode anode)
    //   index 2 — i_V    (branch, voltage-source current)
    //
    // node_count = 2, branch_count = 1.
    // DcAnalysis defaults: ground_node_index = 0 → gmin added only to row 1.
    //
    // Nonlinear KCL:
    //   Row 0  (n1):  G*(v_n1 − v_n2) + i_V = 0
    //   Row 1  (n2):  −G*(v_n1 − v_n2) + I_D(v_n2) = 0
    //                 linearized: −G*v_n1 + (G+gd)*v_n2 = I_eq  where I_eq = I_D − gd*v_n2
    //   Row 2 (KVL):  v_n1 = V_src
    //
    // Hand-computed Shockley solution: v_n2 ≈ 0.6291468589 V.
    // -----------------------------------------------------------------------

    struct DiodeResistorSystem {
        /// Conductance G = 1/R.
        g: f64,
        /// Voltage source magnitude (ramped by set_source_alpha).
        v_src: f64,
        /// Original (full) voltage source magnitude.
        v_src_full: f64,
    }

    impl DiodeResistorSystem {
        fn new() -> Self {
            Self { g: 1e-3, v_src: 1.0, v_src_full: 1.0 }
        }
    }

    impl NonlinearSystem for DiodeResistorSystem {
        fn dim(&self) -> u32 {
            3
        }

        fn linearize(
            &mut self,
            iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            let v_n2 = iterate[1];
            let gd = shockley_gd(v_n2);
            let id = shockley_i(v_n2);

            let triplets = vec![
                // Row 0 = KCL at n1: G*v_n1 − G*v_n2 + i_V = 0
                SparseTriplet { row: 0, col: 0, value: self.g },
                SparseTriplet { row: 0, col: 1, value: -self.g },
                SparseTriplet { row: 0, col: 2, value: 1.0 },
                // Row 1 = KCL at n2 (nonlinear diode):
                //   −G*v_n1 + (G+gd)*v_n2 = gd*v_n2^0 − I_D(v_n2^0)
                // (derived from Newton linearisation: A*v_new = A*v^0 − F(v^0))
                SparseTriplet { row: 1, col: 0, value: -self.g },
                SparseTriplet { row: 1, col: 1, value: self.g + gd },
                // Row 2 = KVL for V1: v_n1 = V_src
                SparseTriplet { row: 2, col: 0, value: 1.0 },
            ];
            // RHS[1] = gd*v_n2^0 − I_D = −i_eq  (Newton-Raphson companion form).
            let rhs = vec![0.0, gd * v_n2 - id, self.v_src];
            // node_count = 2 (n1, n2), branch_count = 1 (i_V).
            SparseLinearSystem::new(3, 2, 1, triplets, rhs)
                .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            let sys = self.linearize(iterate)?;
            let mut f = vec![0.0_f64; 3];
            for t in sys.triplets() {
                f[t.row as usize] += t.value * iterate[t.col as usize];
            }
            for (i, &rhs_i) in sys.rhs().iter().enumerate() {
                f[i] -= rhs_i;
            }
            Ok(f)
        }
    }

    impl SourceSteppableSystem for DiodeResistorSystem {
        fn set_source_alpha(&mut self, alpha: f64) {
            self.v_src = alpha * self.v_src_full;
        }
    }

    // -----------------------------------------------------------------------
    // Fixture 3 — Current-fed diode (bare Shockley, no series resistor)
    //
    // Circuit: I_src = 1 mA (into n1), D1 (n1 anode → gnd cathode).
    //
    // Variable layout (dim = 2, using the 2-row MNA convention):
    //   index 0 — v_gnd  (ground node; trivial row `v_gnd = 0`)
    //   index 1 — v_n1   (diode anode)
    //
    // node_count = 2, branch_count = 0.
    // DcAnalysis defaults: ground_node_index = 0  →  gmin added only to row 1.
    //
    // Row 0 (gnd):  1 * v_gnd = 0                 (anchor)
    // Row 1 (n1):   gd * v_n1 = i_src − I_eq      (KCL, nonlinear diode)
    //   where I_eq = I_D(v_n1^prev) − gd * v_n1^prev
    //
    // At the zero initial guess (v_n1 = 0):
    //   gd  = 0  (diode below forward-bias threshold — zero-conductance stamp)
    //   Jacobian row 1 is all-zero → SINGULAR → UMFPACK reports SingularMatrix
    //   → NewtonRaphsonDriver returns Diverged(iterate=[0,0]) without moving
    //   → DcAnalysis falls through to Gmin stepping.
    //
    // With Gmin = 1e-3 S (first step of the US-021 Gmin schedule):
    //   Jacobian[1,1] = 0 + gmin = 1e-3 S
    //   NR step = 1e-3 / 1e-3 = 1 V  (controlled; converges quickly).
    // -----------------------------------------------------------------------

    /// Shockley diode conductance, truncated to 0.0 below a small forward-
    /// bias threshold.  This models the physical situation where a reverse-
    /// or zero-biased diode contributes no incremental conductance to the
    /// MNA Jacobian — making the Jacobian singular until Gmin inserts a
    /// diagonal shunt.
    #[inline]
    fn shockley_gd_truncated(vd: f64) -> f64 {
        // Below the forward-bias onset, treat the diode as open (gd = 0).
        // This is the MNA equivalent of a zero stamp and causes NR to return
        // Diverged on a singular matrix, triggering the Gmin fallback.
        const FORWARD_ONSET: f64 = 1e-3; // 1 mV
        if vd < FORWARD_ONSET { 0.0 } else { shockley_gd(vd) }
    }

    struct CurrentFedDiodeSystem {
        /// Full-amplitude source current.
        i_src_full: f64,
        /// Current amplitude (ramped by set_source_alpha).
        i_src: f64,
    }

    impl CurrentFedDiodeSystem {
        fn new(i_src: f64) -> Self {
            Self { i_src_full: i_src, i_src }
        }
    }

    impl NonlinearSystem for CurrentFedDiodeSystem {
        fn dim(&self) -> u32 {
            // 2-row system: row 0 = ground anchor, row 1 = diode anode.
            2
        }

        fn linearize(
            &mut self,
            iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            let v = iterate[1]; // diode anode voltage
            // Use the truncated conductance: at v < 1 mV the diode is treated
            // as open (gd = 0), producing a singular Jacobian that triggers the
            // Gmin fallback in DcAnalysis.
            let gd = shockley_gd_truncated(v);
            let id = shockley_i(v);
            // Companion: I_D ≈ gd*v_n1 + I_eq;  I_eq = I_D(v0) − gd*v0
            // KCL row 1:  gd*v_n1 = i_src − I_eq  →  RHS[1] = i_src − I_eq
            let i_eq = id - gd * v;
            let triplets = vec![
                // Row 0 = ground anchor:  1 * v_gnd = 0
                SparseTriplet { row: 0, col: 0, value: 1.0 },
                // Row 1 = KCL at n1:  gd * v_n1 = i_src − I_eq
                SparseTriplet { row: 1, col: 1, value: gd },
            ];
            let rhs = vec![0.0, self.i_src - i_eq];
            // node_count = 2 (gnd, n1), branch_count = 0.
            SparseLinearSystem::new(2, 2, 0, triplets, rhs)
                .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            let v = iterate[1];
            // F(x) = [v_gnd − 0, I_D(v_n1) − i_src]
            Ok(vec![iterate[0], shockley_i(v) - self.i_src])
        }
    }

    impl SourceSteppableSystem for CurrentFedDiodeSystem {
        fn set_source_alpha(&mut self, alpha: f64) {
            self.i_src = alpha * self.i_src_full;
        }
    }

    // -----------------------------------------------------------------------
    // Fixture 4 — Always-failing system for ConvergenceError field check
    //
    // Residue F(x) = [x[0]+1, x[1]+1] never reaches zero.
    // Used to provoke a terminal ConvergenceError and inspect its
    // diagnostic fields (residue_norm and iterations).
    // -----------------------------------------------------------------------

    struct AlwaysFailingSystem;

    impl NonlinearSystem for AlwaysFailingSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(
            &mut self,
            iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            // Identity Jacobian; step = -F(x) moves toward the false root
            // but can never satisfy the sub-1e-300 residue tolerance.
            SparseLinearSystem::new(
                2,
                2,
                0,
                vec![
                    SparseTriplet { row: 0, col: 0, value: 1.0 },
                    SparseTriplet { row: 1, col: 1, value: 1.0 },
                ],
                // RHS set to `iterate` so A*x = b gives Δx=0 — NR stalls.
                vec![iterate[0], iterate[1]],
            )
            .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![iterate[0] + 1.0, iterate[1] + 1.0])
        }
    }

    impl SourceSteppableSystem for AlwaysFailingSystem {
        fn set_source_alpha(&mut self, _alpha: f64) {}
    }

    // -----------------------------------------------------------------------
    // Test 1 — Resistor divider: ≤ 3 NR iterations, v_out = 5 V ± 1e-6
    // -----------------------------------------------------------------------

    #[test]
    fn resistor_divider_converges_within_3_nr_iterations() {
        let mut sys = ResistorDividerSystem::new();
        let driver = DcAnalysis::new();
        let result = driver
            .run(&mut sys, &RussellRealSolver, &[0.0; 3])
            .expect("no hard error expected");

        let sol = result.expect("resistor divider must converge on plain NR");

        // v_n2 is at index 1; analytic value = 5 V.
        let v_out = sol.solution[1];
        assert!(
            (v_out - 5.0_f64).abs() < 1e-6,
            "expected v_n2 = 5.0 V, got {v_out}"
        );

        // Linear circuit converges in 1 iteration; spec allows ≤ 3.
        let iters = sol.steps;
        assert!(
            iters <= 3,
            "expected ≤ 3 NR iterations for linear circuit, got {iters}"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2 — Diode + 1 kΩ: solution within 1 % of hand-computed answer
    //
    // Hand-computed Shockley solution (iterated NR on the scalar KCL):
    //   v_n2 ≈ 0.6291468589 V, I_D ≈ 3.709e-4 A.
    // -----------------------------------------------------------------------

    #[test]
    fn diode_resistor_dc_solution_within_1pct_of_hand_computed() {
        let mut sys = DiodeResistorSystem::new();
        let driver = DcAnalysis::new();
        // Initial guess close to the expected diode forward voltage.
        let result = driver
            .run(&mut sys, &RussellRealSolver, &[1.0, 0.6, 0.0])
            .expect("no hard error expected");

        let sol = result.expect("diode+R circuit must converge");

        // v_n2 (index 1) is the diode anode voltage.
        let v_expected = 0.629_146_858_9_f64;
        let v_actual = sol.solution[1];
        let rel_err = (v_actual - v_expected).abs() / v_expected;
        assert!(
            rel_err < 0.01,
            "expected v_n2 within 1 % of {v_expected:.6}, got {v_actual:.6} \
             (rel_err = {rel_err:.4e})"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3 — Gmin stepping recovers current-fed diode from v = 0 V
    //
    // Plain NR from 0 V diverges (step ≈ 2.6 GV). DcAnalysis falls back
    // to Gmin stepping which recovers the solution. The final answer must
    // satisfy I_D(v_n1) ≈ I_src within 1 %.
    // -----------------------------------------------------------------------

    #[test]
    fn gmin_stepping_recovers_current_fed_diode_from_zero_initial_guess() {
        let i_src = 1e-3_f64; // 1 mA
        let mut sys = CurrentFedDiodeSystem::new(i_src);

        // Use the default DcAnalysis configuration (100 NR iterations).
        // NR returns Diverged on the first step (singular Jacobian at v=0),
        // so the iterate stays at [0.0, 0.0] and gmin stepping takes over.
        let driver = DcAnalysis::new();

        let result = driver
            .run(&mut sys, &RussellRealSolver, &[0.0; 2])
            .expect("no hard error from DcAnalysis");

        let sol = result.expect(
            "Gmin stepping must recover current-fed diode from v=0 initial guess",
        );

        // Verify the solution: I_D(v_n1) ≈ I_src.
        let v_n1 = sol.solution[1];
        let i_diode = shockley_i(v_n1);
        let rel_err = (i_diode - i_src).abs() / i_src;
        assert!(
            rel_err < 0.01,
            "expected I_D ≈ {i_src:.3e} A, got {i_diode:.3e} A at \
             v_n1 = {v_n1:.6} V (rel_err = {rel_err:.4e})"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4 — ConvergenceError carries residue_norm and iteration count
    //
    // Provoke a terminal ConvergenceError and assert that:
    //   - inner_status.diagnostic().residue_norm is finite and > 0.
    //   - inner_status.diagnostic().iterations > 0.
    // -----------------------------------------------------------------------

    #[test]
    fn convergence_error_carries_residue_norm_and_iteration_count() {
        let tight = NewtonRaphsonConfig {
            max_iterations: 2,
            tolerances: ConvergenceTolerances {
                update_tol: 1e-300,
                residue_tol: 1e-300,
            },
        };
        let mut sys = AlwaysFailingSystem;
        let driver = DcAnalysis::new().with_nr_config(tight);
        let result = driver
            .run(&mut sys, &RussellRealSolver, &[0.0; 2])
            .expect("no hard error expected");

        let err: ConvergenceError =
            result.expect_err("always-failing system must return ConvergenceError");

        let diag = err.inner_status.diagnostic();

        // residue_norm must be a finite positive number (not converged).
        assert!(
            diag.residue_norm.is_finite(),
            "residue_norm must be finite, got {}",
            diag.residue_norm
        );
        assert!(
            diag.residue_norm > 0.0,
            "residue_norm must be > 0 for a non-converged system, got {}",
            diag.residue_norm
        );

        // At least one NR iteration must have been performed.
        assert!(
            diag.iterations > 0,
            "iteration count must be > 0, got {}",
            diag.iterations
        );
    }
}
