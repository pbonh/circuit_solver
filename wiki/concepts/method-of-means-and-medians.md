---
title: "Method of Means and Medians (MMM)"
type: concept
tags: [vlsi, clock, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: medium
---

## Definition

The Method of Means and Medians (MMM) is one of the earliest top-down algorithms for asymmetric zero-skew clock-tree synthesis. It recursively partitions the set of clock-sink locations and routes the clock from the source toward the center of mass (mean) of each cluster.

## How It Works

Given a set of sink coordinates, MMM computes the center-of-mass (x_c, y_c) of the cluster, then bipartitions the set (e.g., along x or y coordinate at the median). The clock tree extends a branch to each subset's center of mass. Recursion continues until each cluster is a single sink. The interconnect from one parent to each child equalizes delay if propagation delay is proportional to length.

## Key Parameters

- Number of sinks.
- Partition axis selection (x vs y, alternating).
- Routing geometry (Manhattan).

## When To Use

- Baseline top-down CTS reported in early literature.
- Reference algorithm for comparison with DME and modern methods.

## Risks & Pitfalls

- Larger total wirelength than DME-based bottom-up methods.
- Does not natively exploit useful skew.

## Related Concepts

- [[concepts/clock-tree-synthesis]]
- [[concepts/deferred-merge-embedding]]
- [[concepts/clock-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
