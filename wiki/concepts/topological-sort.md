---
title: "Topological Sort"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A topological sort of a directed graph G = (V, A) is a total order on V such that for every arc (x → y), x precedes y in the order. A topological sort exists iff G is a DAG (directed acyclic graph).

## How It Works

Kahn's algorithm produces a topological sort in O(n + m):
1. Compute the set S of sources (vertices with in-degree 0).
2. Repeatedly remove a source x, append it to the output list L, and remove all outgoing arcs of x. Any vertex that newly becomes a source is added to S.
3. If at the end any arc remains, G has a cycle (no topological sort exists).

Equivalently, depth-first search produces a topological sort by emitting vertices in reverse order of completion.

## Key Parameters

- O(n + m) time using adjacency lists.
- The "simple total ordering problem" (betweenness constraints (a, b, c) with a < b < c) reduces to topological sort of the resulting DAG.

## When To Use

- Build systems, scheduling with dependencies, layer assignment in circuit design.
- Detecting cycles in dependency graphs.

## Risks & Pitfalls

- Topological sort is not unique when there are independent chains.
- The "betweenness" variant (a < b < c OR c < b < a) is NP-complete, distinct from simple total ordering.

## Related Concepts

- [[concepts/dag]]
- [[concepts/kahns-algorithm]]
- [[concepts/cycle]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
