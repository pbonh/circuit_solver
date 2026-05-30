//! `digital-kernel` — Native event-driven digital simulation kernel.
//!
//! This crate implements ADR-0006 ("Native Event-Driven Digital Engine"),
//! which replaces external co-simulation (ADR-0004) with an in-process,
//! DEVS-style event-driven digital kernel. The Mixed-Signal Scheduler
//! drives the kernel via `run_until` — no IPC, no external process.
//!
//! # Core types
//!
//! - [`DigitalKernel`] — top-level type composing the event queue and
//!   net state; exposes the `run_until` API.
//! - [`EventQueue`] — binary min-heap of time-ordered digital events.
//! - [`NetState`] — current value of every digital net (wire).
//!
//! # Event model
//!
//! Events are (time, net, value) tuples processed in non-decreasing
//! time order. Within the same time step, events are processed in
//! FIFO (schedule) order. The kernel uses four-valued logic
//! ([`LogicValue`]) per IEEE 1164.
//!
//! # Integration
//!
//! The kernel is designed to implement the `DigitalSimulator` trait
//! (defined in `analysis-orchestration::mixed_signal`). Task #17 will
//! wire that impl. This crate is standalone — it does not depend on the
//! orchestration crate.
//!
//! # Task scope
//!
//! - Task #11 (this crate): Event queue + `run_until` API.
//! - Task #12: Delta-cycle combinational settling + oscillation detection.
//! - Task #13: Checkpoint/restore for rollback — [`CheckpointManager`]
//!   manages the optimistic rollback lifecycle.
//!
//! # Checkpoint / rollback
//!
//! The [`CheckpointManager`] implements the optimistic time-advance
//! protocol from ADR-0004 (retained under ADR-0006). The scheduler
//! takes checkpoints at predicted digital event boundaries, rolls back
//! on misprediction, and confirms/prunes once the analog solver commits.

#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

pub mod checkpoint;
pub mod event_queue;
pub mod kernel;

// Re-export the primary public API at crate root for convenience.
pub use checkpoint::{CheckpointError, CheckpointManager, TimestampedCheckpoint};
pub use event_queue::{DigitalEvent, EventQueue, LogicValue, NetId, RunUntilReport};
pub use kernel::{DigitalKernel, KernelCheckpoint, KernelRunReport, NetState};
