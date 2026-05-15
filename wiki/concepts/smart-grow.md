---
title: "SmartGrow"
type: concept
tags: [vlsi, routing, algorithm, novel]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt"]
confidence: medium
---

## Definition

SmartGrow is the area-constrained subgraph-growth phase of SPROUT. Starting from a void-filled seed subgraph that connects all terminals, it iteratively adds nodes adjacent to high-current regions (determined by the node current metric) until the total area reaches A_max.

## How It Works

Each iteration: (1) compute the node current metric on the current subgraph; (2) identify the boundary set C of nodes in the available-space graph adjacent to but not yet in the subgraph; (3) add the k boundary nodes whose neighbors carry the highest current. Repeat until subgraph area reaches A_max. Runtime per iteration is dominated by solving the Laplacian system.

## Key Parameters

- Nodes added per iteration k.
- Area constraint A_max.
- Tile dimensions Δx, Δy.

## When To Use

- Inner loop of the SPROUT routing algorithm.
- Any greedy area-constrained graph-densification problem.

## Risks & Pitfalls

- Greedy local choice — needs SmartRefine and subgraph reheating to escape local optima.
- Per-iteration Laplacian solve is the dominant runtime.

## Related Concepts

- [[entities/sprout]]
- [[concepts/smart-refine]]
- [[concepts/node-current-metric]]
- [[concepts/subgraph-reheating]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
