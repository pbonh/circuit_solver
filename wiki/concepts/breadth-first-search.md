---
title: "Breadth-First Search (BFS)"
type: concept
tags: [graph, algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"]
confidence: high
---

## Definition

Breadth-First Search (BFS) is a graph traversal algorithm that explores all nodes at distance k before any node at distance k+1 from the source. First published by Edward F. Moore in 1959 as a maze-solving method.

## How It Works

A FIFO queue holds the frontier. The source node is enqueued; each iteration dequeues a node, enqueues its undiscovered neighbors, and marks them visited. BFS produces a shortest-path tree in an unweighted graph in O(|V| + |E|) time, with maximum queue size |V|.

## Key Parameters

- Starting vertex (single-source).
- Whether to record predecessors for path reconstruction.

## When To Use

- Shortest paths in unweighted graphs.
- Level-order traversal of trees.
- Bipartite-graph testing (alternate coloring).
- Reachability queries.

## Risks & Pitfalls

- Not suitable for weighted graphs (use Dijkstra or Bellman-Ford).
- Memory consumption can spike with high-degree graphs.

## Related Concepts

- [[concepts/depth-first-search]]
- [[concepts/dijkstras-algorithm]]
- [[concepts/graph-theory]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
