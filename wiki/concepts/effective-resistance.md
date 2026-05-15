---
title: "Effective Resistance"
type: concept
tags: [graph, vlsi, analysis, power-integrity, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt"]
confidence: high
---

## Definition

The effective resistance R_ij between nodes i and j of an electrical network (or weighted graph) is the voltage that develops between them when a unit current is injected at i and drawn at j. Equivalently, it is the metric induced by the graph Laplacian's pseudoinverse: R_ij = e_i^T L^+ e_i + e_j^T L^+ e_j − 2 e_i^T L^+ e_j.

## How It Works

Given a grounded Laplacian L_g, the node-voltage vector under unit current injection satisfies L_g V_g = e_i − e_j, and R_ij = v_i − v_j. Effective resistance is a graph metric (satisfies the triangle inequality), is monotone in edge conductances, and is intimately related to random-walk commute times and spanning-tree counts (matrix-tree theorem).

## Key Parameters

- Edge conductances.
- Distance between source and sink nodes.
- Graph topology and connectivity.

## When To Use

- IR drop analysis on power grids.
- Sparsification (effective-resistance sampling).
- Network reliability and commute-time computations.
- Infinite-grid analyses (closed-form effective resistance is exploited by the Infinity Mirror Technique).

## Risks & Pitfalls

- For very large grids, direct solution via L^+ is expensive; iterative or hierarchical methods are needed.
- Effective resistance is defined only for connected (sub)graphs.

## Related Concepts

- [[concepts/laplacian-matrix]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/ir-drop-analysis]]
- [[concepts/infinity-mirror-technique]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-16-a-green-s-function-for-a-truncated-grid]]
