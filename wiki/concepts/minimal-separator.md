---
title: Minimal Separator
type: claim
id: claim-minimal-separator
tags:
- graph
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
confidence:
  base: 0.85
---

## Definition

A minimal a|b-separator is an a|b-separator S such that no proper subset of S is an a|b-separator. A minimal separator of G is a set S that is a minimal a|b-separator for some nonadjacent pair (a, b).

Equivalently, S ⊂ V is a minimal separator iff G - S has two components C_1 and C_2 such that every vertex of S has a neighbor in both C_1 and C_2 (Exercise 1.9).

## How It Works

Minimal separators are the atomic decomposition units for many graph classes:
- A connected graph is chordal iff every minimal separator is a clique.
- The set of minimal separators of a graph can be exponential in general but is polynomial for interval graphs, permutation graphs, chordal graphs, and graphs of bounded tree-degree.

## Key Parameters

- Number of minimal separators σ(G); related to algorithm time bounds for treewidth, vertex ranking, and minimal triangulations.
- For chordal graphs, σ(G) ≤ n - 1 and they correspond to edges of any clique tree.

## When To Use

- Treewidth / minimal triangulation algorithms (Bouchitté-Todinca framework).
- Vertex ranking and clique separator decomposition.

## Risks & Pitfalls

- Different minimal separators can be nested (one strictly contained in another) — this is common in trees and 5-cycles.
- The notion is asymmetric in a/b; a single set may be minimal for one pair but not for another.

## Related Concepts

- [[concepts/separator]]
- [[concepts/chordal-graph]]
- [[concepts/clique-separator]]
- [[concepts/clique-tree]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
