---
title: "Subgraph Reheating"
type: concept
tags: [vlsi, routing, algorithm, novel, optimization]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt"]
confidence: low
---

## Definition

Subgraph reheating is a simulated-annealing-inspired technique within SPROUT that temporarily expands the subgraph beyond its area constraint (dilation) and then prunes back (erosion) to escape local minima of the impedance-minimization landscape.

## How It Works

(1) Dilation: add nodes adjacent to the current subgraph for several iterations, growing beyond A_max. (2) Erosion: compute the node current metric and remove low-current nodes until the area returns to A_max. The temporary excursion allows the optimizer to explore neighborhoods inaccessible by pure SmartGrow/SmartRefine descent.

## Key Parameters

- Number of dilation iterations (controls search radius).
- Number of erosion iterations.
- Tile size and area increment.

## When To Use

- After SmartGrow + SmartRefine appears to have converged.
- Combinatorial routing problems with many local minima.

## Risks & Pitfalls

- Adds runtime; benefit depends on landscape ruggedness.
- Excess dilation can produce difficult-to-erode shapes.

## Related Concepts

- [[entities/sprout]]
- [[concepts/smart-grow]]
- [[concepts/smart-refine]]
- [[concepts/node-current-metric]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
