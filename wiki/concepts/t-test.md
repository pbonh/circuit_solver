---
title: T-Test
type: claim
id: concepts/t-test
tags:
- statistics
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

A t-test is a parametric hypothesis test for population means when the sample size is small (typically n < 30) and the population standard deviation is unknown. It uses the t-statistic from Student's t-distribution.

## How It Works

One-sample: `t = (xbar - mu) / (s / sqrt(n))`, df = n - 1. Two-sample (independent): uses pooled variance with df = n1 + n2 - 2. Paired (dependent samples): uses the mean of differences over their standard deviation. SciPy exposes `stats.ttest_1samp`, `stats.ttest_ind`, and `stats.ttest_rel`. Compare the returned p-value to alpha to decide.

## Key Parameters

- Sample size(s) and standard deviation(s)
- Degrees of freedom
- Pairing (independent vs. matched)
- Significance level alpha

## When To Use

- Small-sample mean comparisons
- Pre/post measurement designs (paired)
- Comparing two independent groups

## Risks & Pitfalls

- Heavy violations of normality
- Unequal variances (consider Welch's t-test)
- Paired vs. independent confusion

## Related Concepts

- [[concepts/hypothesis-testing]]
- [[concepts/z-test]]
- [[concepts/anova]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
