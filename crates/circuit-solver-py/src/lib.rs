//! `circuit-solver-py` — `PyO3` extension module for the Python frontend.
//!
//! This crate is a workspace stub at present. Implementation of the
//! Python-facing API (`CircuitBuilder`, immutable `CircuitGraph`
//! handles, `AnalysisRequest` submission, zero-copy `NumPy` result
//! arrays) lands incrementally as tasks.md items #52–#61.
//!
//! At this revision the crate exists solely to:
//!
//! 1. Reserve the workspace member slot and the `circuit_solver` Python
//!    module name (PEP 8 lowercase, distinct from the Rust crate name
//!    `circuit-solver-py`).
//! 2. Establish the `cdylib + rlib` build surface so subsequent
//!    `PyO3` binding tasks compile and link without re-shaping the
//!    workspace.
//! 3. Pin `PyO3` 0.28 with the `abi3-py39` stable-ABI feature so the
//!    eventual `maturin build --release` wheel is forward-compatible
//!    across `CPython` 3.9–3.14+.
//!
//! # Stability
//!
//! Per [ADR-0010] the public Rust API is **unstable** at v1.0.0, and
//! per [ADR-0001] the Python-facing surface is the in-process `PyO3`
//! binding with an immutable `CircuitGraph` handle. The current module
//! exports a single `PyO3` `#[pymodule]` entry point that registers no
//! symbols; sibling implementer tasks add classes and functions to it.
//!
//! [ADR-0010]: ../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
//! [ADR-0001]: ../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md

#![deny(missing_docs)]

use pyo3::prelude::*;

/// Python module entry point for `import circuit_solver`.
///
/// Registered with the `CPython` interpreter via `PyO3`'s `#[pymodule]`
/// procedural macro. At this revision the module body is empty;
/// classes and free functions are added by tasks #52–#61.
///
/// # Errors
///
/// Returns the underlying `PyErr` if a future registration call fails
/// (none are made at present, so the function is currently infallible
/// in practice — the `PyResult<()>` return is required by the
/// `#[pymodule]` macro signature contract, not by current logic).
#[pymodule]
#[allow(clippy::unnecessary_wraps)] // PyResult<()> is mandated by #[pymodule]; future registrations will use it.
fn circuit_solver(_py: Python<'_>, _module: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
