---
title: Maximal Clique Enumeration
type: claim
id: concepts/maximal-clique-enumeration
tags:
- graph
- graph-mining
- algorithm
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Maximal clique enumeration finds every maximal clique in an undirected graph G — every fully-connected vertex subset that cannot be extended by adding another adjacent vertex. It is a canonical computation-intensive graph-mining problem whose worst-case output size is exponential in |V|.

## How It Works

Classical serial algorithms (Bron-Kerbosch and its variants) recursively grow candidate cliques while pruning by neighborhood intersections. To parallelize without double-counting, the search space is partitioned by seed vertex: define G_i = induced subgraph on {v_i} ∪ Γ_>(v_i), the 1-ego network restricted to neighbors with larger IDs. Every clique C is counted exactly once when processed at its smallest vertex. Each G_i is small relative to G and can be processed by a sequential backtracking enumerator on a single worker — a natural fit for subgraph-centric frameworks like G-thinker. Highly-connected vertices generate large G_i that may be recursively decomposed.

## Key Parameters

- ID ordering (any total order works, but degeneracy ordering is more efficient).
- Recursive-decomposition threshold for large G_i.
- Pruning rules (e.g., dropping non-Γ_> neighbors before enumerating).

## When To Use

- Social-network analysis: community detection, clique-based clustering.
- Knowledge-base search: dense semantic patterns.
- Biological networks: protein-interaction modules.
- Benchmarking subgraph-centric graph-mining systems.

## Risks & Pitfalls

- Output explosion: a single vertex can be in exponentially many maximal cliques.
- Power-law degree distributions create stragglers (one huge G_i dominates).
- Vertex-centric implementations are unnatural and either materialize every candidate embedding or require communication-heavy ego-network construction.

## Related Concepts

- [[concepts/subgraph-centric-computation]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
