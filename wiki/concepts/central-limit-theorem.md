---
title: Central Limit Theorem
type: claim
id: concepts/central-limit-theorem
tags:
- statistics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The central limit theorem states that the distribution of sample means drawn from any population approaches a normal distribution as the sample size grows. The mean of this sampling distribution equals the population mean; its standard deviation (standard error) is `sigma / sqrt(n)`.

## How It Works

Even if the population is non-normal, repeated sampling and averaging produces means that cluster around mu. Larger n yields tighter clusters (smaller standard error). The chapter uses this to justify z-tests and confidence intervals for population means estimated from samples.

## Key Parameters

- Sample size n
- Population standard deviation sigma (or sample s)
- Number of samples drawn

## When To Use

- Inference about population means from sample means
- Constructing confidence intervals
- Justifying parametric tests under non-normal populations (large n)

## Risks & Pitfalls

- Convergence is slow for highly skewed populations
- Standard error shrinks only by sqrt(n) — diminishing returns
- Assumes independent samples

## Related Concepts

- [[concepts/normal-distribution]]
- [[concepts/sampling-methods]]
- [[concepts/confidence-interval]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
