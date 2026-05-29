---
title: Statistical Timing Analysis (SSTA)
type: claim
id: concepts/statistical-timing-analysis
tags:
- statistical
- timing
- process-variation
- vlsi
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/15-10-symbolic-moment-computation.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Statistical Static Timing Analysis treats arrival times, delays, and slacks as random variables (typically modeled by Gaussian or canonical-form distributions) and propagates them through the timing graph to produce timing-distribution estimates rather than single corner values.

## How It Works

Each gate and interconnect delay is decomposed into a nominal value plus contributions from systematic, random, and intra-die variation. Propagation through MAX/MIN nodes is the chief technical challenge (max of Gaussians is non-Gaussian). Symbolic moment computation of interconnect delays gives a closed-form parameter dependence per net, used as input to the gate-level SSTA engine.

## Key Parameters

- Variation source decomposition (Vth, Leff, R, C).
- Correlation structure across the die.
- Moment order retained for net delay.

## When To Use

- High-volume yield-aware sign-off.
- Design centering and budget allocation under tight margins.

## Risks & Pitfalls

- MAX/MIN approximation errors compound across long paths.
- Correlation modeling between layers and intra-net is hard to do right.

## Related Concepts

- [[concepts/process-variation]]
- [[concepts/symbolic-moment-computation]]
- [[concepts/elmore-delay]]
- [[concepts/monte-carlo-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-15-10-symbolic-moment-computation]]
