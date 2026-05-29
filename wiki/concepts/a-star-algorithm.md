---
title: A* (A-star) Algorithm
type: claim
id: claim-a-star-algorithm
tags:
- graph
- algorithm
- foundational
- well-established
- vlsi
- routing
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt
confidence:
  base: 0.85
---

## Definition

A* is a best-first (informed) shortest-path algorithm that extends Dijkstra's algorithm by adding an admissible heuristic h(u) estimating cost from node u to the goal. Nodes are expanded in order of f(u) = g(u) + h(u), where g(u) is the cost from source.

## How It Works

A priority queue orders frontier nodes by f. The heuristic biases exploration toward the goal. If h is admissible (never overestimates true remaining cost) and consistent, A* finds the optimal path and is no worse than Dijkstra; if h is also non-trivial, A* expands far fewer nodes. On a 2D grid the Euclidean or Manhattan distance to the goal is a standard heuristic.

## Key Parameters

- Heuristic function and its admissibility / consistency.
- Tie-breaking strategy on equal f values.
- Open/closed set implementations.

## When To Use

- VLSI maze routing on grid graphs where geometry is known.
- Pathfinding in game AI, robotics, GPS.
- Any shortest-path scenario where a meaningful goal-distance heuristic exists.

## Risks & Pitfalls

- Inadmissible heuristics yield suboptimal paths.
- Heuristic quality determines runtime; a weak heuristic degrades to Dijkstra.
- Obstacles can cause significant detours; A* still finds optimal paths but may explore many nodes.

## Related Concepts

- [[concepts/dijkstras-algorithm]]
- [[concepts/breadth-first-search]]
- [[concepts/graph-theory]]
- [[concepts/interconnect-routing]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
