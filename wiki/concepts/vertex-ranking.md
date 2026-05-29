---
title: Vertex Ranking
type: claim
id: claim-vertex-ranking
tags:
- graph
- algorithm
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

A t-ranking of a graph G is a coloring c : V → [t] such that for any two vertices x and y with c(x) = c(y), every x ~ y path contains a vertex z with c(z) > c(x). The rank χ_r(G) is the smallest t for which a t-ranking exists.

Always χ(G) ≤ χ_r(G), since a ranking is in particular a proper coloring.

## How It Works

A recursive formula gives χ_r(G) = min_S max_C (|S| + χ_r(C)), where S varies over minimal separators of G and C over components of G - S.

For permutation graphs, an O(n^6) algorithm computes χ_r using scanlines and dynamic programming on pieces of the permutation diagram.

## Key Parameters

- χ_r(G) ≤ ⌈log_2 n⌉ for trees with a centroid-balanced ranking.
- For connected graphs, at most one vertex can receive color t.

## When To Use

- Parallel elimination scheduling in numerical linear algebra (Cholesky factorization order).
- VLSI design where ranking corresponds to dependency depth.

## Risks & Pitfalls

- "Vertex ranking" is also called "ordered coloring" or "tree-depth" in adjacent literatures.
- General graph computation is NP-complete; polynomial-time algorithms exist for many structured classes.

## Related Concepts

- [[concepts/chromatic-number]]
- [[concepts/minimal-separator]]
- [[concepts/permutation-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
