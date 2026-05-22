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
//! - [`result`] — mixed-signal Result envelopes (`Waveform`,
//!   `AnalogTrace`, `DigitalEventTrace`, `MixedSignalResult`, and the
//!   scheduler-attached metadata used by the
//!   `optimistic-advance-with-correct-prediction` scenario).
//!
//! # Scope of this crate
//!
//! `circuit-solver-types` carries only the types that cross multiple
//! workspace crates. Richer per-context data — `CircuitGraph`,
//! `FlattenedStructure`, `DeviceModel`, `AnalysisRequest` envelopes,
//! `OperatingPoint`, `TransferFunction`, `TopologyReport` — lives in
//! the bounded-context crate that owns it. This keeps `types` a thin,
//! dependency-free leaf so every other crate can depend on it without
//! pulling in solver internals.

#![deny(missing_docs)]

pub mod analysis;
pub mod branch;
pub mod convergence;
pub mod ids;
pub mod model;
pub mod result;
pub mod time;

pub use analysis::AnalysisType;
pub use branch::BranchId;
pub use convergence::{ConvergenceDiagnostic, ConvergenceStatus, ConvergenceTolerances};
pub use ids::{ElementId, NodeId, SignalName};
pub use model::ModelName;
pub use result::{
    AnalogTrace, DigitalEventTrace, MixedSignalResult, RollbackEvent, SchedulerMetadata, Waveform,
};
pub use time::SimulationTime;
