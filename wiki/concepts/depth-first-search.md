---
title: "Depth-First Search (DFS)"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"]
confidence: high
---

## Definition

Depth-First Search (DFS) is a graph traversal algorithm that explores as far as possible along each branch before backtracking. First documented by Charles Pierre Trémaux in the 19th century; the modern computer formulation is due to Tarjan (1972).

## How It Works

A stack (or recursion) holds the current path. At each step, the algorithm advances to an undiscovered neighbor of the top-of-stack node; if none exists, it backtracks (pops). DFS visits every reachable node and edge in O(|V| + |E|) on a finite connected graph. The stack size is at most |V|.

## Key Parameters

- Starting vertex.
- Visitation/coloring scheme (white/gray/black for cycle detection).
- Recursive vs explicit stack implementation.

## When To Use

- Cycle detection in directed graphs.
- Topological sorting (via post-order reversal).
- Strongly connected components (Tarjan's, Kosaraju's algorithms).
- Tree/forest classification of edges.

## Risks & Pitfalls

- Not guaranteed to find shortest paths.
- Recursion depth can exceed stack limits on large graphs.
- May fail to terminate on infinite graphs.

## Related Concepts

- [[concepts/breadth-first-search]]
- [[concepts/topological-sorting]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
