---
title: "CUDA Programming Model"
type: concept
tags: [gpu, cuda, parallel, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/17-12-statistical-parallel-monte-carlo-analysis-on-gpus.txt"]
confidence: medium
---

## Definition

CUDA (Compute Unified Device Architecture) is NVIDIA's parallel programming model for general-purpose computation on GPUs. Threads are organized into blocks; blocks into grids; each block runs on one streaming multiprocessor and threads in a block share a fast shared-memory region.

## How It Works

A CUDA kernel is launched with grid and block dimensions. Threads execute the kernel function in SIMT (single-instruction multiple-thread) fashion, with warps of 32 threads stepping in lockstep. Coalesced global-memory access requires consecutive thread indices to read consecutive addresses. Texture and constant memory provide cached read-only access for irregular reuse patterns.

## Key Parameters

- Grid and block dimensions (and 1D/2D/3D layout).
- TILE_DIM for shared-memory tiling.
- Memory placement (global, shared, constant, texture).

## When To Use

- Massively parallel numeric workloads with simple data dependencies.
- DDD/BDD evaluation across many samples and frequency points (as in this chapter).
- Linear algebra dense kernels; sparse kernels need extra care.

## Risks & Pitfalls

- Warp divergence on branchy code (DDD irregular structure must be flattened by levelization).
- Memory access patterns dominate performance; un-coalesced reads can be 10-30x slower.
- Double-precision throughput is lower than single-precision on most GPUs.

## Related Concepts

- [[concepts/gpu-parallel-monte-carlo]]
- [[concepts/levelized-ddd]]
- [[concepts/monte-carlo-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-17-12-statistical-parallel-monte-carlo-analysis-on-gpus]]
