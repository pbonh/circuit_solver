---
title: "Kron's Branch Tearing"
type: concept
tags: [graph, decomposition, foundational, interconnect, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/15-10-symbolic-moment-computation.txt"]
confidence: medium
---

## Definition

Kron's branch tearing decomposes a network with resistive/loop couplings into a set of tree-structured subnetworks driven by replacement current (or voltage) sources, then reassembles the global solution from the subnetwork solutions via the torn-branch constraints.

## How It Works

Pick a tearing branch, model its current as an injected source at one terminal and a sink at the other. Solve each resulting subnetwork by tree-recursion (e.g., for moments). The torn-branch constraints (equal-voltage or equal-current) close a small linear system over the tearing variables, yielding the global response. Successive tearings turn a mesh into a forest.

## Key Parameters

- Order of tearing (affects subnetwork sharing).
- Choice of torn variable (current or voltage).
- BDD organization of the tearing decisions for sharing.

## When To Use

- Mesh interconnect / clock-grid moment computation when flat MNA is too expensive.
- Power-grid analysis under multi-source driving.
- Statistical timing on mesh-topology nets.

## Risks & Pitfalls

- Without BDD sharing, exhaustive tearing is exponential in the number of torn branches.
- Numerical conditioning of the tearing linear system can degrade if interfaces have weak coupling.

## Related Concepts

- [[concepts/symbolic-moment-computation]]
- [[concepts/binary-decision-diagram]]
- [[concepts/hierarchical-symbolic-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-15-10-symbolic-moment-computation]]
