//! `DcAnalysis` orchestration driver (US-023).
//!
//! Composes plain Newton-Raphson with the two homotopy fallback
//! strategies implemented in [`crate::homotopy_engine`]:
//!
//! 1. **Plain NR** — attempt [`NewtonRaphsonDriver::solve`] on the raw
//!    system. If it converges, return [`DcSolution`] immediately.
//! 2. **Gmin-stepping** — on [`ConvergenceError`] from NR, invoke
//!    [`HomotopyEngine::gmin_stepping`]. If it converges, return
//!    [`DcSolution`].
//! 3. **Source-stepping** — on a second [`ConvergenceError`] from
//!    Gmin-stepping, invoke [`HomotopyEngine::source_stepping`].
//!    Return [`DcSolution`] on success or the terminal
//!    [`ConvergenceError`] on failure.
//!
//! Hard pre-loop errors ([`HomotopyEngineError`], [`NewtonRaphsonError`])
//! are surfaced as [`DcAnalysisError`] on the `Err` path. Convergence
//! failures (NR did not converge at any step) are surfaced as
//! `Ok(Err(ConvergenceError))` per the same convention used by
//! [`HomotopyEngine`].

#![allow(clippy::module_name_repetitions)]

use crate::homotopy_engine::{ConvergenceError, DcSolution, HomotopyEngine, HomotopyEngineError};
use crate::linear_solver::LinearSolver;
use crate::newton_raphson::{
    NewtonRaphsonConfig, NewtonRaphsonDriver, NewtonRaphsonError, NonlinearSystem,
};

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Hard pre-loop errors from [`DcAnalysis::run`].
///
/// These are structural failures that prevented any of the three solve
/// strategies from running to their natural termination. Convergence
/// failures (NR non-convergence exhausting all fallback strategies) are
/// reported on the `Ok(Err(ConvergenceError))` path instead.
#[derive(Debug, Clone, PartialEq)]
pub enum DcAnalysisError {
    /// The plain-NR solve encountered a hard pre-loop failure (dim
    /// mismatch, system callback error, linear-solver hard failure).
    NewtonRaphson(NewtonRaphsonError),
    /// A homotopy engine call (Gmin or source stepping) encountered a
    /// hard pre-loop failure.
    Homotopy(HomotopyEngineError),
}

impl core::fmt::Display for DcAnalysisError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NewtonRaphson(inner) => {
                write!(f, "dc-analysis: newton-raphson hard failure: {inner}")
            }
            Self::Homotopy(inner) => {
                write!(f, "dc-analysis: homotopy hard failure: {inner}")
            }
        }
    }
}

impl std::error::Error for DcAnalysisError {}

impl From<NewtonRaphsonError> for DcAnalysisError {
    fn from(e: NewtonRaphsonError) -> Self {
        Self::NewtonRaphson(e)
    }
}

impl From<HomotopyEngineError> for DcAnalysisError {
    fn from(e: HomotopyEngineError) -> Self {
        Self::Homotopy(e)
    }
}

// ---------------------------------------------------------------------------
// DcAnalysis
// ---------------------------------------------------------------------------

/// DC operating-point analysis driver that automatically falls through
/// to homotopy when plain Newton-Raphson fails to converge.
///
/// # Strategy chain (US-023)
///
/// 1. **Plain NR** — [`NewtonRaphsonDriver::solve`] on the raw system.
/// 2. **Gmin-stepping** — [`HomotopyEngine::gmin_stepping`] on
///    [`ConvergenceError`] from step 1.
/// 3. **Source-stepping** — [`HomotopyEngine::source_stepping`] on
///    [`ConvergenceError`] from step 2.
///
/// The driver is stateless: construct once, call [`run`](Self::run)
/// repeatedly.
///
/// # Examples
///
/// ```ignore
/// let driver = DcAnalysis::new();
/// match driver.run(&mut system, &RussellRealSolver, vec![0.0; dim]) {
///     Ok(Ok(sol))  => println!("converged: {:?}", sol.solution),
///     Ok(Err(err)) => println!("failed: {err}"),
///     Err(hard)    => println!("hard error: {hard}"),
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DcAnalysis {
    /// Newton-Raphson tuning applied at every step of the chain.
    pub nr_config: NewtonRaphsonConfig,
    /// Ground-node row index forwarded to [`HomotopyEngine`].
    pub ground_node_index: u32,
}

impl DcAnalysis {
    /// Construct with SPICE-default NR configuration and ground at
    /// row 0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nr_config: NewtonRaphsonConfig::DC_DEFAULTS,
            ground_node_index: 0,
        }
    }

    /// Override the Newton-Raphson configuration.
    #[must_use]
    pub fn with_nr_config(mut self, config: NewtonRaphsonConfig) -> Self {
        self.nr_config = config;
        self
    }

    /// Override the ground-node row index passed to [`HomotopyEngine`].
    #[must_use]
    pub fn with_ground_node_index(mut self, index: u32) -> Self {
        self.ground_node_index = index;
        self
    }

    /// Run the NR → Gmin → Source fallback chain on `system`.
    ///
    /// # Parameters
    ///
    /// - `system` — mutable reference to the [`NonlinearSystem`] to
    ///   solve. Borrowed for the duration of the call.
    /// - `solver` — sparse-linear backend.
    /// - `initial_iterate` — starting point for plain NR. Length must
    ///   equal `system.dim()`. The zero vector is a safe default.
    ///
    /// # Returns
    ///
    /// - `Ok(Ok(DcSolution))` — converged (via NR, Gmin, or source
    ///   stepping).
    /// - `Ok(Err(ConvergenceError))` — all three strategies failed to
    ///   converge; the error carries diagnostics from the last
    ///   (source-stepping) attempt.
    /// - `Err(DcAnalysisError)` — a hard pre-loop failure prevented
    ///   at least one strategy from running.
    ///
    /// # Errors
    ///
    /// Returns [`DcAnalysisError`] on hard pre-loop failures from
    /// [`NewtonRaphsonDriver`] or [`HomotopyEngine`].
    pub fn run<S, L>(
        &self,
        system: &mut S,
        solver: &L,
        initial_iterate: &[f64],
    ) -> Result<Result<DcSolution, ConvergenceError>, DcAnalysisError>
    where
        S: NonlinearSystem + crate::source_stepping::SourceSteppableSystem,
        L: LinearSolver<f64>,
    {
        // -------------------------------------------------------------------
        // Step 1: plain Newton-Raphson
        // -------------------------------------------------------------------
        let nr_outcome = NewtonRaphsonDriver
            .solve(self.nr_config, system, solver, initial_iterate.to_owned())
            .map_err(DcAnalysisError::from)?;

        if nr_outcome.status.is_converged() {
            let diagnostic = *nr_outcome.status.diagnostic();
            return Ok(Ok(DcSolution {
                solution: nr_outcome.iterate,
                diagnostic,
                steps: diagnostic.iterations,
            }));
        }

        // NR did not converge — preserve the last iterate as the warm-start
        // for Gmin stepping.
        let nr_last_iterate = nr_outcome.iterate;

        // -------------------------------------------------------------------
        // Step 2: Gmin-stepping homotopy
        // -------------------------------------------------------------------
        let engine = HomotopyEngine::new()
            .with_nr_config(self.nr_config)
            .with_ground_node_index(self.ground_node_index);

        match engine.gmin_stepping(system, solver, nr_last_iterate.clone())? {
            Ok(sol) => return Ok(Ok(sol)),
            Err(_gmin_err) => {
                // Gmin failed — fall through to source stepping.
            }
        }

        // -------------------------------------------------------------------
        // Step 3: source-stepping homotopy
        // -------------------------------------------------------------------
        let source_result = engine.source_stepping(system, solver, nr_last_iterate)?;
        Ok(source_result)
    }
}

impl Default for DcAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use crate::linear_solver::{RussellRealSolver, SparseLinearSystem, SparseTriplet};
    use crate::newton_raphson::{NonlinearSystem, SystemError};
    use crate::source_stepping::SourceSteppableSystem;
    use circuit_solver_types::ConvergenceTolerances;

    // ------------------------------------------------------------------
    // Test fixtures
    // ------------------------------------------------------------------

    /// Well-conditioned 2-node resistive system — converges on plain NR.
    struct ResistiveSystem {
        g_load: f64,
        i_in: f64,
    }

    impl NonlinearSystem for ResistiveSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(
            &mut self,
            _iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            SparseLinearSystem::new(
                2,
                2,
                0,
                vec![
                    SparseTriplet {
                        row: 0,
                        col: 0,
                        value: 1.0,
                    },
                    SparseTriplet {
                        row: 1,
                        col: 1,
                        value: self.g_load,
                    },
                ],
                vec![0.0, self.i_in],
            )
            .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![iterate[0], self.g_load * iterate[1] - self.i_in])
        }
    }

    impl SourceSteppableSystem for ResistiveSystem {
        fn set_source_alpha(&mut self, _alpha: f64) {}
    }

    /// Floating-node system — singular without Gmin.
    struct FloatingNodeSystem;

    impl NonlinearSystem for FloatingNodeSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(
            &mut self,
            _iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            SparseLinearSystem::new(
                2,
                2,
                0,
                vec![SparseTriplet {
                    row: 0,
                    col: 0,
                    value: 1.0,
                }],
                vec![0.0, 0.0],
            )
            .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![iterate[0], 0.0])
        }
    }

    impl SourceSteppableSystem for FloatingNodeSystem {
        fn set_source_alpha(&mut self, _alpha: f64) {}
    }

    /// System that never converges regardless of strategy — for testing
    /// the terminal ConvergenceError path.
    struct AlwaysDivergingSystem;

    impl NonlinearSystem for AlwaysDivergingSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(
            &mut self,
            iterate: &[f64],
        ) -> Result<SparseLinearSystem<f64>, SystemError> {
            // Return a valid system that produces iterates that don't converge.
            // The off-diagonal coupling means the tight tolerance is never met.
            SparseLinearSystem::new(
                2,
                2,
                0,
                vec![
                    SparseTriplet {
                        row: 0,
                        col: 0,
                        value: 1.0,
                    },
                    SparseTriplet {
                        row: 1,
                        col: 1,
                        value: 1.0,
                    },
                ],
                vec![iterate[0], iterate[1]], // RHS = iterate → Δx = 0 after one step
            )
            .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            // Residue never reaches zero with tight tolerance.
            Ok(vec![iterate[0] + 1.0, iterate[1] + 1.0])
        }
    }

    impl SourceSteppableSystem for AlwaysDivergingSystem {
        fn set_source_alpha(&mut self, _alpha: f64) {}
    }

    // ------------------------------------------------------------------
    // Tests
    // ------------------------------------------------------------------

    #[test]
    fn run_converges_on_plain_nr_for_well_conditioned_system() {
        let mut sys = ResistiveSystem {
            g_load: 1e-3, // 1 kΩ
            i_in: 1e-3,   // 1 mA → v1 = 1 V
        };
        let driver = DcAnalysis::new();
        let result = driver
            .run(&mut sys, &RussellRealSolver, &[0.0; 2])
            .expect("no hard error expected");

        let sol = result.expect("well-conditioned system must converge on plain NR");
        assert!(
            (sol.solution[1] - 1.0).abs() < 1e-6,
            "expected v1 ≈ 1.0 V, got {}",
            sol.solution[1]
        );
    }

    #[test]
    fn run_falls_back_to_gmin_for_floating_node() {
        // Floating node: plain NR singular (matrix has a zero row) but
        // Gmin-stepping recovers by adding diagonal shunts.
        let mut sys = FloatingNodeSystem;
        let driver = DcAnalysis::new();
        let result = driver
            .run(&mut sys, &RussellRealSolver, &[0.0; 2])
            .expect("no hard error expected");

        let sol = result.expect("floating-node system must converge via gmin fallback");
        assert_eq!(sol.solution.len(), 2);
        for &v in &sol.solution {
            assert!(
                v.abs() < 1e-9,
                "expected zero solution for floating node, got {v}"
            );
        }
    }

    #[test]
    fn run_returns_convergence_error_when_all_strategies_fail() {
        // AlwaysDivergingSystem produces a residue of `[v0+1, v1+1]`
        // which never reaches zero, so NR, Gmin, and source stepping
        // all fail regardless of tolerance.
        let tight_nr = NewtonRaphsonConfig {
            max_iterations: 1,
            tolerances: ConvergenceTolerances {
                update_tol: 1e-300,
                residue_tol: 1e-300,
            },
        };
        let mut sys = AlwaysDivergingSystem;
        let driver = DcAnalysis::new().with_nr_config(tight_nr);
        let result = driver
            .run(&mut sys, &RussellRealSolver, &[0.0; 2])
            .expect("no hard error expected");

        assert!(
            result.is_err(),
            "all strategies must fail with unreachable tolerance"
        );
    }

    #[test]
    fn run_default_matches_new() {
        let a = DcAnalysis::new();
        let b = DcAnalysis::default();
        assert_eq!(a.ground_node_index, b.ground_node_index);
        assert_eq!(a.nr_config.max_iterations, b.nr_config.max_iterations);
    }

    #[test]
    fn run_builder_overrides_are_applied() {
        use circuit_solver_types::ConvergenceTolerances;
        let config = NewtonRaphsonConfig {
            max_iterations: 42,
            tolerances: ConvergenceTolerances {
                update_tol: 1e-6,
                residue_tol: 1e-9,
            },
        };
        let driver = DcAnalysis::new()
            .with_nr_config(config)
            .with_ground_node_index(3);
        assert_eq!(driver.nr_config.max_iterations, 42);
        assert_eq!(driver.ground_node_index, 3);
    }
}
