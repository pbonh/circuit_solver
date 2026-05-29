---
title: Voltage Regulator Placement
type: claim
id: concepts/voltage-regulator-placement
tags:
- vlsi
- power-integrity
- optimization
- novel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Voltage regulator placement is the optimization problem of selecting the on-chip locations of a fixed number of distributed voltage regulators so as to minimize the worst-case voltage drop within the power distribution network (PDN), subject to whitespace and current-capacity constraints.

## How It Works

Given a PDN modeled as a resistive mesh, a set of loads L = {(x_p, I_p)}, and a target number m of regulators, the objective is min_S max load voltage drop where S = {(x_q, V_q)}. The objective is evaluated using the Infinity Mirror Technique (constant time per node pair) and load clustering (orders-of-magnitude reduction in load count). Convexity is unknown, so a global optimizer such as Discrete Particle Swarm Optimization is used. Each regulator's current is solved by linear system + iterative correction when current limits are exceeded.

## Key Parameters

- Number of regulators m.
- Grid size and pitch.
- Per-regulator maximum current.
- Whitespace map / blockage regions.

## When To Use

- Power-grid design for SoCs adopting heterogeneous power delivery.
- Sign-off optimization combined with IR drop budgeting.

## Risks & Pitfalls

- Restricted placement can substantially increase voltage drop (~10% in case studies).
- Current-limited regulators must be handled iteratively.

## Related Concepts

- [[concepts/on-chip-voltage-regulator]]
- [[concepts/heterogeneous-power-delivery]]
- [[concepts/infinity-mirror-technique]]
- [[concepts/particle-swarm-optimization]]
- [[concepts/load-clustering]]
- [[concepts/ir-drop-analysis]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
- [[summaries/graphs-in-vlsi-15-12-conclusions]]
