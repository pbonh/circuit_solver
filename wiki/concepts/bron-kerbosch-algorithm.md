---
title: Bron-Kerbosch Algorithm
type: claim
id: claim-bron-kerbosch-algorithm
tags:
- graph
- algorithm
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.85
---

## Definition

The Bron-Kerbosch algorithm (developed at Eindhoven in the early 1970s) lists all maximal cliques in an undirected graph by recursive backtracking on three sets R (the current clique), P (candidates that can extend R), and X (already-extended vertices, used to prune duplicates).

Initial call: B&K(∅, V, ∅).

## How It Works

At each recursive call:
- If P ∪ X = ∅, report R as a maximal clique.
- Else, choose a candidate x ∈ P.
- Recurse with (R ∪ {x}, P ∩ N(x), X ∩ N(x)) to extend R by x.
- Then move x from P to X and recurse with (R, P \ {x}, X ∪ {x}) for cliques not containing x.

Invariant: R is a clique; P ∪ X = {y : R ⊆ N(y)}.

A pivot rule (Tomita et al.) replaces the loop with iteration over non-neighbors of a well-chosen vertex; the worst-case bound O(n^2 · 3^(n/3)) is asymptotically tight, matching the Moon-Moser bound on the maximum number of maximal cliques.

## Key Parameters

- Runtime O(n^2 · 3^(n/3)) for worst-case enumeration.
- For sparse graphs, Eppstein et al. give bounds in terms of degeneracy.

## When To Use

- Enumerating all maximal cliques (network analysis, bioinformatics).
- Subroutine for graph property testing.

## Risks & Pitfalls

- Output sensitivity: 3^(n/3) can be enormous for dense graphs.
- Without pivoting, the algorithm enumerates the same clique multiple times via different paths.

## Related Concepts

- [[concepts/clique]]
- [[concepts/maximal-clique]]
- [[concepts/graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
