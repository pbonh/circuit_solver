//! `circuit-solver-py` — `PyO3` extension module for the Python frontend.
//!
//! This crate hosts the Python-facing surface of `circuit-solver`. The
//! module name registered with `CPython` is `circuit_solver` (PEP 8
//! lowercase, distinct from the Rust crate name `circuit-solver-py`).
//!
//! ## Surface as of `tasks.md` items #52, #53, #54, #55, #56, #57, #59, #60
//!
//! - [`CircuitBuilder`](builder::PyCircuitBuilder) — the incremental
//!   construction entry point. Methods: `add_element`, `add_wire`,
//!   `add_model`, `add_subcircuit`, **`build()`**. Delegates to
//!   [`netlist_graph::CircuitBuilder`]. Each `build()` call returns a
//!   fresh immutable [`CircuitGraph`](graph::PyCircuitGraph); the
//!   builder remains reusable, satisfying the
//!   `python-frontend#builder-isolation-across-multiple-builds`
//!   Gherkin scenario (tasks.md #55). The `build()` call releases
//!   the `CPython` GIL around its native compute (tasks.md #59;
//!   scenario `python-frontend#gil-release-during-simulation`).
//! - [`CircuitGraph`](graph::PyCircuitGraph) — the immutable
//!   `#[pyclass(frozen)]` handle `build()` returns. Read-only
//!   accessors: `element_count`, `node_count`, `model_count`,
//!   `node_names`, `element_names`, `is_empty`, `is_fully_expanded`.
//! - [`AnalysisRequest`](analysis_request::PyAnalysisRequest) — the
//!   immutable `#[pyclass(frozen)]` value object describing a
//!   requested analysis. Fields: `analysis_type`, `sweep`,
//!   `integration_method`, `boundary_interpolation` (per ADR-0007).
//!   Tasks.md #56.
//! - [`Result`](result::PyAnalysisResult) — the immutable
//!   `#[pyclass(frozen)]` value object holding the four output
//!   channels of an analysis run: node voltages, branch currents,
//!   waveforms, and transfer functions, each accessible by name.
//!   Tasks.md #57. The submission entry point that consumes an
//!   `AnalysisRequest` + `CircuitGraph` and returns a `Result` is
//!   [`Simulator`](simulator::PySimulator), below.
//! - [`Simulator`](simulator::PySimulator) — the submission entry
//!   point. Stateless v1 class with a single
//!   [`submit(graph, request)`](simulator::PySimulator::submit)
//!   method that dispatches on the `AnalysisRequest`'s
//!   `analysis_type` slug. The DC operating-point branch is wired
//!   through `analysis_orchestration::dc_analysis` with the GIL
//!   released around the native solver work; other analysis types
//!   raise `NotImplementedError` until their dedicated submission
//!   tasks land. Implements the
//!   `python-frontend#analysis-request-and-result-retrieval`
//!   Gherkin scenario.
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
//! - [`NetlistParseError`] — Python exception class raised by
//!   `parse_netlist` when an input SPICE deck contains an
//!   unrecognised device letter. The message identifies the
//!   1-indexed source line number and the unrecognised token, per
//!   tasks.md item #61 (spec scenario
//!   `python-frontend#error-on-malformed-netlist`).
//!
//! `NumPy` result arrays are task #58 (still pending); GIL release
//! around solver entry points is task #59, completed here.
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
pub mod result;
pub mod simulator;

use std::path::PathBuf;

use pyo3::prelude::*;

pub use analysis_request::PyAnalysisRequest;
pub use builder::PyCircuitBuilder;
pub use errors::{CircuitBuilderError, ImmutableHandleError, NetlistParseError};
pub use graph::PyCircuitGraph;
pub use result::PyAnalysisResult;
pub use simulator::PySimulator;

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
/// - `NetlistParseError` if a card's leading character is not one of
///   the recognised SPICE device letters (`R`, `C`, `L`, `V`, `I`,
///   `D`, `Q`, `M`, `X`). The message identifies the 1-indexed line
///   number and the unrecognised token, per tasks.md #61 and the
///   `python-frontend#error-on-malformed-netlist` Gherkin scenario.
/// - `ValueError` if a line is malformed in a way other than the
///   unrecognised-device-letter case (wrong arity, missing model
///   name, malformed numeric value, unterminated `.SUBCKT`, etc.).
///   The broader Python-error-mapping refactor that may migrate
///   these onto the structured taxonomy is tasks.md #58.
/// - `CircuitBuilderError` if the resulting builder-replay sequence
///   is rejected by the underlying `netlist-graph` builder
///   (duplicate element names, unknown subcircuit references,
///   port-arity mismatches, expansion cycles).
///
/// # GIL release
///
/// File I/O, SPICE tokenization, and the builder-replay sweep are
/// pure-Rust work that does not touch `CPython` data structures. We
/// release the GIL around the entire native call via
/// [`pyo3::Python::detach`] (the pyo3 0.28 successor to
/// `allow_threads`) so concurrent Python threads can continue to
/// execute while a netlist is being parsed. This is one of the two
/// principal witness sites for tasks.md #59 / spec scenario
/// `python-frontend#gil-release-during-simulation` (the other being
/// [`PyCircuitBuilder::build`](builder::PyCircuitBuilder::build)).
/// The `PyCircuitGraph::from_inner` re-wrap on the success path does
/// no `CPython` work either, but for clarity we keep that step outside
/// the `detach` boundary — only the long-running native compute is
/// held inside.
#[pyfunction(name = "parse_netlist")]
#[pyo3(text_signature = "(path, /)")]
// PyO3 derives `FromPyObject` for `PathBuf` (path-likes / strings)
// by *constructing* a fresh `PathBuf` and handing ownership to the
// callee; taking it by reference would force a needless clone in
// the binding's prelude. The `needless_pass_by_value` lint is
// suppressed for this reason.
#[allow(clippy::needless_pass_by_value)]
fn parse_netlist_py(py: Python<'_>, path: PathBuf) -> PyResult<PyCircuitGraph> {
    // `PathBuf` is taken by value so PyO3 can convert from the
    // Python str/`os.PathLike` argument without an extra clone;
    // ownership ends at the call to `parser::parse_file` below.
    //
    // The entire parse — file read, tokenization, builder replay,
    // graph materialization — happens inside `py.detach` so other
    // Python threads can run concurrently. The closure's body is
    // pure Rust over `Send` data (`PathBuf`, `CircuitGraph`,
    // `PyErr`), so the move-into-the-closure compiles and the
    // returned `Result<CircuitGraph, PyErr>` ferries across the
    // re-attach point. See tasks.md #59 / spec scenario
    // `python-frontend#gil-release-during-simulation`.
    let graph = py.detach(|| parser::parse_file(path.as_path()))?;
    Ok(PyCircuitGraph::from_inner(graph))
}

/// Python module entry point for `import circuit_solver`.
///
/// Registered with the `CPython` interpreter via `PyO3`'s `#[pymodule]`
/// procedural macro. Registers the `CircuitBuilder` class, the
/// `CircuitGraph` class, the `AnalysisRequest` class, the `Result`
/// class, the `Simulator` class, the `CircuitBuilderError` exception,
/// the `ImmutableHandleError` exception, and the `NetlistParseError`
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
    module.add_class::<PyAnalysisResult>()?;
    module.add_class::<PySimulator>()?;
    module.add("CircuitBuilderError", py.get_type::<CircuitBuilderError>())?;
    module.add(
        "ImmutableHandleError",
        py.get_type::<ImmutableHandleError>(),
    )?;
    module.add("NetlistParseError", py.get_type::<NetlistParseError>())?;
    module.add_function(wrap_pyfunction!(parse_netlist_py, module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Contract-verification tests for the PyO3 binding crate.

    /// Verify ADR-0001 contract: the PyO3 binding crate declares only
    /// `application-frontend` as a direct domain-crate dependency.
    ///
    /// Scenario: `frontend-contract#pyo3-binding-crate-declares-only-frontend-as-direct-dep`
    ///
    /// ```text
    /// Given the binding crate's Cargo.toml
    /// When the [dependencies] section is inspected
    /// Then no domain crate other than application-frontend appears
    ///  And application-frontend is present as a direct dependency
    /// ```
    #[test]
    fn binding_crate_depends_only_on_application_frontend() {
        let manifest = include_str!("../Cargo.toml");

        // Domain crates in the workspace (other than this crate and
        // application-frontend). If any of these appear as a direct
        // [dependency] this test fails — ADR-0001 requires the binding
        // crate to reach them only through application-frontend re-exports.
        let forbidden_deps = [
            "netlist-graph",
            "circuit-solver-types",
            "analysis-orchestration",
            "numeric-solver",
        ];

        let mut in_dependencies = false;
        let mut found_forbidden: Vec<&str> = Vec::new();

        for line in manifest.lines() {
            let trimmed = line.trim();

            if trimmed == "[dependencies]" {
                in_dependencies = true;
                continue;
            }

            // Any other section header ends the [dependencies] block.
            if trimmed.starts_with('[') {
                in_dependencies = false;
                continue;
            }

            if in_dependencies {
                for forbidden in &forbidden_deps {
                    if trimmed.starts_with(forbidden) {
                        found_forbidden.push(forbidden);
                    }
                }
            }
        }

        assert!(
            found_forbidden.is_empty(),
            "ADR-0001 violation: binding crate Cargo.toml declares forbidden domain-crate \
             dependencies: {found_forbidden:?}. Only `application-frontend` is permitted.",
        );

        // Also verify that application-frontend IS present.
        assert!(
            manifest.contains("application-frontend ="),
            "ADR-0001: binding crate must declare `application-frontend` as a dependency",
        );
    }
}
