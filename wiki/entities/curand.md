---
title: "CURAND"
type: entity
tags: [tool, gpu, rng, statistical, cuda]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/17-12-statistical-parallel-monte-carlo-analysis-on-gpus.txt"]
confidence: medium
---

## Overview

Per Chapter 12 ("Statistical Parallel Monte Carlo Analysis on GPUs") of "Advanced Symbolic Analysis for VLSI Systems": "Next, in random number assignment, CURAND library is used to generate variations on nominal values of circuit parameters in GPU kernel function. We need to make sure that one device variation, which may appear in 4 position in the MNA will take the same value and this also reflect on the f the four DDD nodes will reflect the same change. This is done in Line 2 and Line 3 of the pseudo-code in Algorithm 4. The variations introduced in our experiments are Gaussian random values, whose means and deviations can be specified by users from input netlist." cuRAND is NVIDIA's GPU-resident random-number-generation library, part of the CUDA toolkit.

## Characteristics

- Stream-based generator state replicated per thread.
- Multiple distributions (uniform, normal, lognormal, Poisson).
- Used to draw correlated/uncorrelated Gaussian samples for Monte Carlo.

## Common Strategies

- Generate samples inside the same kernel that evaluates the DDD to avoid memory traffic.
- Seed-per-thread management for reproducibility.

## Related Entities

- [[entities/nvidia-cuda]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-17-12-statistical-parallel-monte-carlo-analysis-on-gpus]]
