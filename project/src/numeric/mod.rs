//! Numeric solver — MNA assembly, stamping interface, and solver integration.
//!
//! This module hosts the project-level numeric integration that consumes
//! the `netlist.FlattenedView` contract (ADR-0003) and produces the
//! `numeric.StampInterface` contract (ADR-0002). The assembly follows
//! the two-pass graph flattening architecture: Pass 1 produces
//! [`FlattenedStructure`], Pass 2 stamps element contributions into the
//! MNA matrix via [`StampInterface`].
//!
//! # Design references
//!
//! - **ADR-0002** — Hybrid sparse direct solver backend (Russell + FAER).
//!   Ratifies `numeric.StampInterface` as a shared contract.
//! - **ADR-0003** — Two-pass graph flattening with per-analysis sub-views.
//!   Pass 1 produces the `FlattenedStructure`; Pass 2 (this module)
//!   consumes it.
//! - **ADR-0005** — Closed-enum device model dispatch. The
//!   [`IncrementalMnaBuilder`] stamps [`LinearizedModel`] variants
//!   via exhaustive `match`.
//! - **ADR-0010** — Unstable public Rust API surface for v1.

pub mod lu_real;
pub mod mna;

// LU dispatch convenience functions from lu_real.
pub use lu_real::{dense_to_sparse, solve_assembled, solve_sub_view};

// Solver types re-exported from numeric-solver for downstream consumers.
pub use numeric_solver::{
    LinearSolverError, RussellRealSolver, SolutionVector, SparseLinearSystem, SparseTriplet,
};

// MNA assembly types.
pub use mna::{AssembledSystem, IncrementalMnaBuilder, StampInterface, StampValue};
