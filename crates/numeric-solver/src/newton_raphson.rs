//! Newton-Raphson outer-loop driver (tasks.md item #17, ADR-0006).
//!
//! This module owns the **dual-convergence-criterion** Newton-Raphson
//! driver that gates every DC operating-point and transient-timestep
//! solve. Per [ADR-0006](../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0006-dual-convergence-criterion-newton-raphson.md):
//!
//! > Convergence is declared when **both** ‖Δx‖ < ε_update **and**
//! > ‖F(x)‖ < ε_residue.
//!
//! A single-criterion check (update-only) is the classic source of
//! silently wrong SPICE results, where a stalled iterate satisfies
//! ‖Δx‖ < ε while ‖F(x)‖ remains large (KCL/KVL not actually
//! satisfied). This driver computes *both* norms at every iteration
//! and returns a [`ConvergenceStatus`] that distinguishes the two
//! asymmetric failure modes (`Stalled` vs `MaxIterationsExceeded`).
//!
//! # Where this fits
//!
//! The driver is intentionally **device-agnostic**. It does not know
//! about diodes, BJTs, or MOSFETs; it knows only how to ask a
//! [`NonlinearSystem`] for (a) the linearized step at the current
//! iterate and (b) the nonlinear residue at any iterate. The DC
//! analysis control loop (tasks.md #20) and the Gmin / source-
//! stepping homotopies (tasks.md #18 / #19) sit *above* this driver
//! and supply the [`NonlinearSystem`] implementor that bridges to
//! `device-modeling::LinearizedModel` and `numeric-solver::assemble`.
//!
//! # The two callbacks
//!
//! At iterate `x_k` Newton-Raphson asks two distinct questions of
//! the underlying nonlinear system:
//!
//! 1. **"Linearize me."** The system stamps a [`SparseLinearSystem`]
//!    whose solution `x_{k+1}` is the next iterate. In SPICE-style
//!    MNA this is the *companion-model* form: each nonlinear device
//!    contributes its tangent conductance `G(x_k)` to `A` and its
//!    equivalent current `I_eq(x_k) = I(x_k) - G(x_k)·x_k` to the
//!    RHS, so solving `A · x_{k+1} = b` yields the new iterate
//!    directly (not `Δx`).
//!
//! 2. **"What is your residue at iterate `x`?"** The system
//!    evaluates `F(x)` — the *unlinearized* KCL/KVL residual using
//!    each device's true nonlinear `I(v)` characteristic, not its
//!    tangent. This is the question that catches stall: if
//!    [`LinearizedModel`](device_modeling::stamp::LinearizedModel)'s
//!    tangent slopes were wrong, the linear solve will still produce
//!    a small `Δx` but `F(x_{k+1})` stays large.
//!
//! Keeping the two as separate callbacks lets the linear-step path
//! reuse the same stamping infrastructure as `assemble.rs` while the
//! residue path runs the device's nonlinear evaluator directly.
//!
//! # Norm choice
//!
//! Both norms are the **infinity norm** ‖·‖∞ (= `max |·|`). The
//! infinity norm is `n`-independent (its scale does not grow with
//! the system size, unlike the L2 norm) and aligns with the per-node
//! tolerance philosophy in ADR-0008 (each node's contribution is
//! compared individually rather than summed across the system). The
//! tolerances in [`ConvergenceTolerances`] (`reltol` / `abstol` in
//! SPICE-family vocabulary) are interpreted as ∞-norm bounds.
//!
//! # Failure modes
//!
//! The driver maps every termination into a [`ConvergenceStatus`]
//! variant:
//!
//! - [`ConvergenceStatus::Converged`] — both `‖Δx‖∞` and `‖F(x)‖∞`
//!   fell below their tolerances on the current iterate.
//! - [`ConvergenceStatus::Stalled`] — at termination the iteration
//!   budget was exhausted with `‖Δx‖∞ < update_tol` but
//!   `‖F(x)‖∞ ≥ residue_tol`. This is precisely the ADR-0006
//!   false-convergence mode that a single-criterion check would
//!   have falsely declared converged. The driver does *not* bail
//!   early on the first iteration that exhibits this pattern,
//!   because the residue may still be converging quadratically; we
//!   wait until the budget is exhausted before labeling it stall.
//! - [`ConvergenceStatus::MaxIterationsExceeded`] — the configured
//!   iteration budget was exhausted and neither the stall pattern
//!   nor convergence was reached. Genuine iteration-limit
//!   exhaustion.
//! - [`ConvergenceStatus::Diverged`] — a non-finite update or
//!   residue was produced, or the linear solver itself failed
//!   (singular matrix, non-finite stamp). The diagnostic carries
//!   the last *finite* measurement so the caller can still render
//!   a meaningful failure message.
//!
//! Backend errors from the injected [`LinearSolver<f64>`] are
//! collapsed into `Diverged`; the original [`LinearSolverError`] is
//! available on the [`NewtonRaphsonError::LinearSolver`] return
//! variant when the caller wants the *typed* failure (e.g., to
//! decide whether to trigger Gmin-stepping homotopy in tasks.md
//! #18). The two surfaces are complementary: `Result<…, _>`
//! distinguishes pre-loop / linear-solver hard failures from
//! convergence outcomes; `ConvergenceStatus` reports *which kind of
//! non-convergence* occurred for diagnostics-as-data.
//!
//! # Empty systems
//!
//! `dim == 0` is treated as vacuously converged in zero iterations
//! with zero norms. This matches [`RussellRealSolver`](super::linear_solver::RussellRealSolver)'s
//! short-circuit on empty systems and lets the analysis orchestrator
//! treat the empty-circuit edge case as a uniform success path.
//!
//! # Honored ADRs
//!
//! - **ADR-0006** — dual convergence criterion. This module's loop
//!   condition is the literal AND of the two norm checks; both
//!   norms are reported on every status.
//! - **ADR-0008** — per-node tolerance envelope. The ∞-norm choice
//!   keeps per-node interpretability; the driver itself does not
//!   apply the `max(reltol·|x|, abstol)` mixing rule because that
//!   is a tolerance-vector construction concern that lives one
//!   layer above (in the DC analysis control loop, tasks.md #20).
//! - **ADR-0010** — every type and function exported here is part
//!   of the v1 *unstable* public Rust API.

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::{ConvergenceDiagnostic, ConvergenceStatus, ConvergenceTolerances};

use crate::linear_solver::{LinearSolver, LinearSolverError, SparseLinearSystem};

/// A nonlinear system that the Newton-Raphson driver can iterate on.
///
/// Implementors expose the **two callbacks** the dual-criterion loop
/// needs at every iteration:
///
/// - [`linearize`](Self::linearize): build the companion-model
///   linear system `A · x_{k+1} = b` at the current iterate `x_k`.
///   On success the next iterate is `x_{k+1} = A⁻¹ b`. This is the
///   Pass-2 MNA stamping path that consumes
///   [`device_modeling::stamp::LinearizedModel`] for each nonlinear
///   device.
/// - [`residue`](Self::residue): evaluate the **unlinearized**
///   nonlinear residual `F(x)` at an arbitrary iterate `x`. This
///   must call each device's true nonlinear `I(v)` evaluator (not
///   its tangent slope) so the residue norm reflects genuine
///   KCL/KVL satisfaction.
///
/// The driver never inspects the contents of either return value; it
/// only computes ∞-norms and feeds the linear system to the
/// [`LinearSolver<f64>`] implementation. Implementors are free to
/// cache intermediate state (matrix scratch, device-model parameter
/// lookups) on `&mut self` between calls — the driver borrows the
/// implementor mutably for the duration of the solve.
pub trait NonlinearSystem {
    /// Dimensionality of the nonlinear system (== length of the
    /// initial iterate and of every returned RHS / residue vector).
    fn dim(&self) -> u32;

    /// Build the linearized step `A · x_{k+1} = b` at iterate
    /// `iterate`.
    ///
    /// The returned [`SparseLinearSystem`] is consumed by the
    /// injected linear solver; on success the next iterate equals
    /// the solver's solution vector. Implementors must produce a
    /// system whose `dim()` equals [`Self::dim`] and whose
    /// `node_count + branch_count` partition is stable across
    /// iterations (the driver does not re-validate the partition
    /// across calls — only the dim).
    ///
    /// # Errors
    ///
    /// Returning any error here terminates the loop with a
    /// [`NewtonRaphsonError::System`] result; the driver does not
    /// retry. Implementors should reserve this for *unrecoverable*
    /// modeling errors (e.g., a device whose parameter table is
    /// missing), not for convergence-related conditions. Convergence
    /// problems should surface as large residues, not as errors.
    fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError>;

    /// Evaluate the nonlinear residue `F(iterate)`.
    ///
    /// The returned slice must have length [`Self::dim`]. Each
    /// component is a KCL/KVL imbalance for the corresponding row
    /// of the sub-view layout. Sign convention follows assembler
    /// stamping: `F[i] = (currents stamped into row i by the
    /// unlinearized device evaluation) - (RHS contribution at
    /// row i)`. The driver only ever reads `‖F(iterate)‖∞`, so
    /// implementors may share buffers between calls (the slice is
    /// borrowed for the duration of one iteration).
    ///
    /// # Errors
    ///
    /// Same shape as [`Self::linearize`]: unrecoverable modeling
    /// errors only. The driver collapses [`SystemError`] into
    /// [`NewtonRaphsonError::System`] and stops.
    fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError>;
}

/// Error returned by a [`NonlinearSystem`] implementor when it
/// cannot produce a linearization or residue.
///
/// This is intentionally a thin string-carrying variant: the driver
/// never inspects the contents and propagates the value verbatim
/// into [`NewtonRaphsonError::System`]. Implementors decide the
/// granularity of their own error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemError {
    /// Human-readable description of the modeling failure.
    pub description: String,
}

impl SystemError {
    /// Construct a `SystemError` with a description.
    #[must_use]
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
        }
    }
}

impl core::fmt::Display for SystemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "nonlinear-system error: {}", self.description)
    }
}

impl std::error::Error for SystemError {}

/// Tuning knobs for the Newton-Raphson loop.
///
/// Iteration budget and tolerances are required; everything else is
/// `Default`-friendly. The defaults match the SPICE-conventional
/// `ITL1 = 100` (DC) and `reltol = 1e-3`, `abstol = 1e-12` constants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NewtonRaphsonConfig {
    /// Maximum number of Newton-Raphson iterations. The SPICE
    /// `ITL1` parameter, defaulting to 100, gates DC operating-
    /// point solves; transient timesteps use the smaller `ITL4`
    /// default of 10 supplied by the caller.
    pub max_iterations: u32,
    /// Update- and residue-norm tolerances. See
    /// [`ConvergenceTolerances::SPICE_DEFAULTS`].
    pub tolerances: ConvergenceTolerances,
}

impl NewtonRaphsonConfig {
    /// SPICE-conventional DC operating-point default:
    /// `ITL1 = 100`, `reltol = 1e-3`, `abstol = 1e-12`.
    pub const DC_DEFAULTS: Self = Self {
        max_iterations: 100,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    };

    /// SPICE-conventional transient-timestep default:
    /// `ITL4 = 10`, `reltol = 1e-3`, `abstol = 1e-12`.
    pub const TRANSIENT_DEFAULTS: Self = Self {
        max_iterations: 10,
        tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
    };
}

impl Default for NewtonRaphsonConfig {
    fn default() -> Self {
        Self::DC_DEFAULTS
    }
}

/// Stateless dispatcher implementing the dual-criterion Newton-
/// Raphson outer loop per ADR-0006.
///
/// The driver itself holds no state; it borrows a [`LinearSolver<f64>`]
/// for the inner linear solves and a [`NonlinearSystem`] for the
/// stamping / residue callbacks. The intended composition for DC
/// operating-point (tasks.md #20) is:
///
/// ```text
/// let mut system = DcNonlinearSystem::new(...);
/// let solver = RussellRealSolver;
/// let outcome = NewtonRaphsonDriver
///     .solve(NewtonRaphsonConfig::DC_DEFAULTS,
///            &mut system,
///            &solver,
///            initial_iterate);
/// ```
///
/// Each `solve` call performs at most `config.max_iterations`
/// iterations. A fresh linear factorization happens inside the
/// solver every iteration (see `RussellRealSolver`'s per-call
/// factorization note), because the linearized system changes when
/// the iterate moves.
#[derive(Debug, Clone, Copy, Default)]
pub struct NewtonRaphsonDriver;

/// Outcome of [`NewtonRaphsonDriver::solve`].
///
/// On success returns the final iterate and the
/// [`ConvergenceStatus`] (which itself may report any of the four
/// terminal variants; see module docs). On `Err` returns the
/// underlying typed failure that prevented the loop from running at
/// all (linear-solver crash, dim mismatch, modeling error).
#[derive(Debug, Clone, PartialEq)]
pub struct NewtonRaphsonOutcome {
    /// The final iterate. On `Converged` this is the accepted
    /// solution; on any failure variant this is the last *finite*
    /// iterate produced before the loop terminated. The caller's
    /// "last-iterate node voltages" payload for the DC convergence-
    /// failure scenario reads directly from this field.
    pub iterate: Vec<f64>,
    /// The convergence outcome. Always populated; carries the final
    /// diagnostic norms regardless of variant.
    pub status: ConvergenceStatus,
}

/// Errors returned by [`NewtonRaphsonDriver::solve`] when the loop
/// could not even run to its natural termination (success, stall,
/// divergence, or iteration-limit exhaustion).
///
/// These are *pre-* or *during-loop* hard failures. Convergence
/// outcomes (including divergence and stall) are reported as `Ok`
/// with the appropriate [`ConvergenceStatus`] variant.
#[derive(Debug, Clone, PartialEq)]
pub enum NewtonRaphsonError {
    /// The initial iterate's length did not match the system's
    /// [`NonlinearSystem::dim`].
    InitialIterateDimMismatch {
        /// Length of the supplied initial iterate.
        iterate_len: usize,
        /// Dim reported by the system.
        system_dim: u32,
    },
    /// A linearization step returned a [`SparseLinearSystem`] whose
    /// dim disagreed with the [`NonlinearSystem::dim`] declared on
    /// construction. The driver does not assume per-iteration dim
    /// changes are safe (sub-view layouts are pinned for the
    /// duration of a solve).
    LinearizedDimMismatch {
        /// Iteration at which the mismatch was detected (0-based).
        iteration: u32,
        /// Dim reported by the returned linear system.
        linear_dim: u32,
        /// Dim previously declared by the system.
        system_dim: u32,
    },
    /// A residue evaluation returned a `Vec<f64>` whose length
    /// disagreed with the [`NonlinearSystem::dim`].
    ResidueDimMismatch {
        /// Iteration at which the mismatch was detected (0-based).
        iteration: u32,
        /// Length of the returned residue vector.
        residue_len: usize,
        /// Dim previously declared by the system.
        system_dim: u32,
    },
    /// The injected [`LinearSolver<f64>`] returned an error during
    /// the inner solve. The driver does not retry — homotopy
    /// fallbacks are the caller's responsibility (tasks.md #18 /
    /// #19).
    LinearSolver {
        /// Iteration at which the failure occurred (0-based).
        iteration: u32,
        /// The underlying linear-solver error.
        source: LinearSolverError,
    },
    /// The user-supplied [`NonlinearSystem`] returned an error
    /// (modeling failure, parameter lookup, etc.).
    System {
        /// Iteration at which the system reported the error (0-based).
        iteration: u32,
        /// The underlying modeling error.
        source: SystemError,
    },
}

impl core::fmt::Display for NewtonRaphsonError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InitialIterateDimMismatch {
                iterate_len,
                system_dim,
            } => write!(
                f,
                "newton-raphson: initial iterate length {iterate_len} != system dim {system_dim}",
            ),
            Self::LinearizedDimMismatch {
                iteration,
                linear_dim,
                system_dim,
            } => write!(
                f,
                "newton-raphson: iteration {iteration} produced linearized dim {linear_dim} \
                 != system dim {system_dim}",
            ),
            Self::ResidueDimMismatch {
                iteration,
                residue_len,
                system_dim,
            } => write!(
                f,
                "newton-raphson: iteration {iteration} produced residue length {residue_len} \
                 != system dim {system_dim}",
            ),
            Self::LinearSolver { iteration, source } => write!(
                f,
                "newton-raphson: iteration {iteration} linear-solver error: {source}",
            ),
            Self::System { iteration, source } => write!(
                f,
                "newton-raphson: iteration {iteration} system error: {source}",
            ),
        }
    }
}

impl std::error::Error for NewtonRaphsonError {}

impl NewtonRaphsonDriver {
    /// Run the dual-criterion Newton-Raphson loop.
    ///
    /// Starts from `initial_iterate`, asks `system` for a linearized
    /// step at each iteration, solves it via `solver`, then asks
    /// `system` for the nonlinear residue at the new iterate and
    /// applies the dual convergence test.
    ///
    /// # Returns
    ///
    /// On the natural-termination path (converged, stalled, diverged,
    /// max-iter), returns `Ok(NewtonRaphsonOutcome)` with the final
    /// iterate and the [`ConvergenceStatus`]. On a *hard* failure
    /// that prevented the loop from running to its natural end
    /// (dim mismatch, linear-solver crash, modeling error),
    /// returns `Err(NewtonRaphsonError)`.
    ///
    /// # Errors
    ///
    /// See [`NewtonRaphsonError`] for the full list. The hard-
    /// failure surface is distinct from the *non-convergence*
    /// surface ([`ConvergenceStatus`]) on purpose: the analysis
    /// orchestrator (tasks.md #20) uses the hard-failure boundary
    /// to decide whether to trigger homotopy (tasks.md #18, #19);
    /// homotopy is appropriate on non-convergence but not on a
    /// modeling error.
    #[allow(clippy::too_many_lines)] // The dual-criterion loop is a single conceptual unit;
                                     // splitting it into helpers would obscure the
                                     // ADR-0006 contract more than it would clarify.
    pub fn solve<S, L>(
        self,
        config: NewtonRaphsonConfig,
        system: &mut S,
        solver: &L,
        initial_iterate: Vec<f64>,
    ) -> Result<NewtonRaphsonOutcome, NewtonRaphsonError>
    where
        S: NonlinearSystem,
        L: LinearSolver<f64>,
    {
        let system_dim = system.dim();

        // ─── Pre-loop validation ────────────────────────────────────
        if initial_iterate.len() != system_dim as usize {
            return Err(NewtonRaphsonError::InitialIterateDimMismatch {
                iterate_len: initial_iterate.len(),
                system_dim,
            });
        }

        // Empty system: vacuously converged in zero iterations. This
        // matches `RussellRealSolver`'s empty-system contract and
        // lets the analysis orchestrator treat the empty-circuit
        // case as a uniform success path.
        if system_dim == 0 {
            return Ok(NewtonRaphsonOutcome {
                iterate: initial_iterate,
                status: ConvergenceStatus::Converged(ConvergenceDiagnostic {
                    update_norm: 0.0,
                    residue_norm: 0.0,
                    iterations: 0,
                    tolerances: config.tolerances,
                }),
            });
        }

        let mut iterate = initial_iterate;
        // Track last *finite* norms separately from the most recent
        // norms so the `Diverged` variant can carry meaningful
        // diagnostics even when the latest iterate is NaN/Inf.
        let mut last_finite_update_norm = f64::INFINITY;
        let mut last_finite_residue_norm = f64::INFINITY;
        let mut iterations_completed: u32 = 0;

        // ─── Main loop ──────────────────────────────────────────────
        for k in 0..config.max_iterations {
            // (1) Linearize at current iterate.
            let linear_system =
                system
                    .linearize(&iterate)
                    .map_err(|source| NewtonRaphsonError::System {
                        iteration: k,
                        source,
                    })?;

            if linear_system.dim() != system_dim {
                return Err(NewtonRaphsonError::LinearizedDimMismatch {
                    iteration: k,
                    linear_dim: linear_system.dim(),
                    system_dim,
                });
            }

            // (2) Solve the linear step. On linear-solver failure we
            //     end with `Diverged` if the failure is consistent
            //     with the iterate going off the rails (singular
            //     matrix, non-finite stamp) — otherwise we surface
            //     the typed error. We treat *singular* + *non-finite*
            //     as `Diverged` (last-iterate diagnostics still
            //     available); other backend errors are hard.
            let solution = match solver.solve(&linear_system) {
                Ok(s) => s,
                Err(err) => match err {
                    LinearSolverError::SingularMatrix { .. }
                    | LinearSolverError::NonFiniteEntry { .. } => {
                        return Ok(NewtonRaphsonOutcome {
                            iterate,
                            status: ConvergenceStatus::Diverged(ConvergenceDiagnostic {
                                update_norm: last_finite_update_norm,
                                residue_norm: last_finite_residue_norm,
                                iterations: iterations_completed,
                                tolerances: config.tolerances,
                            }),
                        });
                    }
                    other => {
                        return Err(NewtonRaphsonError::LinearSolver {
                            iteration: k,
                            source: other,
                        });
                    }
                },
            };

            let next_iterate = solution.into_unknowns();
            // (Defensive: the linear solver guarantees `unknowns.len() == dim()`,
            // but we re-check rather than rely on internal invariants of
            // `SolutionVector::from_parts`.)
            debug_assert_eq!(next_iterate.len(), system_dim as usize);

            // (3) Compute update norm ‖x_{k+1} - x_k‖∞.
            let update_norm = infinity_norm_diff(&next_iterate, &iterate);

            // (4) Detect non-finite update — divergence.
            if !update_norm.is_finite() {
                // Try to capture last finite iterate's residue if we
                // can; otherwise keep the previous last-finite norms.
                iterations_completed = k + 1;
                return Ok(NewtonRaphsonOutcome {
                    iterate,
                    status: ConvergenceStatus::Diverged(ConvergenceDiagnostic {
                        update_norm: last_finite_update_norm,
                        residue_norm: last_finite_residue_norm,
                        iterations: iterations_completed,
                        tolerances: config.tolerances,
                    }),
                });
            }

            // Adopt the new iterate.
            iterate = next_iterate;

            // (5) Residue at the new iterate. `F(x_{k+1})` uses the
            //     *unlinearized* device evaluation.
            let residue =
                system
                    .residue(&iterate)
                    .map_err(|source| NewtonRaphsonError::System {
                        iteration: k,
                        source,
                    })?;

            if residue.len() != system_dim as usize {
                return Err(NewtonRaphsonError::ResidueDimMismatch {
                    iteration: k,
                    residue_len: residue.len(),
                    system_dim,
                });
            }

            let residue_norm = infinity_norm(&residue);

            iterations_completed = k + 1;

            // (6) Divergence on residue.
            if !residue_norm.is_finite() {
                return Ok(NewtonRaphsonOutcome {
                    iterate,
                    status: ConvergenceStatus::Diverged(ConvergenceDiagnostic {
                        update_norm: if update_norm.is_finite() {
                            update_norm
                        } else {
                            last_finite_update_norm
                        },
                        residue_norm: last_finite_residue_norm,
                        iterations: iterations_completed,
                        tolerances: config.tolerances,
                    }),
                });
            }

            // Both norms finite — remember them for the next pass'
            // divergence diagnostics.
            last_finite_update_norm = update_norm;
            last_finite_residue_norm = residue_norm;

            let diagnostic = ConvergenceDiagnostic {
                update_norm,
                residue_norm,
                iterations: iterations_completed,
                tolerances: config.tolerances,
            };

            // (7) Dual convergence test (ADR-0006).
            if diagnostic.dual_satisfied() {
                return Ok(NewtonRaphsonOutcome {
                    iterate,
                    status: ConvergenceStatus::Converged(diagnostic),
                });
            }

            // Otherwise: continue iterating. We deliberately do NOT
            // bail early when only the update criterion is satisfied
            // — the residue may still be converging quadratically and
            // would satisfy on the next iteration. The ADR-0006
            // "stall" signal is reported at *termination* (below),
            // when the iteration budget has been exhausted while the
            // update is small but the residue is not: that pattern
            // is precisely the false-convergence mode a single-
            // criterion check would have falsely declared converged.
        }

        // ─── Iteration budget exhausted ─────────────────────────────
        let diagnostic = ConvergenceDiagnostic {
            update_norm: last_finite_update_norm,
            residue_norm: last_finite_residue_norm,
            iterations: iterations_completed,
            tolerances: config.tolerances,
        };
        // Distinguish the two asymmetric failure modes per ADR-0006:
        // - `Stalled`: update satisfied, residue not — the classic
        //   false-convergence mode that a single-criterion (update-
        //   only) check would have falsely declared `Converged`.
        // - `MaxIterationsExceeded`: neither criterion was on the
        //   verge of satisfaction (or only residue was) when we ran
        //   out of budget; this is genuine iteration-limit exhaustion
        //   without the stall signature.
        let status = if diagnostic.update_satisfied() && !diagnostic.residue_satisfied() {
            ConvergenceStatus::Stalled(diagnostic)
        } else {
            ConvergenceStatus::MaxIterationsExceeded(diagnostic)
        };
        Ok(NewtonRaphsonOutcome { iterate, status })
    }
}

/// Infinity norm of a slice. Returns `0.0` for an empty slice and
/// propagates `NaN` / `Inf` (the caller uses `is_finite()` to detect
/// divergence).
#[inline]
fn infinity_norm(v: &[f64]) -> f64 {
    let mut max = 0.0_f64;
    for &x in v {
        let a = x.abs();
        if !a.is_finite() {
            return a; // propagate NaN/Inf immediately
        }
        if a > max {
            max = a;
        }
    }
    max
}

/// Infinity norm of `a - b`. Returns `0.0` for empty inputs.
///
/// # Panics
///
/// In debug builds, panics if `a.len() != b.len()`. Release builds
/// trust the upstream dim checks in [`NewtonRaphsonDriver::solve`].
#[inline]
fn infinity_norm_diff(a: &[f64], b: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), b.len(), "infinity_norm_diff: length mismatch");
    let mut max = 0.0_f64;
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        let d = (ai - bi).abs();
        if !d.is_finite() {
            return d;
        }
        if d > max {
            max = d;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_solver::{RussellRealSolver, SparseTriplet};

    // ─── Test helpers ───────────────────────────────────────────────

    /// Build a 1×1 sparse linear system `[a] · [x] = [b]`. The
    /// `node_count = dim, branch_count = 0` partition mimics a
    /// single ground-suppressed MNA node row.
    fn scalar_system(a: f64, b: f64) -> SparseLinearSystem<f64> {
        SparseLinearSystem::new(
            1,
            1,
            0,
            vec![SparseTriplet {
                row: 0,
                col: 0,
                value: a,
            }],
            vec![b],
        )
        .expect("scalar system construction")
    }

    /// A linear `A · x = b` system: residue is `A · x - b`, so the
    /// driver should converge in exactly one iteration (the linear
    /// solve produces the exact solution; the residue evaluates to
    /// zero modulo round-off).
    ///
    /// Layout: 1×1 system with `A = a, b = rhs`. Initial iterate
    /// arbitrary; the linearization is the same matrix on every call.
    struct LinearScalarSystem {
        a: f64,
        rhs: f64,
        linearize_calls: u32,
        residue_calls: u32,
    }

    impl LinearScalarSystem {
        fn new(a: f64, rhs: f64) -> Self {
            Self {
                a,
                rhs,
                linearize_calls: 0,
                residue_calls: 0,
            }
        }
    }

    impl NonlinearSystem for LinearScalarSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            self.linearize_calls += 1;
            Ok(scalar_system(self.a, self.rhs))
        }
        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            self.residue_calls += 1;
            Ok(vec![self.a * iterate[0] - self.rhs])
        }
    }

    /// A genuinely nonlinear 1-D problem: solve `f(x) = x² - c = 0`.
    /// Tangent at `x_k` is `2·x_k`, so Newton's update is
    /// `x_{k+1} = x_k - (x_k² - c) / (2·x_k) = (x_k + c/x_k) / 2`.
    ///
    /// We linearize as `(2·x_k) · x_{k+1} = (2·x_k)·x_k - (x_k² - c)
    /// = x_k² + c`, which gives `x_{k+1} = (x_k² + c) / (2·x_k)` —
    /// the exact NR update for a square root.
    struct SqrtNonlinear {
        c: f64,
        residue_calls: u32,
    }

    impl SqrtNonlinear {
        fn new(c: f64) -> Self {
            Self {
                c,
                residue_calls: 0,
            }
        }
    }

    impl NonlinearSystem for SqrtNonlinear {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            let xk = iterate[0];
            // Jacobian is 2·x_k; RHS is x_k² + c (= J·x_k - F(x_k)).
            Ok(scalar_system(2.0 * xk, xk * xk + self.c))
        }
        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            self.residue_calls += 1;
            let x = iterate[0];
            Ok(vec![x * x - self.c])
        }
    }

    /// A pathological system that produces a tiny update each
    /// iteration but never reduces the residue — the exact ADR-0006
    /// stall mode.
    ///
    /// `linearize` returns the trivial identity system `x_{k+1} = x_k`
    /// (so the update is exactly zero), while `residue` returns the
    /// constant value `1.0` regardless of iterate. A single-criterion
    /// (update-only) check would falsely declare convergence on
    /// iteration 1; the dual criterion correctly reports `Stalled`.
    struct StallSystem;

    impl NonlinearSystem for StallSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            // `1 · x_{k+1} = x_k` → next iterate equals current iterate
            // → update is identically zero.
            Ok(scalar_system(1.0, iterate[0]))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            // Persistent residue above tolerance: this is what the
            // dual criterion catches that update-only misses.
            Ok(vec![1.0])
        }
    }

    /// A divergent system: each iteration squares the iterate
    /// (`x_{k+1} = x_k²`). Starting from `x_0 = 10` this reaches
    /// `f64::INFINITY` (≈ 1.8e308) in `⌈log₂(308/log₁₀(10))⌉ ≈ 10`
    /// iterations, so divergence is exercised long before any
    /// reasonable iteration budget is hit.
    struct DivergeSystem;

    impl NonlinearSystem for DivergeSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            // `1 · x_{k+1} = x_k²` → squaring growth.
            Ok(scalar_system(1.0, iterate[0] * iterate[0]))
        }
        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            // Residue tracks the iterate so it overflows alongside.
            Ok(vec![iterate[0]])
        }
    }

    /// A system that loops forever but never converges. Update is a
    /// constant `1.0` (above `update_tol`) and residue is `1.0`
    /// (above `residue_tol`) on every iteration.
    struct MaxIterSystem;

    impl NonlinearSystem for MaxIterSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            // `1 · x_{k+1} = x_k + 1` → constant update of 1.0.
            Ok(scalar_system(1.0, iterate[0] + 1.0))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![1.0])
        }
    }

    /// A modeling-failure system that errors in `linearize`.
    struct LinearizeFailureSystem;

    impl NonlinearSystem for LinearizeFailureSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            Err(SystemError::new("device-parameter table missing"))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![0.0])
        }
    }

    /// A modeling-failure system that errors in `residue`.
    struct ResidueFailureSystem;

    impl NonlinearSystem for ResidueFailureSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            Ok(scalar_system(1.0, iterate[0] + 1.0))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Err(SystemError::new("non-finite device current"))
        }
    }

    /// A system that lies about its dim by returning a 2-element
    /// residue instead of 1.
    struct ResidueDimLiarSystem;

    impl NonlinearSystem for ResidueDimLiarSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            Ok(scalar_system(1.0, 0.0))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![0.0, 0.0])
        }
    }

    /// A system that lies about its dim by returning a 2×2 linear
    /// system instead of 1×1.
    struct LinearizeDimLiarSystem;

    impl NonlinearSystem for LinearizeDimLiarSystem {
        fn dim(&self) -> u32 {
            1
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
                        value: 1.0,
                    },
                ],
                vec![0.0, 0.0],
            )
            .map_err(|e| SystemError::new(format!("{e}")))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![0.0])
        }
    }

    /// A system that emits a singular linear system, exercising the
    /// driver's "linear-solver singular → Diverged" path.
    struct SingularSystem;

    impl NonlinearSystem for SingularSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            // Zero coefficient → singular 1×1 matrix.
            Ok(scalar_system(0.0, 1.0))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![1.0])
        }
    }

    // ─── Tests ──────────────────────────────────────────────────────

    #[test]
    fn linear_problem_converges_in_two_iterations() {
        // `5 x = 10` → x = 2. Iteration 1: linearize at x=0 yields
        // `5·x = 10` → solve gives x=2; update is |2-0|=2 which is
        // *above* `update_tol = 1e-3`, so even though the residue at
        // x=2 is exactly zero we keep going (dual criterion).
        // Iteration 2: linearize at x=2 yields the same matrix
        // (linear problem), solve gives x=2; update is |2-2|=0,
        // residue still zero → Converged.
        //
        // This is the canonical "two-iteration convergence" pattern
        // for linear systems under the dual criterion: one to find
        // the solution, one to confirm the update has stabilized.
        let mut sys = LinearScalarSystem::new(5.0, 10.0);
        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(outcome.status.is_converged());
        let d = outcome.status.diagnostic();
        assert_eq!(d.iterations, 2);
        assert!(d.update_norm < 1e-12);
        assert!(d.residue_norm < 1e-12);
        assert!((outcome.iterate[0] - 2.0).abs() < 1e-12);
        assert_eq!(sys.linearize_calls, 2);
        assert_eq!(sys.residue_calls, 2);
    }

    #[test]
    fn nonlinear_sqrt_converges_with_quadratic_rate() {
        // Solve x² = 2, starting from x = 1.5. NR converges in
        // roughly log₂(precision) ≈ 6 iterations to f64 epsilon.
        let mut sys = SqrtNonlinear::new(2.0);
        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![1.5],
            )
            .unwrap();

        assert!(
            outcome.status.is_converged(),
            "expected Converged, got {:?}, iterate={:?}",
            outcome.status,
            outcome.iterate
        );
        let d = outcome.status.diagnostic();
        assert!(
            d.iterations <= 10,
            "expected ≤10 NR iterations, got {}",
            d.iterations
        );
        assert!(
            (outcome.iterate[0] - 2.0_f64.sqrt()).abs() < 1e-12,
            "expected √2, got {}",
            outcome.iterate[0]
        );
    }

    #[test]
    fn dual_criterion_catches_stall_that_update_only_would_miss() {
        // The ADR-0006 false-convergence mode: ‖Δx‖∞ ≡ 0 but
        // residue ≡ 1.0 (above `abstol = 1e-12`). A single-
        // criterion update check would have falsely declared
        // `Converged`. We expect `Stalled` — but only after the
        // iteration budget is exhausted (the driver does not bail
        // early on the first iteration showing the stall signature).
        let mut sys = StallSystem;
        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig {
                    max_iterations: 5,
                    tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.5],
            )
            .unwrap();

        assert!(
            matches!(outcome.status, ConvergenceStatus::Stalled(_)),
            "expected Stalled, got {:?}",
            outcome.status
        );
        let d = outcome.status.diagnostic();
        assert!(d.update_satisfied(), "update should be satisfied");
        assert!(!d.residue_satisfied(), "residue should not be satisfied");
        assert_eq!(
            d.iterations, 5,
            "stall is reported at termination after budget exhaustion"
        );
    }

    #[test]
    fn diverging_system_reports_diverged() {
        let mut sys = DivergeSystem;
        // Initial iterate 10.0; squaring each step reaches
        // f64::INFINITY in ~10 iterations. With max_iterations = 100
        // we are well past overflow.
        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig {
                    max_iterations: 100,
                    tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
                },
                &mut sys,
                &RussellRealSolver,
                vec![10.0],
            )
            .unwrap();

        assert!(
            matches!(outcome.status, ConvergenceStatus::Diverged(_)),
            "expected Diverged, got {:?}",
            outcome.status
        );
    }

    #[test]
    fn max_iterations_exhausted_yields_max_iterations_exceeded() {
        // Update is constantly 1.0, residue is constantly 1.0 —
        // neither criterion is ever satisfied. With max_iterations
        // = 5 we should hit the budget cleanly.
        let mut sys = MaxIterSystem;
        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig {
                    max_iterations: 5,
                    tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(
            matches!(outcome.status, ConvergenceStatus::MaxIterationsExceeded(_)),
            "expected MaxIterationsExceeded, got {:?}",
            outcome.status
        );
        let d = outcome.status.diagnostic();
        assert_eq!(d.iterations, 5);
        // Last-finite norms recorded.
        assert!((d.update_norm - 1.0).abs() < 1e-12);
        assert!((d.residue_norm - 1.0).abs() < 1e-12);
    }

    #[test]
    #[allow(clippy::float_cmp)] // Exact-zero contract for the empty-system short circuit.
    fn empty_system_vacuously_converges() {
        struct EmptySystem;
        impl NonlinearSystem for EmptySystem {
            fn dim(&self) -> u32 {
                0
            }
            fn linearize(
                &mut self,
                _iterate: &[f64],
            ) -> Result<SparseLinearSystem<f64>, SystemError> {
                unreachable!("driver short-circuits on empty system")
            }
            fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
                unreachable!("driver short-circuits on empty system")
            }
        }

        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut EmptySystem,
                &RussellRealSolver,
                vec![],
            )
            .unwrap();

        assert!(outcome.status.is_converged());
        let d = outcome.status.diagnostic();
        assert_eq!(d.iterations, 0);
        assert_eq!(d.update_norm, 0.0);
        assert_eq!(d.residue_norm, 0.0);
        assert!(outcome.iterate.is_empty());
    }

    #[test]
    fn initial_iterate_dim_mismatch_is_a_hard_error() {
        let mut sys = LinearScalarSystem::new(1.0, 1.0);
        let err = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0, 0.0], // dim mismatch: expected 1, got 2
            )
            .unwrap_err();

        assert!(matches!(
            err,
            NewtonRaphsonError::InitialIterateDimMismatch {
                iterate_len: 2,
                system_dim: 1,
            }
        ));
    }

    #[test]
    fn linearize_returning_wrong_dim_is_a_hard_error() {
        let mut sys = LinearizeDimLiarSystem;
        let err = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();

        assert!(matches!(
            err,
            NewtonRaphsonError::LinearizedDimMismatch {
                iteration: 0,
                linear_dim: 2,
                system_dim: 1,
            }
        ));
    }

    #[test]
    fn residue_returning_wrong_length_is_a_hard_error() {
        let mut sys = ResidueDimLiarSystem;
        let err = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();

        assert!(matches!(
            err,
            NewtonRaphsonError::ResidueDimMismatch {
                iteration: 0,
                residue_len: 2,
                system_dim: 1,
            }
        ));
    }

    #[test]
    fn linearize_modeling_error_is_a_hard_error() {
        let mut sys = LinearizeFailureSystem;
        let err = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();

        match err {
            NewtonRaphsonError::System { iteration, source } => {
                assert_eq!(iteration, 0);
                assert!(source.description.contains("device-parameter"));
            }
            other => panic!("expected System error, got {other:?}"),
        }
    }

    #[test]
    fn residue_modeling_error_is_a_hard_error() {
        let mut sys = ResidueFailureSystem;
        let err = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();

        match err {
            NewtonRaphsonError::System { iteration, source } => {
                assert_eq!(iteration, 0);
                assert!(source.description.contains("non-finite"));
            }
            other => panic!("expected System error, got {other:?}"),
        }
    }

    #[test]
    fn singular_linear_system_maps_to_diverged() {
        // `0 · x = 1` → linear solver returns SingularMatrix; driver
        // translates that into ConvergenceStatus::Diverged.
        let mut sys = SingularSystem;
        let outcome = NewtonRaphsonDriver
            .solve(
                NewtonRaphsonConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(
            matches!(outcome.status, ConvergenceStatus::Diverged(_)),
            "expected Diverged, got {:?}",
            outcome.status
        );
    }

    #[test]
    fn spice_default_config_matches_documented_constants() {
        let cfg = NewtonRaphsonConfig::DC_DEFAULTS;
        assert_eq!(cfg.max_iterations, 100);
        assert_eq!(cfg.tolerances, ConvergenceTolerances::SPICE_DEFAULTS);

        let cfg = NewtonRaphsonConfig::TRANSIENT_DEFAULTS;
        assert_eq!(cfg.max_iterations, 10);
        assert_eq!(cfg.tolerances, ConvergenceTolerances::SPICE_DEFAULTS);

        // Default trait matches DC.
        assert_eq!(
            NewtonRaphsonConfig::default(),
            NewtonRaphsonConfig::DC_DEFAULTS
        );
    }

    #[test]
    #[allow(clippy::float_cmp)] // Exact-zero / known-constant comparisons on infinity-norm
                                // helper outputs are part of the test contract.
    fn infinity_norm_handles_empty_and_finite_inputs() {
        assert_eq!(infinity_norm(&[]), 0.0);
        assert_eq!(infinity_norm(&[1.0, -2.5, 0.7]), 2.5);
        assert!(infinity_norm(&[1.0, f64::NAN]).is_nan());
        assert_eq!(infinity_norm(&[1.0, f64::INFINITY]), f64::INFINITY);
    }

    #[test]
    #[allow(clippy::float_cmp)] // Same as above: testing the helper's exact contract.
    fn infinity_norm_diff_zero_when_equal() {
        assert_eq!(infinity_norm_diff(&[], &[]), 0.0);
        assert_eq!(infinity_norm_diff(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), 0.0);
        assert!((infinity_norm_diff(&[1.0, 2.0], &[1.5, 2.0]) - 0.5).abs() < 1e-15);
        assert!(infinity_norm_diff(&[f64::NAN, 0.0], &[0.0, 0.0]).is_nan());
    }

    /// Regression guard for ADR-0006: a *single-criterion* (update-
    /// only) sanity test must NOT be added as the driver's primary
    /// convergence check. The presence of `ConvergenceStatus::Stalled`
    /// in the public API and the dual-satisfied predicate inside the
    /// driver are the two structural barriers. This test asserts on
    /// both by reading the public surface.
    #[test]
    fn dual_criterion_is_structural_not_optional() {
        // Surface check 1: `Stalled` variant must exist as a public
        // variant of `ConvergenceStatus`. The match below would not
        // compile if the variant were removed.
        let d = ConvergenceDiagnostic {
            update_norm: 0.0,
            residue_norm: 1.0,
            iterations: 1,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        };
        let s = ConvergenceStatus::Stalled(d);
        assert!(!s.is_converged());

        // Surface check 2: `ConvergenceDiagnostic::dual_satisfied`
        // must require *both* norms below tolerance.
        let d_update_only = ConvergenceDiagnostic {
            update_norm: 1e-9,
            residue_norm: 1e-3, // > 1e-12 abstol
            iterations: 1,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        };
        assert!(d_update_only.update_satisfied());
        assert!(!d_update_only.residue_satisfied());
        assert!(!d_update_only.dual_satisfied());
    }
}
