---
title: "Elimination Graph"
type: concept
tags: [sparse-matrix, graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt"]
confidence: high
---

## Definition

The elimination graph of a structurally symmetric sparse matrix represents its nonzero pattern as an undirected graph: each variable is a vertex, each above-diagonal nonzero is an edge. The graph evolves during Gaussian elimination — at pivot step k, vertex k and all its incident edges are removed, and a clique is added among k's former neighbors (representing fill-ins).

## How It Works

Reordering algorithms (minimum-degree, minimum-fill-in) operate on this graph. The number of edges incident on vertex i is its degree d_i. When vertex k is eliminated:
- All neighbors of k that were not previously adjacent become adjacent (forming the clique).
- These new edges correspond to fill-in positions in L and U.

Linked-list storage of adjacency sets supports cheap insertion of fill edges and deletion of eliminated vertices. Circular linked lists with list-head pointers simplify the breaking of pointer chains during deletion (Fig. 2.8.3).

## Key Parameters

- Number of vertices (= matrix dimension).
- Number of edges (= number of above-diagonal nonzeros).
- Degree distribution.
- Storage scheme (linked-list adjacency or compressed-row).

## When To Use

- Implementing minimum-degree or minimum-fill-in reordering.
- Analyzing the cost of a given elimination order.
- Debugging unexpected fill-in in a sparse solver.

## Risks & Pitfalls

- Applicable only to structurally symmetric matrices; for asymmetric cases, use directed graphs or bipartite models.
- Naive degree updates after each pivot are expensive; production codes use multiple-elimination or quotient-graph techniques.

## Related Concepts

- [[concepts/reordering]]
- [[concepts/minimum-degree-ordering]]
- [[concepts/minimum-fill-in]]
- [[concepts/fill-in]]
- [[concepts/sparse-matrix-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
