---
title: "Load Clustering"
type: concept
tags: [vlsi, power-integrity, optimization, novel]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt"]
confidence: low
---

## Definition

Load clustering is a power-grid preprocessing technique that aggregates many small current loads into a smaller number of representative "cluster" loads, each placed at the centroid of its constituent loads. The total current and approximate spatial distribution are preserved while the load-count input to per-iteration optimization is reduced by orders of magnitude.

## How It Works

Loads within a circuit are spatially clustered (e.g., k-means) into N representative locations. Because the power grid is smooth — small position perturbations produce small voltage perturbations — the minimum and average voltages within the grid are essentially unchanged for clustering ratios well below 1%. The reduced load set is then used in inner-loop voltage-regulator placement optimization.

## Key Parameters

- Number of clusters N.
- Clustering algorithm (k-means, hierarchical).
- Trade-off between cluster fidelity and optimization runtime.

## When To Use

- As preprocessing for any optimization that repeatedly evaluates a function of the load set on a smooth power grid.
- IR drop sign-off where modest accuracy loss is acceptable.

## Risks & Pitfalls

- Over-clustering (e.g., < 1% of loads) materially degrades minimum-voltage estimates.
- Spatial smoothness assumption can fail at chip edges or near current spikes.

## Related Concepts

- [[concepts/voltage-regulator-placement]]
- [[concepts/power-distribution-network]]
- [[concepts/ir-drop-analysis]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
