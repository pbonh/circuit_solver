---
title: "Hypothesis Testing"
type: concept
tags: [statistics, data-analysis, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/04-introduction.txt"]
confidence: medium
---

## Definition

Hypothesis testing is a statistical framework for deciding whether observed data provides sufficient evidence to reject a null hypothesis in favor of an alternative. The book covers ANOVA, chi-squared, z-test, and t-test as primary instances.

## How It Works

A practitioner formulates null (H0) and alternative (H1) hypotheses, picks a significance level (alpha), selects an appropriate test statistic for the data type (continuous vs. categorical) and design (one vs. two samples, paired vs. independent), computes a p-value, and decides whether to reject H0. SciPy provides Python implementations of these tests.

## Key Parameters

- Significance level (alpha)
- Test statistic and its distribution under H0
- One-sided vs. two-sided alternative
- Effect size and power

## When To Use

- Comparing group means (t-test, ANOVA)
- Testing relationships between categorical variables (chi-squared)
- Large-sample mean comparisons with known variance (z-test)

## Risks & Pitfalls

- p-hacking from running many tests without correction
- Conflating statistical significance with practical importance
- Ignoring test assumptions (normality, independence)

## Related Concepts

- [[concepts/probability]]
- [[concepts/descriptive-statistics]]
- [[entities/scipy]]

## Sources

- [[summaries/python-data-analysts-toolkit-04-introduction]]
- [[summaries/python-data-analysts-toolkit-13-chapter-9-statistics-and-probability-with-python]]
