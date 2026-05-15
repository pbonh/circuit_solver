---
title: "Kahn's Algorithm"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

Kahn's algorithm (1962) computes a topological sort of a directed acyclic graph G = (V, A) in O(n + m) time. It maintains a set S of current sources and an output list L.

## How It Works

```
L ← ∅
S ← {x ∈ V : x has no incoming arc}
while S ≠ ∅:
    pick x ∈ S; remove x from S; append x to L
    for each (x, y) ∈ A:
        remove (x, y) from A
        if y has no remaining incoming arc: add y to S
if A is non-empty: G has a cycle, report failure
else: report L
```

Implementation uses doubly-linked adjacency lists or in-degree counters; both run in O(1) per arc removal, giving total O(n + m).

## Key Parameters

- Linear time O(n + m).
- The order in which sources are extracted is arbitrary — any source-first order yields a valid topological sort.

## When To Use

- The standard algorithm for topological sort in build systems, dependency resolution, layer ordering.
- As a cycle detector for DAG validation.

## Risks & Pitfalls

- Removing arcs in place is destructive; in-degree counters avoid mutating the graph.
- The algorithm only works on DAGs — for general digraphs it fails to terminate (reports a cycle).

## Related Concepts

- [[concepts/topological-sort]]
- [[concepts/dag]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
