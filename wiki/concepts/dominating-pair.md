---
title: Dominating Pair
type: claim
id: concepts/dominating-pair
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A dominating pair in a graph G is a pair of vertices (s, t) such that every s ~ t path in G is a dominating set of G.

## How It Works

Theorem (Corneil-Olariu-Stewart): every connected AT-free graph has a dominating pair. This is the key structural property used in the O(n^4) independent set algorithm and the bandwidth approximation for AT-free graphs.

Permutation graphs have dominating pairs that correspond to the endpoints of the longest scanline. Interval graphs have dominating pairs at the leftmost / rightmost intervals.

## Key Parameters

- Existence of a dominating pair characterizes some structural properties of the graph.
- Number of dominating pairs varies.

## When To Use

- AT-free graph algorithms.
- Bandwidth computation in AT-free graphs.

## Risks & Pitfalls

- Not every graph has a dominating pair (e.g. cycles of length ≥ 5).
- Finding a dominating pair in arbitrary graphs is NP-complete.

## Related Concepts

- [[concepts/at-free-graph]]
- [[concepts/asteroidal-triple]]
- [[concepts/dominating-set]]
- [[concepts/permutation-graph]]
- [[concepts/interval-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
