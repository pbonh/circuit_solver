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
//! As of `tasks.md` item #35 the public surface is:
//!
//! - [`FlattenedStructure`] — the canonical hand-off type between the
//!   netlist crate and the assembler (item #3). Defined in
//!   `circuit-solver-types` to avoid a netlist-graph ↔ numeric-solver
//!   dependency cycle; re-exported here for convenience.
//! - [`flatten::flatten`] — Pass 1 itself: read a `CircuitGraph` and
//!   return a `FlattenedStructure` (item #6).
//! - [`assemble::assemble`] — Pass 2: stamp the flattened incidence
//!   (and any linearized device contributions) into the full MNA
//!   matrix, producing an [`assemble::MnaSystem`] with ground row and
//!   column intact (item #14).
//! - [`sub_view`] — per-analysis sub-view extraction: apply ground
//!   suppression, Gmin-stepping, and source-stepping masks to the
//!   full MNA matrix from item #14, producing the [`sub_view::SubView`]
//!   the linear solver consumes (item #15).
//! - [`integration`] — implicit-integration companion models for
//!   reactive elements. [`integration::backward_euler`] lands in
//!   tasks.md #29 (capacitor + inductor); sibling modules for
//!   Trapezoidal (#30) and Gear-2 BDF (#31) attach under the same
//!   shape; [`integration::adaptive`] (tasks.md #32) provides the
//!   LTE controller; the `From<&TimestepHistory>` impl in
//!   `integration::adaptive` (tasks.md **#35**) lifts the
//!   controller's audit trail into
//!   [`circuit_solver_types::TimestepHistoryMetadata`] for the
//!   user-facing
//!   [`circuit_solver_types::TransientResult`].
//!
//! - [`linear_solver`] — sparse-direct LU dispatch (ADR-0002).
//!   [`linear_solver::FaerComplexSolver`] implements
//!   [`linear_solver::LinearSolver`] for `Complex<f64>` matrices and
//!   is the AC / noise small-signal backend (tasks.md #23). The
//!   real-valued (`russell`) implementation lands in tasks.md #16.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod assemble;
pub mod flatten;
pub mod integration;
pub mod linear_solver;
pub mod sub_view;

pub use assemble::{assemble, MnaAssemblyError, MnaSystem};
pub use circuit_solver_types::flattened::{
    ElementIncidence, FlattenedStructure, FlattenedStructureError, TopologyReport,
};
pub use flatten::{flatten, FlattenError};
pub use integration::{
    advance_capacitor_history, advance_inductor_history, capacitor_companion, inductor_companion,
    next_step_size, step_decision, AdaptiveError, CapacitorHistory, CompanionInputError,
    CompanionStamp, InductorHistory, LteEstimator, LteToleranceEnvelope, NodeHistorySample,
    StepDecision, StepOutcome, StepSizeBounds, TimestepHistory, TimestepRecord,
};
pub use linear_solver::{
    FaerComplexSolver, LinearSolver, LinearSolverError, SolutionVector, SparseLinearSystem,
    SparseTriplet, C64,
};
pub use sub_view::{source_rhs_from, SubView, SubViewBuilder, SubViewError};
