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
//!   net state; exposes the `run_until` API with optional delta-cycle
//!   settling.
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
//! # Delta-cycle settling (task #12)
//!
//! When a [`CombinationalEvaluator`] is installed on the kernel,
//! `run_until` processes events one time point at a time and runs
//! delta-cycle settling after each. This propagates zero-delay
//! combinational logic until the net state stabilizes. Oscillation
//! is detected and reported — the kernel **never hangs**.
//!
//! Without an evaluator, `run_until` behaves exactly as in task #11
//! (backward compatible).
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
//! - Task #11: Event queue + `run_until` API.
//! - Task #12 (this addition): Delta-cycle combinational settling +
//!   oscillation detection.
//! - Task #13: Checkpoint/restore for rollback.

#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

pub mod equivalence;
pub mod event_queue;
pub mod kernel;
pub mod settle;

// Re-export the primary public API at crate root for convenience.
pub use equivalence::{
    check_equivalence, check_equivalence_per_net, EquivalenceResult, EquivalenceTolerance,
    EventTrace, TraceEvent,
};
pub use event_queue::{DigitalEvent, EventQueue, LogicValue, NetId, RunUntilReport};
pub use kernel::{
    DigitalKernel, KernelCheckpoint, KernelRunReport, NetState, NetStateCheckpoint,
    TimePointSettleReport,
};
pub use settle::{CombinationalEvaluator, FnEvaluator, SettleConfig, SettleOutcome};
