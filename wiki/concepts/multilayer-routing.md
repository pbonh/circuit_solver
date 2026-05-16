---
title: "Multilayer Routing"
type: concept
tags: [vlsi, routing, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt"]
confidence: medium
---

## Definition

GraphsInVLSI Chapter 10 routes the power network across the board's metal layers and treats single-layer routing as a special case. From Sect. 10.1: "After [polygon] removal, the available space on each layer may become disjoint, leaving no valid path between terminals on the same layer ... In this case, routing is accomplished using multiple layers." A "modification of SPROUT to support multilayer routing is presented in the Appendix C" of the book.

## How It Works

A canonical decomposition partitions the multilayer connectivity into single-layer subproblems. Each subproblem is solved independently using single-layer routing (Dijkstra, A*, maze routing, or SPROUT's SmartGrow/SmartRefine). Vias connect the partial solutions at coordinates determined by global layer-assignment.

## Key Parameters

- Number of layers.
- Via cost (impedance, area).
- Per-layer routing-resource budget.

## When To Use

- PCB and IC routing when terminals cannot be connected on a single layer.
- Power and ground plane design with stacked metal layers.

## Risks & Pitfalls

- Via inductance and resistance impact power integrity.
- Inter-layer congestion can be hard to balance.

## Related Concepts

- [[concepts/board-level-routing]]
- [[concepts/interconnect-routing]]
- [[entities/sprout]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
- [[summaries/graphs-in-vlsi-18-c-multilayer-routing-algorithm]]
