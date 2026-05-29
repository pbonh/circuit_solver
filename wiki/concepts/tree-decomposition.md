---
title: Tree Decomposition
type: claim
id: claim-tree-decomposition
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

A tree decomposition of a graph G = (V, E) is a pair (T, {X_i}) where T is a rooted tree and each tree node i has a "bag" X_i ⊆ V satisfying:
1. Every vertex of G is in some bag.
2. Every edge of G is in some bag.
3. For each vertex v ∈ V, the set of nodes whose bag contains v forms a subtree of T.

The width is max_i |X_i| - 1.

The treewidth tw(G) is the minimum width over all tree decompositions.

## How It Works

A nice tree decomposition has only four kinds of nodes: start (one-vertex bag), introduce (bag = child bag + one new vertex), forget (bag = child bag - one vertex), join (bag = both children's bags = current bag). Nice decompositions facilitate dynamic programming.

Bodlaender (1996) computes a minimum-width tree decomposition in linear time for bounded-treewidth graphs.

## Key Parameters

- Width = max bag size - 1.
- A nice tree decomposition has O(n) nodes (≤ 4n by Exercise 4.11).

## When To Use

- Dynamic programming for NP-hard problems on bounded-treewidth graphs (Steiner tree, independent set, dominating set, vertex cover, etc.).
- Courcelle's theorem implementation.

## Risks & Pitfalls

- Computing optimal tree decomposition is NP-complete; approximations and FPT algorithms are used.
- "Tree decomposition" ≠ "tree representation": e.g. cotrees are different.

## Related Concepts

- [[concepts/treewidth]]
- [[concepts/chordal-graph]]
- [[concepts/courcelle-theorem]]
- [[concepts/steiner-tree]]
- [[concepts/clique-tree]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
