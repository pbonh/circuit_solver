---
title: Splitgraph
type: claim
id: concepts/splitgraph
tags:
- graph
- foundational
- well-established
- np-hard
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A graph is a splitgraph if its vertex set partitions into a clique C and an independent set S. Equivalently, a graph is a splitgraph iff it does not contain 2K_2, C_4, or C_5 as an induced subgraph.

## How It Works

Splitgraphs are recognized in linear time via degree-sequence properties (the sorted degree sequence d_1 ≥ … ≥ d_n must satisfy a specific majorization condition). They are simultaneously chordal and co-chordal.

The class is closed under complementation (the clique becomes the independent set and vice versa).

## Key Parameters

- |C| and |S| partition sizes.
- Δ_S = max degree of vertices in C into S.

## When To Use

- Test bed for NP-completeness reductions: many problems are easy on splitgraphs (clique, independent set) but others stay NP-complete (equivalence cover, black-and-white coloring).

## Risks & Pitfalls

- The partition (C, S) is not always unique (vertices of full degree might go in either).
- "Splitgraph" is distinct from "bipartite": splitgraphs partition into clique + independent set, bipartite into two independent sets.

## Related Concepts

- [[concepts/clique]]
- [[concepts/independent-set]]
- [[concepts/threshold-graph]]
- [[concepts/chordal-graph]]
- [[concepts/equivalence-cover]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
