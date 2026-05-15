---
title: "Hanan Grid"
type: concept
tags: [graph, vlsi, routing, well-established, algorithm]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/05-2-graph-fundamentals.txt"]
confidence: high
---

## Definition

A Hanan grid is the grid formed by drawing horizontal and vertical lines through every point in a set of terminals in the plane. Hanan's 1966 result: an optimal rectilinear Steiner minimum tree (RSMT) has all Steiner points on the Hanan grid.

## How It Works

For n terminals there are up to O(n^2) Hanan grid intersections. The grid bounds the search space for candidate Steiner points to a polynomial set, enabling heuristic RSMT algorithms (1-Steiner insertion, edge-based heuristics, FLUTE) to operate on a finite candidate pool. Despite this reduction, RSMT remains NP-hard.

## Key Parameters

- Number of terminals n.
- Number of grid intersections, up to (n+1)^2.
- Spacing between terminals.

## When To Use

- Rectilinear Steiner tree construction in VLSI global and detailed routing.
- Wirelength estimation in placement and floorplanning.

## Risks & Pitfalls

- Even on the Hanan grid, exact RSMT remains NP-hard; heuristics are necessary at scale.
- The Hanan grid grows quadratically with terminal count; very-high-fanout nets benefit from specialized topology generation.

## Related Concepts

- [[concepts/steiner-minimal-tree]]
- [[concepts/interconnect-routing]]
- [[concepts/vlsi-design]]
- [[concepts/minimum-spanning-tree]]

## Sources

- [[summaries/graphs-in-vlsi-05-2-graph-fundamentals]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
