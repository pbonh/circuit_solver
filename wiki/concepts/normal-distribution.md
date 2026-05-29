---
title: Normal Distribution
type: claim
id: concepts/normal-distribution
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

The normal distribution is a symmetric bell-shaped continuous distribution parameterized by mean (mu) and standard deviation (sigma). The standard normal has mu=0 and sigma=1. Any normal variable can be standardized via `z = (x - mu) / sigma`.

## How It Works

In a normal distribution mean = median = mode. The empirical rule says about 68% of values fall within one sigma, 95% within two, and 99.8% within three. SciPy provides `stats.norm.cdf` for cumulative probability and `stats.norm.ppf` for the inverse (quantile). Many real-world processes are approximately normal, and the central limit theorem guarantees normality of sample means under broad conditions.

## Key Parameters

- Mean (mu)
- Standard deviation (sigma)
- z-score for standardization

## When To Use

- Modeling natural measurements (height, errors, test scores)
- Underpinning z-tests, t-tests, and confidence intervals
- Approximating other distributions under large samples

## Risks & Pitfalls

- Assuming normality without checking
- Outliers shifting mean / sigma estimates
- Heavy-tailed phenomena (use t or other distributions)

## Related Concepts

- [[concepts/probability-distributions]]
- [[concepts/central-limit-theorem]]
- [[concepts/z-test]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
