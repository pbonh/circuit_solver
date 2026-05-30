---
title: "Topology Checker for Floating-Node Detection in Pass 1"
type: decision
tags: [decision, circuit-solver, netlist-graph, topology, floating-node, dc-analysis, convergence]
created: 2025-07-18
sources:
  - "openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0009-topology-checker-floating-node-detection.md"
confidence: high
---

"In the context of the netlist-graph crate's Pass 1 structure flattening, facing the risk that circuits with floating nodes produce structurally singular MNA matrices that cause Newton-Raphson to fail, we decided for embedding a topology checker in Pass 1 that traverses the flattened incidence structure and flags nodes with no DC path to the ground reference, and against relying solely on runtime Gmin-stepping homotopy or deferring topology checks to the numeric solver, to achieve early, deterministic failure reporting before the solver attempts an expensive and ultimately futile factorization, accepting that the topology checker may produce false positives on nodes grounded only through nonlinear devices."

## Status

accepted

## Architecturally Significant Requirement

Floating nodes are the most common cause of DC convergence failure in SPICE. The [[concepts/dc-analysis]] and [[concepts/newton-raphson-method]] pitfalls confirm that non-isolated equilibria from floating nodes cannot be reached by NR. Early detection prevents wasted computation and provides actionable diagnostics.

## Options Considered

- **Gmin-stepping only** — no extra code, but expensive and gives generic failure messages.
- **Topology check in numeric solver (Pass 2)** — numerically precise, but delayed until after matrix assembly.
- **Topology checker in Pass 1 (chosen)** — early O(N) graph traversal; specific node names in diagnostics; three-tier element classification (always / possibly / never conductive at DC).

## Consequences

- Users receive immediate, specific diagnostics ("node n5 has no DC path to ground").
- Topology checker runs once per CircuitGraph, consistent with ADR-0003.
- Minor dependency on device-model categorization in netlist-graph crate.
- Loops-of-shorts detection deferred to a future ADR.
- `TopologyReport` attached to `FlattenedStructure`; Python frontend surfaces it.

## Source

- OpenSpec ADR: `openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0009-topology-checker-floating-node-detection.md`
