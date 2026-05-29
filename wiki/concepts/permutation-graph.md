---
title: Permutation Graph
type: claim
id: concepts/permutation-graph
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.95
  source_count: 2
  contradicted: false
  effective: 0.988
  inputs_hash: bb5f665aaf5cec77
---

## Definition

A permutation graph is the intersection graph of a set of line segments whose endpoints lie on two parallel lines. Equivalently, a graph is a permutation graph iff both G and Ḡ are comparability graphs (graphs with a transitive orientation of their edges).

## How It Works

Recognition is linear time via modular decomposition (McConnell-Spinrad 1999). Permutation graphs are AT-free, perfect, and have O(n^2) minimal separators (each corresponds to a scanline in the permutation model).

α(G), ω(G), and χ(G) are computable in polynomial time given a permutation model; vertex ranking χ_r is computable in O(n^6).

## Key Parameters

- Number of minimal separators = O(n^2).
- Width of permutation pattern equals chromatic number.

## When To Use

- VLSI floorplanning (channel routing).
- Sorting with restricted-position elements.
- Test bed for "two comparability orderings" problems.

## Risks & Pitfalls

- Permutation graphs are AT-free but not chordal; they generalize co-bipartite chains.
- A graph is a permutation graph iff it has a "2-dimensional partial-order" representation.

## Related Concepts

- [[concepts/graph]]
- [[concepts/at-free-graph]]
- [[concepts/modular-decomposition]]
- [[concepts/perfect-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
