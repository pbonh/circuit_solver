---
title: Subgraph Reheating
type: claim
id: concepts/subgraph-reheating
tags:
- vlsi
- routing
- algorithm
- novel
- optimization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Per GraphsInVLSI Sect. 10.1.6: "The graph-based power routing problem can be viewed as an optimization problem, `Minimize : R(n_s, n) s.t. : A(n) ≤ A_max` (Eq. 10.5) ... These algorithms are, therefore, a form of local optimization where the result is not guaranteed to be a global minimum. To mitigate this issue, the subgraph reheating technique is presented in this section, inspired by the simulated annealing algorithm [577] where the objective function can temporarily increase to explore the design space." The chapter introduces this as a SPROUT-specific escape from local minima of the impedance-minimization landscape.

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
