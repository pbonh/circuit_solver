---
title: Monte Carlo Analysis
type: claim
id: concepts/monte-carlo-analysis
tags:
- monte-carlo
- statistical
- process-variation
- analog
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/00-preface.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Monte Carlo analysis is a statistical method that samples parameter values from their joint distribution, runs a circuit simulation for each sample, and aggregates the resulting performance values to estimate distributions, yields, or tail probabilities.

## How It Works

Random parameter vectors are drawn (often after Cholesky decorrelation of correlated Gaussians). For each sample, a numerical (SPICE) or symbolic (DDD-evaluation) solver computes the performance metric. Statistics are aggregated incrementally; rare events may require importance sampling or specialized acceleration.

## Key Parameters

- Sample count vs. accuracy/confidence trade-off.
- Sampling strategy (plain, Latin hypercube, importance sampling, quasi-MC).
- Solver per sample (numeric SPICE vs. symbolic DDD re-evaluation).

## When To Use

- Yield estimation under process variation.
- Validation of analytic statistical models.
- High-sigma estimation when accelerated by symbolic methods or importance sampling.

## Risks & Pitfalls

- Convergence for rare events is impractically slow without acceleration.
- Garbage-in/garbage-out for the input parameter distribution and correlation.

## Related Concepts

- [[concepts/process-variation]]
- [[concepts/symbolic-analysis]]
- [[concepts/determinant-decision-diagram]]
- [[concepts/gpu-parallel-monte-carlo]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-00-preface]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-17-12-statistical-parallel-monte-carlo-analysis-on-gpus]]
- [[summaries/modeling-simulation-systems-23-19-devs-support-for-markov-modeling-and-simulation]]
