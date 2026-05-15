---
title: "Directed Acyclic Graph (DAG)"
type: concept
tags: [graph, foundational, well-established, algorithm]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"]
confidence: high
---

## Definition

A directed acyclic graph (DAG) is a directed graph containing no directed cycles. Equivalently, a directed graph admitting a topological ordering: a mapping f : V → {1,...,|V|} such that for every edge (u,v), f(u) < f(v).

## How It Works

DAGs are produced whenever a precedence or causal relationship exists without feedback. They are central to combinational logic, dataflow analysis, task scheduling, version-control commit history, Bayesian networks, build systems, and many EDA representations. Topological sorting on a DAG runs in O(|V|+|E|).

## Key Parameters

- Source nodes (zero indegree) and sink nodes (zero outdegree).
- Longest path (critical path) length.
- Branching factor and depth.

## When To Use

- Modeling acyclic precedence: combinational circuits, task graphs, build dependencies.
- As an intermediate representation in compilers and EDA tools.

## Risks & Pitfalls

- Cycles introduced by errors break DAG-based algorithms; cycle detection is critical.
- Different valid topological orderings can affect downstream heuristics' results.

## Related Concepts

- [[concepts/topological-sort]]
- [[concepts/graph-theory]]
- [[concepts/finite-state-machine]]
- [[concepts/timing-graph]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
