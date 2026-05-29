---
title: Maze Routing (Lee's Algorithm)
type: claim
id: claim-maze-routing
tags:
- vlsi
- routing
- algorithm
- graph
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt
confidence:
  base: 0.85
---

## Definition

Maze routing is a grid-based wire routing technique introduced by C. Y. Lee in 1961 that finds a shortest path between two terminals on a routing grid by an exhaustive breadth-first traversal. It was one of the first VLSI routing algorithms and remains the conceptual basis for many modern detailed routers.

## How It Works

The grid is treated as a graph with each open cell a node and edges between orthogonal neighbors. Starting at the source, BFS labels every cell with its shortest-path distance from the source. Once the target is reached, the path is reconstructed by descending labels. Variants minimize bends, wire crossings, or congestion by modifying the cost function.

## Key Parameters

- Grid dimensions and obstacle map.
- Cost function (wirelength, bends, crossings).
- Single-net vs. concurrent multi-net.

## When To Use

- Detailed routing on Manhattan grids.
- Foundation for A*-accelerated path finding.

## Risks & Pitfalls

- Worst-case O(|V|) time but very large grids waste effort exploring far from the target — A* mitigates this by using a distance heuristic.
- Net-order dependence: subsequent nets see obstacles laid down by earlier nets (routing-order problem).

## Related Concepts

- [[concepts/breadth-first-search]]
- [[concepts/a-star-algorithm]]
- [[concepts/interconnect-routing]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
