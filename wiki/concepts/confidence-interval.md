---
title: Confidence Interval
type: claim
id: claim-confidence-interval
tags:
- statistics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
confidence:
  base: 0.85
---

## Definition

A confidence interval (CI) is a range of values, computed from sample data, that with a stated confidence level (commonly 95%) is believed to contain the unknown population parameter. The width depends on sample size, variability, and the chosen confidence level.

## How It Works

For a known population sigma: `xbar +/- z * sigma / sqrt(n)`. For unknown sigma (small samples): `xbar +/- t * s / sqrt(n)` using the t-distribution and degrees of freedom n-1. SciPy: `stats.norm.interval(0.95, mu, sigma/sqrt(n))` or `stats.t.interval(0.95, df, xbar, stats.sem(data))`.

## Key Parameters

- Confidence level (e.g., 0.95)
- Sample mean and standard deviation
- Sample size and degrees of freedom

## When To Use

- Reporting estimates with uncertainty
- Comparing values to a null hypothesis
- Communicating results to non-technical stakeholders

## Risks & Pitfalls

- Misinterpreting CI as the range containing 95% of the data
- Conflating CI width with practical importance
- Wrong distribution choice (normal vs. t)

## Related Concepts

- [[concepts/hypothesis-testing]]
- [[concepts/central-limit-theorem]]
- [[concepts/normal-distribution]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
