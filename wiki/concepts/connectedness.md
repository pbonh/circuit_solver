---
title: "Connectedness"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

A graph G is connected if |V| = 1 or, for every partition {A, B} of V, there exists a ∈ A and b ∈ B with {a, b} ∈ E. Equivalently, there is a walk between every pair of vertices. A graph that is not connected is disconnected.

## How It Works

Connectedness is the global "reachability" property of a graph. It is equivalent to "the set of components has size 1." Many other notions are layered on top: spanning trees only exist for connected graphs, separators measure how far a graph is from being disconnected, and biconnectedness/k-connectedness measure how many vertex removals it takes to disconnect.

## Key Parameters

- The number of components of G (denoted κ_0(G) in some texts).
- Connectivity κ(G) — minimum size of a vertex separator.
- Edge-connectivity λ(G) — minimum size of an edge separator.

## When To Use

- As a precondition for spanning-tree algorithms, MST, Steiner trees.
- When proving properties of separators, paths between vertices, or homomorphisms — connected images of connected graphs are connected.

## Risks & Pitfalls

- The definition via partitions requires |V| ≥ 2 implicitly through the partition definition; the |V| = 1 case is a degenerate connected graph.
- Disconnected graphs have ∅ as a (trivial) minimal separator, but ∅ is not a clique — so the chordal characterization "minimal separators are cliques" requires restricting to connected graphs.

## Related Concepts

- [[concepts/graph]]
- [[concepts/component]]
- [[concepts/separator]]
- [[concepts/spanning-tree]]
- [[concepts/path]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
