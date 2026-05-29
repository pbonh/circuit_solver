---
title: Triangulation
type: claim
id: claim-triangulation
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

A triangulation (or chordal embedding) of a graph G is a chordal graph H with V(H) = V(G) and E(G) ⊆ E(H). The treewidth tw(G) is the minimum ω(H) - 1 over all triangulations H.

A triangulation is minimal if removing any added edge creates an induced 4-cycle (the smallest non-triangulated induced cycle).

## How It Works

Minimal triangulations correspond bijectively to maximal sets of pairwise parallel (non-crossing) minimal separators of G. Two minimal separators S_1, S_2 cross if there exist two components of G - S_2 each containing a vertex of S_1.

For chordal graphs, the unique minimal triangulation is the graph itself. For circle graphs, treewidth is computable via plane triangulations of an associated 2n-corner polygon (O(n^3)).

## Key Parameters

- ω(H) - 1 = tw(G) for any optimal triangulation H.
- Number of minimal triangulations can be exponential (e.g. for cycles, Catalan-number-many).

## When To Use

- Finding tree-decompositions for DP.
- Modeling sparse-matrix elimination (fill-in for Cholesky).

## Risks & Pitfalls

- Minimum triangulation (fewest added edges) is NP-complete; treewidth (min max-clique-size) is NP-complete in general too.
- Minimal ≠ minimum.

## Related Concepts

- [[concepts/chordal-graph]]
- [[concepts/treewidth]]
- [[concepts/minimal-separator]]
- [[concepts/circle-graph]]
- [[concepts/clique-tree]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
