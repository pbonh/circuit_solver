---
title: ANOVA
type: claim
id: concepts/anova
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

ANOVA (analysis of variance) compares the means of three or more populations using the F-distribution. The F-statistic is the ratio of variation between sample means to variation within samples; large values indicate the groups differ.

## How It Works

The null hypothesis is `mu1 = mu2 = ... = muk`; the alternative is that at least one mean differs. SciPy's `stats.f_oneway(group1, group2, group3, ...)` returns the F-statistic and p-value. Reject H0 if p < alpha. ANOVA does not say which means differ — follow-up post-hoc tests (Tukey, Bonferroni) are needed for that.

## Key Parameters

- Number of groups
- Sample sizes per group
- Significance level alpha
- Assumed equal variance across groups

## When To Use

- Comparing more than two treatments
- Factorial designs and experimental analyses
- Avoiding inflated alpha from pairwise t-tests

## Risks & Pitfalls

- Sensitivity to unequal variances
- Non-normal residuals
- Need post-hoc tests after a significant result

## Related Concepts

- [[concepts/hypothesis-testing]]
- [[concepts/t-test]]
- [[concepts/probability-distributions]]

## Sources

- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
