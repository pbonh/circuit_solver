//! Native Digital Kernel — in-process event-driven engine (DEVS-style).
//!
//! This module implements the Native Digital Kernel container per ADR-0006,
//! superseding the external co-simulation approach of ADR-0004. The kernel
//! provides an in-process `run-until` API driven by the Mixed-Signal Scheduler,
//! with delta-cycle settling and checkpoint/rollback support.

pub mod checkpoint;
pub mod event_queue;
