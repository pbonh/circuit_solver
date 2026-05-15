---
title: "Neighborhood"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/04-graphs.txt"]
confidence: high
---

## Definition

The open neighborhood N(x) of a vertex x is the set of vertices adjacent to x. The closed neighborhood is N[x] = N(x) ∪ {x}. For a set W of vertices, N(W) = (∪_{w∈W} N(w)) \ W and N[W] = N(W) ∪ W. The degree of x is d(x) = |N(x)|.

## How It Works

Neighborhoods give a vertex-local view of the graph. Many graph properties (regularity, twin-ness, simpliciality, claw-freeness) are stated purely in terms of neighborhood structure. Algorithmic routines like BFS, DFS, Bron-Kerbosch, and Rem's algorithm iterate by examining N(x).

## Key Parameters

- d(x) = |N(x)| — degree.
- Minimum degree δ and maximum degree Δ are standard graph invariants.
- "Regular" graphs have all d(x) equal.

## When To Use

- Whenever local structure suffices to decide a property or to drive a local search.
- Twin detection (vertices with N(x) = N(y) or N[x] = N[y]) underlies cograph and distance-hereditary recognition.

## Risks & Pitfalls

- Be careful to specify open vs. closed neighborhood; many definitions (simpliciality, dominating sets, universal vertices) depend on the choice.
- Updating neighborhoods after edge contractions / deletions can be expensive if not implemented carefully.

## Related Concepts

- [[concepts/graph]]
- [[concepts/twin]]
- [[concepts/simplicial-vertex]]
- [[concepts/claw-free-graph]]
- [[concepts/dominating-set]]

## Sources

- [[summaries/guide-to-graph-algorithms-04-graphs]]
