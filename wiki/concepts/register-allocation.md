---
title: Register Allocation
type: claim
id: concepts/register-allocation
tags:
- vlsi
- digital
- graph
- compiler
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

Register allocation assigns program variables (or hardware datapath signals) to a limited number of physical registers. When the live ranges of two variables overlap, they must occupy different registers; allocation can be formulated as a graph coloring problem on the interference graph.

## How It Works

An interference graph has variables as nodes and edges connecting variables whose live ranges overlap. K-coloring this graph with K = number of available registers gives a valid allocation; uncolorable variables must be spilled to memory. Chaitin's 1981 algorithm modifies greedy coloring with a removal-order heuristic; modern systems use combinations of coloring and linear-scan allocators.

## Key Parameters

- Number of available registers K.
- Spill cost (memory latency penalty).
- Live-range granularity (basic block vs. interprocedural).

## When To Use

- Compiler back-ends for general-purpose CPUs.
- High-level synthesis for VLSI datapath allocation.

## Risks & Pitfalls

- Graph coloring is NP-hard; suboptimal heuristics may force unnecessary spills.
- Spill choice strongly affects program performance.

## Related Concepts

- [[concepts/graph-coloring]]
- [[concepts/interference-graph]]
- [[concepts/register-transfer-level]]
- [[concepts/vlsi-design]]

## Sources

- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
