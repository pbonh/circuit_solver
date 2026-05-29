---
title: Z-Test
type: claim
id: concepts/z-test
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

A z-test is a parametric hypothesis test for population means or proportions when the sample size is large (n > 30), the population is approximately normal, and the population standard deviation is known. It uses the z-statistic from the standard normal distribution.

## How It Works

One-sample mean: `z = (xbar - mu) / (sigma / sqrt(n))`. Two-sample mean: `z = (xbar1 - xbar2) / sqrt(sigma1^2/n1 + sigma2^2/n2)`. Proportion z-tests use the proportion formulas. The p-value comes from `stats.norm.cdf(z)`; compare to alpha=0.05 (typical). Reject H0 if calculated test statistic exceeds critical value (or p < alpha).

## Key Parameters

- Sample mean(s) and size(s)
- Known population standard deviation(s)
- Significance level alpha
- One-tail vs. two-tail

## When To Use

- Large-sample mean comparison with known sigma
- Proportion comparisons in surveys
- Quality control with established population variance

## Risks & Pitfalls

- Sigma rarely known in practice (use t-test instead)
- Small samples violate the normality assumption
- Confusing one-tail and two-tail rejection regions

## Related Concepts

- [[concepts/hypothesis-testing]]
- [[concepts/t-test]]
- [[concepts/normal-distribution]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
