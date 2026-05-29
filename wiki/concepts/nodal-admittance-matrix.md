---
title: Nodal Admittance Matrix (NAM)
type: claim
id: claim-nodal-admittance-matrix
tags:
- analog
- sparse-matrix
- foundational
- netlist
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/10-6-generalized-two-graph-theory.txt
confidence:
  base: 0.85
---

## Definition

The Nodal Admittance Matrix is the `n x n` matrix in `Y * V = I` formed by writing one KCL equation per non-ground node of a network composed entirely of admittance-stamp-compatible elements (R, C, L, VCCS, or pathological elements after precollapse). NAM is the most compressed form of MNA.

## How It Works

Each element contributes a 2x2 (or larger) stamp at its terminal node indices. Pathological elements (nullor, VM, CM) are precollapsed in the two-graph (NL/VM in V-graph, NR/CM in I-graph), causing rows or columns to merge into node-sets that label the resulting reduced matrix. Sign flips arise when a node-set contains a negatively-signed index.

## Key Parameters

- Precollapse order (matters for resulting matrix sparsity).
- Node-set sign conventions for VM/CM reference orientations.

## When To Use

- Symbolic analysis of large active-filter networks expressed via pathological elements.
- Reduced-dimensional MNA for DDD or GPDD downstream symbolic computation.

## Risks & Pitfalls

- Independent voltage sources still need branch equations; pure NAM is voltage-source-free.
- Sign-flip rules must be tracked for every admittance attached to a signed node-set.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/two-graph-method]]
- [[concepts/nullor]]
- [[concepts/pathological-element]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
