//! High-level `HomotopyEngine` wrapping [`GminSteppingDriver`] with a
//! fixed 10-step DC-convergence schedule (US-021).
//!
//! The engine is a thin facade that:
//!
//! 1. Configures a 10-step log-spaced [`GminSchedule`] from
//!    1e-3 S down to 1e-12 S (one decade per step) plus the terminal
//!    step at exactly `1e-12 S`.
//! 2. Delegates to [`GminSteppingDriver::solve`], warm-starting each
//!    step from the previous step's solution.
//! 3. Maps the typed [`GminSteppingOutcome`] into [`DcSolution`] (on
//!    success) or [`ConvergenceError`] (on step failure or hard error).
//!
//! # Schedule
//!
//! The spec (US-021) requires "10 log-spaced steps from 1e-3 S to
//! 1e-12 S". With `initial_gmin = 1e-3`, `final_gmin = 1e-12`, and
//! `ratio = 10.0` the geometric walk emits exactly:
//!
//! ```text
//! 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12
//! ```
//!
//! 10 steps total (the terminal step `1e-12` is the `final_gmin`, so no
//! extra zero step is appended). `max_steps = 16` provides a safety
//! guard that is comfortably above the expected 10.
//!
//! # Return types
//!
//! [`DcSolution`] is a newtype over the converged solution vector, with
//! the final-step [`ConvergenceDiagnostic`] and the homotopy step count
//! attached for diagnostics.
//!
//! [`ConvergenceError`] carries the step index at which NR failed, the
//! Gmin value at that step, the inner NR convergence status, and the
//! last available iterate for caller diagnostics.

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::{ConvergenceDiagnostic, ConvergenceStatus};

use crate::gmin_stepping::{
    GminSchedule, GminScheduleError, GminSteppingConfig, GminSteppingDriver, GminSteppingError,
    HomotopyStatus,
};
use crate::linear_solver::LinearSolver;
use crate::newton_raphson::{NewtonRaphsonConfig, NonlinearSystem};
use crate::source_stepping::{
    SourceSteppableSystem, SourceSteppingConfig, SourceSteppingDriver, SourceSteppingError,
};

// ---------------------------------------------------------------------------
// Public return types
// ---------------------------------------------------------------------------

/// Converged DC operating-point solution produced by
/// [`HomotopyEngine::gmin_stepping`].
///
/// The `solution` vector has the same length as the system's `dim()`.
/// `node_voltages` for ground-suppressed systems occupy the first
/// `node_count` entries; branch currents follow.
#[derive(Debug, Clone, PartialEq)]
pub struct DcSolution {
    /// The converged solution vector (volts / amperes per the MNA
    /// convention).
    pub solution: Vec<f64>,
    /// Convergence diagnostic at the final (lowest Gmin) step.
    pub diagnostic: ConvergenceDiagnostic,
    /// Number of homotopy steps executed to reach convergence.
    pub steps: u32,
}

/// Convergence failure returned by [`HomotopyEngine::gmin_stepping`].
///
/// A [`ConvergenceError`] is produced when NR fails to converge at any
/// step in the homotopy schedule (NR status was not `Converged`). It is
/// *not* produced for hard pre-loop errors (schedule invariant violations,
/// dim mismatches, ground index out of range) — those surface as
/// [`HomotopyEngineError`] on the `Err` path instead.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvergenceError {
    /// Zero-based index of the homotopy step at which NR failed.
    pub step_index: u32,
    /// Gmin value (siemens) at the failing step.
    pub gmin_siemens: f64,
    /// The inner NR convergence status at the failing step.
    /// Always a non-`Converged` variant.
    pub inner_status: ConvergenceStatus,
    /// The last iterate produced at the failing step, for diagnostic
    /// reporting. Length equals the system's `dim()`.
    pub last_iterate: Vec<f64>,
}

impl core::fmt::Display for ConvergenceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "gmin-stepping homotopy did not converge at step {} (gmin = {:.3e} S): {:?}",
            self.step_index, self.gmin_siemens, self.inner_status,
        )
    }
}

impl std::error::Error for ConvergenceError {}

/// Hard pre-loop errors from [`HomotopyEngine::gmin_stepping`].
///
/// These indicate misuse (bad schedule constants, dim mismatch, ground
/// index out of range, or a linear-solver hard failure). Non-convergence
/// outcomes land on the `Ok` path as [`ConvergenceError`] instead.
#[derive(Debug, Clone, PartialEq)]
pub enum HomotopyEngineError {
    /// The internal [`GminSchedule`] invariant failed. Should not
    /// happen with the built-in constants; exposed for completeness.
    Schedule(GminScheduleError),
    /// The initial iterate's length did not match the system's `dim()`.
    DimMismatch {
        /// Supplied iterate length.
        iterate_len: usize,
        /// System's reported dimension.
        system_dim: u32,
    },
    /// The configured `ground_node_index` exceeded the system's node
    /// count.
    GroundIndexOutOfRange {
        /// Configured ground index.
        ground_node_index: u32,
        /// Inner system's node count.
        node_count: u32,
    },
    /// A hard linear-solver or modeling error from the inner NR driver.
    Newton {
        /// Homotopy step index at which the error occurred.
        step_index: u32,
        /// Gmin value at the failing step.
        gmin_siemens: f64,
        /// Error string from the inner NR hard failure.
        message: String,
    },
}

impl core::fmt::Display for HomotopyEngineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Schedule(e) => write!(f, "homotopy engine: {e}"),
            Self::DimMismatch {
                iterate_len,
                system_dim,
            } => write!(
                f,
                "homotopy engine: initial iterate len {iterate_len} != system dim {system_dim}",
            ),
            Self::GroundIndexOutOfRange {
                ground_node_index,
                node_count,
            } => write!(
                f,
                "homotopy engine: ground_node_index {ground_node_index} >= node_count {node_count}",
            ),
            Self::Newton {
                step_index,
                gmin_siemens,
                message,
            } => write!(
                f,
                "homotopy engine: step {step_index} (gmin = {gmin_siemens:.3e} S) hard error: {message}",
            ),
        }
    }
}

impl std::error::Error for HomotopyEngineError {}

impl From<GminSteppingError> for HomotopyEngineError {
    fn from(e: GminSteppingError) -> Self {
        match e {
            GminSteppingError::Schedule(s) => Self::Schedule(s),
            GminSteppingError::InitialIterateDimMismatch {
                iterate_len,
                system_dim,
            } => Self::DimMismatch {
                iterate_len,
                system_dim,
            },
            GminSteppingError::GroundIndexOutOfRange {
                ground_node_index,
                node_count,
            } => Self::GroundIndexOutOfRange {
                ground_node_index,
                node_count,
            },
            GminSteppingError::Newton {
                step_index,
                gmin_siemens,
                source,
            } => Self::Newton {
                step_index,
                gmin_siemens,
                message: source.to_string(),
            },
        }
    }
}

impl From<SourceSteppingError> for HomotopyEngineError {
    fn from(e: SourceSteppingError) -> Self {
        match e {
            SourceSteppingError::InvalidSchedule { reason } => {
                // The engine controls its own schedule; this should not
                // happen in practice. Map through Newton with step 0.
                Self::Newton {
                    step_index: 0,
                    gmin_siemens: 0.0,
                    message: format!("source-stepping invalid schedule: {reason}"),
                }
            }
            SourceSteppingError::InitialIterateDimMismatch {
                iterate_len,
                system_dim,
            } => Self::DimMismatch {
                iterate_len,
                system_dim,
            },
            SourceSteppingError::Inner { alpha, source } => Self::Newton {
                step_index: 0,
                gmin_siemens: alpha,
                message: source.to_string(),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// HomotopyEngine
// ---------------------------------------------------------------------------

/// DC convergence engine that applies Gmin-stepping homotopy as a
/// fallback when plain Newton-Raphson fails.
///
/// The engine is stateless: each [`gmin_stepping`](Self::gmin_stepping)
/// call is independent. Construct with [`HomotopyEngine::new`] (or the
/// `Default` impl) and configure with the builder methods.
///
/// # Default schedule (US-021)
///
/// The built-in schedule ramps Gmin from **1e-3 S** down to **1e-12 S**
/// in **10 log-spaced steps** (÷10 per step). This is a tighter starting
/// point than the SPICE-default 1 S (which risks large steady-state
/// distortion in sensitive nodes), while still providing sufficient
/// diagonal dominance to make the first step trivially convergent for
/// most topologies.
#[derive(Debug, Clone, Copy)]
pub struct HomotopyEngine {
    /// Newton-Raphson configuration applied at every homotopy step.
    pub nr_config: NewtonRaphsonConfig,
    /// Ground-node row index in the inner system. Defaults to `0`
    /// per the v1 convention that the flattener always pins ground at
    /// node 0.
    pub ground_node_index: u32,
}

/// The built-in US-021 schedule: 10 log-spaced steps 1e-3 → 1e-12.
///
/// `max_steps = 10` caps the total at exactly 10: the geometric walk
/// fills 9 slots (1e-3 … 1e-11, noting that f64 arithmetic can make
/// the 9th division land slightly above 1e-12 rather than exactly on
/// it), then the terminal step is appended at exactly `1e-12`,
/// yielding exactly 10 steps.
const US021_SCHEDULE: GminSchedule = GminSchedule {
    initial_gmin: 1e-3,
    final_gmin: 1e-12,
    ratio: 10.0,
    max_steps: 10,
};

impl HomotopyEngine {
    /// Construct with the SPICE-default NR configuration and ground at
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

    /// Override the ground-node row index.
    #[must_use]
    pub fn with_ground_node_index(mut self, index: u32) -> Self {
        self.ground_node_index = index;
        self
    }

    /// Run the 10-step Gmin-stepping homotopy on `system`.
    ///
    /// Ramps `g_min` from 1e-3 S down to 1e-12 S in 10 log-spaced
    /// steps. Each step warm-starts NR from the previous step's
    /// solution. Returns [`Ok(DcSolution)`](DcSolution) when the final
    /// step converges, or [`Ok(Err(ConvergenceError))`](ConvergenceError)
    /// when any step fails to converge.
    ///
    /// # Parameters
    ///
    /// - `system` — mutable reference to the [`NonlinearSystem`] to
    ///   solve. Borrowed for the duration of the call; released on return.
    /// - `solver` — sparse-linear backend (e.g. [`RussellRealSolver`]).
    /// - `initial_iterate` — starting point for the first NR step.
    ///   Length must equal `system.dim()`.
    ///
    /// # Returns
    ///
    /// - `Ok(Ok(DcSolution))` — all 10 steps converged; the final
    ///   solution and diagnostics are in `DcSolution`.
    /// - `Ok(Err(ConvergenceError))` — NR failed at some step; the
    ///   step index, Gmin value, inner NR status, and last iterate
    ///   are in `ConvergenceError`.
    /// - `Err(HomotopyEngineError)` — a hard pre-loop error (bad
    ///   initial iterate dimension, ground index out of range, inner
    ///   NR linear-solver hard failure).
    ///
    /// # Errors
    ///
    /// Returns [`HomotopyEngineError`] on pre-loop hard failures.
    pub fn gmin_stepping<S, L>(
        self,
        system: &mut S,
        solver: &L,
        initial_iterate: Vec<f64>,
    ) -> Result<Result<DcSolution, ConvergenceError>, HomotopyEngineError>
    where
        S: NonlinearSystem,
        L: LinearSolver<f64>,
    {
        let config = GminSteppingConfig {
            newton_raphson: self.nr_config,
            schedule: US021_SCHEDULE,
            ground_node_index: self.ground_node_index,
        };

        let outcome = GminSteppingDriver
            .solve(config, system, solver, initial_iterate)
            .map_err(HomotopyEngineError::from)?;

        match outcome.status {
            HomotopyStatus::ConvergedViaHomotopy {
                steps,
                final_diagnostic,
            } => Ok(Ok(DcSolution {
                solution: outcome.iterate,
                diagnostic: final_diagnostic,
                steps,
            })),
            HomotopyStatus::StepFailed {
                step_index,
                gmin_siemens,
                inner_status,
            } => Ok(Err(ConvergenceError {
                step_index,
                gmin_siemens,
                inner_status,
                last_iterate: outcome.iterate,
            })),
        }
    }
}

impl Default for HomotopyEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::float_cmp)]
#[allow(clippy::match_wildcard_for_single_variants)]
mod tests {
    use super::*;
    use crate::linear_solver::{RussellRealSolver, SparseLinearSystem, SparseTriplet};
    use crate::newton_raphson::{NonlinearSystem, SystemError};
    use circuit_solver_types::ConvergenceTolerances;

    // ------------------------------------------------------------------
    // Helpers
    // ------------------------------------------------------------------

    /// Floating-node system: singular without Gmin, trivially solvable
    /// with any Gmin > 0. Solution is the zero vector.
    struct FloatingNodeSystem;

    impl NonlinearSystem for FloatingNodeSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            // Ground row: 1·v0 = 0. Floating row: 0·v1 = 0.
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

    /// Well-conditioned 2-node resistive system.
    /// A[0,0]=1, A[1,1]=g_load; RHS=[0, i_in].
    struct ResistiveSystem {
        g_load: f64,
        i_in: f64,
    }

    impl NonlinearSystem for ResistiveSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
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

    // ------------------------------------------------------------------
    // Tests
    // ------------------------------------------------------------------

    #[test]
    fn gmin_stepping_converges_floating_node() {
        let mut sys = FloatingNodeSystem;
        let result = HomotopyEngine::new()
            .gmin_stepping(&mut sys, &RussellRealSolver, vec![0.0, 0.0])
            .expect("no hard error expected");

        let sol = result.expect("homotopy must converge on floating-node system");
        assert_eq!(sol.solution.len(), 2);
        for &v in &sol.solution {
            assert!(
                v.abs() < 1e-9,
                "expected zero solution for floating node, got {v}"
            );
        }
        assert!(sol.steps >= 1, "at least one step must have been taken");
        assert!(
            sol.diagnostic.dual_satisfied(),
            "final NR step must satisfy dual criterion: {:?}",
            sol.diagnostic
        );
    }

    #[test]
    fn gmin_stepping_exactly_10_steps() {
        // The US-021 schedule: 1e-3 → 1e-12 in ÷10 steps should
        // produce exactly 10 steps (1e-3, 1e-4, ..., 1e-12).
        let mut sys = FloatingNodeSystem;
        let result = HomotopyEngine::new()
            .gmin_stepping(&mut sys, &RussellRealSolver, vec![0.0, 0.0])
            .expect("no hard error expected");

        let sol = result.expect("homotopy must converge");
        assert_eq!(
            sol.steps, 10,
            "US-021 schedule must produce exactly 10 steps, got {}",
            sol.steps
        );
    }

    #[test]
    fn gmin_stepping_well_conditioned_circuit_converges() {
        // A resistive circuit that converges directly; homotopy should
        // succeed in all 10 steps.
        let mut sys = ResistiveSystem {
            g_load: 1e-3, // 1 kΩ
            i_in: 1e-3,   // 1 mA → v1 = 1 V
        };
        let result = HomotopyEngine::new()
            .gmin_stepping(&mut sys, &RussellRealSolver, vec![0.0, 0.0])
            .expect("no hard error expected");

        let sol = result.expect("well-conditioned system must converge via homotopy");
        // At gmin = 0 (final step = 1e-12), the solution is dominated
        // by the resistive load. With gmin = 1e-12 still present, the
        // solution is very close to v1 = i_in / g_load = 1 V.
        assert!(
            (sol.solution[1] - 1.0).abs() < 1e-6,
            "expected v1 ≈ 1.0 V, got {}",
            sol.solution[1]
        );
    }

    #[test]
    fn gmin_stepping_returns_convergence_error_on_unsolvable_system() {
        // A system that is always singular even with Gmin (both diagonal
        // entries are explicitly zero regardless of what the wrapper adds,
        // by always returning only off-diagonal stamps).
        //
        // We simulate this by using a system whose linearize always returns
        // a matrix that is zero in the first row even after Gmin shunting
        // (the ground-node row at index 0 is skipped by the Gmin wrapper;
        // if we put our singular row at index 0 with a 1 already there,
        // the wrapper will not touch it). Instead, we construct a system
        // where the NR diverges at gmin = initial because of an extreme
        // nonlinearity: a very tight iteration budget.
        //
        // Practical approach: tight NR budget (1 iteration) on a system
        // that needs many iterations. Use a stiff resistive system where
        // 1 NR iteration is not enough to converge.
        //
        // Actually the simplest approach: configure a 1-iteration budget
        // where the tolerance is impossible to reach in one step.
        let tight_nr = NewtonRaphsonConfig {
            max_iterations: 1,
            tolerances: ConvergenceTolerances {
                update_tol: 1e-30,  // unreachably tight
                residue_tol: 1e-30, // unreachably tight
            },
        };
        let mut sys = ResistiveSystem {
            g_load: 1.0,
            i_in: 1.0,
        };
        let result = HomotopyEngine::new()
            .with_nr_config(tight_nr)
            .gmin_stepping(&mut sys, &RussellRealSolver, vec![0.0, 0.0])
            .expect("no hard error (tight budget is a convergence failure, not a hard error)");

        // With 1 iteration and unreachable tolerance, the first step
        // should fail to converge, returning a ConvergenceError.
        let err = match result {
            Ok(_) => panic!("expected ConvergenceError with impossible tolerance"),
            Err(e) => e,
        };
        assert_eq!(err.step_index, 0, "failure should be at step 0");
        assert!(
            (err.gmin_siemens - 1e-3).abs() < 1e-15,
            "failure gmin should be 1e-3 S (initial), got {}",
            err.gmin_siemens
        );
        assert_eq!(
            err.last_iterate.len(),
            2,
            "last_iterate must have system dim"
        );
    }

    #[test]
    fn gmin_stepping_dim_mismatch_returns_hard_error() {
        let mut sys = FloatingNodeSystem;
        // Supply iterate of wrong length.
        let result = HomotopyEngine::new().gmin_stepping(
            &mut sys,
            &RussellRealSolver,
            vec![0.0, 0.0, 0.0], // dim=2 but 3 elements
        );
        match result {
            Err(HomotopyEngineError::DimMismatch {
                iterate_len: 3,
                system_dim: 2,
            }) => {}
            other => panic!("expected DimMismatch, got {other:?}"),
        }
    }

    #[test]
    fn homotopy_engine_default_matches_new() {
        let a = HomotopyEngine::new();
        let b = HomotopyEngine::default();
        // Both should have DC_DEFAULTS NR config and ground index 0.
        assert_eq!(a.ground_node_index, 0);
        assert_eq!(b.ground_node_index, 0);
        assert_eq!(
            a.nr_config.max_iterations,
            b.nr_config.max_iterations
        );
    }
}
