//! Shared types for the `circuit-solver` workspace.
//!
//! This crate hosts the small, dependency-free types passed across crate
//! boundaries: identifier newtypes, time and simulation-time units,
//! convergence status, the analysis-type discriminator, and the unified
//! `AnalysisResult` envelopes returned to the application frontend.
//!
//! # Stability
//!
//! Per [ADR-0010](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public Rust API is **unstable** at v1.0.0. Consumers must pin to
//! exact versions until a future stabilization ADR.
//!
//! # Module map
//!
//! - [`ids`] — identifier newtypes for analog nodes, elements, and named
//!   boundary signals (`NodeId`, `ElementId`, `SignalName`).
//! - [`branch`] — MNA branch identifier newtype (`BranchId`).
//! - [`model`] — device-model name identifier (`ModelName`).
//! - [`time`] — picosecond-resolution `SimulationTime` on the shared
//!   scheduler timeline.
//! - [`convergence`] — Newton-Raphson outcome (`ConvergenceStatus`),
//!   diagnostic norms, and tolerances. ADR-0006 dictates the dual
//!   update/residue criterion encoded here.
//! - [`analysis`] — the closed `AnalysisType` enum the analysis
//!   orchestrator dispatches on.
//! - [`flattened`] — Pass-1 flattened incidence structure
//!   (`FlattenedStructure`, `ElementIncidence`, `TopologyReport`) and
//!   its constructor errors. Originally placed in `numeric-solver`, these
//!   types were promoted here because `netlist-graph` (topology checker,
//!   tasks.md item #4) and `numeric-solver` (flattener, tasks.md item #6)
//!   both need them; keeping them in `circuit-solver-types` breaks what
//!   would otherwise be a netlist-graph ↔ numeric-solver dependency
//!   cycle.
//! - [`result`] — mixed-signal Result envelopes (`Waveform`,
//!   `AnalogTrace`, `DigitalEventTrace`, `MixedSignalResult`, and the
//!   scheduler-attached metadata used by the
//!   `optimistic-advance-with-correct-prediction` scenario).
//!
//! # Scope of this crate
//!
//! `circuit-solver-types` carries the types that cross multiple
//! workspace crates. `FlattenedStructure`, `ElementIncidence`, and
//! `TopologyReport` were promoted here specifically to break the
//! netlist-graph ↔ numeric-solver dependency cycle that would arise
//! if the topology checker (in `netlist-graph`, tasks.md item #4)
//! imported them from `numeric-solver` while the flattener (in
//! `numeric-solver`, tasks.md item #6) imported `CircuitGraph` from
//! `netlist-graph`. Other richer per-context data — `CircuitGraph`,
//! `DeviceModel`, `AnalysisRequest` envelopes, `OperatingPoint`,
//! `TransferFunction` — still lives in the bounded-context crate that
//! owns it.

#![deny(missing_docs)]

pub mod analysis;
pub mod branch;
pub mod convergence;
pub mod flattened;
pub mod ids;
pub mod model;
pub mod result;
pub mod time;

pub use analysis::AnalysisType;
pub use branch::BranchId;
pub use convergence::{ConvergenceDiagnostic, ConvergenceStatus, ConvergenceTolerances};
pub use flattened::{
    ElementIncidence, FlattenedStructure, FlattenedStructureError, TopologyReport,
};
pub use ids::{ElementId, NodeId, SignalName};
pub use model::ModelName;
pub use result::{
    AnalogTrace, DigitalEventTrace, MixedSignalResult, RollbackEvent, SchedulerMetadata, Waveform,
};
pub use time::SimulationTime;
