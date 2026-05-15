---
title: "Clique Tree"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A clique tree of a graph G is a pair (T, C) where T is a tree and C is the set of all maximal cliques of G, with a bijection V(T) ↔ C satisfying the subtree property: for every vertex v ∈ V(G), the set of nodes of T whose maximal clique contains v induces a subtree of T.

A graph has a clique tree iff it is chordal (Theorem 4.7).

## How It Works

The clique tree encodes the recursive structure of a chordal graph:
- Each edge {C_i, C_j} of T corresponds to a minimal separator S = C_i ∩ C_j of G.
- The number of maximal cliques is at most n.
- The subtree property generalizes to "every chordal graph is the intersection graph of a set of subtrees of a tree" (Exercise 4.5).

Algorithms on chordal graphs (tw, independent set, coloring) use the clique tree directly.

## Key Parameters

- |V(T)| ≤ n.
- Edges of T are in bijection with the minimal separators of G.

## When To Use

- Compact representation of a chordal graph.
- DP for treewidth-bounded problems via clique-tree traversal.

## Risks & Pitfalls

- The clique tree is not unique: different trees can satisfy the subtree property.
- Construction via lexicographic BFS is linear-time but requires care to maintain the bijection.

## Related Concepts

- [[concepts/chordal-graph]]
- [[concepts/triangulation]]
- [[concepts/minimal-separator]]
- [[concepts/maximal-clique]]
- [[concepts/treewidth]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
