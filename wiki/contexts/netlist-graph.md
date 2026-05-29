---
title: Netlist Graph
type: entity
id: contexts/netlist-graph
tags:
- netlist
- graph
- circuit-solver
- bounded-context
created: 2026-05-17
updated: 2026-05-17
sources:
- concepts/graph
- concepts/modified-nodal-analysis
- concepts/branch-stamping
---

## Model

The netlist-graph context owns the structural representation of circuits. Core entities:
- `CircuitGraph` — a typed graph whose vertices are `Node`s (electrical nodes, including ground) and `SubcircuitPort`s, and whose edges are `Branch`es carrying `Element`s.
- `Node` — an electrical node with a unique identifier, a ground flag, and a set of incident branches.
- `Branch` — a directed edge between two nodes carrying one or more `Element`s.
- `Element` — an abstract circuit element (resistor, capacitor, inductor, voltage source, current source, controlled source, semiconductor device, subcircuit instance). Each element references a `ModelName` and carries instance parameters.
- `Subcircuit` — a reusable circuit fragment with a defined port interface; instantiation expands into the parent graph.
- `NetlistDeck` — the raw text representation (SPICE-style `.cir` file) that is parsed into a `CircuitGraph`.

Key invariants: The graph is connected (after ground reference). Every element terminal maps to a valid node. Subcircuit expansion is acyclic. Ground is a single distinguished node.

## Boundary

- Starts at SPICE-style netlist text or programmatic graph construction.
- Ends at the `CircuitGraph` query API and element enumeration used by device-modeling and numeric-solver contexts.
- Adjacent contexts:
  - `device-modeling` receives element-to-model-name references and terminal lists.
  - `numeric-solver` receives the incidence structure (node→branch→element mapping) for [[concepts/modified-nodal-analysis]] assembly.
- Artifacts crossing the boundary: `CircuitGraph` (read-only shared reference), `ElementList`, `NodeIndexMap`.

## Ubiquitous Language

- `Node` — an electrical node in the circuit graph; the reference node is called `Ground`.
- `Branch` — a directed connection between two nodes.
- `Element` — a circuit component with terminals, a model reference, and instance parameters.
- `ModelName` — the string key that binds an element to its device-model definition.
- `Subcircuit` — a reusable circuit module with named ports.
- `Netlist` — the textual input format describing the circuit.
- `CircuitGraph` — the canonical in-memory graph representation.
- `Terminal` — one pin of an element, mapped to a node.
- `Ground` — the datum node (node 0) against which all voltages are measured.

## Relationships

- [[context-maps/circuit-solver]]

## Architecture

- [[architecture/circuit-solver]] — C4 diagrams showing how the netlist-graph context participates in the end-to-end parsing, flattening, stamping, and solving pipeline.
