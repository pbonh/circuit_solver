---
title: Laplacian Matrix
type: claim
id: claim-laplacian-matrix
tags:
- graph
- sparse-matrix
- foundational
- well-established
- linear-algebra
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/04-1-introduction.txt
confidence:
  base: 0.85
---

## Definition

The Laplacian matrix L of a graph G = (V, E) is L = D - A, where D is the diagonal matrix of vertex degrees (or weighted degrees) and A is the adjacency matrix. For an electrical circuit graph with conductance weights, the Laplacian encodes Kirchhoff's current law in matrix form.

## How It Works

For each node, the diagonal entry of L sums the conductances incident to that node; off-diagonal entries are the negative of the conductance between the corresponding pair of nodes. L is symmetric, positive semidefinite, and singular (it has a null vector along the all-ones direction for connected graphs). In circuit analysis, grounding one node (removing its row/column) yields a non-singular reduced Laplacian that maps node currents to node voltages, the core of modified nodal analysis (MNA).

## Key Parameters

- Number of vertices (system order).
- Sparsity pattern (degree of each node).
- Conductance / edge weight distribution.
- Conditioning of the matrix (affects iterative solver convergence).

## When To Use

- DC circuit analysis (Ohmic networks) via MNA.
- Spectral graph algorithms (clustering, partitioning).
- Effective-resistance computations between graph nodes.

## Risks & Pitfalls

- Matrix can be very large (millions of nodes) for VLSI power grids; direct factorization is expensive.
- Ill-conditioning slows iterative solvers; preconditioning is essential.
- Inductive and capacitive elements require MNA extensions beyond a pure Laplacian.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/graph-theory]]
- [[concepts/ir-drop-analysis]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-04-1-introduction]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
