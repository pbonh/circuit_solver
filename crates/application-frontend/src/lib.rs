//! `application-frontend` — public Rust API (`CircuitGraph`,
//! `AnalysisRequest`, `AnalysisResult`).
//!
//! This crate is the sole dependency of the PyO3 binding crate
//! (`circuit-solver-py`) per ADR-0001. It re-exports every
//! domain-crate type the binding needs so that no domain crate
//! appears as a direct dependency of `circuit-solver-py`.
//!
//! Per ADR-0010 the public Rust API at v1.0.0 is **unstable** —
//! consumers must pin exact versions until a future stabilization
//! ADR.

#![deny(missing_docs)]

// --- Re-exports from `circuit-solver-types` ---------------------------
// The binding crate uses AnalysisType (analysis_request, simulator),
// BranchId/NodeId (simulator), FlattenedStructure (simulator), and
// ModelName (builder, parser).
pub use circuit_solver_types::{
    AnalysisType, BranchId, ModelName, NodeId,
    flattened::FlattenedStructure,
};

// --- Re-exports from `netlist-graph` ----------------------------------
// The binding crate uses CircuitBuilder, ElementDecl, ElementKind,
// SubcircuitDefinition (builder, parser), NetlistGraphError (errors),
// and CircuitGraph (graph, parser, simulator).
pub use netlist_graph::{
    CircuitBuilder, CircuitGraph, ElementDecl, ElementKind, NetlistGraphError,
    SubcircuitDefinition,
};

// --- Re-exports from `analysis-orchestration` -------------------------
// The binding crate uses dc_analysis, DcAnalysisError, DcAnalysisRequest,
// OperatingPoint (simulator).
pub use analysis_orchestration::{
    dc_analysis, DcAnalysisError, DcAnalysisRequest, OperatingPoint,
};

// --- Re-exports from `netlist-graph` (flatten) ------------------------
// The binding crate uses flatten (simulator). Per ADR-0003 the flatten
// function and FlattenedView contract are owned by netlist-graph.
pub use netlist_graph::{flatten, FlattenError, FlattenedView};

// --- PyO3-adjacent result types (task #26) ----------------------------
// SimulationResult and NamedScalarData provide contiguous Vec<f64>
// storage for zero-copy NumPy transfer at the Python boundary.
// Spec scenario: frontend-contract#results-zero-copy-numpy.
pub mod pymodule;
pub use pymodule::{NamedScalarData, SimulationResult};
