//! `circuit-solver-py` — `PyO3` extension module for the Python frontend.
//!
//! This crate hosts the Python-facing surface of `circuit-solver`. The
//! module name registered with `CPython` is `circuit_solver` (PEP 8
//! lowercase, distinct from the Rust crate name `circuit-solver-py`).
//!
//! ## Surface as of `tasks.md` items #52, #53, #55
//!
//! - [`CircuitBuilder`](builder::PyCircuitBuilder) — the incremental
//!   construction entry point. Methods: `add_element`, `add_wire`,
//!   `add_model`, `add_subcircuit`, **`build()`**. Delegates to
//!   [`netlist_graph::CircuitBuilder`]. Each `build()` call returns a
//!   fresh immutable [`CircuitGraph`](graph::PyCircuitGraph); the
//!   builder remains reusable, satisfying the
//!   `python-frontend#builder-isolation-across-multiple-builds`
//!   Gherkin scenario (tasks.md #55).
//! - [`CircuitGraph`](graph::PyCircuitGraph) — the immutable
//!   `#[pyclass(frozen)]` handle `build()` returns. Read-only
//!   accessors: `element_count`, `node_count`, `model_count`,
//!   `node_names`, `element_names`, `is_empty`, `is_fully_expanded`.
//! - [`CircuitBuilderError`] — Python exception class covering every
//!   error variant of `netlist_graph::NetlistGraphError`.
//! - [`ImmutableHandleError`] — Python exception class raised when
//!   Python code attempts to invoke a builder-mutation method on an
//!   already-built `CircuitGraph` handle (tasks.md item #54; scenario
//!   `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`).
//!   The `#[pyclass(frozen)]` attribute on `PyCircuitGraph` is the
//!   structural belt; trap-methods that raise `ImmutableHandleError`
//!   are the diagnostic suspenders, so attempted mutation surfaces as
//!   a typed, actionable error rather than the bare `AttributeError`
//!   the missing-method path would otherwise produce.
//!
//! `AnalysisRequest`, `NumPy` result arrays, GIL release, and SPICE
//! netlist parsing are tasks #56–#61.
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
pub mod graph;

use pyo3::prelude::*;

pub use builder::PyCircuitBuilder;
pub use errors::{CircuitBuilderError, ImmutableHandleError};
pub use graph::PyCircuitGraph;

/// Python module entry point for `import circuit_solver`.
///
/// Registered with the `CPython` interpreter via `PyO3`'s `#[pymodule]`
/// procedural macro. Registers the `CircuitBuilder` class, the
/// `CircuitGraph` class, the `CircuitBuilderError` exception, and the
/// `ImmutableHandleError` exception.
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
    module.add_class::<PyCircuitGraph>()?;
    module.add("CircuitBuilderError", py.get_type::<CircuitBuilderError>())?;
    module.add(
        "ImmutableHandleError",
        py.get_type::<ImmutableHandleError>(),
    )?;
    Ok(())
}
