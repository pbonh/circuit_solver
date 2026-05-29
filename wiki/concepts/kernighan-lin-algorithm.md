---
title: Kernighan-Lin Algorithm
type: claim
id: claim-kernighan-lin-algorithm
tags:
- graph
- algorithm
- partitioning
- vlsi
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/06-3-graphs-in-vlsi-circuits-and-systems.txt
confidence:
  base: 0.85
---

## Definition

The Kernighan-Lin (KL) algorithm (1970) is a heuristic for balanced graph bipartitioning that iteratively swaps pairs of vertices between two equal-size partitions to minimize the cut size. It is considered the first partitioning algorithm applied to integrated-circuit design.

## How It Works

Starting from an arbitrary equal-size bipartition (A, B), define for each vertex v: I(v) = number of neighbors in its own partition, E(v) = number of neighbors in the other partition, D(v) = E(v) − I(v). The swap gain for pair (a, b) is G_ab = D_a + D_b − 2c_ab where c_ab = 1 if (a,b) is an edge else 0. Each iteration: compute all G_ab, swap the pair with maximum gain, lock both nodes, repeat until all nodes are locked. The partition with the smallest cut size encountered is the output. One pass runs in O(|V|^3); a full KL run typically does multiple passes.

## Key Parameters

- Initial partition choice.
- Number of passes.
- Equal-size constraint (no balance slack).

## When To Use

- Classical baseline for graph and hypergraph partitioning.
- Educational benchmark and starting point for modern multilevel methods.

## Risks & Pitfalls

- Local optimum dependence on initial partition.
- Strict equal-size constraint limits applicability.
- Cubic per-pass cost is prohibitive at modern scale; FM and multilevel methods are preferred.

## Related Concepts

- [[concepts/graph-partitioning]]
- [[concepts/fiduccia-mattheyses-algorithm]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
