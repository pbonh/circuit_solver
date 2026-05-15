---
title: "Component"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

A component of a graph G is a maximal vertex set W ⊆ V(G) such that G[W] is connected. The set of components forms a partition of V(G) iff G is disconnected (in the special connected case, the single component covers V).

## How It Works

Components are computed in O(n + m) by a single BFS/DFS, or in O(n^2) by Rem's algorithm (Algorithm 1) which maintains a representative function δ as edges are inserted. Components decompose many graph problems: connectivity, MST, and other parameters often reduce to per-component subproblems.

## Key Parameters

- Number of components.
- Sizes of components.
- "Big" component (giant component) in random graphs.

## When To Use

- Always as a preprocessing step before running connected-only algorithms.
- Dynamic / streaming connectivity uses union-find (close to Rem's algorithm).

## Risks & Pitfalls

- A graph with no edges has |V| components of size 1; the empty graph (E = ∅) is the extreme case.
- The cocomponent (component of Ḡ) is the relevant decomposition unit for cographs and modular decomposition.

## Related Concepts

- [[concepts/graph]]
- [[concepts/connectedness]]
- [[concepts/rems-algorithm]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
