---
title: Node Current Metric
type: claim
id: concepts/node-current-metric
tags:
- vlsi
- routing
- graph
- novel
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

The node current metric is a scalar quantity assigned to each node of a routing subgraph that quantifies the total current carried by the edges incident to that node. In SPROUT it serves as a heuristic gradient guiding where to add (high current) or remove (low current) nodes to reduce subgraph impedance under an area constraint.

## How It Works

For a subgraph Γ_ns with grounded Laplacian L and current injection matrix E (one column per terminal pair), node voltages V = L^{-1} E and edge currents J = B V (B is the directed incidence matrix). The current per edge across all terminal pairs is summed; the node current J_p is the sum over neighbors j of g_pj |V_i − V_j|. Nodes with high J_p are reinforced (SmartGrow); nodes with low J_p are removed (SmartRefine).

## Key Parameters

- Per-terminal-pair injected current magnitude.
- Graph weights (inter-tile conductances).
- Aggregation rule (sum or max over pairs).

## When To Use

- Iterative power-routing algorithms minimizing impedance under area constraints.
- Any gradient-style optimization on resistive graphs.

## Risks & Pitfalls

- Requires repeated Laplacian solves (main runtime cost of SPROUT).
- Pure local heuristic — does not guarantee global optimum without reheating.

## Related Concepts

- [[entities/sprout]]
- [[concepts/smart-grow]]
- [[concepts/smart-refine]]
- [[concepts/laplacian-matrix]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
