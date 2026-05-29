---
title: Module
type: claim
id: claim-module
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

A module of a graph G is a set X ⊆ V such that every vertex of V \ X is either adjacent to every vertex of X or to no vertex of X. The trivial modules are ∅, V, and singletons. A graph is prime if its only modules are trivial.

A strong module does not overlap any other module (where X and Y overlap if X \ Y, Y \ X, and X ∩ Y are all nonempty).

## How It Works

The set of modules of a graph is a "partitive family": closed under intersection, union, symmetric difference, and complement when overlapping. Strong modules form a tree structure — the modular decomposition tree.

A pair of vertices is a twin iff {x, y} is a module. Cographs have rich module structure (every induced subgraph with ≥ 2 vertices has a twin module). Modules also characterize the action of homomorphisms via "quotient graphs."

## Key Parameters

- Trivial modules: ∅, V, {x}.
- A module M containing two vertices at distance > 2 must equal V (in connected G).

## When To Use

- Decomposing graphs by uniform-neighborhood substructures.
- Reducing problems on G to problems on quotients R(G).

## Risks & Pitfalls

- Overlap rules: only non-overlapping (strong) modules form tree nodes.
- Connected components and cocomponents are modules; these are the easy starting cases.

## Related Concepts

- [[concepts/modular-decomposition]]
- [[concepts/twin]]
- [[concepts/cograph]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
