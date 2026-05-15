---
title: "Deferred Merge Embedding (DME)"
type: concept
tags: [vlsi, clock, routing, algorithm, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: high
---

## Definition

Deferred Merge Embedding (DME) is an algorithm for embedding a clock-tree topology into a physical layout to achieve zero (or bounded) clock skew while minimizing total wirelength. Assuming linear-with-length delay and Manhattan routing, each pair of clock leaves is merged along a tilted-rectangular (TRR) "merging segment" — the locus of points equidistant in delay from both leaves.

## How It Works

DME processes the tree bottom-up. For each pair of children, it computes a merging segment whose points are equally delayed from the two children. Subsequent levels merge these segments with their siblings, producing higher-level merging regions. After the root is reached, a top-down pass selects exact merge points to minimize total wirelength. Extensions handle Elmore delay (RC trees), buffered trees, octilinear merging regions for bounded skew, and useful skew.

## Key Parameters

- Delay model (linear, Elmore, higher-order).
- Skew bound s_max.
- Wirelength target.
- Manhattan vs. octilinear geometry.

## When To Use

- Standard embedding step in clock tree synthesis flows.
- Zero-skew and bounded-skew clock-tree generation.

## Risks & Pitfalls

- Linear-delay assumption is inaccurate for long buffered RC paths.
- Cannot exploit useful skew without extensions.
- Layout obstacles may prevent ideal merging segments.

## Related Concepts

- [[concepts/clock-tree-synthesis]]
- [[concepts/elmore-delay]]
- [[concepts/clock-distribution-network]]
- [[concepts/clock-skew-scheduling]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
