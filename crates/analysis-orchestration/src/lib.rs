//! `analysis-orchestration` — analysis control loops and the
//! Mixed-Signal Scheduler.
//!
//! This crate hosts the per-analysis driver loops (DC, AC, transient,
//! noise) and the [`MixedSignalScheduler`][mixed_signal::MixedSignalScheduler]
//! that orchestrates a continuous-time analog solver against an
//! event-driven digital simulator per ADR-0004 ("Optimistic Mixed-Signal
//! Synchronization via Shared Scheduler").
//!
//! At present only the mixed-signal scheduler skeleton and its
//! `optimistic-advance-with-correct-prediction` happy path are
//! implemented; sibling implementer tasks fill in mis-prediction
//! rollback, boundary signal exchange interpolation, conformance, etc.
//!
//! # Stability
//!
//! Per ADR-0010 the public API surface is unstable at v1.0.0.

#![deny(missing_docs)]

pub mod mixed_signal;

pub use mixed_signal::{
    AnalogSolver, AnalogStepReport, BoundarySignals, DigitalAdapterKind, DigitalSimulator,
    DigitalStepReport, MixedSignalScheduler, NextEventReport, SchedulerError, SchedulerOutcome,
};
