---
title: Timing Graph
type: claim
id: concepts/timing-graph
tags:
- vlsi
- digital
- graph
- synchronization
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

A timing graph is a directed graph in which nodes represent synchronous elements (flip-flops, latches, clocked gates) and edges represent combinational datapaths between them. Each edge typically carries minimum and maximum propagation delays.

## How It Works

Timing analysis traverses the timing graph computing arrival times at each register, comparing against setup and hold constraints, and identifying critical paths. Clock skew scheduling assigns non-uniform clock arrival times by solving a linear or quadratic program on the timing graph subject to permissible-range constraints PR_ij = [l_ij, u_ij] per datapath.

## Key Parameters

- Number of clocked elements and datapaths.
- Min/max delays per edge.
- Setup and hold times at each clocked element.
- Target clock period.

## When To Use

- Static timing analysis (STA) sign-off.
- Clock skew scheduling and clock tree synthesis.
- Performance optimization of synchronous digital circuits.

## Risks & Pitfalls

- Variation-induced delays can perturb the schedule.
- Hold violations on short paths are easy to overlook compared with setup violations.

## Related Concepts

- [[concepts/clock-skew-scheduling]]
- [[concepts/clock-tree-synthesis]]
- [[concepts/clock-distribution-network]]
- [[concepts/graph-theory]]
- [[concepts/directed-acyclic-graph]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
