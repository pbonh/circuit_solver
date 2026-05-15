---
title: "Triangle Partition"
type: concept
tags: [graph, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A graph has a triangle partition if its edges can be partitioned into triangles (cliques of size 3). For planar graphs, deciding whether such a partition exists is linear-time (Gao-Kloks-Poon); for general graphs it is NP-complete.

## How It Works

A planar graph G admits a triangle partition iff:
1. G is biconnected (after reductions).
2. The dual H is bipartite.
3. Every vertex of G has even degree at least 4.
4. Every edge is in at least two triangles.
5. Every even separating triangle, after Baker-layer decomposition, is of Type 1 or Type 2 (consistent inside/outside structure).

The algorithm recursively processes outermost even separating triangles via the Type a/b dichotomy from Lemma 4.113.

## Key Parameters

- Number of separating triangles bounded by n.
- Linear-time after Baker's layer partition.

## When To Use

- Planar layout decomposition.
- Mesh refinement on triangulated surfaces.

## Risks & Pitfalls

- The minimum number of triangles to cover all edges (not partition) is NP-complete even for planar graphs.

## Related Concepts

- [[concepts/triangulation]]
- [[concepts/bakers-method]]
- [[concepts/outerplanar-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
