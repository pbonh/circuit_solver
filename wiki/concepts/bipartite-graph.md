---
title: Bipartite Graph
type: claim
id: concepts/bipartite-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A graph G is bipartite if |V| = 1 or there is a 2-partition {A, B} of V (called color classes) such that every edge has one endpoint in A and the other in B. Equivalently, χ(G) ≤ 2. Theorem 1.19: a graph is bipartite iff all cycles in it are even.

A complete bipartite graph K_{a,b} has every pair (a, b) ∈ A × B adjacent.

## How It Works

Bipartite recognition takes O(n + m) via BFS (assign alternating colors and check for conflict). Many problems are easier on bipartite graphs: maximum matching is O(√n · m) (Hopcroft-Karp), the chromatic number is ≤ 2, edge clique cover collapses to a vertex cover problem (König's theorem on linegraphs of bipartite graphs).

## Key Parameters

- The two color classes A and B.
- ω ≤ 2 for any bipartite graph (no triangles).

## When To Use

- For matching problems in scheduling, assignment, and resource allocation.
- As a natural graph model for two-sorted data (rows vs. columns, jobs vs. machines, etc.).
- As the bipartite-double-cover for solving certain edge-coloring problems on general graphs.

## Risks & Pitfalls

- The empty graph on one vertex is technically bipartite by convention.
- Bipartiteness is not preserved under edge-contraction (a triangle can form).

## Related Concepts

- [[concepts/graph]]
- [[concepts/cycle]]
- [[concepts/chromatic-number]]
- [[concepts/matching]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/guide-to-graph-algorithms-04-graphs]]
