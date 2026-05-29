---
title: "Two-Pass Graph Flattening with Per-Analysis Sub-Views"
type: claim
id: claim-decision-0003-two-pass-graph-flattening-with-per-analysis-sub-views
tags: [decision, circuit-solver, netlist, graph, flattening, mna, sparse-matrix, analysis]
created: 2026-05-17
updated: 2026-05-18
sources:
  - "architecture/circuit-solver"
  - "grills/circuit-solver"
  - "contexts/netlist-graph"
  - "contexts/numeric-solver"
  - "contexts/analysis-orchestration"
  - "vision/circuit-solver"
confidence:
  base: 0.85
---

"In the context of circuit graph flattening for multi-analysis simulation, facing ground-reference and constraint-mask differences between operating-point and small-signal analyses, we decided for a two-pass flattening approach with per-analysis sub-views to achieve zero re-flattening overhead when switching analysis types, accepting that the full matrix (including ground) is built once and that sub-view extraction adds a small per-solve masking cost."

## Status

accepted

## Context

The numeric solver must handle ground-reference and constraint-mask differences between operating-point and small-signal analyses without re-flattening the netlist graph. This is an [[concepts/architecturally-significant-requirement|architecturally significant requirement]] (ASR) because it constrains the boundary between the [[contexts/netlist-graph|netlist-graph]] context (which owns the immutable `CircuitGraph`) and the [[contexts/numeric-solver|numeric-solver]] context (which assembles and solves the MNA matrix). Re-flattening on every analysis switch would violate the performance expectations of interactive Python workflows and would duplicate structural work that is independent of analysis type.

The [[vision/circuit-solver|Circuit Solver vision]] bounds scope to DC, AC small-signal, transient, and noise analyses. DC operating-point solves require ground suppression (the ground node is eliminated from the unknown vector) and may apply source-stepping or Gmin-stepping constraint masks. AC small-signal solves build a complex-valued MNA matrix around the same nodes but may apply different boundary-condition masks (e.g., fixed-input small-signal excitation). Transient solves reuse the real-valued DC structure but add dynamic companion-model stamps. All four analysis types share the same underlying node and branch topology.

The [[grills/circuit-solver|grill Q&A]] explored whether flattening should be analysis-aware (one flattening per analysis type), fully lazy (flatten on first solve), or one-pass with views. It converged on one-pass flattening plus view extraction because the graph structure is analysis-independent and the cost of building the full incidence structure dominates the cost of extracting a sub-view or applying a constraint mask.

## Decision

We commit to a two-pass graph flattening strategy inside the Numeric Solver Engine container:

1. **Pass 1 — Structure flattening.** The Numeric Solver Engine reads the `CircuitGraph` once from the Netlist Graph Builder and constructs the full incidence structure: node-to-branch mapping, element enumeration, and ground-reference bookkeeping. This pass is analysis-agnostic and executes exactly once per `CircuitGraph`.

2. **Pass 2 — Matrix assembly with per-analysis sub-views.** At solve time, the Numeric Solver Engine builds the full MNA matrix (including the ground row/column) from the flattened structure, then the Analysis Orchestrator extracts the relevant sub-view (e.g., ground-suppressed unknowns for DC, complex augmented matrix for AC) or applies constraint masks (source stepping, Gmin stepping, small-signal excitation boundaries) without rebuilding the underlying graph or re-enumerating elements.

This decision means that switching from a DC operating-point solve to an AC small-signal solve reuses the same flattened incidence structure and only re-stamps the MNA matrix with the linearized device models appropriate to the new analysis type. The ground node remains in the full matrix for structural completeness; it is masked out at the sub-view level rather than removed during flattening.

## Consequences

**Positive:**
- Zero re-flattening overhead when switching analysis types on the same netlist, preserving sub-second setup latency for interactive Python workflows.
- Structural analysis (connectivity, floating-node detection, ground reference insertion) is decoupled from numerical analysis, allowing the Netlist Graph Builder to remain immutable and analysis-agnostic.
- Constraint masks (source stepping, Gmin stepping, small-signal excitation) can be applied, removed, and reapplied without invalidating the flattened structure, supporting iterative solver strategies such as damped Newton and homotopy.
- The full matrix (including ground) simplifies debugging and structural verification: the ground row provides a consistency check that the MNA assembly is correct before any sub-view is extracted.

**Negative:**
- Per-solve sub-view extraction adds a small masking cost (index permutation or logical mask application) on every linear solve. For very large circuits this cost is non-zero, though it is typically dwarfed by sparse-LU factorization time.
- The full matrix includes the ground row/column even when they are never part of the unknown vector, increasing the symbolic size of the matrix by one row/column. In most circuits this is negligible; in extremely small test circuits it is a minor overhead.
- Constraint masks must be carefully validated to ensure they do not introduce structural singularities that would not appear in the full matrix. A mask that suppresses too many nodes can create a rank-deficient sub-view even when the full matrix is nonsingular.

**Neutral:**
- The decision does not preclude incremental netlist modification (adding or removing elements mid-session), but it implies that any structural change triggers a re-flattening. Incremental modification is out of scope unless a later ADR expands it.
- The sub-view extraction layer is a natural insertion point for future features such as hierarchical Schur-complement reduction or multi-port small-signal analysis, because the full structure is always available.

## Related Decisions

- [[decisions/0001-pyo3-in-process-binding-with-immutable-circuit-graph|ADR-0001]] — Preceding ADR that commits to an immutable `CircuitGraph`; two-pass flattening leverages that immutability by treating the graph as a read-only source of truth.
- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]] — Preceding ADR on solver backends; the sub-view extraction layer sits upstream of the `LinearSolver` abstraction and feeds the same backend dispatch logic.
- [[architecture/circuit-solver]] — The container diagram that surfaces this decision under `## Decisions Surfaced`.
- [[grills/circuit-solver]] — Q&A log where flattening strategy alternatives were interrogated.
- [[vision/circuit-solver]] — Scope declaration that mandates DC, AC, transient, and noise analyses on the same netlist.
- [[contexts/netlist-graph]] — Bounded context that owns the `CircuitGraph` and the flattening Pass 1 structure.
- [[contexts/numeric-solver]] — Bounded context that owns the MNA matrix assembly, sub-view extraction, and solver dispatch.
- [[contexts/analysis-orchestration]] — Bounded context that requests sub-views and applies constraint masks per analysis type.
