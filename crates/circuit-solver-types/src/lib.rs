//! Shared types for the `circuit-solver` workspace.
//!
//! This crate hosts the small, dependency-free types passed across crate
//! boundaries: identifier newtypes, time and simulation-time units,
//! convergence status, and the unified `AnalysisResult` envelope returned
//! to the application frontend.
//!
//! # Stability
//!
//! Per [ADR-0010](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public Rust API is **unstable** at v1.0.0. Consumers must pin to
//! exact versions until a future stabilization ADR.
//!
//! # Scope of this revision
//!
//! Only the subset needed by the Mixed-Signal Scheduler scenario
//! `optimistic-advance-with-correct-prediction` is fleshed out here:
//!
//! - `SimulationTime` — picosecond-resolution monotonic time on the
//!   shared scheduler timeline (mediates analog continuous time and
//!   digital event time).
//! - `NodeId`, `SignalName` — opaque identifiers for circuit nodes and
//!   named boundary signals.
//! - `Waveform`, `AnalogTrace`, `DigitalEventTrace` — the time-indexed
//!   data carriers populated by the analog solver and digital simulator.
//! - `RollbackEvent`, `SchedulerMetadata` — diagnostic envelopes the
//!   scheduler attaches to a `MixedSignalResult`.
//! - `MixedSignalResult` — the unified Result for mixed-signal analyses,
//!   per the spec's acceptance criterion "the Result contains both
//!   analog Waveforms and digital event traces in VCD format."
//!
//! Other workspace types (`ConvergenceStatus`, `AnalysisType`, `NodeId`
//! ground bookkeeping, etc.) are reserved for sibling tasks #2 and
//! later; they appear here as forward-compatible stubs.

#![deny(missing_docs)]

pub mod ids;
pub mod result;
pub mod time;

pub use ids::{ElementId, NodeId, SignalName};
pub use result::{
    AnalogTrace, DigitalEventTrace, MixedSignalResult, RollbackEvent, SchedulerMetadata, Waveform,
};
pub use time::SimulationTime;
