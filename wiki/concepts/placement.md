---
title: Placement
type: claim
id: concepts/placement
tags:
- vlsi
- physical-design
- graph
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Placement determines the precise locations of standard cells and macro blocks within a VLSI layout, subject to physical design rules and minimizing metrics such as total wirelength, routing congestion, timing criticality, and power.

## How It Works

Modern placers run in stages: global placement (analytic or quadratic methods spread cells), legalization (snap to legal sites), and detailed placement (local swaps and shifts). Net wirelength is commonly estimated via Half-Perimeter Wire Length (HPWL) and rectilinear Steiner trees on the Hanan grid. Congestion maps and A*-based traversal guide cell movement.

## Key Parameters

- Number of cells and nets.
- Routing-track availability.
- Timing critical-path delay budgets.
- Power and IR drop targets.

## When To Use

- Between floorplanning and routing in the physical-design flow.
- Iteratively as part of timing-driven flows where placement is rerun to close timing.

## Risks & Pitfalls

- Poor placement creates routing congestion that may be infeasible.
- Wirelength minimization alone can produce timing-critical configurations.

## Related Concepts

- [[concepts/floorplanning]]
- [[concepts/interconnect-routing]]
- [[concepts/steiner-minimal-tree]]
- [[concepts/hanan-grid]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
