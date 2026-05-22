//! `numeric-solver` — MNA assembly, NR driver, sparse direct solvers,
//! integration methods.
//!
//! This crate hosts the numeric core of the solver: it consumes the
//! flattened netlist topology produced by `netlist-graph`'s Pass-1
//! pass and progressively builds the Pass-2 MNA matrix, runs the
//! Newton-Raphson outer loop, and dispatches to sparse-LU backends
//! per ADR-0002. Most of the implementation lands incrementally as
//! `tasks.md` items #14–#35.
//!
//! As of `tasks.md` item #3 the only public surface is
//! [`flattened::FlattenedStructure`] — the canonical hand-off type
//! between the netlist crate and the assembler.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod flattened;

pub use flattened::{
    ElementIncidence, FlattenedStructure, FlattenedStructureError, TopologyReport,
};
