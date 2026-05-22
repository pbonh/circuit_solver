//! `circuit-solver-py` — `PyO3` extension module for the Python frontend.
//!
//! This crate hosts the Python-facing surface of `circuit-solver`. The
//! module name registered with `CPython` is `circuit_solver` (PEP 8
//! lowercase, distinct from the Rust crate name `circuit-solver-py`).
//!
//! ## Surface as of `tasks.md` item #52
//!
//! - [`CircuitBuilder`](builder::PyCircuitBuilder) — the incremental
//!   construction entry point. Methods: `add_element`, `add_wire`,
//!   `add_model`, `add_subcircuit`. Delegates to
//!   [`netlist_graph::CircuitBuilder`].
//! - [`CircuitBuilderError`] — single Python exception class covering
//!   every error variant of `netlist_graph::NetlistGraphError`.
//!
//! `CircuitBuilder.build()` (returning an immutable
//! `CircuitGraph` `PyO3` handle) is owned by tasks.md item #53;
//! `AnalysisRequest`, `NumPy` result arrays, GIL release, and SPICE
//! netlist parsing are tasks #54–#61.
//!
//! ## Build profiles
//!
//! - **Release / `maturin develop`** — depends on `pyo3` with the
//!   `extension-module` feature. The resulting `cdylib` is loaded by
//!   `CPython` at import time.
//! - **`cargo test`** — uses a `[dev-dependencies]` override of `pyo3`
//!   that drops `extension-module` and enables `auto-initialize`, so
//!   the test binary embeds `CPython` and can exercise the `#[pyclass]`
//!   surface directly via `Python::with_gil`.
//!
//! # Stability
//!
//! Per [ADR-0010] the public Rust API is **unstable** at v1.0.0, and
//! per [ADR-0001] the Python-facing surface is the in-process `PyO3`
//! binding with an immutable `CircuitGraph` handle.
//!
//! [ADR-0010]: ../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md
//! [ADR-0001]: ../../wiki/decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph.md

#![deny(missing_docs)]

pub mod builder;
pub mod errors;

use pyo3::prelude::*;

pub use builder::PyCircuitBuilder;
pub use errors::CircuitBuilderError;

/// Python module entry point for `import circuit_solver`.
///
/// Registered with the `CPython` interpreter via `PyO3`'s `#[pymodule]`
/// procedural macro. Registers the `CircuitBuilder` class and the
/// `CircuitBuilderError` exception.
///
/// # Errors
///
/// Returns the underlying `PyErr` if `module.add_class::<...>` or
/// `module.add` fails — both are infallible in current `PyO3` versions
/// under normal initialization, but the `PyResult<()>` return is
/// mandated by the `#[pymodule]` macro contract.
#[pymodule]
fn circuit_solver(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyCircuitBuilder>()?;
    module.add("CircuitBuilderError", py.get_type::<CircuitBuilderError>())?;
    Ok(())
}
