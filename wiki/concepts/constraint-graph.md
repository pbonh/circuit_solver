---
title: Constraint Graph
type: claim
id: claim-constraint-graph
tags:
- graph
- vlsi
- timing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt
confidence:
  base: 0.85
---

## Definition

A constraint graph is a directed weighted graph representing a system of difference constraints x_i − x_j ≤ b_ji. In VLSI timing it is derived from a timing graph by adding (1) one edge per datapath with weight derived from the hold-time constraint and (2) one reverse edge per datapath weighted by the setup-time constraint that depends on clock period T_CP, plus zero-weight edges from a virtual source v_0 to every node.

## How It Works

A feasible clock arrival time t assignment exists if and only if the constraint graph contains no negative cycle (or, equivalently, the corresponding system of difference constraints is satisfiable). The Bellman-Ford algorithm both detects negative cycles and produces shortest-path-based feasible solutions. The minimum clock period T_CP^min corresponds to a zero-weight cycle in the constraint graph; further reductions require delay insertion or physical modification of datapaths.

## Key Parameters

- Number of registers and datapaths.
- Setup and hold times.
- Min/max combinational delays per datapath.
- Clock period parameterizing the setup-edge weights.

## When To Use

- Clock skew scheduling formulated as a system of difference constraints.
- General difference-constraint problems in compiler scheduling and resource allocation.

## Risks & Pitfalls

- Sensitivity to delay-estimate errors near zero-weight cycles.
- Cycle detection cost grows with graph size; sparsification or partitioning may be required.

## Related Concepts

- [[concepts/timing-graph]]
- [[concepts/clock-skew-scheduling]]
- [[concepts/bellman-ford-algorithm]]
- [[concepts/permissible-range]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
