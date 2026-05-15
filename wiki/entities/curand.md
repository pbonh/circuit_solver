---
title: "CURAND"
type: entity
tags: [tool, gpu, rng, statistical, cuda]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/17-12-statistical-parallel-monte-carlo-analysis-on-gpus.txt"]
confidence: low
---

## Overview

CURAND is NVIDIA's GPU-resident random-number-generation library, part of the CUDA toolkit. It supports several pseudo- and quasi-random generators (XORWOW, MRG32k3a, Sobol, Philox) and is used by the chapter for on-device parameter sampling without host-device round trips.

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
