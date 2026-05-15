---
title: "Modular Decomposition"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

A module of a graph G is a vertex subset X ⊆ V such that every vertex outside X is adjacent to all of X or none of X. The modular decomposition tree of G has internal nodes labeled parallel (G is disconnected), series (Ḡ is disconnected), or prime (G and Ḡ are both connected). Strong modules (those that don't overlap any other module) form the tree nodes.

## How It Works

Tedder-Corneil-Habib-Paul (2008) compute the modular decomposition tree in linear time using BFS layers, three procedures (refinement, promotion, assembly), and a factorizing-permutation invariant.

The decomposition tree drives recognition of cographs (no prime nodes), distance-hereditary graphs, permutation graphs, and many other classes. It is the standard tool for "uniformly structured neighborhoods" reasoning.

## Key Parameters

- O(n + m) construction time.
- Tree has O(n) nodes.

## When To Use

- Recognition of cograph-like classes.
- Solving graph isomorphism on classes via canonical decompositions.
- Transitive orientation of comparability graphs.

## Risks & Pitfalls

- Prime nodes are unavoidable for graphs that don't admit complete recursive structure (e.g. C_5).
- Different conventions for module definitions exist (open vs. closed neighborhoods).

## Related Concepts

- [[concepts/module]]
- [[concepts/cograph]]
- [[concepts/cotree]]
- [[concepts/permutation-graph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
