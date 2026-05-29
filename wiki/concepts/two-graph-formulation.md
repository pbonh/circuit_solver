---
title: Two-Graph Formulation (I-Graph and V-Graph)
type: claim
id: claim-two-graph-formulation
tags:
- foundational
- graph
- analog
- sparse-matrix
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
confidence:
  base: 0.85
---

## Definition

The two-graph formulation uses two separate graphs of a network — the current graph (I-graph) and the voltage graph (V-graph) — to eliminate redundant variables that the single-graph tableau and modified-nodal formulations retain. KCL is written using A_i I = 0 with the I-graph; KVL is written using V = A_v^T V_n with the V-graph.

## How It Works

Rules for drawing the two graphs:
1. I-graph: collapse an edge if its current is uninteresting (e.g., voltage source); delete an edge if its current is zero (e.g., VVT input).
2. V-graph: delete an edge if its voltage is uninteresting (e.g., current source); collapse an edge if its voltage is zero (e.g., nullator).

Variables that enter constitutive equations are never eliminated. The graphs may end up with different numbers of nodes and edges. The tableau and MNA formulations are then constructed on the two graphs separately, giving Y_b and Z_b that are no longer square.

For an admittance y with I-graph edge (j_i → j_i') and V-graph edge (j_v → j_v'), the stamp into the nodal portion contributes +y at (j_i, j_v) and (j_i', j_v'), -y at (j_i, j_v') and (j_i', j_v).

## Key Parameters

- Numbers of nodes and edges on each graph (independent after collapse/deletion).
- Choice of which variables are "of no interest" — affects graph size.
- For ideal elements: each type has fixed I-graph and V-graph representations (Fig. 4.6.1 of the book).

## When To Use

- Compact MNA formulations that drastically shrink the system size compared to single-graph methods.
- Switched-capacitor networks (Chapter 14 of Vlach & Singhal) where the formulation is particularly advantageous.
- Any time minimal matrix sizes are needed for symbolic or efficient numerical analysis.

## Risks & Pitfalls

- Bookkeeping is more complex than single-graph formulations.
- Collapsed nodes require careful renumbering.
- Implementation involves managing two parallel data structures rather than one.

## Related Concepts

- [[concepts/two-graph-modified-nodal]]
- [[concepts/tableau-formulation]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/incidence-matrix]]
- [[concepts/branch-stamping]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
