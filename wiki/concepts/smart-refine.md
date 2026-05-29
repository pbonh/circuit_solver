---
title: SmartRefine
type: claim
id: claim-smart-refine
tags:
- vlsi
- routing
- algorithm
- novel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt
confidence:
  base: 0.65
---

## Definition

SmartRefine is the area-preserving subgraph-refinement phase of SPROUT. It removes nodes carrying minimal current and replaces them with nodes in high-current regions, further reducing impedance without exceeding the area constraint.

## How It Works

Each iteration: (1) compute the node current metric; (2) remove the k nodes with smallest current from the subgraph; (3) run a SmartGrow pass to add k new nodes in high-current regions. The number of nodes moved per iteration is typically larger early (for fast convergence) and smaller late (to avoid increasing impedance).

## Key Parameters

- Nodes moved per iteration k (schedule).
- Number of refinement iterations.
- Convergence tolerance.

## When To Use

- After SmartGrow completes the initial subgraph at A_max.
- As post-processing for any area-constrained routing problem.

## Risks & Pitfalls

- Moving too many nodes per iteration can increase impedance.
- Convergence sensitive to k schedule.

## Related Concepts

- [[entities/sprout]]
- [[concepts/smart-grow]]
- [[concepts/node-current-metric]]
- [[concepts/subgraph-reheating]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
