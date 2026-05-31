//! Python exception classes raised by the `circuit_solver` bindings.
//!
//! Three exception classes are exposed today:
//!
//! - [`CircuitBuilderError`] — raised by `CircuitBuilder` Python methods
//!   when the underlying `netlist_graph::CircuitBuilder` rejects an
//!   operation (duplicate names, unknown subcircuits, terminal arity
//!   mismatches, expansion cycles). Carries the `Display` impl of the
//!   originating `NetlistGraphError`. Introduced by tasks.md #52.
//! - [`ImmutableHandleError`] — raised by `CircuitGraph` Python methods
//!   when Python code attempts to invoke a builder-mutation method
//!   (`add_element`, `add_wire`, `add_model`, `add_subcircuit`) on an
//!   already-built, immutable `CircuitGraph` handle. Introduced by
//!   tasks.md #54; lights up the
//!   `python-frontend#immutable-circuit-graph-prevents-post-build-mutation`
//!   Gherkin scenario.
//! - [`NetlistParseError`] — raised by `circuit_solver.parse_netlist`
//!   when an input SPICE deck contains an unrecognised device letter
//!   (i.e. the leading character of an element card is not one of
//!   `R`, `C`, `L`, `V`, `I`, `D`, `Q`, `M`, `X`). Carries the
//!   1-indexed source line number and the unrecognised token (the
//!   element-name token whose first character was the unknown
//!   letter) directly in its message. Introduced by tasks.md #61;
//!   lights up the `python-frontend#error-on-malformed-netlist`
//!   Gherkin scenario.
//!
//! Further task-driven refinement of the taxonomy is tracked under
//! tasks.md #58 (Python error mapping).
//!
//! # Why three exception classes (not one, not many)
//!
//! `CircuitBuilderError`, `ImmutableHandleError`, and
//! `NetlistParseError` cover the qualitatively-distinct failure modes
//! the Gherkin scenarios force us to surface:
//!
//! 1. **Construction failed** — the builder accepted a Python call but
//!    the underlying graph rejected the operation. The user wrote a
//!    semantically-invalid circuit; the message identifies the
//!    invariant violated.
//! 2. **Mutation rejected** — the user called a builder method on a
//!    handle that is by-design immutable (post-`build()`). The error
//!    is structural, not semantic; the user has the wrong object.
//! 3. **Netlist text malformed** — the parser could not recognise a
//!    card as a valid SPICE element. The user's deck text is wrong;
//!    the message identifies the line number and the offending token.
//!
//! Differentiated per-variant exception types (e.g. `DuplicateName`
//! vs. `TerminalArity`) would be premature surface that ADR-0010's
//! unstable-v1 stance would have to support anyway. The current
//! Gherkin scenarios assert on exception **type** and on the message
//! string, where the message is the `Display` impl of the originating
//! Rust error — a stable contract owned by the appropriate Rust
//! crate.

use application_frontend::NetlistGraphError;
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

create_exception!(
    circuit_solver,
    ImmutableHandleError,
    PyException,
    "Raised when Python code attempts to invoke a builder-mutation \
     method on an already-built, immutable `CircuitGraph` handle. \
     The handle returned by `CircuitBuilder.build()` is frozen per \
     ADR-0001; mutation must be performed on a fresh `CircuitBuilder` \
     instance and a new graph produced via `build()`."
);

create_exception!(
    circuit_solver,
    NetlistParseError,
    PyException,
    "Raised by `circuit_solver.parse_netlist` when the input SPICE \
     deck contains an unrecognised device letter (i.e. the leading \
     character of an element card is not one of R, C, L, V, I, D, \
     Q, M, X). The exception message identifies the 1-indexed source \
     line number of the offending card and the unrecognised token \
     (the element-name token whose first character was unknown)."
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

/// Construct an [`ImmutableHandleError`] for an attempted mutation on
/// an already-built `CircuitGraph` handle.
///
/// `method` is the Python method name the caller attempted (e.g.
/// `"add_element"`). The message names both the method and the
/// invariant violated so the user can locate the misuse in their
/// Python source without consulting the docs.
#[must_use]
pub fn immutable_handle_err(method: &str) -> PyErr {
    ImmutableHandleError::new_err(format!(
        "`CircuitGraph.{method}` is not callable: a `CircuitGraph` returned by \
         `CircuitBuilder.build()` is immutable (ADR-0001). To add elements, \
         construct a fresh `CircuitBuilder` and call `build()` again."
    ))
}

/// Construct a [`NetlistParseError`] for an unrecognised device letter
/// encountered while parsing a SPICE element card.
///
/// `token` is the element-name token whose first character was the
/// unrecognised device letter — the full token rather than just the
/// offending character so the user can locate it verbatim in their
/// source. `letter` is the leading character itself, surfaced
/// separately so the message reads naturally without having to
/// re-derive it from `token`.
///
/// **Line-number annotation is added by the parser-side
/// `annotate_with_line` wrapper**, which preserves the exception
/// type (`PyErr::from_type`) while prepending a `line N:` segment to
/// the message. Callers therefore raise this error from element-card
/// parsing without naming a line number themselves, and the wrapper
/// adds it as the error bubbles out.
///
/// The Gherkin scenario `python-frontend#error-on-malformed-netlist`
/// asserts that the message "identifies the line number and the
/// unrecognized token". The token is named here; the line number is
/// added by `annotate_with_line`.
#[must_use]
pub fn netlist_parse_error_unrecognised_device(token: &str, letter: char) -> PyErr {
    NetlistParseError::new_err(format!(
        "unrecognised SPICE device letter '{letter}' on token '{token}'; \
         expected one of R, C, L, V, I, D, Q, M, X"
    ))
}
