---
title: "Bellman-Ford Algorithm"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"]
confidence: high
---

## Definition

The Bellman-Ford algorithm (independently by Shimbel 1954, Ford 1956, Bellman 1958) computes shortest paths from a single source to all nodes in a weighted directed graph, including graphs with negative-weight edges. It also detects negative-weight cycles.

## How It Works

Each node holds (cost, predecessor) with the source initialized to (0, None). At each of |V|−1 iterations, all edges are relaxed: for edge (u,v) with weight w, if c_u + w < c_v then c_v ← c_u + w. A |V|-th iteration that still relaxes any edge indicates a reachable negative cycle. Runtime O(|V||E|).

## Key Parameters

- Number of iterations (at most |V|−1 for cycle-free shortest path).
- Edge relaxation order.

## When To Use

- Shortest paths in graphs with negative weights (e.g., financial arbitrage, certain timing analyses).
- Negative cycle detection.
- As a subroutine in Johnson's all-pairs shortest paths algorithm.

## Risks & Pitfalls

- Slower than Dijkstra for non-negative-weight graphs.
- Cannot find shortest path in presence of a reachable negative cycle (problem becomes ill-defined; NP-hard for simple paths).

## Related Concepts

- [[concepts/dijkstras-algorithm]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-18-c-multilayer-routing-algorithm]]
