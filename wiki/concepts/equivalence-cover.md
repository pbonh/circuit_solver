---
title: Equivalence Cover
type: claim
id: concepts/equivalence-cover
tags:
- graph
- np-hard
- advanced
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

An equivalence graph is a P3-free graph — equivalently, a disjoint union of cliques. An equivalence cover of a graph G is a set of equivalence subgraphs of G that covers E(G). The equivalence cover number q(G) is the minimum number of equivalence graphs in a cover.

## How It Works

For triangle-free graphs, q(G) = χ'(G) (chromatic index). For splitgraphs, computing q is NP-complete (Blokhuis-Kloks). The reduction uses Holyer's NP-completeness of the chromatic index of cubic graphs.

For a splitgraph with partition (C, S) and Δ = max_{x ∈ C} |N(x) ∩ S|, the bounds Δ ≤ q ≤ Δ + 1 hold, and the value is NP-complete to decide.

## Key Parameters

- q(G).
- The chromatic index χ'(G), bounded by Δ or Δ + 1 (Vizing).

## When To Use

- Edge-decomposition problems in network analysis.
- Modeling "groups of conflict-free objects" as cliques.

## Risks & Pitfalls

- q is NP-complete in general and on splitgraphs; even checking q = D vs. q = D + 1 is hard.

## Related Concepts

- [[concepts/splitgraph]]
- [[concepts/chromatic-index]]
- [[concepts/edge-clique-cover]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
