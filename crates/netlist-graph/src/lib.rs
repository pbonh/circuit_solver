//! `netlist-graph` — circuit graph construction, flattening, topology
//! checking.
//!
//! This crate owns the structural representation of circuits. From the
//! [`netlist-graph` bounded-context manifest][ctx]:
//!
//! > The netlist-graph context owns the structural representation of
//! > circuits. Core entities: `CircuitGraph` — a typed graph whose
//! > vertices are `Node`s (electrical nodes, including ground) and
//! > `SubcircuitPort`s, and whose edges are `Branch`es carrying
//! > `Element`s. ... Key invariants: The graph is connected (after
//! > ground reference). Every element terminal maps to a valid node.
//! > Subcircuit expansion is acyclic. Ground is a single distinguished
//! > node.
//!
//! # tasks.md item #5
//!
//! This file delivers item #5 of `circuit-solver/2026-05-21-v1-spec`:
//! the `CircuitGraph` builder with `add_element`, `add_wire`,
//! `add_model`, `add_subcircuit`, `expand_subcircuits`, and `build()`
//! returning an immutable `CircuitGraph`. The headline scenario it
//! enables is `python-frontend#incremental-circuit-construction-via-builder-api`.
//!
//! The remaining netlist-graph responsibilities — Pass 1 structure
//! flattening (item #6), the topology checker (item #4 / ADR-0009),
//! and SPICE deck parsing (items #15, #52..#55) — land in subsequent
//! tasks.
//!
//! # Public surface
//!
//! - [`CircuitBuilder`] — the incremental construction entry point.
//! - [`CircuitGraph`] — the immutable graph handle.
//! - [`Element`], [`ElementKind`], [`ElementName`], [`Node`] — the
//!   graph contents.
//! - [`SubcircuitDefinition`], [`SubcircuitName`] — reusable circuit
//!   modules and their port interfaces.
//! - [`NetlistGraphError`] — the closed enumeration of builder errors.
//!
//! # Stability
//!
//! Per ADR-0010 the public Rust API surface is **unstable** at v1.0.0.
//! Consumers must pin to exact versions until a future stabilization
//! ADR.
//!
//! [ctx]: ../../../../wiki/contexts/netlist-graph.md

#![deny(missing_docs)]

pub mod builder;
pub mod element;
pub mod error;
pub mod graph;
pub mod subcircuit;

pub use builder::{CircuitBuilder, ElementDecl, NetName, GROUND_NET};
pub use element::{Element, ElementKind, ElementName, SubcircuitName};
pub use error::NetlistGraphError;
pub use graph::{CircuitGraph, Node};
pub use subcircuit::SubcircuitDefinition;
