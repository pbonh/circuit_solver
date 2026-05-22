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
//! As of `tasks.md` item #29 the public surface is:
//!
//! - [`FlattenedStructure`] — the canonical hand-off type between the
//!   netlist crate and the assembler (item #3). Defined in
//!   `circuit-solver-types` to avoid a netlist-graph ↔ numeric-solver
//!   dependency cycle; re-exported here for convenience.
//! - [`flatten::flatten`] — Pass 1 itself: read a `CircuitGraph` and
//!   return a `FlattenedStructure` (item #6).
//! - [`integration`] — implicit-integration companion models for
//!   reactive elements. [`integration::backward_euler`] lands in
//!   tasks.md #29 (capacitor + inductor); sibling modules for
//!   Trapezoidal (#30) and Gear-2 BDF (#31) attach under the same
//!   shape.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod flatten;
pub mod integration;

pub use circuit_solver_types::flattened::{
    ElementIncidence, FlattenedStructure, FlattenedStructureError, TopologyReport,
};
pub use flatten::{flatten, FlattenError};
pub use integration::{
    advance_capacitor_history, advance_inductor_history, capacitor_companion, inductor_companion,
    CapacitorHistory, CompanionInputError, CompanionStamp, InductorHistory,
};
