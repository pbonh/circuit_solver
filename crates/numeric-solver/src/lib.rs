//! `numeric-solver` — MNA assembly, NR driver, sparse direct solvers,
//! integration methods.
//!
//! This crate hosts the numeric core of the solver: it consumes the
//! flattened netlist topology produced by Pass 1 (this crate, item #6)
//! over a `CircuitGraph` from `netlist-graph`, and progressively builds
//! the Pass-2 MNA matrix, runs the Newton-Raphson outer loop, and
//! dispatches to sparse-LU backends per ADR-0002. Most of the
//! implementation lands incrementally as `tasks.md` items #14–#35.
//!
//! Per ADR-0003 the flattener lives inside this crate (the Numeric
//! Solver Engine "reads the `CircuitGraph` once" and produces the full
//! incidence structure); `netlist-graph` owns *construction* of the
//! immutable graph, this crate owns *consumption* of it.
//!
//! As of `tasks.md` item #6 the public surface is:
//!
//! - [`FlattenedStructure`] — the canonical hand-off type between the
//!   netlist crate and the assembler (item #3). Defined in
//!   `circuit-solver-types` to avoid a netlist-graph ↔ numeric-solver
//!   dependency cycle; re-exported here for convenience.
//! - [`flatten::flatten`] — Pass 1 itself: read a `CircuitGraph` and
//!   return a `FlattenedStructure` (item #6, this task).
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod flatten;

pub use circuit_solver_types::flattened::{
    ElementIncidence, FlattenedStructure, FlattenedStructureError, TopologyReport,
};
pub use flatten::{flatten, FlattenError};
