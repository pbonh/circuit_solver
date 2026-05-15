---
title: "H-Tree"
type: concept
tags: [vlsi, clock, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/07-4-synchronization-in-vlsi.txt"]
confidence: high
---

## Definition

An H-tree is a recursively self-similar fractal interconnect structure shaped like the letter H, used as a symmetric balanced clock distribution topology. Every leaf is the same wire-length distance from the root, producing zero clock skew when delay is proportional to length.

## How It Works

Starting from a root point, two horizontal wires extend left and right to two intermediate nodes. From each intermediate node, two vertical wires extend symmetrically; from each new endpoint, smaller H-shaped patterns recurse. After n levels there are 2^n equidistant endpoints. Each branch is sized to drive its downstream capacitance.

## Key Parameters

- Number of levels n (gives 2^n leaves).
- Branch resistance and capacitance per length.
- Buffer insertion strategy.
- Layout area required for full H-tree.

## When To Use

- Highly regular clock distribution for arrays and tiled layouts.
- Reference topology for zero-skew clock distribution.

## Risks & Pitfalls

- Layout constraints (irregular blocks, obstacles) limit applicability.
- Significant area and routing overhead for sparse register distributions.
- Process variation makes literal "zero skew" practically unattainable.

## Related Concepts

- [[concepts/clock-tree-synthesis]]
- [[concepts/clock-distribution-network]]
- [[concepts/deferred-merge-embedding]]

## Sources

- [[summaries/graphs-in-vlsi-07-4-synchronization-in-vlsi]]
