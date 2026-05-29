---
title: AT-Free Graph
type: claim
id: claim-at-free-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

A graph is AT-free if it has no asteroidal triple. The class includes interval graphs, permutation graphs, cocomparability graphs, and complements of comparability graphs.

## How It Works

AT-free graphs are χ-bounded by f(ω) with f explicit (Kierstead-Penrice). Polynomial-time algorithms for:
- Maximum independent set α(G): O(n^4) (Exercises 4.111-4.114).
- Bandwidth approximation: factor 6 in linear time, factor 2 in O(n^3) (Theorem 4.192).
- Paired dominating set: polynomial time.
- Strong immersion well-quasi-ordering (Exercise 4.110).

AT-free orders correspond to convex geometries (antimatroids) via betweenness relations encoding bull-nose / 6-chain / P_5-midpoint structures.

Every connected AT-free graph has a dominating pair (Corneil-Olariu-Stewart).

## Key Parameters

- α computed in O(n^4).
- Bandwidth approximated within factor 6 in linear time.

## When To Use

- Models with "linear-like" reachability constraints.
- Generalization of interval and permutation graph algorithms.

## Risks & Pitfalls

- AT-free graphs are not perfect; C_5 is AT-free.
- ω is NP-complete on AT-free (reduces from α on triangle-free).

## Related Concepts

- [[concepts/asteroidal-triple]]
- [[concepts/interval-graph]]
- [[concepts/permutation-graph]]
- [[concepts/dominating-pair]]
- [[concepts/antimatroid]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
