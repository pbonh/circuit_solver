---
title: "Topological Sorting"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"]
confidence: high
---

## Definition

A topological sort of a directed acyclic graph (DAG) is a linear ordering f : V → {1,...,|V|} such that for every directed edge (u, v), f(u) < f(v). Equivalently: ancestors appear before descendants.

## How It Works

Two classic algorithms run in O(|V| + |E|):
- Kahn's algorithm (1962): repeatedly remove a zero-indegree node, decrement indegrees of its successors, enqueue successors that reach zero indegree. Using a queue vs stack produces different valid orderings. Naturally detects cycles when fewer than |V| nodes are processed.
- DFS-based: run DFS; record nodes in post-order; reverse the order. Cycle detection requires marking nodes currently on the stack.

## Key Parameters

- Data structure (queue, stack, priority queue) determines tie-breaking among valid orderings.
- Graph must be a DAG (acyclic) for a valid order to exist.

## When To Use

- Build systems / makefiles (compile order).
- Task scheduling under precedence constraints.
- High-level synthesis and combinational logic evaluation order.
- DAG-based dataflow analysis in compilers.

## Risks & Pitfalls

- Cycles in input make topological sorting impossible — detect early.
- Output is not unique; downstream algorithms must not rely on a specific ordering.

## Related Concepts

- [[concepts/directed-acyclic-graph]]
- [[concepts/depth-first-search]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
