//! Linear-solver abstraction: sparse-direct LU dispatch for the
//! Numeric Solver Engine, per ADR-0002.
//!
//! This module covers `tasks.md` item #23 of
//! `circuit-solver/2026-05-21-v1-spec`: the **complex-valued** sparse
//! LU backend (`faer`), used by AC small-signal (and later, noise)
//! analyses to solve `(G + jωC) · V = I` at each frequency point.
//!
//! # Trait shape
//!
//! ADR-0002 commits to "abstract\[ing\] the two backends behind a single
//! `LinearSolver` trait or dispatch layer". `design.md` (§ ADR Treatment)
//! restates this as "a `LinearSolver` trait with `solve_real` and
//! `solve_complex` methods". The natural Rust expression of that
//! contract is a **scalar-generic trait**:
//!
//! ```text
//! pub trait LinearSolver<Scalar> { … }
//! ```
//!
//! with two concrete implementations:
//!
//! - [`FaerComplexSolver`] : `LinearSolver<Complex<f64>>` — this task
//!   (#23), implemented here.
//! - `RussellSolver`        : `LinearSolver<f64>`           — task #16
//!   (real-valued), lands in a separate change against the same
//!   trait surface.
//!
//! Routing by `Scalar` keeps each backend `match`-free and
//! monomorphized; the analysis orchestrator never names the concrete
//! solver — it just calls `<S as LinearSolver<Complex<f64>>>::solve`
//! with an `S = FaerComplexSolver` it received as a generic
//! parameter or trait-object adapter chosen by `AnalysisType`.
//!
//! # Inputs
//!
//! The trait operates on a [`SparseLinearSystem`] which is a
//! **sparse triplet bundle** plus a dense RHS vector. This is one
//! layer below [`crate::assemble::MnaSystem`] (which is dense `f64`,
//! per task #14's documented "no sparse representation" note). The
//! Pass-2 / sub-view boundary (task #15) is expected to lower
//! [`crate::assemble::MnaSystem`] into a [`SparseLinearSystem`] before
//! handing it to the solver; for the complex case, the AC sub-view
//! extractor (task #24) also augments with `jωC` contributions in the
//! process. That lowering lives outside this module; what we own here
//! is the **trait surface + the faer-backed complex implementation**.
//!
//! # Outputs
//!
//! On success, [`SolutionVector`] — a thin owned `Vec<Scalar>` wrapper
//! with `node_count` / `branch_count` book-keeping for callers that
//! want to split the unknowns back into node-voltage and branch-current
//! slices. On failure, [`LinearSolverError`], which distinguishes
//! upstream-data errors (non-finite entries, dimension mismatch) from
//! genuine-singularity errors and from backend-internal failures.
//!
//! # What this module does *not* do
//!
//! - **No assembly.** Triplets come in already stamped; building them
//!   is `assemble.rs`' job (real path) or the AC sub-view extractor's
//!   (complex path, task #24).
//! - **No homotopy / NR.** Linear LU only. The Newton-Raphson driver
//!   (#17) and the Gmin / source-stepping homotopies (#18, #19) sit
//!   above this module.
//! - **No real-valued solve.** Task #16 (russell) lives in a sibling
//!   `russell_real.rs` module that will be added in its own change.
//!
//! # Honored ADRs
//!
//! - **ADR-0002** — "`russell_sparse` for `f64`, `faer` for
//!   `Complex<f64>`". This module's [`FaerComplexSolver`] is the
//!   `faer` half.
//! - **ADR-0010** — All types and functions exported from this
//!   module are part of the v1 *unstable* public Rust API. The
//!   shape may change between v1.x.

#![allow(clippy::module_name_repetitions)]

mod faer_complex;
mod system;

pub use faer_complex::FaerComplexSolver;
pub use system::{LinearSolverError, SolutionVector, SparseLinearSystem, SparseTriplet};

use num_complex::Complex;

/// Sparse-direct linear solver abstraction.
///
/// `Scalar` is the matrix element type. The two in-flight
/// implementations of this trait dispatch by scalar:
///
/// | `Scalar`         | Backend           | Driving tasks.md item | Driving ADR  |
/// |------------------|-------------------|-----------------------|--------------|
/// | `f64`            | `russell_sparse`  | #16 (not yet landed)  | ADR-0002     |
/// | `Complex<f64>`   | `faer` (sparse)   | #23 (this task)       | ADR-0002     |
///
/// Implementors are stateless solver "dispatchers" — typically
/// zero-sized types. State (factorization cache, symbolic ordering)
/// lives inside the backend's own LU object, created per-solve. The
/// trait does **not** expose a "factor once, solve many" surface;
/// AC's per-frequency loop creates a new LU per call because the
/// complex augmentation `jωC` changes with `ω` (see `wiki/decisions/0002…`
/// "no shared symbolic analysis or factorization cache across
/// real/complex boundaries" — and the same point applies *within*
/// the complex side across frequencies, since the matrix changes).
///
/// # Errors
///
/// Implementors must convert their backend-specific error type into
/// the unified [`LinearSolverError`] enum so the Newton-Raphson driver
/// and analysis orchestrator can match on a stable variant set
/// without depending on `faer` or `russell` types.
///
/// # Example shape
///
/// ```ignore
/// use numeric_solver::linear_solver::{FaerComplexSolver, LinearSolver,
///                                     SparseLinearSystem};
/// use num_complex::Complex;
///
/// fn ac_solve(sys: &SparseLinearSystem<Complex<f64>>) {
///     let solver = FaerComplexSolver;
///     let solution = solver.solve(sys).expect("non-singular AC system");
///     // solution.unknowns()[i] is the complex node voltage at sub-view index i.
///     let _ = solution;
/// }
/// ```
pub trait LinearSolver<Scalar> {
    /// Factor and solve `A · x = b`, returning the dense unknowns
    /// vector `x` on success.
    ///
    /// The caller hands ownership of triplet data to the solver via
    /// the borrow. Implementors must not mutate the input system.
    ///
    /// # Errors
    ///
    /// Returns [`LinearSolverError`] on:
    /// - upstream-data violations (non-finite triplets,
    ///   dimension mismatches, RHS length mismatch) — these are
    ///   caller bugs that the solver detects before invoking the
    ///   backend;
    /// - genuine numerical singularity detected by the backend; or
    /// - backend-internal failures (allocator, capacity overflow).
    fn solve(
        &self,
        system: &SparseLinearSystem<Scalar>,
    ) -> Result<SolutionVector<Scalar>, LinearSolverError>;
}

/// Convenience re-export so callers do not need to depend on
/// `num-complex` directly when the only complex type they touch is
/// the one this module exposes.
pub type C64 = Complex<f64>;
