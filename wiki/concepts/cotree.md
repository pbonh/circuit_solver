---
title: "Cotree"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A cotree for a cograph G is a rooted tree T together with a bijection from V(G) to the leaves of T. Each internal node has a label ⊕ (union) or ⊗ (join). Two vertices x, y are adjacent in G iff the least common ancestor of their leaves is labeled ⊗.

## How It Works

The cotree captures the recursive disjoint-union / join structure of a cograph. It is unique up to relabeling and node merging when consecutive nodes have the same label (collapsed cotrees are canonical).

Cotree construction in linear time enables linear/quadratic-time solutions to many problems: α(G), ω(G), χ(G), χ_r(G), tensor capacity, treewidth, black-and-white coloring.

## Key Parameters

- Number of leaves = |V(G)|.
- Tree height bounds recursion depth.

## When To Use

- Whenever you have a cograph and want to apply dynamic programming.
- For verifying cograph-class membership.

## Risks & Pitfalls

- A cotree is defined only for cographs; other classes have different decomposition trees (modular, clique-tree).
- Each internal node must have at least two children; binary cotrees are sometimes preferred.

## Related Concepts

- [[concepts/cograph]]
- [[concepts/modular-decomposition]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
