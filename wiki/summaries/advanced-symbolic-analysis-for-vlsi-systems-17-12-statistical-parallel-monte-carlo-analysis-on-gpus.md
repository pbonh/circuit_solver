---
title: "Advanced Symbolic Analysis for VLSI Systems — Chapter 12: Statistical Parallel Monte-Carlo Analysis on GPUs"
type: summary
tags: [gpu, monte-carlo, ddd, statistical, parallel, advanced, vlsi, cuda]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/17-12-statistical-parallel-monte-carlo-analysis-on-gpus.txt"]
confidence: high
---

## Key Points

- Statistical analog/mixed-signal analysis under process variation needs Monte Carlo; CMOS mismatch roughly doubles per process node below 90 nm. Traditional SPICE MC is prohibitively slow.
- GPU peak FLOPS (Kepler K20X: 4 TFLOPS over 2688 cores) far exceed CPU (Intel i7 quad-core: 80-100 GFLOPS); however SPICE-style LU factorization is hard to parallelize on GPUs due to irregular memory access.
- DDD-based numerical evaluation is fundamentally a depth-first DAG traversal where each node depends on only two children — ideal for massively parallel GPU evaluation across (i) DDD nodes at the same level, (ii) Monte Carlo samples, and (iii) frequency points.
- CUDA model: threads -> blocks -> grids; host (CPU) and device (GPU) have separate DRAM; coalesced memory access requires consecutive thread indices to map to consecutive addresses.
- The chapter contributes a continuous, levelized DDD representation: DDD nodes sorted by level (longest distance to 1-terminal); within a level, nodes stored in contiguous arrays of (value, left_child_index, right_child_index, level, sign). This enables coalesced reads and minimizes branch divergence — yields 2-3x GPU speedup over a non-levelized layout.
- CUDA grid dimension `N_MC x |DDD|`; each block holds `TILE_DIM = 256` threads, one per frequency point. So one kernel launch evaluates one DDD node, for one Monte Carlo sample, across 256 frequencies simultaneously.
- Random parameter assignment uses CURAND on-device; care is taken so the same physical device variation propagates to all (up to 4) MNA stamp positions and into the corresponding DDD nodes via tracked stamp patterns held in GPU texture memory.
- Parallel DDD evaluation algorithm: outer host loop over levels; inner GPU loops over Monte Carlo samples and DDD nodes at that level evaluate `v_tree[i] = v_self[i] + sign * top * v_tree_child1[i]` (or the appropriate DDD recurrence) for all frequencies in parallel.
- Memory budget for muA741 (6205 DDD nodes, 2400 frequency points) on a Tesla C2070: 20 MC samples per kernel; for more runs the host re-launches. Total speedup reported: 1-2 orders of magnitude over serial CPU DDD evaluation, and 2-3x over numerical SPICE-based Monte Carlo on larger benchmarks.
- The same parallelization template applies to other BDD-based applications (logic synthesis, formal verification) — the chapter positions DDD-on-GPU as a generic decision-diagram evaluation framework.

## Relevant Concepts

- [[concepts/gpu-parallel-monte-carlo]] — central method of the chapter.
- [[concepts/determinant-decision-diagram]] — DDD is the symbolic substrate.
- [[concepts/monte-carlo-analysis]] — statistical workload that DDD-on-GPU accelerates.
- [[concepts/process-variation]] — application driver.
- [[concepts/cuda-programming-model]] — GPU programming abstraction used.
- [[concepts/levelized-ddd]] — chapter's key data-structure innovation for coalesced GPU memory access.
- [[concepts/modified-nodal-analysis]] — provides the matrix whose determinant the DDD represents.
- [[entities/nvidia-cuda]] — GPU runtime/library.
- [[entities/curand]] — RNG library used for parameter sampling.

## Source Metadata

- Source type: book chapter
- Book title: Advanced Symbolic Analysis for VLSI Systems
- Chapter: 12 — Statistical Parallel Monte-Carlo Analysis on GPUs
- File path: `raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/17-12-statistical-parallel-monte-carlo-analysis-on-gpus.txt`
- Author: Sheldon X.-D. Tan
