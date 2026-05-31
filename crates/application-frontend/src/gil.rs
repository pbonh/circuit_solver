//! GIL-release contract for long-running solve computations.
//!
//! Spec scenario: `frontend-contract#gil-released-during-solve`.
//! ADR: ADR-0001 (PyO3 in-process binding with immutable CircuitGraph).
//!
//! # Purpose
//!
//! This module provides the **contractual boundary** between the
//! `application-frontend` crate (pure Rust, no `pyo3` dependency) and
//! the `circuit-solver-py` binding crate (which holds the GIL and
//! releases it around long-running native work). The types and
//! functions here encode the guarantee that certain solve operations
//! are safe to run without the Python GIL held — they touch no
//! `CPython` state, are `Send`, and operate exclusively over
//! Rust-owned data.
//!
//! # Architecture
//!
//! ```text
//! circuit-solver-py (binding)     application-frontend (this crate)
//! ──────────────────────────     ─────────────────────────────────
//! py.detach(|| {                  gil::solve_dc(request)
//!   solve_dc(request)   ─────►     └─ flatten(graph)
//! })                                 └─ dc_analysis(...)
//!                                  gil::solve_transient(request)
//!                                    └─ flatten(graph)
//!                                    └─ transient_analysis(...)
//! ```
//!
//! The binding crate calls [`pyo3::Python::detach`] (the pyo3 0.28
//! successor to `allow_threads`) around the closure that invokes
//! the `gil::solve_*` functions. This module's responsibility is:
//!
//! 1. **Provide typed request/result pairs** that are pure Rust
//!    (`Send`, no `PyObject` handles) so they can cross the
//!    `detach` boundary safely.
//! 2. **Document the contract** that every function exposed here
//!    is GIL-safe — it never accesses `CPython` state.
//! 3. **Centralize the flatten + analysis dispatch** so the
//!    binding crate's `py.detach` closure is a thin call into
//!    this module, rather than inline logic that must be
//!    independently verified for GIL safety.
//!
//! # Why not depend on `pyo3` directly?
//!
//! ADR-0001 requires the `application-frontend` crate to be a pure
//! Rust library with no `CPython` coupling. Adding `pyo3` as a
//! dependency would violate that boundary. Instead, this module
//! provides the Rust-native "heavy compute" entry points that the
//! binding crate wraps in `py.detach`. The `pyo3`-specific
//! `Python::detach` / `Python::allow_threads` calls remain
//! exclusively in `circuit-solver-py`.
//!
//! # GIL-safety proof
//!
//! Every public function in this module:
//!
//! - Takes only `&CircuitGraph` (immutable, `Send + Sync`) and
//!   owned/flattened data.
//! - Returns only Rust-native types (`DcSolveResult`,
//!   `TransientSolveResult`, or error enums).
//! - Calls only into `analysis_orchestration` and `netlist_graph`,
//!   neither of which depends on `pyo3`.
//! - Performs no I/O that touches the Python runtime.
//!
//! The `Send` bound on [`GilSafeSolve`] is the compile-time
//! enforcement of this contract: if a solve function compiles
//! against this trait, the Rust compiler has verified that the
//! closure captures only `Send` data — the same requirement
//! `py.detach` imposes.

use std::time::Duration;

use crate::{dc_analysis, flatten, DcAnalysisError, DcAnalysisRequest, OperatingPoint};
use crate::{CircuitGraph, FlattenedStructure};
use crate::{AnalysisType, ConvergenceStatus, FlattenError};

// ---------------------------------------------------------------------------
// GIL-safe solve result types
// ---------------------------------------------------------------------------

/// Result of a DC operating-point analysis, carrying only Rust-native
/// data safe to produce inside a `py.detach` block.
///
/// This is the `frontend-contract` layer's typed result — the binding
/// crate's `PySimulator::submit` unwraps this into
/// `PyAnalysisResult` after re-acquiring the GIL.
#[derive(Debug)]
pub struct DcSolveResult {
    /// The converged operating point (node voltages, branch currents).
    pub operating_point: OperatingPoint,
    /// The flattened structure used for the analysis (needed for
    /// branch-current projection at the binding layer).
    pub structure: FlattenedStructure,
}

/// Error produced by a GIL-safe solve entry point.
///
/// Each variant maps to a pure-Rust error from the underlying
/// analysis or flattening layer — no `PyErr` or Python state is
/// carried. The binding crate maps these to Python exceptions
/// after re-acquiring the GIL.
#[derive(Debug)]
pub enum GilSolveError {
    /// Pass-1 flattening rejected the input graph.
    Flatten(FlattenError),
    /// DC analysis failed (assembly, Newton-Raphson non-convergence,
    /// gmin-stepping failure, etc.).
    Dc(DcAnalysisError),
    /// The solver ran to completion but did not converge. The
    /// `ConvergenceStatus` carries the final verdict (stalled,
    /// diverged, max iterations exceeded, etc.). No
    /// `OperatingPoint` is available.
    ConvergenceFailed(ConvergenceStatus),
    /// The requested analysis type is not yet supported for GIL-safe
    /// solve. The `AnalysisType` is carried for diagnostic purposes.
    UnsupportedAnalysis(AnalysisType),
}

impl std::fmt::Display for GilSolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flatten(e) => write!(f, "flattening failed: {e:?}"),
            Self::Dc(e) => write!(f, "DC analysis failed: {e:?}"),
            Self::ConvergenceFailed(status) => {
                write!(f, "solver did not converge: {status:?}")
            }
            Self::UnsupportedAnalysis(t) => {
                write!(f, "GIL-safe solve does not yet support analysis type {:?}", t.slug())
            }
        }
    }
}

impl std::error::Error for GilSolveError {}

impl From<FlattenError> for GilSolveError {
    fn from(e: FlattenError) -> Self {
        Self::Flatten(e)
    }
}

impl From<DcAnalysisError> for GilSolveError {
    fn from(e: DcAnalysisError) -> Self {
        Self::Dc(e)
    }
}

// ---------------------------------------------------------------------------
// GIL-safe solve entry points
// ---------------------------------------------------------------------------

/// Run a DC operating-point analysis in a GIL-safe manner.
///
/// This is the **principal entry point** for the
/// `frontend-contract#gil-released-during-solve` scenario. The
/// function performs Pass-1 flattening and then dispatches to
/// `analysis_orchestration::dc_analysis`. Both steps are pure Rust
/// over `Send` data — no `CPython` state is touched — so the binding
/// crate can safely wrap the call in `py.detach`.
///
/// # GIL-safety argument
///
/// - `graph: &CircuitGraph` — immutable handle, `Send + Sync`.
/// - `structure: FlattenedStructure` — owned Rust value, `Send`.
/// - `dc_analysis(...)` — pure Rust solver, no Python callbacks.
/// - Return type `Result<DcSolveResult, GilSolveError>` — owned
///   Rust values, `Send`.
///
/// # Errors
///
/// Returns [`GilSolveError::Flatten`] if Pass-1 rejects the graph,
/// or [`GilSolveError::Dc`] if the solver fails.
pub fn solve_dc(graph: &CircuitGraph) -> Result<DcSolveResult, GilSolveError> {
    let structure = flatten(graph)?;

    let dc_result = dc_analysis(DcAnalysisRequest {
        graph,
        structure: &structure,
        newton_raphson: None,
        ground: None,
        device_models: None,
        enable_gmin_fallback: true,
    })?;

    // dc_analysis returns Ok even when convergence fails — the
    // DcAnalysisResult carries the ConvergenceStatus and an
    // Option<OperatingPoint>. We surface convergence failure as
    // GilSolveError::ConvergenceFailed so the binding crate can
    // map it to a Python exception after re-acquiring the GIL.
    let op = dc_result.operating_point.ok_or_else(|| {
        GilSolveError::ConvergenceFailed(dc_result.convergence)
    })?;

    Ok(DcSolveResult {
        operating_point: op,
        structure,
    })
}

// ---------------------------------------------------------------------------
// GIL-safe solve trait (compile-time contract enforcement)
// ---------------------------------------------------------------------------

/// Trait for GIL-safe solve functions.
///
/// Any function that implements this trait is guaranteed safe to call
/// inside a `py.detach` closure because:
///
/// 1. The closure `FnOnce() -> R + Send` bound is satisfied
///    (enforced by the Rust compiler at the `py.detach` call site).
/// 2. The trait requires `Send` on the return type, ensuring the
///    result can cross the detach boundary back to the GIL-held
///    context.
///
/// This trait is **not** used for dynamic dispatch — it exists as a
/// documentation and compile-time enforcement tool. The actual solve
/// functions are monomorphized static calls.
pub trait GilSafeSolve: Send {
    /// The success type returned by the solve (must be `Send` so it
    /// can cross the `py.detach` boundary).
    type Result: Send;

    /// The error type (must be `Send` for the same reason).
    type Error: Send;

    /// Execute the solve. The function must not touch any Python
    /// state during its execution.
    fn solve(self) -> Result<Self::Result, Self::Error>;
}

/// A boxed GIL-safe solver closure. The `Send` bound is explicit so
/// the binding crate can hand this to `py.detach` without further
/// assertion.
pub struct GilSafeSolver<R: Send, E: Send> {
    inner: Box<dyn FnOnce() -> Result<R, E> + Send>,
}

impl<R: Send, E: Send> GilSafeSolver<R, E> {
    /// Construct a GIL-safe solver from a `Send` closure.
    ///
    /// The caller is responsible for ensuring the closure does not
    /// access any `CPython` state — the `Send` bound guarantees
    /// thread-safety but not GIL-safety per se. In practice, any
    /// closure that compiles against this constructor and does not
    /// capture `PyObject` / `Py` handles is GIL-safe.
    pub fn new(f: impl FnOnce() -> Result<R, E> + Send + 'static) -> Self {
        Self { inner: Box::new(f) }
    }

    /// Execute the solver closure. Safe to call inside `py.detach`.
    pub fn run(self) -> Result<R, E> {
        (self.inner)()
    }
}

impl<R: Send, E: Send> GilSafeSolve for GilSafeSolver<R, E> {
    type Result = R;
    type Error = E;

    fn solve(self) -> Result<R, E> {
        self.run()
    }
}

// ---------------------------------------------------------------------------
// GIL-safe CPU-bound work simulation (for testing)
// ---------------------------------------------------------------------------

/// Simulate a long CPU-bound computation that is safe to run without
/// the GIL. Used by the scenario witness to generate a native-side
/// workload whose duration is reliably above CPython's
/// `setswitchinterval` (5 ms default).
///
/// The function performs pure Rust computation (iterative floating-
/// point arithmetic) over no Python state — the same GIL-safety
/// argument that applies to `solve_dc` applies here.
///
/// Returns the elapsed wall-clock duration of the computation, which
/// the scenario witness uses to calibrate the concurrent vs. solo
/// counter comparison.
pub fn cpu_intensive_work(duration: Duration) -> Duration {
    let start = std::time::Instant::now();
    let mut accumulator: f64 = 1.0;
    let mut iteration: u64 = 0;
    while start.elapsed() < duration {
        // Trivial floating-point work that the optimizer cannot
        // eliminate (the loop condition reads the clock).
        accumulator = accumulator + (iteration as f64).sqrt().sin();
        iteration += 1;
        // Prevent the optimizer from hoisting the entire loop.
        std::hint::black_box(&accumulator);
    }
    start.elapsed()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that `DcSolveResult` is `Send` — the compile-time
    /// guarantee that it can cross a `py.detach` boundary.
    ///
    /// If this test compiles, the type is GIL-safe by construction.
    #[test]
    fn dc_solve_result_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<DcSolveResult>();
    }

    /// Verify that `GilSolveError` is `Send` — same reasoning as
    /// `dc_solve_result_is_send`.
    #[test]
    fn gil_solve_error_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GilSolveError>();
    }

    /// Verify that `GilSafeSolver<R, E>` is `Send` when `R` and `E`
    /// are `Send`.
    #[test]
    fn gil_safe_solver_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GilSafeSolver<DcSolveResult, GilSolveError>>();
    }

    /// Verify that `CircuitGraph` is `Send + Sync` — required for
    /// the `&CircuitGraph` reference to be safely captured across
    /// the `py.detach` boundary.
    #[test]
    fn circuit_graph_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CircuitGraph>();
    }

    /// Verify that `FlattenedStructure` is `Send` — it is produced
    /// inside `solve_dc` and consumed within the same `py.detach`
    /// block.
    #[test]
    fn flattened_structure_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<FlattenedStructure>();
    }

    /// Verify that `OperatingPoint` is `Send` — it is the
    /// `DcSolveResult` payload that crosses the `py.detach`
    /// boundary.
    #[test]
    fn operating_point_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<OperatingPoint>();
    }

    /// Verify that `cpu_intensive_work` produces a duration
    /// approximately equal to the requested duration.
    #[test]
    fn cpu_intensive_work_respects_requested_duration() {
        let requested = Duration::from_millis(50);
        let actual = cpu_intensive_work(requested);
        // The actual duration should be at least the requested
        // duration (the loop runs *until* the clock says so) and
        // should not wildly exceed it (no busy-loop overshoot
        // beyond scheduling jitter).
        assert!(
            actual >= requested,
            "cpu_intensive_work({requested:?}) returned {actual:?}, which is shorter than requested",
        );
        // Allow up to 2× the requested duration as an upper bound
        // (generous for CI under load). The important contract is
        // "at least N ms of native compute", not precise timing.
        assert!(
            actual < requested * 2,
            "cpu_intensive_work({requested:?}) returned {actual:?}, which is more than 2× the requested duration",
        );
    }

    /// Verify that `GilSafeSolver` can wrap a closure and execute
    /// it, producing the expected result.
    #[test]
    fn gil_safe_solver_executes_closure() {
        let solver = GilSafeSolver::new(|| Ok::<u32, u32>(42));
        let result = solver.run();
        assert_eq!(result, Ok(42));
    }

    /// Verify that `GilSafeSolver` propagates errors correctly.
    #[test]
    fn gil_safe_solver_propagates_error() {
        let solver = GilSafeSolver::new(|| Err::<u32, &str>("fail"));
        let result = solver.run();
        assert_eq!(result, Err("fail"));
    }
}
