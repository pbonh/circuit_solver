---
title: Dijkstra's Algorithm
type: claim
id: claim-dijkstras-algorithm
tags:
- graph
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt
confidence:
  base: 0.85
---

## Definition

Dijkstra's algorithm (1956) computes the shortest path from a single source node to every other node in a weighted graph with non-negative edge weights, using a greedy expansion strategy.

## How It Works

Each node has tentative cost (initially ∞, 0 for source) and predecessor (initially none). At each iteration, the unvisited node with smallest tentative cost is selected as current; for each unvisited neighbor v, c_v ← min(c_v, c_u + w_uv) and predecessor is updated. The algorithm runs in O(|V|^2) naively, O((|V|+|E|) log |V|) with a binary heap, and O(|E| + |V| log |V|) with a Fibonacci heap.

## Key Parameters

- Source node.
- Priority queue / heap implementation.
- Target (single or all nodes).

## When To Use

- Shortest-path routing in graphs with non-negative weights (transportation networks, communication networks).
- As a subroutine for A* with admissible heuristic.
- VLSI maze routing on unweighted or non-negative grids.

## Risks & Pitfalls

- Fails on graphs with negative edge weights — use Bellman-Ford instead.
- Heap implementation choice substantially affects performance.

## Related Concepts

- [[concepts/bellman-ford-algorithm]]
- [[concepts/a-star-algorithm]]
- [[concepts/breadth-first-search]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
- [[summaries/graphs-in-vlsi-18-c-multilayer-routing-algorithm]]
