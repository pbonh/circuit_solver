---
title: "Probability Distributions"
type: concept
tags: [statistics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt"]
confidence: high
---

## Definition

A probability distribution assigns likelihoods to the outcomes of a random variable. Discrete distributions use a probability mass function (PMF); continuous distributions use a probability density function (PDF). The cumulative distribution function (CDF) gives the probability of being less than or equal to a given value.

## How It Works

For discrete variables (Likert scores, dice rolls, counts) the PMF gives exact probabilities. For continuous variables (height, weight, temperature) the PDF gives density; absolute probabilities are computed by integrating over ranges or via the CDF. SciPy exposes both via `stats.<dist>.pmf` / `.pdf` and `.cdf`.

## Key Parameters

- Distribution family (binomial, Poisson, normal, t, chi-square, F, ...)
- Family-specific parameters (n, p, lambda, mu, sigma)
- Support (range over which probabilities are non-zero)

## When To Use

- Modeling uncertainty in observed phenomena
- Foundation for hypothesis tests and confidence intervals
- Generating synthetic data via sampling

## Risks & Pitfalls

- Mis-specifying the distribution family
- Using PMF formulas on continuous variables (or vice versa)
- Ignoring boundary effects

## Related Concepts

- [[concepts/probability]]
- [[concepts/normal-distribution]]
- [[concepts/binomial-distribution]]
- [[concepts/poisson-distribution]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
