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
//! # tasks.md item #4
//!
//! The [`topology`] module delivers item #4 of
//! `circuit-solver/2026-05-21-v1-spec`: Pass-1 floating-node
//! detection per [ADR-0009]. It consumes a
//! [`FlattenedStructure`](circuit_solver_types::flattened::FlattenedStructure)
//! (`circuit-solver-types`) and a parallel conductivity-class slice,
//! and emits a [`TopologyReport`](circuit_solver_types::flattened::TopologyReport)
//! (`circuit-solver-types`) that the analysis orchestrator uses to
//! decide whether to enable Gmin-stepping pre-solve.
//!
//! # Cross-crate dependency note
//!
//! The data types the topology checker reads (`FlattenedStructure`,
//! `ElementIncidence`) and writes (`TopologyReport`) were promoted to
//! `circuit-solver-types` to break the netlist-graph ↔ numeric-solver
//! dependency cycle that would arise if `netlist-graph` imported them
//! from `numeric-solver` while `numeric-solver`'s flattener imported
//! `CircuitGraph` from `netlist-graph`. This keeps the dataflow
//! strictly unidirectional per design.md.
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
//! - [`topology::check_topology`], [`topology::ConductivityClass`] —
//!   Pass-1 floating-node detection (ADR-0009).
//! - [`topology::validate_topology`], [`topology::TopologyError`] —
//!   high-level circuit-graph topology validator (US-008): floating-node
//!   detection plus KVL-loop detection for voltage-source and inductor loops.
//!
//! # Stability
//!
//! Per ADR-0010 the public Rust API surface is **unstable** at v1.0.0.
//! Consumers must pin to exact versions until a future stabilization
//! ADR.
//!
//! [ctx]: ../../../../wiki/contexts/netlist-graph.md
//! [ADR-0009]: ../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0009-topology-checker-floating-node-detection.md

#![deny(missing_docs)]

pub mod builder;
pub mod element;
pub mod error;
pub mod graph;
pub mod subcircuit;
pub mod topology;

pub use builder::{CircuitBuilder, ElementDecl, NetName, GROUND_NET};
pub use element::{Element, ElementKind, ElementName, SubcircuitName};
pub use error::NetlistGraphError;
pub use graph::{CircuitGraph, Node, VoltageSourceOverrideError};
pub use subcircuit::SubcircuitDefinition;
