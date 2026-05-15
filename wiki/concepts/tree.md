---
title: "Tree"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

A tree is a connected graph with no cycles. Equivalent characterizations:
- A connected graph on n vertices is a tree iff it has exactly n - 1 edges.
- A connected graph is a tree iff every minimal separator has cardinality 1.
- A connected graph is a tree iff every connected induced subgraph with at least two vertices has a vertex of degree 1 (a leaf).
- A graph is a tree iff it has an elimination order [x_1, …, x_n] such that x_i is a leaf in G[V_i] for V_i = {x_i, …, x_n}.

## How It Works

Trees are the canonical "no redundancy" connected structures. Leaves can be pruned iteratively, giving inductive proofs and bottom-up dynamic programming. Many decomposition theorems represent a graph by a tree of pieces (clique trees, modular decomposition trees, cotrees, tree-decompositions).

## Key Parameters

- |V(T)| - |E(T)| = 1.
- Leaves (vertices of degree ≤ 1, also called pendant vertices).
- Diameter and radius.

## When To Use

- As the backbone of decompositions (clique tree, tree-decomposition, cotree, etc.).
- For dynamic programming, since trees have width 1.
- Spanning trees as a compact representation of connectedness.

## Risks & Pitfalls

- A single-vertex graph is a tree by convention; some properties (e.g. "two leaves") only hold for trees on ≥ 2 vertices.
- "Forest" is the appropriate generalization for disconnected acyclic graphs.

## Related Concepts

- [[concepts/graph]]
- [[concepts/forest]]
- [[concepts/spanning-tree]]
- [[concepts/clique-tree]]
- [[concepts/tree-decomposition]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
