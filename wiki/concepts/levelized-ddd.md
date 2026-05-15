---
title: "Levelized DDD (GPU-Friendly Layout)"
type: concept
tags: [ddd, gpu, parallel, data-structure, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/17-12-statistical-parallel-monte-carlo-analysis-on-gpus.txt"]
confidence: medium
---

## Definition

A levelized DDD is a re-layout of a Determinant Decision Diagram in which nodes are sorted by their level (longest distance to the 1-terminal) and stored in contiguous arrays per level. This layout enables coalesced GPU memory access and warp-aligned parallel evaluation.

## How It Works

Compute each node's level via DFS. Allocate one contiguous array per level holding (v_self, left_child_index, right_child_index, sign). Terminal nodes use sentinel indices (-1 for 1-terminal, -2 for 0-terminal). Evaluation proceeds level by level from the deepest: at each level, all nodes can be evaluated in parallel because their children have already been computed.

## Key Parameters

- Number of levels (usually << number of nonzero matrix entries).
- TILE_DIM and grid dimensions (chapter uses 256 threads per block for frequency-axis parallelism).

## When To Use

- GPU/multi-core parallel DDD evaluation across many Monte Carlo samples and frequency points.
- Any DAG evaluation kernel where dependency depth is shallow and breadth is large.

## Risks & Pitfalls

- Storage overhead: arrays are padded per level.
- Conversion from a tree-linked DDD to the levelized form is sequential on CPU; only the evaluation is GPU-parallel.

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/gpu-parallel-monte-carlo]]
- [[concepts/cuda-programming-model]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-17-12-statistical-parallel-monte-carlo-analysis-on-gpus]]
