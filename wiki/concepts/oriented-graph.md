---
title: Oriented Graph (of a Network)
type: claim
id: claim-oriented-graph
tags:
- foundational
- graph
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt
confidence:
  base: 0.85
---

## Definition

The oriented graph of an electrical network is obtained by replacing each two-terminal element with a directed edge connecting its two nodes. Node numbering carries over to the graph; edge orientations are chosen by convention (passive elements: arbitrary; current sources: along the arrow; voltage sources: from + to -). After substitution, the graph captures only interconnection structure, independent of element types.

## How It Works

Once the oriented graph is drawn:
- KCL applies as A i = 0 at every node, where A is the incidence matrix.
- KVL applies as v = A^T v_n linking edge voltages to node voltages.

These relations are topological — they do not depend on whether each edge is a resistor, capacitor, or source.

## Key Parameters

- Number of nodes n+1 (n ungrounded).
- Number of edges b.
- Edge orientations (a choice per edge).
- Connectedness (assumed throughout the chapter).

## When To Use

- As the structural starting point for all topological formulation methods.
- Symbolic analysis, sensitivity analysis, and proofs of properties independent of element values.
- Discussion of tree selection, cutset/loopset, state-variable formulation, and so on.

## Risks & Pitfalls

- Orientation sign errors propagate to all subsequent matrices.
- Disconnected graphs require separate treatment per connected component.

## Related Concepts

- [[concepts/incidence-matrix]]
- [[concepts/tree-cotree]]
- [[concepts/kirchhoff-current-law]]
- [[concepts/kirchhoff-voltage-law]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
