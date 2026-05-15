---
title: "Antimatroid"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A convex geometry on a finite set V is a collection C of subsets of V (the convex sets) satisfying:
1. ∅, V ∈ C.
2. A, B ∈ C ⇒ A ∩ B ∈ C.
3. A ∈ C, A ≠ V ⇒ ∃x ∈ V \ A with A ∪ {x} ∈ C.

An antimatroid is the system of "shelling sequences" (elimination orders) of a convex geometry. A betweenness relation generates a convex geometry: a set is convex iff for each (K, r), K \ {r} ⊆ C ⇒ r ∈ C.

## How It Works

Chang-Kloks-Wang: a graph has a betweenness convex geometry iff it has no asteroidal triple. The betweenness relations on AT-free graphs use:
- Bull-noses.
- Roots of 6-chains.
- Midpoints of P_5.
- Pendants of P_5-midpoints.

Algorithm 9 in the text computes an AT-free order by repeatedly removing vertices that are not the root of any current betweenness.

Examples:
- Poset antimatroids: elements with no descendants.
- Chordal graph simplicial elimination antimatroid.
- Interval graph simple elimination antimatroid.

## Key Parameters

- Betweenness relation specification.
- Number of convex sets.

## When To Use

- Greedy algorithms on AT-free graphs.
- Modeling sequential elimination constraints.

## Risks & Pitfalls

- Different betweenness relations give different antimatroids.
- "Greedoid" generalizes antimatroid further (allows more general accessibility).

## Related Concepts

- [[concepts/at-free-graph]]
- [[concepts/asteroidal-triple]]
- [[concepts/chordal-graph]]
- [[concepts/interval-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
