---
title: Chi-Square Test
type: claim
id: claim-chi-square-test
tags:
- statistics
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/13-chapter-9-statistics-and-probability-with-python.txt
confidence:
  base: 0.85
---

## Definition

The chi-square test of association is a non-parametric hypothesis test for whether two categorical variables are independent. It compares observed frequencies in a contingency table against expected frequencies under independence: `X^2 = sum((f_obs - f_exp)^2 / f_exp)`.

## How It Works

Degrees of freedom = `(rows - 1) * (cols - 1)`. The chi-square distribution is right-skewed for small df and approaches normal for large df. SciPy's `stats.chi2_contingency(observations)` returns the test statistic, p-value, degrees of freedom, and expected frequencies. Reject H0 if p < alpha.

## Key Parameters

- Contingency table dimensions
- Cell counts
- Significance level alpha

## When To Use

- Testing association between two categorical variables
- Goodness-of-fit testing against expected distributions
- Survey analysis with categorical responses

## Risks & Pitfalls

- Low expected counts (< 5) invalidate the test (use Fisher's exact)
- Confusing association with causation
- Non-independence between observations breaks the test

## Related Concepts

- [[concepts/hypothesis-testing]]
- [[concepts/probability-distributions]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
