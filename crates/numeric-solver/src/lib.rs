//! `numeric-solver` — MNA assembly, NR driver, sparse direct solvers,
//! integration methods.
//!
//! This crate hosts the numeric core of the solver: it consumes the
//! flattened netlist topology produced by Pass 1 (now in `netlist-graph`,
//! item #6) over a `CircuitGraph` from `netlist-graph`, and
//! progressively builds the Pass-2 MNA matrix, runs the Newton-Raphson
//! outer loop, and dispatches to sparse-LU backends per ADR-0002. Most
//! of the implementation lands incrementally as `tasks.md` items
//! #14–#35.
//!
//! Per ADR-0003 the [`FlattenedView`](netlist_graph::FlattenedView)
//! contract is owned by the `netlist-graph` crate; the
//! [`flatten`] function and [`FlattenError`] type are re-exported
//! here for backward compatibility. `netlist-graph` owns *construction*
//! of the immutable graph and *flattening* of it; this crate owns
//! *consumption* of the flattened incidence for MNA assembly and solve.
//!
//! As of `tasks.md` item #35 the public surface is:
//!
//! - [`FlattenedStructure`] — the canonical hand-off type between the
//!   netlist crate and the assembler (item #3). Defined in
//!   `circuit-solver-types` to avoid a netlist-graph ↔ numeric-solver
//!   dependency cycle; re-exported here for convenience.
//! - [`flatten::flatten`] — Pass 1 itself: re-exported from
//!   `netlist_graph::flatten` per ADR-0003 (item #6).
//! - [`assemble::assemble`] — Pass 2: stamp the flattened incidence
//!   (and any linearized device contributions) into the full MNA
//!   matrix, producing an [`assemble::MnaSystem`] with ground row and
//!   column intact (item #14).
//! - [`sub_view`] — per-analysis sub-view extraction: apply ground
//!   suppression, Gmin-stepping, and source-stepping masks to the
//!   full MNA matrix from item #14, producing the [`sub_view::SubView`]
//!   the linear solver consumes (item #15).
//! - [`ac_sub_view`] — AC-analysis sub-view extraction: augment the
//!   operating-point MNA matrix with `jωC` capacitor and `jωL`
//!   inductor stamps at a single angular frequency, apply ground
//!   suppression, and lower to the complex [`linear_solver::SparseLinearSystem`]
//!   the [`linear_solver::FaerComplexSolver`] consumes (item #24).
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
//!   is the AC / noise small-signal backend (tasks.md #23).
//!   [`linear_solver::RussellRealSolver`] is the real-valued
//!   (`f64`) sibling backed by `russell_sparse` / `SuiteSparse`
//!   UMFPACK; it drives DC operating-point and transient timestep
//!   solves (tasks.md #16).
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod ac_sub_view;
pub mod assemble;
pub mod flatten;
pub mod gmin_stepping;
pub mod integration;
pub mod linear_solver;
pub mod newton_raphson;
pub mod source_stepping;
pub mod sub_view;

pub use ac_sub_view::{AcSubView, AcSubViewBuilder, AcSubViewError};
pub use assemble::{assemble, MnaAssemblyError, MnaSystem};
pub use circuit_solver_types::flattened::{
    ElementIncidence, FlattenedStructure, FlattenedStructureError, TopologyReport,
};
pub use flatten::{flatten, FlattenError, FlattenedView};
pub use gmin_stepping::{
    GminAugmentedSystem, GminSchedule, GminScheduleError, GminSteppingConfig, GminSteppingDriver,
    GminSteppingError, GminSteppingOutcome, HomotopyStatus,
};
pub use integration::{
    advance_capacitor_history, advance_inductor_history, capacitor_companion, inductor_companion,
    next_step_size, step_decision, AdaptiveError, CapacitorHistory, CompanionInputError,
    CompanionStamp, InductorHistory, LteEstimator, LteToleranceEnvelope, NodeHistorySample,
    StepDecision, StepOutcome, StepSizeBounds, TimestepHistory, TimestepRecord,
};
pub use linear_solver::{
    FaerComplexSolver, LinearSolver, LinearSolverError, RussellRealSolver, SolutionVector,
    SparseLinearSystem, SparseTriplet, C64,
};
pub use newton_raphson::{
    NewtonRaphsonConfig, NewtonRaphsonDriver, NewtonRaphsonError, NewtonRaphsonOutcome,
    NonlinearSystem, SystemError,
};
pub use source_stepping::{
    SourceSteppableSystem, SourceSteppingConfig, SourceSteppingDriver, SourceSteppingError,
    SourceSteppingOutcome,
};
pub use sub_view::{source_rhs_from, SubView, SubViewBuilder, SubViewError};
