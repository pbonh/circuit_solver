//! Source-stepping homotopy driver (tasks.md item #19).
//!
//! Source stepping is the classical SPICE-family continuation aid for
//! DC analysis: every independent source's value is multiplied by a
//! continuation parameter `α ∈ [0, 1]` which is then walked from
//! `α = 0` (trivial zero solution) to `α = 1` (the user's specified
//! operating point). At each step the
//! [`NewtonRaphsonDriver`] is re-run, **warm-started** from the
//! previous step's converged iterate. The continuity of the
//! solution trajectory means the previous solution sits inside
//! Newton's basin of attraction at the next α.
//!
//! See [`wiki/concepts/source-stepping.md`](../../../../wiki/concepts/source-stepping.md)
//! for the conceptual reference and
//! [`wiki/concepts/homotopy-method.md`](../../../../wiki/concepts/homotopy-method.md)
//! for the family of methods this belongs to.
//!
//! # Where this fits
//!
//! The DC analysis control loop (tasks.md #20) calls
//! [`NewtonRaphsonDriver`] directly. **On non-convergence** (one of
//! `Diverged`, `Stalled`, `MaxIterationsExceeded`) the orchestrator
//! falls back to a homotopy driver — source-stepping here, or
//! Gmin-stepping (tasks.md #18, sibling module) — to coax the
//! iteration into a converging trajectory. The two homotopy
//! drivers are interchangeable from the orchestrator's
//! perspective: each takes the same [`NonlinearSystem`]
//! abstraction (extended with the variant-specific `set_alpha` /
//! `set_gmin` callback) and returns a comparable
//! [`SourceSteppingOutcome`].
//!
//! # The trait
//!
//! Source-stepping needs **one additional capability** beyond the
//! plain [`NonlinearSystem`] contract: a way for the driver to
//! adjust the system's source-scaling factor `α` between NR runs.
//! The [`SourceSteppableSystem`] super-trait adds exactly that
//! callback. Implementors typically thread `α` into their internal
//! [`SubViewBuilder::with_source_step`](crate::sub_view::SubViewBuilder::with_source_step)
//! call when next building the linearized step (the assembler's
//! source-RHS contribution is multiplied by `α` post-assembly per
//! tasks.md #15 of the sub-view extractor).
//!
//! # Schedule and adaptive halving
//!
//! The driver walks a **schedule** of α values, in increasing order,
//! starting from `α = 0` and ending at `α = 1`. At each α the
//! NR driver is run with the previous step's iterate as the initial
//! guess.
//!
//! - If NR **converges**, the iterate is accepted and the driver
//!   advances to the next scheduled α.
//! - If NR **fails** (any non-`Converged` status), the driver
//!   inserts a midpoint between the *last accepted* α and the
//!   current α and retries — i.e., halves the step. Up to
//!   [`max_step_halvings`](SourceSteppingConfig::max_step_halvings)
//!   halvings are attempted before declaring the homotopy itself
//!   has failed.
//!
//! The total **homotopy step count** reported in
//! [`SourceSteppingOutcome::homotopy_steps`] counts the number of
//! *accepted* α values (including `α = 0` and `α = 1`). The
//! corresponding [`ConvergenceStatus`] reflects the final NR run at
//! `α = 1`.
//!
//! # Defaults
//!
//! [`SourceSteppingConfig::dc_defaults`] aligns with the wiki spec
//! `circuit-solver.md` "Roya runs a CMOS inverter at its metastable
//! point" scenario which fixes the homotopy retry budget at **10**.
//! The default schedule is a uniform 11-point ramp `0.0, 0.1, ...,
//! 1.0`; adaptive halving sits on top and can extend the work
//! budget when intermediate α values themselves fail.
//!
//! # Honored ADRs
//!
//! - **ADR-0006** — convergence at each α is the dual-criterion NR
//!   loop. This module does not redefine convergence; it composes
//!   the inner driver's `ConvergenceStatus` decisions.
//! - **ADR-0010** — every type and function exported here is part
//!   of the v1 *unstable* public Rust API.

#![allow(clippy::module_name_repetitions)]

use circuit_solver_types::{ConvergenceDiagnostic, ConvergenceStatus};

use crate::linear_solver::LinearSolver;
use crate::newton_raphson::{
    NewtonRaphsonConfig, NewtonRaphsonDriver, NewtonRaphsonError, NewtonRaphsonOutcome,
    NonlinearSystem,
};

/// A [`NonlinearSystem`] that can have its independent-source
/// scaling factor `α` adjusted between Newton-Raphson runs.
///
/// Implementors store the most recent `α` and apply it to the
/// source-RHS contribution the next time
/// [`linearize`](NonlinearSystem::linearize) or
/// [`residue`](NonlinearSystem::residue) is called. The driver
/// always calls [`set_source_alpha`](Self::set_source_alpha) **before**
/// the NR run at that α begins.
///
/// `α = 0` must yield the *trivial* system in which every
/// independent-source contribution to the RHS is suppressed; the
/// zero vector is the exact solution. `α = 1` recovers the
/// user-specified operating point.
pub trait SourceSteppableSystem: NonlinearSystem {
    /// Set the current source-scaling factor `α`.
    ///
    /// Called by [`SourceSteppingDriver::solve`] before each NR run.
    /// Implementors should retain the value and apply it the next
    /// time [`linearize`](NonlinearSystem::linearize) or
    /// [`residue`](NonlinearSystem::residue) is invoked. The driver
    /// guarantees `α ∈ [0, 1]` and that `α` is finite.
    fn set_source_alpha(&mut self, alpha: f64);
}

/// Tuning knobs for the source-stepping homotopy loop.
///
/// `schedule` is the *initial* (pre-adaptive-halving) sequence of α
/// values. It must be sorted strictly ascending, start with `0.0`,
/// and end with `1.0`; [`SourceSteppingDriver::solve`] returns
/// [`SourceSteppingError::InvalidSchedule`] otherwise.
///
/// `inner` is the NR configuration used at every scheduled step.
/// Same tolerances and iteration budget at every α — the driver
/// does *not* loosen tolerances at intermediate α; the only
/// adaptation lever is α step size itself.
///
/// `max_step_halvings` caps the **per-step** adaptive-halving
/// budget. A value of 0 disables adaptive halving entirely (one
/// shot per scheduled α, fail if NR doesn't converge).
#[derive(Debug, Clone, PartialEq)]
pub struct SourceSteppingConfig {
    /// Ascending sequence of α values starting at `0.0` and ending
    /// at `1.0`. The driver walks this list in order, running NR
    /// at each entry.
    pub schedule: Vec<f64>,
    /// NR configuration applied at every scheduled α.
    pub inner: NewtonRaphsonConfig,
    /// Maximum number of *adaptive halvings* between two scheduled
    /// α values on NR failure. A value of `0` disables adaptive
    /// halving; the driver fails out on the first NR non-convergence.
    pub max_step_halvings: u32,
}

impl SourceSteppingConfig {
    /// SPICE-conventional defaults: uniform 11-point ramp `0.0,
    /// 0.1, ..., 1.0`, DC-defaults NR config, up to 10 adaptive
    /// halvings per failing step.
    ///
    /// The 10-halvings ceiling aligns with the
    /// "homotopy retry budget is set to the default 10 steps"
    /// scenario in `wiki/specs/circuit-solver.md`.
    #[must_use]
    pub fn dc_defaults() -> Self {
        Self {
            schedule: vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
            inner: NewtonRaphsonConfig::DC_DEFAULTS,
            max_step_halvings: 10,
        }
    }
}

impl Default for SourceSteppingConfig {
    fn default() -> Self {
        Self::dc_defaults()
    }
}

/// Outcome of [`SourceSteppingDriver::solve`].
///
/// On success (`status` is `Converged`) `iterate` holds the solution
/// at `α = 1`. On any non-`Converged` status, `iterate` is the last
/// *successfully converged* iterate produced by the inner NR loop
/// (or the user-supplied initial iterate, if no NR run converged).
/// `homotopy_steps` reports the number of *accepted* α values
/// (including the trivial `α = 0` if it was attempted; including
/// intermediate adaptive midpoints; including `α = 1` on success).
#[derive(Debug, Clone, PartialEq)]
pub struct SourceSteppingOutcome {
    /// The iterate at the end of the homotopy walk. On success
    /// this is the operating point at `α = 1`; on failure it is
    /// the last iterate at which the inner NR loop converged.
    pub iterate: Vec<f64>,
    /// Convergence status of the final NR run.
    ///
    /// On success this is `Converged(_)` for the NR run at `α = 1`.
    /// On failure this is whatever non-`Converged` status the
    /// last inner NR run produced (with the last-finite norms).
    pub status: ConvergenceStatus,
    /// Number of accepted α values, including `0.0` and (on
    /// success) `1.0`, plus any adaptive midpoints that were
    /// accepted. Does *not* count failed attempts that were rolled
    /// back via halving.
    pub homotopy_steps: u32,
    /// Total Newton-Raphson iterations summed across every NR run
    /// the homotopy driver invoked (both accepted and failed +
    /// retried). This is the user's headline cost metric.
    pub total_nr_iterations: u32,
    /// The α value at which the loop terminated. On success this
    /// is `1.0`; on failure it is the last α attempted (which may
    /// be an adaptive midpoint, not a scheduled value).
    pub final_alpha: f64,
}

/// Hard errors returned by [`SourceSteppingDriver::solve`] when the
/// homotopy walk could not run to its natural end.
///
/// These mirror the convention in [`NewtonRaphsonError`]: pre-loop
/// configuration mistakes and modeling errors are returned as `Err`;
/// non-convergence outcomes are reported as `Ok` with the
/// appropriate [`ConvergenceStatus`] inside
/// [`SourceSteppingOutcome`].
#[derive(Debug, Clone, PartialEq)]
pub enum SourceSteppingError {
    /// The schedule was empty, did not start at `0.0`, did not end
    /// at `1.0`, was not strictly ascending, or contained a
    /// non-finite value.
    InvalidSchedule {
        /// A human-readable explanation of which invariant was
        /// violated.
        reason: String,
    },
    /// The initial iterate's length did not match the system's
    /// [`NonlinearSystem::dim`].
    InitialIterateDimMismatch {
        /// Length of the supplied initial iterate.
        iterate_len: usize,
        /// Dim reported by the system.
        system_dim: u32,
    },
    /// The inner NR driver returned a hard error that the homotopy
    /// driver cannot recover from (dim mismatch, modeling error,
    /// linear-solver crash). The α value at which the failure
    /// occurred is captured for diagnostics.
    Inner {
        /// The α value at which the inner driver was running when
        /// the hard error surfaced.
        alpha: f64,
        /// The underlying NR error.
        source: NewtonRaphsonError,
    },
}

impl core::fmt::Display for SourceSteppingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSchedule { reason } => {
                write!(f, "source-stepping: invalid schedule: {reason}")
            }
            Self::InitialIterateDimMismatch {
                iterate_len,
                system_dim,
            } => write!(
                f,
                "source-stepping: initial iterate length {iterate_len} != system dim {system_dim}",
            ),
            Self::Inner { alpha, source } => {
                write!(f, "source-stepping: inner NR error at α={alpha}: {source}")
            }
        }
    }
}

impl std::error::Error for SourceSteppingError {}

/// Stateless source-stepping homotopy dispatcher.
///
/// The driver itself holds no state; it borrows a
/// [`LinearSolver<f64>`] for the inner NR runs and a
/// [`SourceSteppableSystem`] for the stamping / residue callbacks
/// (the latter borrowed `&mut` so the driver can adjust α between
/// runs).
///
/// The intended composition for DC operating-point convergence
/// fallback (tasks.md #20) is:
///
/// ```text
/// // Plain NR failed:
/// match NewtonRaphsonDriver.solve(...) {
///     Ok(NewtonRaphsonOutcome { status, .. }) if status.is_failure() => {
///         // Fall back to source stepping.
///         let homotopy = SourceSteppingDriver
///             .solve(
///                 &SourceSteppingConfig::dc_defaults(),
///                 &mut system,
///                 &RussellRealSolver,
///                 zero_initial_iterate);
///     }
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct SourceSteppingDriver;

impl SourceSteppingDriver {
    /// Run the source-stepping homotopy loop.
    ///
    /// Walks `config.schedule` from `α = 0` to `α = 1`, running the
    /// dual-criterion NR loop at each α with the previous accepted
    /// iterate as the initial guess. On NR failure the driver
    /// inserts midpoints (up to `config.max_step_halvings` per
    /// transition) before giving up.
    ///
    /// # Returns
    ///
    /// On natural termination — homotopy converged at `α = 1` or
    /// gave up — returns `Ok(SourceSteppingOutcome)`. On a hard
    /// failure that prevented the loop from running to its natural
    /// end (bad schedule, dim mismatch, inner-NR hard error),
    /// returns `Err(SourceSteppingError)`.
    ///
    /// # Errors
    ///
    /// See [`SourceSteppingError`] for the full list.
    #[allow(clippy::too_many_lines)] // The adaptive-halving body
                                     // is a single conceptual unit;
                                     // splitting it would obscure
                                     // the homotopy contract more
                                     // than it would clarify.
    pub fn solve<S, L>(
        self,
        config: &SourceSteppingConfig,
        system: &mut S,
        solver: &L,
        initial_iterate: Vec<f64>,
    ) -> Result<SourceSteppingOutcome, SourceSteppingError>
    where
        S: SourceSteppableSystem,
        L: LinearSolver<f64>,
    {
        // ─── Pre-loop validation ────────────────────────────────────
        validate_schedule(&config.schedule)?;

        let system_dim = system.dim();
        if initial_iterate.len() != system_dim as usize {
            return Err(SourceSteppingError::InitialIterateDimMismatch {
                iterate_len: initial_iterate.len(),
                system_dim,
            });
        }

        // Empty system: vacuously converged with zero homotopy steps
        // and zero NR iterations. Matches the NR driver's empty-
        // system contract; the analysis orchestrator can treat the
        // empty-circuit case as a uniform success path regardless
        // of which driver it ended up calling.
        if system_dim == 0 {
            return Ok(SourceSteppingOutcome {
                iterate: initial_iterate,
                status: ConvergenceStatus::Converged(ConvergenceDiagnostic {
                    update_norm: 0.0,
                    residue_norm: 0.0,
                    iterations: 0,
                    tolerances: config.inner.tolerances,
                }),
                homotopy_steps: 0,
                total_nr_iterations: 0,
                final_alpha: 1.0,
            });
        }

        // ─── Main loop ──────────────────────────────────────────────
        //
        // We maintain:
        //   - `accepted_iterate`  — the iterate at the last accepted α
        //   - `accepted_alpha`    — the corresponding α
        //   - `homotopy_steps`    — count of accepted α values
        //   - `total_nr_iterations` — sum across every NR call
        //   - `last_status`       — the most recent inner NR status
        //
        // For each transition `accepted_alpha → schedule[i]` we
        // attempt up to `max_step_halvings + 1` NR runs: the full
        // step first, then halvings inserted between
        // `accepted_alpha` and the current target.

        let mut accepted_iterate = initial_iterate;
        let mut accepted_alpha = 0.0_f64;
        let mut homotopy_steps: u32 = 0;
        let mut total_nr_iterations: u32 = 0;
        // `last_status` is set on the *first* NR run and updated on
        // every subsequent run. The empty-system branch above
        // returned early, so dim > 0 here and at least one NR run
        // will happen.
        let mut last_status: Option<ConvergenceStatus> = None;
        // `last_alpha_attempted` is set the first time we call
        // `set_source_alpha` below. We track it so a failure path
        // can report the α value at which the homotopy halted
        // (which may be a midpoint, not a scheduled entry).
        let mut last_alpha_attempted: f64;

        for &target_alpha in &config.schedule {
            // Skip the very first entry only if it equals the
            // initial accepted_alpha (= 0.0). On the first scheduled
            // value we still want to run NR at α = 0 to obtain a
            // *verified* trivial-system iterate (the user-supplied
            // initial_iterate may not exactly satisfy F(x) = 0 even
            // at α = 0; for instance, a nonzero starting guess on a
            // nonlinear system with α = 0 still requires NR to drive
            // x → 0).
            //
            // We do, however, special-case the case where the
            // schedule's first entry is exactly the accepted_alpha
            // *and* we have not yet run NR. Then we run NR once at
            // α = 0 to seed the chain. On subsequent identical α
            // values (none allowed by validate_schedule; strictly
            // ascending) we would skip.

            // Target for this transition.
            let mut current_target = target_alpha;
            let mut halvings_used: u32 = 0;
            let transition_outcome = loop {
                system.set_source_alpha(current_target);
                last_alpha_attempted = current_target;

                let nr_outcome = NewtonRaphsonDriver.solve(
                    config.inner,
                    system,
                    solver,
                    accepted_iterate.clone(),
                );

                match nr_outcome {
                    Err(e) => {
                        return Err(SourceSteppingError::Inner {
                            alpha: current_target,
                            source: e,
                        });
                    }
                    Ok(NewtonRaphsonOutcome { iterate, status }) => {
                        total_nr_iterations =
                            total_nr_iterations.saturating_add(status.diagnostic().iterations);
                        last_status = Some(status);
                        if status.is_converged() {
                            // Accept this step.
                            break Ok((iterate, status, current_target));
                        }
                        // NR failed at `current_target`. Try
                        // halving the step from accepted_alpha
                        // toward current_target. Note we do not
                        // halve below the smallest representable
                        // f64 progress; we cap at
                        // `max_step_halvings` total.
                        if halvings_used >= config.max_step_halvings {
                            break Err(status);
                        }
                        halvings_used += 1;
                        let midpoint = 0.5 * (accepted_alpha + current_target);
                        // Floating-point guard: if the midpoint is
                        // not strictly between accepted_alpha and
                        // current_target (e.g., they have collapsed
                        // to within an ULP of each other), no
                        // further progress is possible; give up.
                        if !(midpoint > accepted_alpha && midpoint < current_target) {
                            break Err(status);
                        }
                        current_target = midpoint;
                    }
                }
            };

            match transition_outcome {
                Ok((iterate, _status, accepted_at)) => {
                    accepted_iterate = iterate;
                    accepted_alpha = accepted_at;
                    homotopy_steps = homotopy_steps.saturating_add(1);

                    // If we landed at a midpoint (adaptive halving
                    // succeeded but we have not reached the
                    // scheduled target yet), re-run this iteration
                    // of the outer loop targeting the same
                    // schedule entry. We do this by *not*
                    // advancing the outer iterator; instead we
                    // detect not-yet-reached-target and rebuild
                    // the inner loop here.
                    //
                    // We achieve this by treating the schedule as
                    // a *hint*: after each accepted midpoint we
                    // immediately re-aim at the same scheduled
                    // target. The cleanest way to express that is
                    // a while-loop inside the outer for-loop. But
                    // since we already exited the inner loop with
                    // `break Ok(...)`, we re-enter via a `while`
                    // here:
                    while accepted_alpha < target_alpha {
                        let mut current_target = target_alpha;
                        let mut halvings_used: u32 = 0;
                        let inner = loop {
                            system.set_source_alpha(current_target);
                            last_alpha_attempted = current_target;

                            let nr_outcome = NewtonRaphsonDriver.solve(
                                config.inner,
                                system,
                                solver,
                                accepted_iterate.clone(),
                            );

                            match nr_outcome {
                                Err(e) => {
                                    return Err(SourceSteppingError::Inner {
                                        alpha: current_target,
                                        source: e,
                                    });
                                }
                                Ok(NewtonRaphsonOutcome { iterate, status }) => {
                                    total_nr_iterations = total_nr_iterations
                                        .saturating_add(status.diagnostic().iterations);
                                    last_status = Some(status);
                                    if status.is_converged() {
                                        break Ok((iterate, status, current_target));
                                    }
                                    if halvings_used >= config.max_step_halvings {
                                        break Err(status);
                                    }
                                    halvings_used += 1;
                                    let midpoint = 0.5 * (accepted_alpha + current_target);
                                    if !(midpoint > accepted_alpha && midpoint < current_target) {
                                        break Err(status);
                                    }
                                    current_target = midpoint;
                                }
                            }
                        };
                        match inner {
                            Ok((iterate, _status, accepted_at)) => {
                                accepted_iterate = iterate;
                                accepted_alpha = accepted_at;
                                homotopy_steps = homotopy_steps.saturating_add(1);
                            }
                            Err(status) => {
                                // Homotopy gave up while chasing
                                // this scheduled target.
                                return Ok(SourceSteppingOutcome {
                                    iterate: accepted_iterate,
                                    status,
                                    homotopy_steps,
                                    total_nr_iterations,
                                    final_alpha: last_alpha_attempted,
                                });
                            }
                        }
                    }
                }
                Err(status) => {
                    // Halving budget exhausted on the very first
                    // attempt at this scheduled target.
                    return Ok(SourceSteppingOutcome {
                        iterate: accepted_iterate,
                        status,
                        homotopy_steps,
                        total_nr_iterations,
                        final_alpha: last_alpha_attempted,
                    });
                }
            }
        }

        // ─── Schedule exhausted ──────────────────────────────────────
        //
        // We reached the final scheduled α. By construction (see
        // `validate_schedule`) that α is 1.0. The last accepted
        // iterate is the solution; its status is `last_status`
        // (which must be `Converged` because we only exit the inner
        // loop on success when reaching a scheduled target).
        //
        // Defensive default: with a valid schedule (at least one
        // entry) and a non-empty system, `last_status` is always
        // set. If somehow not, synthesize a trivial `Converged`
        // diagnostic so the caller never sees `None`.
        let status = last_status.unwrap_or(ConvergenceStatus::Converged(ConvergenceDiagnostic {
            update_norm: 0.0,
            residue_norm: 0.0,
            iterations: 0,
            tolerances: config.inner.tolerances,
        }));

        Ok(SourceSteppingOutcome {
            iterate: accepted_iterate,
            status,
            homotopy_steps,
            total_nr_iterations,
            final_alpha: accepted_alpha,
        })
    }
}

/// Validate that a source-stepping schedule starts at `0.0`, ends
/// at `1.0`, is strictly ascending, and contains only finite values.
fn validate_schedule(schedule: &[f64]) -> Result<(), SourceSteppingError> {
    if schedule.is_empty() {
        return Err(SourceSteppingError::InvalidSchedule {
            reason: "schedule must contain at least the endpoints 0.0 and 1.0".to_string(),
        });
    }
    // Same exact-equality guard as for the endpoint at 1.0: the
    // schedule is configuration, not a computed quantity.
    #[allow(clippy::float_cmp)]
    let start_ok = schedule[0] == 0.0;
    if !start_ok {
        return Err(SourceSteppingError::InvalidSchedule {
            reason: format!("schedule must start at 0.0, got {}", schedule[0]),
        });
    }
    let last = *schedule.last().expect("non-empty checked above");
    // We require an *exact* endpoint at 1.0. Float-equality is
    // intentional here: the schedule is configuration, not a
    // computed quantity, and any user-supplied schedule that does
    // not literally end at 1.0 is malformed by contract.
    #[allow(clippy::float_cmp)]
    let end_ok = last == 1.0;
    if !end_ok {
        return Err(SourceSteppingError::InvalidSchedule {
            reason: format!("schedule must end at 1.0, got {last}"),
        });
    }
    for w in schedule.windows(2) {
        let (a, b) = (w[0], w[1]);
        if !a.is_finite() || !b.is_finite() {
            return Err(SourceSteppingError::InvalidSchedule {
                reason: "schedule contains non-finite values".to_string(),
            });
        }
        if b <= a {
            return Err(SourceSteppingError::InvalidSchedule {
                reason: format!("schedule must be strictly ascending; {a} >= {b}"),
            });
        }
    }
    // The strictly-ascending + start-0 + end-1 invariants imply
    // every entry sits in [0, 1].
    Ok(())
}

// ─── Internal note for the test module ─────────────────────────────
//
// Below are unit tests against synthetic `SourceSteppableSystem`
// implementations. They cover:
//
// 1. Linear system passes through trivially (NR converges at
//    every α with no warm-start drift).
// 2. A "hard-from-zero" system that diverges at α = 1 with a
//    zero initial guess but converges step-by-step from α = 0 →
//    1.0 via the homotopy chain.
// 3. Adaptive halving recovers when the schedule itself is too
//    coarse but a finer interpolation works.
// 4. Halving budget exhaustion surfaces as a non-Converged status.
// 5. Validation rejects malformed schedules.
// 6. Dim mismatch and inner NR hard errors are surfaced as
//    typed `SourceSteppingError` variants.
// 7. Empty system short-circuits.

#[cfg(test)]
#[allow(clippy::float_cmp)] // tests assert on configured-endpoint α values (0.0 / 1.0) which are exact by contract.
mod tests {
    use super::*;
    use crate::linear_solver::{RussellRealSolver, SparseLinearSystem, SparseTriplet};
    use crate::newton_raphson::SystemError;

    // ── Test helpers ────────────────────────────────────────────────

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

    /// A linear `A · x = α · b` source-steppable system. With NR
    /// the linear solve gives `x = α·b/A` directly; at every α the
    /// driver converges in two iterations (linear-convergence
    /// pattern from the NR test suite).
    struct LinearAlphaSystem {
        a: f64,
        rhs: f64,
        alpha: f64,
        set_alpha_calls: u32,
    }

    impl LinearAlphaSystem {
        fn new(a: f64, rhs: f64) -> Self {
            Self {
                a,
                rhs,
                alpha: 0.0,
                set_alpha_calls: 0,
            }
        }
    }

    impl NonlinearSystem for LinearAlphaSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            Ok(scalar_system(self.a, self.alpha * self.rhs))
        }
        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![self.a * iterate[0] - self.alpha * self.rhs])
        }
    }

    impl SourceSteppableSystem for LinearAlphaSystem {
        fn set_source_alpha(&mut self, alpha: f64) {
            self.alpha = alpha;
            self.set_alpha_calls += 1;
        }
    }

    /// A *cliff* nonlinear system: it converges from a warm start
    /// within distance `d_max` of the answer, and diverges (returns
    /// non-finite residue) otherwise. Specifically:
    ///
    /// At α the operating point is `α · target`. If the initial
    /// iterate sits within `d_max` of `α · target`, the next NR
    /// step lands exactly on `α · target` and the residue norm is
    /// zero. Otherwise the linearization deliberately produces a
    /// non-finite update so NR reports `Diverged`.
    ///
    /// This is the canonical "homotopy actually helps" pattern:
    /// from a zero initial guess at α = 1 the iterate is farther
    /// than `d_max` from `target`, NR diverges; walking α from 0
    /// keeps the iterate inside the basin at every step.
    struct CliffNonlinearSystem {
        target: f64,
        d_max: f64,
        alpha: f64,
    }

    impl CliffNonlinearSystem {
        fn new(target: f64, d_max: f64) -> Self {
            Self {
                target,
                d_max,
                alpha: 0.0,
            }
        }
    }

    impl NonlinearSystem for CliffNonlinearSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            let x = iterate[0];
            let goal = self.alpha * self.target;
            if (x - goal).abs() <= self.d_max {
                // Inside the basin: linearize so the solve lands on `goal`.
                // `1 · x_{k+1} = goal` → next iterate is `goal`.
                Ok(scalar_system(1.0, goal))
            } else {
                // Outside the basin: produce a divergent step.
                // `1 · x_{k+1} = NaN` causes the linear solver to
                // emit `NonFiniteEntry`, which NR collapses into
                // `Diverged`.
                Ok(scalar_system(1.0, f64::NAN))
            }
        }
        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            let x = iterate[0];
            let goal = self.alpha * self.target;
            Ok(vec![x - goal])
        }
    }

    impl SourceSteppableSystem for CliffNonlinearSystem {
        fn set_source_alpha(&mut self, alpha: f64) {
            self.alpha = alpha;
        }
    }

    /// A two-cliff system that the *coarse default schedule* fails
    /// on but adaptive halving rescues. The basin radius is set so
    /// that a step of 0.1 in α from a converged iterate at one α
    /// lands *outside* the next α's basin, but a step of 0.05 lands
    /// inside.
    ///
    /// Concretely: `target = 100.0`, basin half-width `d_max = 6.0`.
    /// Iterate at α=0.5 is `50.0`. At α=0.6 the goal is `60.0`;
    /// `|50.0 - 60.0| = 10 > d_max = 6`. The midpoint α=0.55 has
    /// goal `55.0`; `|50 - 55| = 5 ≤ d_max`. So one halving rescues.
    struct CoarseCliffSystem {
        target: f64,
        d_max: f64,
        alpha: f64,
    }

    impl CoarseCliffSystem {
        fn new(target: f64, d_max: f64) -> Self {
            Self {
                target,
                d_max,
                alpha: 0.0,
            }
        }
    }

    impl NonlinearSystem for CoarseCliffSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            let x = iterate[0];
            let goal = self.alpha * self.target;
            if (x - goal).abs() <= self.d_max {
                Ok(scalar_system(1.0, goal))
            } else {
                Ok(scalar_system(1.0, f64::NAN))
            }
        }
        fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            let x = iterate[0];
            let goal = self.alpha * self.target;
            Ok(vec![x - goal])
        }
    }

    impl SourceSteppableSystem for CoarseCliffSystem {
        fn set_source_alpha(&mut self, alpha: f64) {
            self.alpha = alpha;
        }
    }

    /// A system whose linearize errors at a configured α threshold.
    struct ErrorAtAlphaSystem {
        threshold: f64,
        alpha: f64,
    }

    impl NonlinearSystem for ErrorAtAlphaSystem {
        fn dim(&self) -> u32 {
            1
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            if self.alpha >= self.threshold {
                Err(SystemError::new("modeling failure"))
            } else {
                Ok(scalar_system(1.0, 0.0))
            }
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![0.0])
        }
    }

    impl SourceSteppableSystem for ErrorAtAlphaSystem {
        fn set_source_alpha(&mut self, alpha: f64) {
            self.alpha = alpha;
        }
    }

    /// Empty (`dim == 0`) source-steppable system.
    struct EmptySystem;

    impl NonlinearSystem for EmptySystem {
        fn dim(&self) -> u32 {
            0
        }
        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            SparseLinearSystem::new(0, 0, 0, vec![], vec![])
                .map_err(|e| SystemError::new(format!("{e}")))
        }
        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            Ok(vec![])
        }
    }

    impl SourceSteppableSystem for EmptySystem {
        fn set_source_alpha(&mut self, _alpha: f64) {}
    }

    // ── Tests ───────────────────────────────────────────────────────

    #[test]
    fn default_config_is_eleven_point_uniform_ramp_with_ten_halvings() {
        let c = SourceSteppingConfig::dc_defaults();
        assert_eq!(c.schedule.len(), 11);
        assert_eq!(c.schedule.first().copied(), Some(0.0));
        assert_eq!(c.schedule.last().copied(), Some(1.0));
        assert_eq!(c.max_step_halvings, 10);
        // Default-config and dc_defaults agree.
        assert_eq!(c, SourceSteppingConfig::default());
    }

    #[test]
    fn linear_system_converges_at_every_scheduled_alpha() {
        // `5 · x = α · 10` → x = 2·α. Final solution at α=1 is 2.0.
        let mut sys = LinearAlphaSystem::new(5.0, 10.0);
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(outcome.status.is_converged(), "{:?}", outcome.status);
        assert!((outcome.iterate[0] - 2.0).abs() < 1e-9);
        // 11 scheduled α values, every one accepted on the first try.
        assert_eq!(outcome.homotopy_steps, 11);
        assert_eq!(outcome.final_alpha, 1.0);
        // set_source_alpha called once per scheduled α (no halvings).
        assert_eq!(sys.set_alpha_calls, 11);
        // NR iteration count: at α=0 the linearized RHS is 0 and
        // the initial iterate is already 0, so the dual criterion
        // is satisfied in *one* iteration (update zero, residue
        // zero on the very first solve). Each subsequent α needs
        // *two* iterations (the canonical linear-system pattern:
        // one solve to land on the new operating point, one to
        // confirm the update has stabilized). 1 + 10·2 = 21.
        assert_eq!(outcome.total_nr_iterations, 21);
    }

    #[test]
    fn cliff_system_converges_via_warm_start_chain() {
        // basin half-width 0.2; target 1.0. From x=0.0 at α=1 the
        // distance is 1.0 — outside the basin. But the schedule
        // step is 0.1, so each step the goal moves by 0.1, which
        // fits inside the 0.2 basin.
        let mut sys = CliffNonlinearSystem::new(1.0, 0.2);
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(outcome.status.is_converged(), "{:?}", outcome.status);
        assert!((outcome.iterate[0] - 1.0).abs() < 1e-9);
        assert_eq!(outcome.homotopy_steps, 11);
        assert_eq!(outcome.final_alpha, 1.0);
    }

    #[test]
    fn coarse_cliff_recovered_by_adaptive_halving() {
        // basin half-width 6.0, target 100.0. From x=0 at α=0 the
        // first scheduled step to α=0.1 lands at goal=10.0; |0-10|
        // = 10 > 6. One halving → α=0.05, goal=5.0; |0-5| = 5 ≤ 6.
        // Subsequent steps move the iterate by 10.0 in goal each
        // step (α step 0.1) which is outside the basin again →
        // every step needs one halving.
        let mut sys = CoarseCliffSystem::new(100.0, 6.0);
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(outcome.status.is_converged(), "{:?}", outcome.status);
        assert!((outcome.iterate[0] - 100.0).abs() < 1e-7);
        // 10 scheduled transitions, each requiring exactly one
        // halving → 20 accepted steps total (each scheduled target
        // and each midpoint counts as one step). Plus the very
        // first NR run at α=0 which we count as 1 accepted step.
        // Total: 1 + 2·10 = 21 accepted.
        assert_eq!(outcome.homotopy_steps, 21);
        assert_eq!(outcome.final_alpha, 1.0);
    }

    #[test]
    fn halving_budget_exhausted_reports_failure() {
        // basin half-width 0.0001 (essentially zero). No halving
        // budget will be enough. Status should be `Diverged` (from
        // the inner NR's NaN-update path).
        let mut sys = CliffNonlinearSystem::new(1.0, 0.0001);
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![0.0, 1.0],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 3,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(!outcome.status.is_converged());
        // We did succeed at α=0 → at least 1 step accepted.
        assert!(outcome.homotopy_steps >= 1);
        assert!(outcome.final_alpha < 1.0);
    }

    #[test]
    fn halving_disabled_fails_on_first_non_convergence() {
        // No halving allowed. CliffSystem with d_max = 0.5 fails at
        // the first α=0.1 step because |0 - 1.0·0.1| = 0.1 ≤ 0.5
        // — actually fits. Use d_max = 0.05 so step 0.1 escapes.
        // Then with max_step_halvings = 0, the driver must report
        // failure on the first inner failure.
        let mut sys = CliffNonlinearSystem::new(1.0, 0.05);
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![0.0, 0.1, 0.2, 1.0],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 0,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();

        assert!(!outcome.status.is_converged());
        // The driver successfully ran NR at α=0 and accepted that
        // step (the trivial solution 0.0 is inside the basin
        // around goal=0). It then attempts α=0.1 and fails with
        // no halving allowed.
        assert_eq!(outcome.homotopy_steps, 1);
        assert!((outcome.final_alpha - 0.1).abs() < 1e-12);
    }

    #[test]
    fn invalid_schedule_empty_is_rejected() {
        let mut sys = LinearAlphaSystem::new(1.0, 1.0);
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 0,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();
        assert!(matches!(err, SourceSteppingError::InvalidSchedule { .. }));
    }

    #[test]
    fn invalid_schedule_wrong_start_rejected() {
        let mut sys = LinearAlphaSystem::new(1.0, 1.0);
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![0.1, 1.0],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 0,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();
        assert!(matches!(err, SourceSteppingError::InvalidSchedule { .. }));
    }

    #[test]
    fn invalid_schedule_wrong_end_rejected() {
        let mut sys = LinearAlphaSystem::new(1.0, 1.0);
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![0.0, 0.5, 0.9],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 0,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();
        assert!(matches!(err, SourceSteppingError::InvalidSchedule { .. }));
    }

    #[test]
    fn invalid_schedule_not_strictly_ascending_rejected() {
        let mut sys = LinearAlphaSystem::new(1.0, 1.0);
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![0.0, 0.5, 0.5, 1.0],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 0,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();
        assert!(matches!(err, SourceSteppingError::InvalidSchedule { .. }));
    }

    #[test]
    fn invalid_schedule_non_finite_rejected() {
        let mut sys = LinearAlphaSystem::new(1.0, 1.0);
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig {
                    schedule: vec![0.0, f64::NAN, 1.0],
                    inner: NewtonRaphsonConfig::DC_DEFAULTS,
                    max_step_halvings: 0,
                },
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();
        assert!(matches!(err, SourceSteppingError::InvalidSchedule { .. }));
    }

    #[test]
    fn initial_iterate_dim_mismatch_rejected() {
        let mut sys = LinearAlphaSystem::new(1.0, 1.0);
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0, 0.0], // dim 2 but system is dim 1
            )
            .unwrap_err();
        match err {
            SourceSteppingError::InitialIterateDimMismatch {
                iterate_len,
                system_dim,
            } => {
                assert_eq!(iterate_len, 2);
                assert_eq!(system_dim, 1);
            }
            other => panic!("expected InitialIterateDimMismatch, got {other:?}"),
        }
    }

    #[test]
    fn inner_nr_hard_error_is_surfaced_with_alpha() {
        // Errors when α >= 0.3 — the third scheduled step.
        let mut sys = ErrorAtAlphaSystem {
            threshold: 0.3,
            alpha: 0.0,
        };
        let err = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap_err();
        match err {
            SourceSteppingError::Inner { alpha, source: _ } => {
                assert!(alpha >= 0.3 - 1e-12);
            }
            other => panic!("expected Inner, got {other:?}"),
        }
    }

    #[test]
    fn empty_system_short_circuits_to_converged() {
        let mut sys = EmptySystem;
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![],
            )
            .unwrap();
        assert!(outcome.status.is_converged());
        assert_eq!(outcome.iterate.len(), 0);
        assert_eq!(outcome.homotopy_steps, 0);
        assert_eq!(outcome.total_nr_iterations, 0);
        assert_eq!(outcome.final_alpha, 1.0);
    }

    #[test]
    fn final_alpha_is_one_on_success() {
        let mut sys = LinearAlphaSystem::new(2.0, 4.0);
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();
        assert!(outcome.status.is_converged());
        assert_eq!(outcome.final_alpha, 1.0);
        // Linear solution: 2·x = 1·4 → x = 2.
        assert!((outcome.iterate[0] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn ascending_alpha_passed_to_set_source_alpha_includes_zero_and_one() {
        // Capture every α value handed to set_source_alpha and
        // confirm 0.0 and 1.0 are both present in monotonic order.
        struct RecordingSystem {
            inner: LinearAlphaSystem,
            alphas: Vec<f64>,
        }
        impl NonlinearSystem for RecordingSystem {
            fn dim(&self) -> u32 {
                self.inner.dim()
            }
            fn linearize(
                &mut self,
                iterate: &[f64],
            ) -> Result<SparseLinearSystem<f64>, SystemError> {
                self.inner.linearize(iterate)
            }
            fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
                self.inner.residue(iterate)
            }
        }
        impl SourceSteppableSystem for RecordingSystem {
            fn set_source_alpha(&mut self, alpha: f64) {
                self.alphas.push(alpha);
                self.inner.set_source_alpha(alpha);
            }
        }

        let mut sys = RecordingSystem {
            inner: LinearAlphaSystem::new(5.0, 10.0),
            alphas: vec![],
        };
        let outcome = SourceSteppingDriver
            .solve(
                &SourceSteppingConfig::dc_defaults(),
                &mut sys,
                &RussellRealSolver,
                vec![0.0],
            )
            .unwrap();
        assert!(outcome.status.is_converged());
        assert_eq!(sys.alphas.first().copied(), Some(0.0));
        assert_eq!(sys.alphas.last().copied(), Some(1.0));
        // Strictly ascending (no adaptive halving on this linear system).
        for w in sys.alphas.windows(2) {
            assert!(
                w[0] < w[1],
                "alphas not strictly ascending: {:?}",
                sys.alphas
            );
        }
    }
}
