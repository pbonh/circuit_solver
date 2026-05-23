//! `circuit-solver-py` — `PyO3` extension module for the Python frontend.
//!
//! This crate hosts the Python-facing surface of `circuit-solver`. The
//! module name registered with `CPython` is `circuit_solver` (PEP 8
//! lowercase, distinct from the Rust crate name `circuit-solver-py`).
//!
//! ## Surface as of `tasks.md` items #52, #53, #54, #55, #56, #60
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
//! - [`AnalysisRequest`](analysis_request::PyAnalysisRequest) — the
//!   immutable `#[pyclass(frozen)]` value object describing a
//!   requested analysis. Fields: `analysis_type`, `sweep`,
//!   `integration_method`, `boundary_interpolation` (per ADR-0007).
//!   Tasks.md #56. The submission entry point that consumes an
//!   `AnalysisRequest` + `CircuitGraph` and returns a `Result` is a
//!   downstream task (#57+).
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
//! - `parse_netlist` — free function exposed on the `circuit_solver`
//!   Python module as `circuit_solver.parse_netlist(path)` (registered
//!   by the crate-private `parse_netlist_py` `#[pyfunction]` shim).
//!   Reads a SPICE-format netlist file from disk and returns a
//!   [`CircuitGraph`](graph::PyCircuitGraph) constructed the same way
//!   the [`CircuitBuilder`](builder::PyCircuitBuilder) would build it
//!   incrementally (tasks.md item #60; spec scenario
//!   `python-frontend#spice-netlist-file-parsing`).
//!
//! A dedicated `NetlistParseError` carrying line-number and
//! unrecognised-token detail is tasks.md item #61 (spec scenario
//! `python-frontend#error-on-malformed-netlist`); until that task
//! lands, parse failures surface as `ValueError` / `IOError` /
//! `CircuitBuilderError`.
//! `NumPy` result arrays and GIL release are tasks #57–#59.
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

pub mod analysis_request;
pub mod builder;
pub mod errors;
pub mod graph;
pub mod parser;

use std::path::PathBuf;

use pyo3::prelude::*;

pub use analysis_request::PyAnalysisRequest;
pub use builder::PyCircuitBuilder;
pub use errors::{CircuitBuilderError, ImmutableHandleError};
pub use graph::PyCircuitGraph;

/// Parse a SPICE netlist file from disk and return a
/// [`PyCircuitGraph`].
///
/// Implements tasks.md item #60 — the Python-facing entry point for
/// the `python-frontend#spice-netlist-file-parsing` Gherkin scenario:
///
/// ```text
/// Given CircuitDesigner has a SPICE netlist file on disk
/// When CircuitDesigner calls circuit_solver.parse_netlist(path)
/// Then the returned object is a CircuitGraph
/// And the CircuitGraph contains all elements, models, and
///     subcircuits declared in the netlist
/// And the CircuitGraph is identical to one built incrementally
///     with the same topology
/// ```
///
/// The implementation lives in [`parser::parse_file`]; this binding
/// is a thin wrapper that converts the Python `path` argument into
/// a [`PathBuf`] and re-wraps the returned [`CircuitGraph`] as
/// [`PyCircuitGraph`].
///
/// # Errors
///
/// - `IOError` if the file cannot be read.
/// - `ValueError` if a line is unrecognised, malformed, or violates
///   the SPICE subset documented at [`parser`]. A dedicated
///   `NetlistParseError` carrying line-number and unrecognised-token
///   detail is tasks.md item #61.
/// - `CircuitBuilderError` if the resulting builder-replay sequence
///   is rejected by the underlying `netlist-graph` builder
///   (duplicate element names, unknown subcircuit references,
///   port-arity mismatches, expansion cycles).
#[pyfunction(name = "parse_netlist")]
#[pyo3(text_signature = "(path, /)")]
// PyO3 derives `FromPyObject` for `PathBuf` (path-likes / strings)
// by *constructing* a fresh `PathBuf` and handing ownership to the
// callee; taking it by reference would force a needless clone in
// the binding's prelude. The `needless_pass_by_value` lint is
// suppressed for this reason.
#[allow(clippy::needless_pass_by_value)]
fn parse_netlist_py(path: PathBuf) -> PyResult<PyCircuitGraph> {
    // `PathBuf` is taken by value so PyO3 can convert from the
    // Python str/`os.PathLike` argument without an extra clone;
    // ownership ends at the call to `parser::parse_file` below.
    let graph = parser::parse_file(path.as_path())?;
    Ok(PyCircuitGraph::from_inner(graph))
}

/// Python module entry point for `import circuit_solver`.
///
/// Registered with the `CPython` interpreter via `PyO3`'s `#[pymodule]`
/// procedural macro. Registers the `CircuitBuilder` class, the
/// `CircuitGraph` class, the `AnalysisRequest` class, the
/// `CircuitBuilderError` exception, and the `ImmutableHandleError`
/// exception.
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
    module.add_class::<PyAnalysisRequest>()?;
    module.add("CircuitBuilderError", py.get_type::<CircuitBuilderError>())?;
    module.add(
        "ImmutableHandleError",
        py.get_type::<ImmutableHandleError>(),
    )?;
    module.add_function(wrap_pyfunction!(parse_netlist_py, module)?)?;
    Ok(())
}
