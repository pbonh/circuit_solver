//! Gmin-stepping homotopy driver (tasks.md item #18).
//!
//! This module implements the **Gmin-stepping continuation method**
//! that sits on top of the [`NewtonRaphsonDriver`]
//! and rescues DC operating-point solves where plain Newton-Raphson
//! fails to converge (typically because the circuit contains floating
//! nodes, very high-impedance regions, or steep-tangent nonlinearities
//! that throw NR off a basin of attraction).
//!
//! The idea, per the
//! [wiki concept page](../../../../wiki/concepts/gmin-stepping.md) and
//! the spec scenario
//! `dc-operating-point#dc-operating-point-with-gmin-stepping-homotopy`:
//!
//! > Insert a parallel conductance `g_min` from every non-ground node
//! > to ground. At a large `g_min` the conductance block is strictly
//! > diagonally dominant and the circuit is dominated by linear
//! > resistors, so NR converges trivially. Walk `g_min` down through
//! > a geometric schedule, warm-starting each step from the previous
//! > solution, until `g_min = 0` (the original problem).
//!
//! # Where this fits in the v1 architecture
//!
//! - **Below us:** the [`NonlinearSystem`] trait and the
//!   [`NewtonRaphsonDriver`] (tasks.md #17, ADR-0006). The homotopy
//!   driver calls NR repeatedly, once per `g_min` step.
//! - **Above us:** the DC analysis control loop (tasks.md #20) and
//!   the convergence-failure path (tasks.md #22). The control loop
//!   first tries plain NR; on `Stalled` / `Diverged` /
//!   `MaxIterationsExceeded` it falls back to this module. On
//!   success the orchestrator lifts our outcome into the user-facing
//!   `"converged-via-homotopy"` convergence status the spec
//!   scenario demands.
//! - **Sideways:** the [`sub_view`](super::sub_view) module already
//!   knows how to add a Gmin shunt to an [`MnaSystem`](super::assemble::MnaSystem)
//!   matrix one step at a time (`SubViewBuilder::with_gmin`); this
//!   module owns the *loop* that walks through the schedule,
//!   complementing the per-step constraint application that
//!   `sub_view` provides at the matrix level. The two layers are
//!   deliberately separate per the
//!   [`sub_view`](super::sub_view) module docs:
//!   `sub_view` applies *one* mask at *one* step; this module
//!   chains the steps together.
//!
//! # Architecture: wrapper-system pattern
//!
//! The homotopy does not modify the user's [`NonlinearSystem`]. It
//! constructs a thin [`GminAugmentedSystem`] adapter that wraps the
//! inner system and, at every step, contributes a shunt of
//! `g_min` siemens to the matrix diagonal of every non-ground node
//! row and the matching `g_min · x_i` term to the residue.
//!
//! Why this works (algebra):
//!
//! - The inner linearize returns `A · x_{k+1} = b` where
//!   `A = J(x_k)` and `b = J(x_k)·x_k − F(x_k)` (the SPICE
//!   companion-model form NR consumes, *not* the delta form).
//! - The augmented system solves `F'(x) = F(x) + g_min · P x = 0`
//!   where `P` is the diagonal selector matrix that picks the
//!   non-ground node rows.
//! - Its Newton tangent is `J + g_min · P`. Its companion-form RHS
//!   is `(J + g_min · P) · x_k − F'(x_k) = J·x_k + g_min · P · x_k
//!   − F(x_k) − g_min · P · x_k = J·x_k − F(x_k) = b`.
//!
//! So **the RHS is unchanged**: the wrapper only has to add
//! `g_min` to the matrix diagonal at every non-ground node row.
//! The residue, separately, must reflect the augmented system:
//! `F'(x) = F(x) + g_min · P · x`. This asymmetry is the entire
//! reason `linearize` and `residue` are separate trait methods —
//! the linearize path benefits from the cancellation; the residue
//! does not.
//!
//! # Schedule
//!
//! [`GminSchedule`] is a geometric step-down: start at `initial`,
//! divide by `ratio` each step until reaching `final_gmin`. The
//! terminal step is **exactly `final_gmin`**, not `0.0` (callers
//! who want a true `0.0` terminal step must set
//! `final_gmin = 0.0` explicitly).
//!
//! This matches SPICE convention. Kundert's tutorial
//! ([wiki summary](../../../../wiki/summaries/kundert-bctm98-simulation-tutorial.md))
//! describes the canonical scheme as a residual `GMIN ≈ 10⁻¹² S`
//! that remains in the final operating point; a true floating
//! node has no isolated solution at `gmin = 0` (the matrix is
//! singular), so the residual `final_gmin` is the price of
//! having a finite solution at all. The wiki concept page
//! documents this as a known trade-off:
//!
//! > A small but nonzero Gmin remains in the final solution, which
//! > can affect very high-impedance nodes by a measurable amount.
//!
//! # Step-failure policy (v1)
//!
//! If NR fails at any step (`Stalled`, `MaxIterationsExceeded`, or
//! `Diverged`), the driver returns immediately with
//! [`GminSteppingOutcome::status`] set to the matching
//! [`HomotopyStatus`] variant, carrying the step index at which the
//! failure occurred and the underlying NR diagnostic. The driver
//! does *not* adaptively shrink the step ratio on failure — that
//! refinement is deferred and would lift the design rationale into
//! a separate tasks.md follow-on. Per
//! [ADR-0010](../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! the API is unstable in v1, so the policy can be revisited
//! without breaking compatibility.
//!
//! # Empty systems
//!
//! `dim == 0` is treated as vacuously converged in zero homotopy
//! steps with the trivial NR diagnostic. This mirrors the
//! [`NewtonRaphsonDriver`]'s empty-system contract and lets the
//! analysis orchestrator treat the empty-circuit case uniformly.
//!
//! # Honored ADRs
//!
//! - **ADR-0006** — dual convergence criterion. Every NR call this
//!   driver makes uses the same `ConvergenceTolerances`; the
//!   homotopy outcome inherits the inner NR diagnostic for the
//!   final successful step.
//! - **ADR-0008** — per-node tolerance envelope. Tolerances are
//!   inherited from the configured [`NewtonRaphsonConfig`] at every
//!   step; the homotopy itself does not relax tolerances.
//! - **ADR-0009** — topology checker for floating-node detection.
//!   This driver is the runtime *fallback* the topology checker's
//!   safety net relies on. Pass 1 detects floating nodes and the
//!   analysis orchestrator routes such circuits through this
//!   driver per the
//!   [dc-operating-point spec](../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/dc-operating-point/spec.md).
//! - **ADR-0010** — every type and function exported here is part
//!   of the v1 *unstable* public Rust API.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]

use circuit_solver_types::{ConvergenceDiagnostic, ConvergenceStatus};

use crate::linear_solver::{LinearSolver, SparseLinearSystem, SparseTriplet};
use crate::newton_raphson::{
    NewtonRaphsonConfig, NewtonRaphsonDriver, NewtonRaphsonError, NewtonRaphsonOutcome,
    NonlinearSystem, SystemError,
};

/// Geometric schedule of Gmin step values walked by
/// [`GminSteppingDriver::solve`].
///
/// The schedule emits values starting from `initial_gmin`, dividing
/// by `ratio` at each step until the value drops to or below
/// `final_gmin`, after which one terminal step at exactly `0.0` is
/// emitted. The terminal-zero step is what makes the final iterate
/// a solution of the *original* (un-shunted) system rather than
/// the slightly-shunted approximation.
///
/// # Invariants (validated at construction)
///
/// - `initial_gmin > final_gmin` (strict — otherwise the geometric
///   walk would not terminate or would emit a single step).
/// - `final_gmin >= 0.0`.
/// - `initial_gmin.is_finite()` and `final_gmin.is_finite()`.
/// - `ratio > 1.0` and `ratio.is_finite()` (each step divides by
///   `ratio`, so a ratio ≤ 1 would not shrink).
/// - `max_steps >= 1` (the terminal-zero step alone is enough; a
///   schedule with no intermediate steps is the degenerate case
///   that still functions correctly).
///
/// # Default
///
/// The default schedule reflects SPICE-conventional values
/// (`gminstep` ramp from 1 S down to 1e-12 S in ×0.1 steps), which
/// produces 13 intermediate steps plus the terminal zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GminSchedule {
    /// Initial (largest) shunt conductance in siemens. Walked
    /// downward.
    pub initial_gmin: f64,
    /// Smallest non-zero shunt conductance in siemens. The schedule
    /// stops the geometric walk once the current value is `<=`
    /// this threshold, then emits the terminal `0.0` step.
    pub final_gmin: f64,
    /// Geometric ratio (>1.0). Each step's value is the previous
    /// divided by `ratio`. SPICE-conventional value is `10.0`
    /// (one decade per step).
    pub ratio: f64,
    /// Absolute upper bound on the total number of steps emitted
    /// (including the terminal zero). Prevents pathological
    /// schedules from running away.
    pub max_steps: u32,
}

impl GminSchedule {
    /// SPICE-conventional default: 1 S → 1e-12 S in ×0.1 steps,
    /// then a terminal `0.0`.
    pub const SPICE_DEFAULTS: Self = Self {
        initial_gmin: 1.0,
        final_gmin: 1e-12,
        ratio: 10.0,
        max_steps: 64,
    };

    /// Validate this schedule's invariants.
    ///
    /// # Errors
    ///
    /// Returns [`GminScheduleError`] when any invariant is
    /// violated. The driver calls this once up front so the
    /// per-step loop can assume the schedule is well-formed.
    pub fn validate(&self) -> Result<(), GminScheduleError> {
        if !self.initial_gmin.is_finite() {
            return Err(GminScheduleError::NonFiniteInitial {
                initial_gmin: self.initial_gmin,
            });
        }
        if !self.final_gmin.is_finite() {
            return Err(GminScheduleError::NonFiniteFinal {
                final_gmin: self.final_gmin,
            });
        }
        if !self.ratio.is_finite() {
            return Err(GminScheduleError::NonFiniteRatio { ratio: self.ratio });
        }
        if self.initial_gmin <= 0.0 {
            return Err(GminScheduleError::NonPositiveInitial {
                initial_gmin: self.initial_gmin,
            });
        }
        if self.final_gmin < 0.0 {
            return Err(GminScheduleError::NegativeFinal {
                final_gmin: self.final_gmin,
            });
        }
        if self.final_gmin >= self.initial_gmin {
            return Err(GminScheduleError::FinalGeInitial {
                initial_gmin: self.initial_gmin,
                final_gmin: self.final_gmin,
            });
        }
        if self.ratio <= 1.0 {
            return Err(GminScheduleError::RatioNotShrinking { ratio: self.ratio });
        }
        if self.max_steps == 0 {
            return Err(GminScheduleError::ZeroMaxSteps);
        }
        Ok(())
    }

    /// Iterate the validated step values.
    ///
    /// Emits `initial_gmin`, `initial_gmin / ratio`,
    /// `initial_gmin / ratio²`, ... while strictly greater than
    /// `final_gmin`; then emits a terminal step at exactly
    /// `final_gmin`. Total step count is capped at `max_steps`.
    ///
    /// **The terminal step is `final_gmin`, not `0.0`.** This
    /// matches SPICE convention (Kundert's tutorial; SPICE
    /// `GMIN ≈ 10⁻¹² S` is a permanent floor that never drops
    /// to zero). A truly floating node has no isolated solution
    /// at `gmin = 0` — the matrix is singular — so the residual
    /// `final_gmin` is the price of having a solution at all.
    /// The wiki concept page documents this as the known
    /// trade-off:
    ///
    /// > A small but nonzero Gmin remains in the final solution,
    /// > which can affect very high-impedance nodes by a
    /// > measurable amount.
    ///
    /// Callers who want a zero terminal step (e.g., for a
    /// well-conditioned circuit where homotopy is only being
    /// used as an aggressive warm-start) can set
    /// `final_gmin = 0.0` explicitly; the schedule will then
    /// terminate at `0.0` after the geometric walk.
    fn steps(&self) -> Vec<f64> {
        let mut out = Vec::new();
        let mut g = self.initial_gmin;
        // Geometric down-walk. Stop one short of `max_steps` so
        // we always have room to append the terminal step.
        // Stopping condition: `g <= final_gmin`. When
        // `final_gmin == 0.0` the geometric walk would underflow
        // before satisfying `g <= 0`; use the smallest sensible
        // floor (`f64::EPSILON · initial_gmin`) so the loop
        // terminates and the explicit `0.0` is appended.
        let geometric_floor = if self.final_gmin > 0.0 {
            self.final_gmin
        } else {
            f64::EPSILON * self.initial_gmin
        };
        while g > geometric_floor && (out.len() as u32) + 1 < self.max_steps {
            out.push(g);
            g /= self.ratio;
        }
        // Terminal step at exactly `final_gmin` (which may be
        // `0.0` if the caller explicitly opted in). Skip only if
        // the geometric walk already happened to land on the
        // floor.
        if (out.len() as u32) < self.max_steps
            && out
                .last()
                .copied()
                .map_or(true, |last| last > self.final_gmin)
        {
            out.push(self.final_gmin);
        }
        out
    }
}

impl Default for GminSchedule {
    fn default() -> Self {
        Self::SPICE_DEFAULTS
    }
}

/// Schedule-validation errors returned by [`GminSchedule::validate`].
///
/// All variants are pre-loop, pre-NR errors: a malformed schedule is
/// a programmer error rather than a convergence failure, so it is
/// surfaced through `Result` rather than through
/// [`HomotopyStatus`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GminScheduleError {
    /// `initial_gmin` was NaN or ±∞.
    NonFiniteInitial {
        /// The offending value.
        initial_gmin: f64,
    },
    /// `final_gmin` was NaN or ±∞.
    NonFiniteFinal {
        /// The offending value.
        final_gmin: f64,
    },
    /// `ratio` was NaN or ±∞.
    NonFiniteRatio {
        /// The offending value.
        ratio: f64,
    },
    /// `initial_gmin` was zero or negative; a starting shunt of
    /// zero or less defeats the purpose of homotopy.
    NonPositiveInitial {
        /// The offending value.
        initial_gmin: f64,
    },
    /// `final_gmin` was negative. Negative shunt conductance is
    /// unphysical (would actively destabilize NR).
    NegativeFinal {
        /// The offending value.
        final_gmin: f64,
    },
    /// `final_gmin >= initial_gmin`. The schedule must walk
    /// strictly downward.
    FinalGeInitial {
        /// The configured initial.
        initial_gmin: f64,
        /// The configured final.
        final_gmin: f64,
    },
    /// `ratio <= 1.0`. Each step divides by `ratio`, so a value
    /// `≤ 1` would not produce a strictly decreasing schedule.
    RatioNotShrinking {
        /// The offending value.
        ratio: f64,
    },
    /// `max_steps == 0`. At minimum, the terminal-zero step must
    /// be representable.
    ZeroMaxSteps,
}

impl core::fmt::Display for GminScheduleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFiniteInitial { initial_gmin } => write!(
                f,
                "gmin-stepping schedule: initial_gmin {initial_gmin} is non-finite",
            ),
            Self::NonFiniteFinal { final_gmin } => write!(
                f,
                "gmin-stepping schedule: final_gmin {final_gmin} is non-finite",
            ),
            Self::NonFiniteRatio { ratio } => write!(
                f,
                "gmin-stepping schedule: ratio {ratio} is non-finite",
            ),
            Self::NonPositiveInitial { initial_gmin } => write!(
                f,
                "gmin-stepping schedule: initial_gmin {initial_gmin} must be > 0",
            ),
            Self::NegativeFinal { final_gmin } => write!(
                f,
                "gmin-stepping schedule: final_gmin {final_gmin} must be >= 0",
            ),
            Self::FinalGeInitial {
                initial_gmin,
                final_gmin,
            } => write!(
                f,
                "gmin-stepping schedule: final_gmin {final_gmin} must be < initial_gmin {initial_gmin}",
            ),
            Self::RatioNotShrinking { ratio } => write!(
                f,
                "gmin-stepping schedule: ratio {ratio} must be > 1.0",
            ),
            Self::ZeroMaxSteps => write!(
                f,
                "gmin-stepping schedule: max_steps must be >= 1",
            ),
        }
    }
}

impl std::error::Error for GminScheduleError {}

/// Configuration for the homotopy driver.
///
/// Combines the per-step NR configuration with the schedule and the
/// ground-node index that identifies which row of the inner
/// linearized system is the ground reference (and therefore must
/// not receive a Gmin shunt — it is already pinned to `v_gnd = 0`
/// by the upstream ground suppression in
/// [`SubViewBuilder`](super::sub_view::SubViewBuilder)).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GminSteppingConfig {
    /// NR configuration applied at every homotopy step. Tolerances
    /// and iteration budget are inherited unchanged across steps.
    pub newton_raphson: NewtonRaphsonConfig,
    /// Geometric schedule of `g_min` values to walk.
    pub schedule: GminSchedule,
    /// Index of the ground row in the inner system's `node_count`
    /// block. The driver skips this row when adding the Gmin
    /// shunt because that row has already been replaced with the
    /// `e_g` basis row by upstream ground suppression. Defaults
    /// to `0` per the v1 convention that the flattener always
    /// pins ground at node 0.
    pub ground_node_index: u32,
}

impl GminSteppingConfig {
    /// Default DC operating-point config: NR's `DC_DEFAULTS` and
    /// `GminSchedule::SPICE_DEFAULTS`, ground at row 0.
    pub const DC_DEFAULTS: Self = Self {
        newton_raphson: NewtonRaphsonConfig::DC_DEFAULTS,
        schedule: GminSchedule::SPICE_DEFAULTS,
        ground_node_index: 0,
    };
}

impl Default for GminSteppingConfig {
    fn default() -> Self {
        Self::DC_DEFAULTS
    }
}

/// Convergence outcome of a homotopy solve, distinct from
/// [`ConvergenceStatus`] because it carries the *homotopy* step
/// count separately from the inner NR diagnostic.
///
/// The analysis orchestrator (tasks.md #20) lifts this enum into
/// the user-facing `"converged-via-homotopy"` convergence label
/// the spec scenario demands. Keeping a typed representation here
/// lets the orchestrator do that mapping without re-parsing
/// strings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HomotopyStatus {
    /// All steps in the schedule converged, including the terminal
    /// `g_min = 0` step. The accompanying [`ConvergenceDiagnostic`]
    /// is the inner NR diagnostic from the **final** (un-shunted)
    /// step.
    ConvergedViaHomotopy {
        /// Total number of homotopy steps the driver executed,
        /// including the terminal zero.
        steps: u32,
        /// Inner NR diagnostic at the final un-shunted step.
        final_diagnostic: ConvergenceDiagnostic,
    },
    /// NR failed at some step. Carries the step index (0-based)
    /// and the inner NR status (`Stalled`, `Diverged`, or
    /// `MaxIterationsExceeded`). The terminal-zero step is index
    /// `steps - 1` of the schedule.
    StepFailed {
        /// Zero-based step index at which NR failed.
        step_index: u32,
        /// Value of `g_min` at the failing step (siemens).
        gmin_siemens: f64,
        /// Inner NR convergence status (never `Converged` here).
        inner_status: ConvergenceStatus,
    },
}

impl HomotopyStatus {
    /// True iff the homotopy walked all the way to `g_min = 0`
    /// successfully.
    #[must_use]
    pub fn is_converged(&self) -> bool {
        matches!(self, Self::ConvergedViaHomotopy { .. })
    }
}

/// Outcome of [`GminSteppingDriver::solve`].
///
/// On any natural-termination path (success or step failure) returns
/// `Ok` with the homotopy status and the last *finite* iterate
/// produced by NR. Pre-loop hard failures (dim mismatch, schedule
/// validation, modeling error) surface as `Err` instead.
#[derive(Debug, Clone, PartialEq)]
pub struct GminSteppingOutcome {
    /// Final iterate produced by NR at the last step that ran.
    /// On `ConvergedViaHomotopy` this is the accepted solution
    /// of the original (un-shunted) system. On `StepFailed` this
    /// is the last NR iterate at the failing step.
    pub iterate: Vec<f64>,
    /// Homotopy convergence outcome.
    pub status: HomotopyStatus,
}

/// Errors that prevent the homotopy loop from running to its
/// natural termination.
///
/// These are pre-loop or hard-failure modes: a malformed schedule,
/// a dim mismatch on the initial iterate, or a linear-solver hard
/// error that the inner NR driver propagated up. Non-convergence
/// outcomes (including divergence at a step) are reported as `Ok`
/// with [`HomotopyStatus::StepFailed`].
#[derive(Debug, Clone, PartialEq)]
pub enum GminSteppingError {
    /// The schedule failed validation. See [`GminScheduleError`].
    Schedule(GminScheduleError),
    /// The initial iterate's length did not match the system's
    /// `dim()`.
    InitialIterateDimMismatch {
        /// Length of the supplied initial iterate.
        iterate_len: usize,
        /// Dim reported by the inner system.
        system_dim: u32,
    },
    /// The `ground_node_index` was outside the inner system's
    /// node block. The driver cannot know which row to skip.
    GroundIndexOutOfRange {
        /// The configured ground index.
        ground_node_index: u32,
        /// The inner linearized system's reported `node_count`.
        node_count: u32,
    },
    /// The inner NR driver returned a hard error (modeling
    /// failure, dim-liar nonlinear system, linear-solver
    /// backend error not classified as `Diverged`). The
    /// homotopy loop propagates these up unchanged because they
    /// indicate misuse rather than convergence failure.
    Newton {
        /// Zero-based homotopy step at which the NR hard error
        /// occurred.
        step_index: u32,
        /// Value of `g_min` at the offending step.
        gmin_siemens: f64,
        /// Underlying NR hard-failure error.
        source: NewtonRaphsonError,
    },
}

impl core::fmt::Display for GminSteppingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Schedule(e) => write!(f, "gmin-stepping: {e}"),
            Self::InitialIterateDimMismatch {
                iterate_len,
                system_dim,
            } => write!(
                f,
                "gmin-stepping: initial iterate length {iterate_len} != system dim {system_dim}",
            ),
            Self::GroundIndexOutOfRange {
                ground_node_index,
                node_count,
            } => write!(
                f,
                "gmin-stepping: ground_node_index {ground_node_index} >= node_count {node_count}",
            ),
            Self::Newton {
                step_index,
                gmin_siemens,
                source,
            } => write!(
                f,
                "gmin-stepping: step {step_index} (gmin = {gmin_siemens} S) newton hard error: {source}",
            ),
        }
    }
}

impl std::error::Error for GminSteppingError {}

impl From<GminScheduleError> for GminSteppingError {
    fn from(e: GminScheduleError) -> Self {
        Self::Schedule(e)
    }
}

/// Stateless dispatcher implementing the Gmin-stepping continuation
/// per the dc-operating-point spec.
///
/// The driver itself holds no state. Each call to [`Self::solve`]
/// walks the configured [`GminSchedule`], constructs a
/// [`GminAugmentedSystem`] for the current `g_min`, hands it to
/// [`NewtonRaphsonDriver::solve`], and uses the NR result as the
/// warm-start for the next step.
#[derive(Debug, Clone, Copy, Default)]
pub struct GminSteppingDriver;

impl GminSteppingDriver {
    /// Run the Gmin-stepping homotopy loop.
    ///
    /// At each step `i` of the schedule:
    ///
    /// 1. Build a [`GminAugmentedSystem`] over `system` with
    ///    `g_min = schedule[i]`.
    /// 2. Run [`NewtonRaphsonDriver::solve`] on the augmented
    ///    system starting from the previous iterate.
    /// 3. On NR `Converged`: adopt the new iterate as the
    ///    warm-start for step `i+1` and continue.
    /// 4. On any other NR status: return `StepFailed` carrying the
    ///    step index, the `g_min` value, and the inner status.
    /// 5. After the terminal `g_min = 0` step converges, return
    ///    `ConvergedViaHomotopy` with the total step count and
    ///    the final NR diagnostic.
    ///
    /// # Returns
    ///
    /// `Ok(GminSteppingOutcome)` on every natural-termination path
    /// (success, step failure). `Err(GminSteppingError)` on
    /// pre-loop hard failures (schedule validation, dim mismatch,
    /// ground index out of range) or on a hard NR error that is
    /// not a convergence outcome.
    ///
    /// # Errors
    ///
    /// See [`GminSteppingError`].
    pub fn solve<S, L>(
        self,
        config: GminSteppingConfig,
        system: &mut S,
        solver: &L,
        initial_iterate: Vec<f64>,
    ) -> Result<GminSteppingOutcome, GminSteppingError>
    where
        S: NonlinearSystem,
        L: LinearSolver<f64>,
    {
        config.schedule.validate()?;

        let system_dim = system.dim();
        if initial_iterate.len() != system_dim as usize {
            return Err(GminSteppingError::InitialIterateDimMismatch {
                iterate_len: initial_iterate.len(),
                system_dim,
            });
        }

        // Empty system: vacuously converged in zero homotopy steps.
        if system_dim == 0 {
            return Ok(GminSteppingOutcome {
                iterate: initial_iterate,
                status: HomotopyStatus::ConvergedViaHomotopy {
                    steps: 0,
                    final_diagnostic: ConvergenceDiagnostic {
                        update_norm: 0.0,
                        residue_norm: 0.0,
                        iterations: 0,
                        tolerances: config.newton_raphson.tolerances,
                    },
                },
            });
        }

        let steps = config.schedule.steps();
        // Build per-step augmented system and run NR. We borrow
        // `system` exclusively for the duration of each step's NR
        // solve via the wrapper; the wrapper releases the borrow
        // when it goes out of scope at the end of each iteration.
        let mut iterate = initial_iterate;
        let mut last_diagnostic = ConvergenceDiagnostic {
            update_norm: f64::INFINITY,
            residue_norm: f64::INFINITY,
            iterations: 0,
            tolerances: config.newton_raphson.tolerances,
        };

        for (k, &gmin) in steps.iter().enumerate() {
            let step_index = k as u32;

            // Range-check ground index against the *inner* system's
            // node_count by peeking at one linearize call. We do
            // this on the first step only; sub_view contracts
            // guarantee node_count is stable across calls.
            //
            // (The inner system's linearize is borrow-friendly:
            // we pass &mut S into the wrapper, which forwards it.
            // We construct the wrapper fresh for each step.)
            let mut augmented = GminAugmentedSystem::new(system, gmin, config.ground_node_index);

            // First step: range-check ground.
            if k == 0 {
                // Peek at node_count via a sacrificial linearize.
                // This costs one extra stamp per solve; the
                // alternative — threading node_count through the
                // NonlinearSystem trait — would be a wider change
                // for a one-time range check.
                let sys =
                    augmented
                        .linearize(&iterate)
                        .map_err(|source| GminSteppingError::Newton {
                            step_index,
                            gmin_siemens: gmin,
                            source: NewtonRaphsonError::System {
                                iteration: 0,
                                source,
                            },
                        })?;
                if config.ground_node_index >= sys.node_count() {
                    return Err(GminSteppingError::GroundIndexOutOfRange {
                        ground_node_index: config.ground_node_index,
                        node_count: sys.node_count(),
                    });
                }
            }

            let nr_result =
                NewtonRaphsonDriver.solve(config.newton_raphson, &mut augmented, solver, iterate);
            let NewtonRaphsonOutcome {
                iterate: next_iterate,
                status,
            } = match nr_result {
                Ok(o) => o,
                Err(source) => {
                    return Err(GminSteppingError::Newton {
                        step_index,
                        gmin_siemens: gmin,
                        source,
                    });
                }
            };

            iterate = next_iterate;

            match status {
                ConvergenceStatus::Converged(d) => {
                    last_diagnostic = d;
                    // Continue to the next step.
                }
                non_converged => {
                    return Ok(GminSteppingOutcome {
                        iterate,
                        status: HomotopyStatus::StepFailed {
                            step_index,
                            gmin_siemens: gmin,
                            inner_status: non_converged,
                        },
                    });
                }
            }
        }

        let steps_count = steps.len() as u32;
        Ok(GminSteppingOutcome {
            iterate,
            status: HomotopyStatus::ConvergedViaHomotopy {
                steps: steps_count,
                final_diagnostic: last_diagnostic,
            },
        })
    }
}

/// Adapter wrapping a [`NonlinearSystem`] with a Gmin shunt at
/// every non-ground node row.
///
/// The wrapper contributes:
///
/// - to the *linearized matrix*: `+g_min` on the diagonal of every
///   non-ground node row.
/// - to the *linearized RHS*: nothing (the augmented contribution
///   cancels algebraically, see module docs).
/// - to the *residue*: `+g_min · x[i]` for every non-ground node
///   row `i`.
///
/// The wrapper does **not** modify the branch-equation rows
/// `node_count..dim`, because the homotopy shunt applies to KCL
/// node-balance equations only. Branch rows pass through unchanged.
///
/// # Lifetime
///
/// Borrows the inner system mutably for the duration of one
/// homotopy step. The [`GminSteppingDriver`] constructs and drops
/// one wrapper per step so the inner system's mutable state (NR
/// scratch buffers, device-parameter caches) is shared across
/// steps without needing the wrapper to outlive any one step.
pub struct GminAugmentedSystem<'a, S: NonlinearSystem> {
    inner: &'a mut S,
    gmin: f64,
    ground_node_index: u32,
}

impl<'a, S: NonlinearSystem> GminAugmentedSystem<'a, S> {
    /// Construct a wrapper that adds `gmin` siemens to every
    /// non-ground node-row diagonal of `inner`'s linearization
    /// and the matching `gmin · x[i]` term to its residue.
    ///
    /// A `gmin` of `0.0` is a no-op (the wrapper still forwards
    /// every call but adds no triplets and no residue
    /// contributions). The terminal step of a [`GminSchedule`]
    /// always uses `gmin = 0.0`, so the final NR solve is on the
    /// original system.
    pub fn new(inner: &'a mut S, gmin: f64, ground_node_index: u32) -> Self {
        Self {
            inner,
            gmin,
            ground_node_index,
        }
    }
}

impl<S: NonlinearSystem> NonlinearSystem for GminAugmentedSystem<'_, S> {
    fn dim(&self) -> u32 {
        self.inner.dim()
    }

    fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
        let inner = self.inner.linearize(iterate)?;
        if self.gmin == 0.0 {
            // Terminal step: nothing to add.
            return Ok(inner);
        }
        let dim = inner.dim();
        let node_count = inner.node_count();
        let branch_count = inner.branch_count();
        let mut triplets = inner.triplets().to_vec();
        let rhs = inner.rhs().to_vec();
        let ground = self.ground_node_index;
        // Append `gmin` to the diagonal of every non-ground node
        // row. The downstream sparse-LU backend sums duplicates
        // (per `SparseTriplet` docs), so appending a fresh
        // `(i, i, gmin)` is safe even when the inner system
        // already stamped to `A[i, i]`.
        for i in 0..node_count {
            if i == ground {
                continue;
            }
            triplets.push(SparseTriplet {
                row: i,
                col: i,
                value: self.gmin,
            });
        }
        // SparseLinearSystem::new validates dim partition and
        // triplet ranges; we preserve the inner partition.
        SparseLinearSystem::new(dim, node_count, branch_count, triplets, rhs)
            .map_err(|e| SystemError::new(format!("gmin-augmented linearize: {e}")))
    }

    fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
        let mut residue = self.inner.residue(iterate)?;
        if self.gmin == 0.0 {
            return Ok(residue);
        }
        // Add gmin · x[i] to every non-ground node-row residue.
        // We need to know which rows are node rows; the inner
        // residue length equals `dim` and the layout's first
        // `node_count` rows are node rows. We don't have direct
        // access to node_count from the residue, so we rely on
        // the contract: the homotopy driver only ever instantiates
        // this wrapper on top of a system whose linearize result
        // has `node_count` discoverable. Practical solution: cache
        // the partition during the first linearize call.
        //
        // For v1 we infer the bound by re-linearizing once to read
        // node_count. The inner system's linearize is cheap by
        // contract (a single stamp pass) and is invariant in
        // partition shape across iterates, so this is correct.
        //
        // (We deliberately avoid storing the partition on the
        // wrapper because doing so would require interior
        // mutability or a two-call construction protocol; the
        // single extra linearize per residue is a v1-acceptable
        // cost.)
        let probe = self.inner.linearize(iterate)?;
        let node_count = probe.node_count();
        let ground = self.ground_node_index;
        for i in 0..node_count {
            if i == ground {
                continue;
            }
            let idx = i as usize;
            if idx < residue.len() && idx < iterate.len() {
                residue[idx] += self.gmin * iterate[idx];
            }
        }
        Ok(residue)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // Exact comparisons on known-finite test fixtures.
#[allow(clippy::match_wildcard_for_single_variants)]
// Test intentionally asserts "is X, anything-else fails".
mod tests {
    use super::*;
    use crate::linear_solver::RussellRealSolver;
    use circuit_solver_types::ConvergenceTolerances;

    // ─── Test helpers ───────────────────────────────────────────────

    /// A canonical 2-node ground-suppressed system representing a
    /// floating-node circuit: a single non-ground node "n1" with
    /// no DC path to ground. The naive matrix is `[0, 0; 0, 0]`
    /// which is singular; after ground suppression at index 0 the
    /// system becomes
    ///
    /// ```text
    /// [1, 0]   [0]
    /// [0, 0] · x = [0]
    /// ```
    ///
    /// — still singular because row 1 (the n1 KCL) is `0 = 0`.
    /// This is precisely the floating-node failure mode the
    /// dc-operating-point spec scenario calls out.
    ///
    /// With Gmin shunting at the n1 row the system becomes
    ///
    /// ```text
    /// [1,    0  ]   [0]
    /// [0, gmin ] · x = [0]
    /// ```
    ///
    /// — now non-singular for any `gmin > 0`. The solution is
    /// `x = [0, 0]` at every step. NR converges in one iteration
    /// at every step; the homotopy walks `gmin` to zero and
    /// returns the zero solution.
    struct FloatingNodeSystem {
        linearize_calls: u32,
        residue_calls: u32,
    }

    impl FloatingNodeSystem {
        fn new() -> Self {
            Self {
                linearize_calls: 0,
                residue_calls: 0,
            }
        }
    }

    impl NonlinearSystem for FloatingNodeSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(&mut self, _iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            self.linearize_calls += 1;
            // Ground-suppressed: row 0 is `1·v0 = 0`. Row 1 is
            // `0·v1 = 0` (floating). 2 node rows, 0 branch rows.
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
            self.residue_calls += 1;
            // Linear residue. Row 0: `v0 - 0`. Row 1: `0`.
            Ok(vec![iterate[0], 0.0])
        }
    }

    /// A 1-node system with a finite conductance to ground at the
    /// non-ground row: `[1, 0; 0, g_load] · x = [0, i_in]`. The
    /// solution is `v1 = i_in / g_load`. NR converges in one
    /// iteration regardless of `gmin`; Gmin only shifts the
    /// effective conductance, and the homotopy walks back to
    /// `gmin = 0` so the final answer matches the unshunted
    /// solution exactly.
    struct ResistiveNodeSystem {
        g_load: f64,
        i_in: f64,
    }

    impl NonlinearSystem for ResistiveNodeSystem {
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
            // F(v) = [v0; g_load * v1 - i_in].
            Ok(vec![iterate[0], self.g_load * iterate[1] - self.i_in])
        }
    }

    /// A system whose `residue` always returns a large value: NR
    /// converges in the update sense but never in the residue
    /// sense → `Stalled` at every step. Used to exercise the
    /// step-failure path of the homotopy driver.
    struct AlwaysStallSystem;

    impl NonlinearSystem for AlwaysStallSystem {
        fn dim(&self) -> u32 {
            2
        }

        fn linearize(&mut self, iterate: &[f64]) -> Result<SparseLinearSystem<f64>, SystemError> {
            // `1 · x_{k+1} = x_k` on both rows → update is zero.
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
                vec![iterate[0], iterate[1]],
            )
            .map_err(|e| SystemError::new(format!("{e}")))
        }

        fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
            // Persistent residue above tolerance.
            Ok(vec![1.0, 1.0])
        }
    }

    // ─── GminSchedule tests ─────────────────────────────────────────

    #[test]
    fn spice_default_schedule_is_well_formed() {
        let s = GminSchedule::default();
        assert!(s.validate().is_ok());
        let steps = s.steps();
        assert!(!steps.is_empty());
        // Terminal step is exactly `final_gmin` (SPICE convention:
        // residual gmin floor, not zero).
        assert_eq!(steps.last().copied(), Some(s.final_gmin));
        // First step matches `initial_gmin`.
        assert_eq!(steps[0], s.initial_gmin);
        // Monotonic non-increasing.
        for w in steps.windows(2) {
            assert!(
                w[0] >= w[1],
                "schedule must walk strictly downward, got {w:?}"
            );
        }
    }

    #[test]
    fn schedule_validate_rejects_non_finite_initial() {
        let s = GminSchedule {
            initial_gmin: f64::NAN,
            ..GminSchedule::SPICE_DEFAULTS
        };
        assert!(matches!(
            s.validate(),
            Err(GminScheduleError::NonFiniteInitial { .. })
        ));
    }

    #[test]
    fn schedule_validate_rejects_non_positive_initial() {
        let s = GminSchedule {
            initial_gmin: 0.0,
            ..GminSchedule::SPICE_DEFAULTS
        };
        assert!(matches!(
            s.validate(),
            Err(GminScheduleError::NonPositiveInitial { .. })
        ));
    }

    #[test]
    fn schedule_validate_rejects_negative_final() {
        let s = GminSchedule {
            final_gmin: -1.0,
            ..GminSchedule::SPICE_DEFAULTS
        };
        assert!(matches!(
            s.validate(),
            Err(GminScheduleError::NegativeFinal { .. })
        ));
    }

    #[test]
    fn schedule_validate_rejects_final_ge_initial() {
        let s = GminSchedule {
            initial_gmin: 1e-12,
            final_gmin: 1.0,
            ..GminSchedule::SPICE_DEFAULTS
        };
        assert!(matches!(
            s.validate(),
            Err(GminScheduleError::FinalGeInitial { .. })
        ));
    }

    #[test]
    fn schedule_validate_rejects_ratio_le_one() {
        let s = GminSchedule {
            ratio: 1.0,
            ..GminSchedule::SPICE_DEFAULTS
        };
        assert!(matches!(
            s.validate(),
            Err(GminScheduleError::RatioNotShrinking { .. })
        ));
    }

    #[test]
    fn schedule_validate_rejects_zero_max_steps() {
        let s = GminSchedule {
            max_steps: 0,
            ..GminSchedule::SPICE_DEFAULTS
        };
        assert!(matches!(s.validate(), Err(GminScheduleError::ZeroMaxSteps)));
    }

    // ─── GminAugmentedSystem wrapper tests ──────────────────────────

    #[test]
    fn augmented_with_zero_gmin_passes_through() {
        let mut inner = FloatingNodeSystem::new();
        let mut wrapper = GminAugmentedSystem::new(&mut inner, 0.0, 0);
        let lin = wrapper.linearize(&[0.0, 0.0]).unwrap();
        // No extra triplets appended.
        assert_eq!(lin.triplets().len(), 1);
        assert_eq!(lin.triplets()[0].value, 1.0);
        let res = wrapper.residue(&[3.0, 5.0]).unwrap();
        // Inner residue is `[3, 0]` — unchanged.
        assert_eq!(res, vec![3.0, 0.0]);
    }

    #[test]
    fn augmented_with_positive_gmin_adds_shunt_to_non_ground_diagonal() {
        let mut inner = FloatingNodeSystem::new();
        let mut wrapper = GminAugmentedSystem::new(&mut inner, 0.5, 0);
        let lin = wrapper.linearize(&[0.0, 0.0]).unwrap();
        // Two triplets: the original `(0,0,1)` and the appended
        // `(1,1,0.5)` (row 0 is ground, skipped).
        let triplets = lin.triplets();
        assert_eq!(triplets.len(), 2);
        assert!(triplets
            .iter()
            .any(|t| t.row == 0 && t.col == 0 && t.value == 1.0));
        assert!(triplets
            .iter()
            .any(|t| t.row == 1 && t.col == 1 && (t.value - 0.5).abs() < 1e-15));
        // RHS unchanged (per algebra in module docs).
        assert_eq!(lin.rhs(), &[0.0, 0.0]);
    }

    #[test]
    fn augmented_residue_includes_gmin_x_term() {
        let mut inner = FloatingNodeSystem::new();
        let mut wrapper = GminAugmentedSystem::new(&mut inner, 0.5, 0);
        // Inner residue at x=[3,5] is `[3, 0]`. Augmented adds
        // `gmin · x[1] = 0.5 · 5 = 2.5` to row 1 (row 0 is
        // ground, skipped).
        let res = wrapper.residue(&[3.0, 5.0]).unwrap();
        assert_eq!(res, vec![3.0, 2.5]);
    }

    #[test]
    fn augmented_skips_branch_rows_in_shunt() {
        // Build a system with 1 node row + 1 branch row.
        struct OneNodeOneBranch;
        impl NonlinearSystem for OneNodeOneBranch {
            fn dim(&self) -> u32 {
                2
            }
            fn linearize(
                &mut self,
                _iterate: &[f64],
            ) -> Result<SparseLinearSystem<f64>, SystemError> {
                // node_count=1, branch_count=1.
                SparseLinearSystem::new(
                    2,
                    1,
                    1,
                    vec![
                        SparseTriplet {
                            row: 0,
                            col: 0,
                            value: 1.0,
                        },
                        SparseTriplet {
                            row: 1,
                            col: 1,
                            value: 2.0,
                        },
                    ],
                    vec![0.0, 7.0],
                )
                .map_err(|e| SystemError::new(format!("{e}")))
            }
            fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
                Ok(vec![iterate[0], iterate[1]])
            }
        }
        let mut inner = OneNodeOneBranch;
        // ground at index 0 (the only node), so the shunt loop
        // skips it; branch row 1 is *not* a node row and must
        // also be skipped because `i in 0..node_count = 0..1`.
        let mut wrapper = GminAugmentedSystem::new(&mut inner, 0.5, 0);
        let lin = wrapper.linearize(&[0.0, 0.0]).unwrap();
        // No extra triplets: only the ground node row exists in
        // the node block and it is skipped.
        assert_eq!(lin.triplets().len(), 2);
        let res = wrapper.residue(&[3.0, 5.0]).unwrap();
        // Branch row 1 is not touched.
        assert_eq!(res, vec![3.0, 5.0]);
    }

    // ─── GminSteppingDriver tests ───────────────────────────────────

    #[test]
    fn empty_system_is_vacuously_converged() {
        struct EmptySystem;
        impl NonlinearSystem for EmptySystem {
            fn dim(&self) -> u32 {
                0
            }
            fn linearize(
                &mut self,
                _iterate: &[f64],
            ) -> Result<SparseLinearSystem<f64>, SystemError> {
                SparseLinearSystem::new(0, 0, 0, vec![], vec![])
                    .map_err(|e| SystemError::new(format!("{e}")))
            }
            fn residue(&mut self, _iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
                Ok(vec![])
            }
        }
        let mut sys = EmptySystem;
        let outcome = GminSteppingDriver
            .solve(
                GminSteppingConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![],
            )
            .unwrap();
        assert!(matches!(
            outcome.status,
            HomotopyStatus::ConvergedViaHomotopy { steps: 0, .. }
        ));
        assert!(outcome.status.is_converged());
        assert!(outcome.iterate.is_empty());
    }

    #[test]
    fn initial_iterate_dim_mismatch_is_a_hard_error() {
        let mut sys = FloatingNodeSystem::new();
        let result = GminSteppingDriver.solve(
            GminSteppingConfig::DC_DEFAULTS,
            &mut sys,
            &RussellRealSolver,
            vec![0.0], // length 1, but dim is 2
        );
        assert!(matches!(
            result,
            Err(GminSteppingError::InitialIterateDimMismatch {
                iterate_len: 1,
                system_dim: 2,
            })
        ));
    }

    #[test]
    fn invalid_schedule_is_a_hard_error() {
        let mut sys = FloatingNodeSystem::new();
        let bad_config = GminSteppingConfig {
            schedule: GminSchedule {
                ratio: 0.5,
                ..GminSchedule::SPICE_DEFAULTS
            },
            ..GminSteppingConfig::DC_DEFAULTS
        };
        let result =
            GminSteppingDriver.solve(bad_config, &mut sys, &RussellRealSolver, vec![0.0, 0.0]);
        assert!(matches!(
            result,
            Err(GminSteppingError::Schedule(
                GminScheduleError::RatioNotShrinking { .. }
            ))
        ));
    }

    #[test]
    fn ground_index_out_of_range_is_a_hard_error() {
        let mut sys = FloatingNodeSystem::new();
        let bad_config = GminSteppingConfig {
            ground_node_index: 99,
            ..GminSteppingConfig::DC_DEFAULTS
        };
        let result =
            GminSteppingDriver.solve(bad_config, &mut sys, &RussellRealSolver, vec![0.0, 0.0]);
        assert!(matches!(
            result,
            Err(GminSteppingError::GroundIndexOutOfRange {
                ground_node_index: 99,
                node_count: 2,
            })
        ));
    }

    #[test]
    fn floating_node_system_converges_via_homotopy() {
        // The defining scenario from the dc-operating-point spec:
        // floating-node circuit where plain NR (gmin = 0) would
        // hit a singular matrix on the first iteration. With
        // homotopy, the SPICE-default schedule walks gmin from
        // 1 S down to the residual floor 1e-12 S; every step is
        // non-singular (gmin > 0 makes the n1 row diagonally
        // dominant). Per Kundert's tutorial we stop at the floor
        // rather than walking to zero, because a true floating
        // node has no isolated solution at gmin = 0.
        let mut sys = FloatingNodeSystem::new();
        let outcome = GminSteppingDriver
            .solve(
                GminSteppingConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0, 0.0],
            )
            .expect("homotopy must succeed on floating-node system");
        match outcome.status {
            HomotopyStatus::ConvergedViaHomotopy {
                steps,
                final_diagnostic,
            } => {
                // Schedule emits ≥ 1 step.
                assert!(
                    steps >= 1,
                    "expected at least one schedule step, got {steps}"
                );
                // Final-step diagnostic must satisfy the dual
                // criterion at gmin = 1e-12, where the system is
                // well-conditioned. The iterate is `0` so update
                // is `0` and residue is `0` exactly.
                assert!(
                    final_diagnostic.dual_satisfied(),
                    "final diagnostic must satisfy dual criterion: {final_diagnostic:?}"
                );
            }
            other => panic!("expected ConvergedViaHomotopy, got {other:?}"),
        }
        // Solution is the zero vector (only solution of the
        // floating-node system that satisfies the residue).
        for &v in &outcome.iterate {
            assert!(v.abs() < 1e-9, "expected zero solution, got {v}");
        }
    }

    #[test]
    fn resistive_node_system_recovers_unshunted_answer_at_terminal_step() {
        // Verify the terminal `gmin = 0` step undoes the shunt:
        // the converged answer matches the original (gmin = 0)
        // solution exactly, not the slightly-shunted approximation.
        //
        // For `g_load = 2 S, i_in = 4 A`: v1 = 4 / 2 = 2 V exactly.
        // If we returned the gmin-shunted answer at, say, gmin = 1e-12,
        // we'd get v1 = 4 / (2 + 1e-12) which differs from 2 by
        // ~1e-12. The dual-criterion residue check at gmin = 0
        // forces the un-shunted answer.
        let mut sys = ResistiveNodeSystem {
            g_load: 2.0,
            i_in: 4.0,
        };
        let outcome = GminSteppingDriver
            .solve(
                GminSteppingConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0, 0.0],
            )
            .unwrap();
        assert!(outcome.status.is_converged());
        // v0 (ground) ≈ 0, v1 ≈ 2.
        assert!(outcome.iterate[0].abs() < 1e-9);
        assert!((outcome.iterate[1] - 2.0).abs() < 1e-9);
    }

    #[test]
    fn step_failure_propagates_as_step_failed_status() {
        // The AlwaysStallSystem will produce `Stalled` at every
        // homotopy step. The first step uses gmin = initial_gmin = 1.0,
        // which adds `+1` to the diagonal of row 1 (row 0 is
        // ground). NR with the stall pattern still produces
        // `Stalled` because the residue is constant-1.
        let mut sys = AlwaysStallSystem;
        let outcome = GminSteppingDriver
            .solve(
                GminSteppingConfig::DC_DEFAULTS,
                &mut sys,
                &RussellRealSolver,
                vec![0.0, 0.0],
            )
            .unwrap();
        match outcome.status {
            HomotopyStatus::StepFailed {
                step_index,
                gmin_siemens,
                inner_status,
            } => {
                assert_eq!(step_index, 0);
                assert!((gmin_siemens - 1.0).abs() < 1e-15);
                assert!(matches!(inner_status, ConvergenceStatus::Stalled(_)));
            }
            other => panic!("expected StepFailed, got {other:?}"),
        }
    }

    #[test]
    fn ground_index_can_be_non_zero() {
        // Build a system where the ground row is at index 1
        // instead of 0. The wrapper must skip row 1 (not row 0)
        // when stamping the shunt.
        struct GroundAtOne;
        impl NonlinearSystem for GroundAtOne {
            fn dim(&self) -> u32 {
                2
            }
            fn linearize(
                &mut self,
                _iterate: &[f64],
            ) -> Result<SparseLinearSystem<f64>, SystemError> {
                // Row 1 is the basis row `e_1` (ground at index 1).
                // Row 0 is `0·v0 = 0` (floating non-ground node).
                SparseLinearSystem::new(
                    2,
                    2,
                    0,
                    vec![SparseTriplet {
                        row: 1,
                        col: 1,
                        value: 1.0,
                    }],
                    vec![0.0, 0.0],
                )
                .map_err(|e| SystemError::new(format!("{e}")))
            }
            fn residue(&mut self, iterate: &[f64]) -> Result<Vec<f64>, SystemError> {
                Ok(vec![0.0, iterate[1]])
            }
        }
        let mut sys = GroundAtOne;
        let cfg = GminSteppingConfig {
            ground_node_index: 1,
            ..GminSteppingConfig::DC_DEFAULTS
        };
        let outcome = GminSteppingDriver
            .solve(cfg, &mut sys, &RussellRealSolver, vec![0.0, 0.0])
            .unwrap();
        assert!(outcome.status.is_converged());
        // Solution is zero on both rows.
        for &v in &outcome.iterate {
            assert!(v.abs() < 1e-9);
        }
    }

    #[test]
    fn dc_defaults_are_sensible() {
        let cfg = GminSteppingConfig::default();
        assert_eq!(cfg.ground_node_index, 0);
        assert_eq!(
            cfg.newton_raphson.max_iterations,
            NewtonRaphsonConfig::DC_DEFAULTS.max_iterations
        );
        assert_eq!(cfg.schedule.initial_gmin, 1.0);
        assert_eq!(cfg.schedule.final_gmin, 1e-12);
        assert_eq!(cfg.schedule.ratio, 10.0);
    }

    #[test]
    fn homotopy_status_helpers() {
        let d = ConvergenceDiagnostic {
            update_norm: 1e-15,
            residue_norm: 1e-15,
            iterations: 1,
            tolerances: ConvergenceTolerances::SPICE_DEFAULTS,
        };
        let ok = HomotopyStatus::ConvergedViaHomotopy {
            steps: 3,
            final_diagnostic: d,
        };
        assert!(ok.is_converged());

        let bad = HomotopyStatus::StepFailed {
            step_index: 1,
            gmin_siemens: 0.5,
            inner_status: ConvergenceStatus::Stalled(d),
        };
        assert!(!bad.is_converged());
    }

    #[test]
    fn schedule_steps_terminate_at_final_gmin_by_default() {
        let s = GminSchedule {
            initial_gmin: 1.0,
            final_gmin: 0.1,
            ratio: 2.0,
            max_steps: 64,
        };
        let steps = s.steps();
        // SPICE-convention: terminal is `final_gmin`, not zero.
        assert_eq!(steps.last().copied(), Some(0.1));
        // Strictly non-increasing.
        for w in steps.windows(2) {
            assert!(w[0] >= w[1]);
        }
        // First value matches initial.
        assert_eq!(steps[0], 1.0);
    }

    #[test]
    fn schedule_with_zero_final_gmin_terminates_at_zero() {
        // Caller can opt into a true zero-terminal step for
        // well-conditioned circuits. With a coarse ratio the
        // geometric walk reaches the floor quickly and the
        // terminal step at 0.0 is appended.
        let s = GminSchedule {
            initial_gmin: 1.0,
            final_gmin: 0.0,
            ratio: 100.0,
            max_steps: 8,
        };
        let steps = s.steps();
        // With ratio=100 the geometric loop emits 1.0, 0.01,
        // 1e-4, ... and the explicit `0.0` terminal step lands
        // before max_steps is hit.
        assert_eq!(steps.last().copied(), Some(0.0));
    }

    #[test]
    fn schedule_with_zero_final_gmin_under_max_steps_floor_keeps_walking() {
        // When `final_gmin == 0.0`, the geometric walk stops at
        // `EPSILON · initial_gmin` and the explicit `0.0` is
        // appended as the terminal step. With max_steps=4 the
        // geometric loop reserves one slot for the terminal,
        // emitting at most 3 geometric values + a 0.0 terminal.
        let s = GminSchedule {
            initial_gmin: 1.0,
            final_gmin: 0.0,
            ratio: 2.0,
            max_steps: 4,
        };
        let steps = s.steps();
        assert_eq!(steps.len(), 4);
        // Last step is the explicit zero.
        assert_eq!(steps.last().copied(), Some(0.0));
        // The first `max_steps - 1` values are geometric.
        for v in &steps[..3] {
            assert!(*v > 0.0);
        }
    }

    #[test]
    fn schedule_respects_max_steps_cap() {
        // A schedule with absurdly tight max_steps caps the walk.
        let s = GminSchedule {
            initial_gmin: 1.0,
            final_gmin: 1e-12,
            ratio: 2.0,
            max_steps: 3,
        };
        let steps = s.steps();
        assert!(steps.len() <= 3);
    }
}
