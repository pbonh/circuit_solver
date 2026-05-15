---
title: "Multilayer Routing"
type: concept
tags: [vlsi, routing, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping.txt"]
confidence: low
---

## Definition

Multilayer routing synthesizes wire connections across multiple physical layers of an IC or PCB, exploiting via transitions to circumvent layer congestion. In SPROUT's context, when single-layer available space is disjoint, multilayer routing decomposes the problem into per-layer routing problems linked by vias.

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
