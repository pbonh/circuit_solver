---
title: "DAG (Directed Acyclic Graph)"
type: concept
tags: [graph, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt"]
confidence: high
---

## Definition

A digraph is a graph in which every edge has a direction (an arc), x → y. A DAG (directed acyclic graph) is a digraph that contains no directed cycles. Sources are vertices with no incoming arcs; sinks are vertices with no outgoing arcs.

## How It Works

DAGs admit topological sorts (linear orders respecting all arcs). Game positions, dependency relations, and partial orders are typically represented as DAGs. Kahn's algorithm gives an O(n + m) topological sort.

Any graph can be oriented into a DAG by choosing any vertex ordering and orienting edges from lower to higher index.

## Key Parameters

- Number of sources (start-nodes) and sinks.
- Longest path (critical path) — useful in scheduling and chip-design timing analysis.

## When To Use

- Dependency resolution, scheduling, build systems.
- Game tree analysis where each position is reachable through a unique forward direction.
- Representing partial orders (Hasse diagrams).

## Risks & Pitfalls

- A digraph in general need not be a DAG; cycle detection is mandatory before relying on topological-sort guarantees.
- For undirected graphs, "acyclic" gives forests; for digraphs it gives DAGs, which can have considerably more edges (up to n(n-1)/2).

## Related Concepts

- [[concepts/topological-sort]]
- [[concepts/kahns-algorithm]]
- [[concepts/cycle]]
- [[concepts/tournament]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
