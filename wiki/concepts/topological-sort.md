---
title: "Topological Sort"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources:
  - "raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"
  - "raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"
confidence: high
---

## Definition

A topological sort (also called topological sorting or topological ordering) of a directed graph G = (V, A) is a linear ordering f : V → {1, …, |V|} — equivalently a total order on V — such that for every directed edge (u, v) (alternatively written as the arc u → v), f(u) < f(v). Informally: ancestors appear before descendants. A topological sort exists if and only if G is a [[concepts/dag|DAG]] (a [[concepts/directed-acyclic-graph|directed acyclic graph]]).

## How It Works

Two classic algorithms each run in O(|V| + |E|) using adjacency lists:

1. **[[concepts/kahns-algorithm|Kahn's algorithm]] (1962).**
   - Compute the set S of sources (vertices with in-degree 0).
   - Repeatedly remove a source x, append it to the output list L, and remove all outgoing arcs of x. Decrement the in-degree of each successor; any successor that newly reaches in-degree 0 enters S.
   - If at the end any arc remains (equivalently fewer than |V| nodes have been emitted), G contains a [[concepts/cycle]] and no topological sort exists.
   - The data structure backing S (queue, stack, priority queue) determines tie-breaking among the many valid orderings.

2. **[[concepts/depth-first-search|DFS]]-based.** Run DFS over G; record vertices in post-order (as they finish); reverse the list. Cycle detection requires marking nodes currently on the recursion stack.

Equivalently: any DFS produces a topological sort by emitting vertices in reverse order of completion.

## Key Parameters

- O(|V| + |E|) time and space using adjacency lists.
- Tie-breaking among valid orders depends on the data structure backing the source set (FIFO queue, LIFO stack, priority queue).
- The "simple total ordering problem" — given betweenness constraints (a, b, c) with a < b < c — reduces to topological sort of the resulting DAG.

## When To Use

- Build systems and Makefiles (compile order).
- Task scheduling under precedence constraints.
- Layer assignment and high-level synthesis in [[concepts/vlsi-design|VLSI design]].
- Combinational logic evaluation order; DAG-based dataflow analysis in compilers.
- Detecting cycles in dependency graphs (a missing topological order ⇔ a cycle).

## Risks & Pitfalls

- Cycles in the input make topological sort impossible — detect early; both Kahn and DFS variants do so naturally.
- **The output is not unique** when there are independent chains; downstream algorithms must not rely on a specific ordering.
- The "betweenness" variant (a < b < c **or** c < b < a) is NP-complete, distinct from simple total ordering — do not confuse them.

## Related Concepts

- [[concepts/dag]]
- [[concepts/directed-acyclic-graph]]
- [[concepts/kahns-algorithm]]
- [[concepts/depth-first-search]]
- [[concepts/cycle]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
