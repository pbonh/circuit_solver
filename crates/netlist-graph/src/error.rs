//! Errors produced by the `netlist-graph` builder and graph queries.
//!
//! Per ADR-0010 the error type is part of the v1 *unstable* surface;
//! variants may be added or refined as downstream tasks light up
//! richer validation (topology checker — tasks.md #4, Pass 1 structure
//! flattening — tasks.md #6).

use crate::element::{ElementName, SubcircuitName};
use core::fmt;

/// Errors that can arise while building a `CircuitGraph` or expanding
/// its subcircuit instances.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum NetlistGraphError {
    /// Two elements were added with the same `ElementName` within a
    /// single builder scope. Names must be unique in `circuit-solver`'s
    /// netlist-graph context (the SPICE-style flat namespace is the
    /// authoritative rule).
    DuplicateElementName(ElementName),

    /// Two subcircuit definitions were registered under the same
    /// `SubcircuitName`.
    DuplicateSubcircuit(SubcircuitName),

    /// A `SubcircuitInstance` element refers to a `SubcircuitName`
    /// that was never registered with `add_subcircuit`.
    UnknownSubcircuit(SubcircuitName),

    /// A subcircuit-instance port-binding list has a different length
    /// than the subcircuit definition's port list.
    SubcircuitPortArityMismatch {
        /// The subcircuit whose port list was mismatched.
        subcircuit: SubcircuitName,
        /// Expected number of ports (from the definition).
        expected: usize,
        /// Actual number of ports (from the instance).
        actual: usize,
    },

    /// An element was added with a terminal count that does not match
    /// what its kind requires. For example a two-terminal resistor
    /// passed three terminals.
    TerminalArityMismatch {
        /// The offending element.
        element: ElementName,
        /// What its kind requires.
        expected: usize,
        /// What was actually supplied.
        actual: usize,
    },

    /// `expand_subcircuits` detected a cycle: subcircuit A instantiates
    /// B which instantiates A (possibly transitively). The wiki context
    /// explicitly states "Subcircuit expansion is acyclic"; this is the
    /// dynamic check that enforces it.
    SubcircuitCycle(Vec<SubcircuitName>),
}

impl fmt::Display for NetlistGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateElementName(name) => {
                write!(f, "duplicate element name: {name}")
            }
            Self::DuplicateSubcircuit(name) => {
                write!(f, "duplicate subcircuit definition: {name}")
            }
            Self::UnknownSubcircuit(name) => {
                write!(f, "unknown subcircuit referenced by instance: {name}")
            }
            Self::SubcircuitPortArityMismatch {
                subcircuit,
                expected,
                actual,
            } => write!(
                f,
                "subcircuit {subcircuit} expects {expected} port(s); instance supplied {actual}"
            ),
            Self::TerminalArityMismatch {
                element,
                expected,
                actual,
            } => write!(
                f,
                "element {element}: expected {expected} terminal(s), got {actual}"
            ),
            Self::SubcircuitCycle(chain) => {
                f.write_str("subcircuit expansion cycle detected: ")?;
                for (i, name) in chain.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" -> ")?;
                    }
                    write!(f, "{name}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for NetlistGraphError {}
