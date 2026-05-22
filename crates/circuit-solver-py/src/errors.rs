//! Error conversion: `netlist_graph::NetlistGraphError` → Python exception.
//!
//! For task #52 the bindings need exactly one Python exception class —
//! every error variant from `NetlistGraphError` surfaces as
//! `CircuitBuilderError` carrying the `Display` impl's message. Task
//! #53 (which lights up the `ImmutableHandleError` scenario) introduces
//! a second exception class; further task-driven refinement of the
//! taxonomy is tracked under tasks.md #58 (Python error mapping).
//!
//! # Why a single exception class for now
//!
//! The Gherkin scenario this task enables
//! (`python-frontend#incremental-circuit-construction-via-builder-api`)
//! has no negative-path step that distinguishes between, say,
//! `DuplicateElementName` and `TerminalArityMismatch`. Differentiated
//! exception types would be premature surface that ADR-0010's
//! unstable-v1 stance would have to support anyway. Tests assert on
//! the message string, which is the `Display` impl of
//! `NetlistGraphError` — a stable contract owned by the netlist-graph
//! crate.

use netlist_graph::NetlistGraphError;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(
    circuit_solver,
    CircuitBuilderError,
    PyException,
    "Raised by the `CircuitBuilder` Python class when the underlying \
     `netlist-graph` builder rejects an operation (duplicate names, \
     unknown subcircuits, arity mismatches, expansion cycles)."
);

/// Convert a `NetlistGraphError` into a `PyErr` carrying a
/// `CircuitBuilderError`. The exception payload is the `Display` impl
/// of the variant, which is a stable contract of the netlist-graph
/// crate. Takes the error by reference so callers can keep ownership
/// for logging if needed; `Display` is all this conversion requires.
#[must_use]
pub fn to_py_err(err: &NetlistGraphError) -> PyErr {
    CircuitBuilderError::new_err(err.to_string())
}
