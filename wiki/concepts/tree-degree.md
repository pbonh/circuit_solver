---
title: "Tree-Degree"
type: concept
tags: [graph, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

The tree-degree τ(G) of a graph G is the smallest k such that G is the edge-intersection graph of a family of subtrees of a tree of maximum degree ≤ k.

Every graph is an edge-intersection graph of subtrees of some tree (use a star), so τ is finite. τ ≤ edge-clique-cover number cc(G).

## How It Works

Tree-degree relates to several well-known classes:
- τ(G) = 1 iff G is a clique with ≥ 2 vertices (connected case).
- τ(G) ≤ 2 iff G is an interval graph.
- τ(G) ≤ 3 iff G is a chordal graph.

For bounded τ, the number of minimal separators is O(m · 2^τ), and Bouchitté-Todinca give a polynomial-time algorithm to compute treewidth of graphs with polynomially many minimal separators.

Computing τ is NP-complete even on plane triangulations.

## Key Parameters

- τ(G) — main parameter.
- For τ ≤ k, polynomial treewidth algorithm.
- Bound: tw ≤ τ · ω.

## When To Use

- Computational treewidth on classes with bounded tree-degree.
- Generalization of interval and chordal classes.

## Risks & Pitfalls

- Computing τ is hard; equality τ = cc holds only when G has no clique separator.

## Related Concepts

- [[concepts/interval-graph]]
- [[concepts/chordal-graph]]
- [[concepts/edge-clique-cover]]
- [[concepts/treewidth]]
- [[concepts/minimal-separator]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
