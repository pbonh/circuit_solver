---
title: "Correlation"
type: concept
tags: [statistics, data-analysis, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/DataAnalysisAndVisualizationsPython/_txt/04-chapter-1-introduction-to-data-science-with-python.txt"]
confidence: medium
---

## Definition

Correlation is a statistical measure of the degree to which two variables move together. The book demonstrates pairwise correlations in the Iris dataset using Seaborn's `pairplot` to inspect sepal and petal length-vs-width relationships across three species.

## How It Works

Common metrics include Pearson's correlation coefficient (linear), Spearman's rank correlation (monotonic), and Kendall's tau. Values range from -1 (perfectly negative) through 0 (no linear relationship) to +1 (perfectly positive). Pairplots provide a visual matrix of scatter plots to reveal correlation patterns across many variables.

## Key Parameters

- Choice of coefficient (Pearson, Spearman, Kendall)
- Sample size affecting significance
- Variable scaling and outlier treatment

## When To Use

- Exploratory analysis to identify candidate predictors
- Feature selection before model building
- Detecting redundancy among variables

## Risks & Pitfalls

- Correlation does not imply causation
- Pearson's coefficient measures linear relationships only
- Spurious correlations from confounders or small samples

## Related Concepts

- [[concepts/linear-regression]]
- [[concepts/descriptive-statistics]]
- [[concepts/data-visualization]]

## Sources

- [[summaries/data-analysis-visualizations-python-04-chapter-1-introduction-to-data-science-with-python]]
- [[summaries/data-analysis-visualizations-python-09-chapter-6-data-exploring-and-analysis]]
- [[summaries/data-analysis-visualizations-python-10-chapter-7-data-visualization]]
