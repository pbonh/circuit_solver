//! Pass-1 structure flattening — delegation to `netlist-graph`.
//!
//! Per ADR-0003 the `FlattenedView` contract is owned by the
//! `netlist-graph` crate. The canonical [`flatten`] function and
//! [`FlattenError`] type now live in `netlist_graph::flatten`; this
//! module re-exports them so that existing `crate::flatten::flatten`
//! call sites inside `numeric-solver` continue to compile without
//! changes.
//!
//! # Migration note
//!
//! The implementation and tests that originally lived here were moved
//! to `crates/netlist-graph/src/flatten.rs` as part of task-6
//! (ADR-0003 contract alignment). This module exists solely as a
//! re-export facade for backward compatibility within this crate.
//! External consumers should prefer `netlist_graph::flatten` directly.

pub use netlist_graph::{flatten, FlattenError, FlattenedView};
