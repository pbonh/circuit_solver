---
title: "Clock Tree Synthesis"
type: concept
tags: [vlsi, digital, clock, synchronization, graph, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: high
---

## Definition

Clock tree synthesis (CTS) is the EDA process that designs the clock distribution network of a synchronous IC. It generates an abstract clock tree topology and embeds it as a physical layout that delivers the clock signal to every register with controlled skew, slew, jitter, and power.

## How It Works

CTS typically proceeds in two stages: (1) topological synthesis — produce an abstract tree (bottom-up via nearest-neighbor merging or top-down via recursive bipartitioning) using clock arrival times and register locations; (2) embedding — determine physical placement of buffers and routes via algorithms such as Method of Means and Medians (MMM), Deferred Merge Embedding (DME), bounded-skew variants (BST), and useful-skew variants (UST). Modern flows use Elmore or higher-order delay models to estimate propagation delay.

## Key Parameters

- Target clock period and skew bound.
- Number of registers and tree depth.
- Allowed buffer sizes / drive strengths.
- Wirelength and power budgets.

## When To Use

- Required at the end of physical-design flow before sign-off in every synchronous digital IC.
- Specialized variants exist for SFQ circuits (QuCTS) and 3D ICs.

## Risks & Pitfalls

- Mismatch between estimated and actual delay degrades skew control.
- Clock power dominates total chip power if buffer placement is not optimized.
- Layout obstacles can prevent ideal merging segments.

## Related Concepts

- [[concepts/clock-distribution-network]]
- [[concepts/clock-skew-scheduling]]
- [[concepts/deferred-merge-embedding]]
- [[concepts/method-of-means-and-medians]]
- [[concepts/h-tree]]
- [[concepts/elmore-delay]]
- [[entities/qucts]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
- [[summaries/graphs-in-vlsi-14-11-qucts-single-flux-quantum-clock-tree-synthesis]]
- [[summaries/graphs-in-vlsi-15-12-conclusions]]
