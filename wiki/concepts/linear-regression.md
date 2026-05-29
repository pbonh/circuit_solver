---
title: Linear Regression
type: claim
id: concepts/linear-regression
tags:
- statistics
- regression
- data-analysis
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/04-chapter-1-introduction-to-data-science-with-python.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Linear regression models the relationship between a dependent variable and one or more independent variables as a straight line; both variables enter the equation to the first power. When plotted, the fit appears as a line, while exponents other than one produce curves.

## How It Works

The model fits parameters (slope and intercept for simple regression, or a coefficient vector for multiple regression) by minimizing the squared residuals between observed and predicted values. The book illustrates this with Seaborn's `regplot` applied to the built-in Tips dataset to relate total bill to tip.

## Key Parameters

- Predictor and response variables
- Optimization criterion (typically least squares)
- Confidence/prediction intervals around the fitted line

## When To Use

- Quantifying linear relationships between continuous variables
- Producing baseline predictive models before trying nonlinear methods
- Visualizing trend strength and direction in exploratory analysis

## Risks & Pitfalls

- Assumes linearity, independence, and homoscedastic residuals
- Sensitive to outliers and influential observations
- Misleading R-squared when the underlying relationship is nonlinear

## Related Concepts

- [[concepts/correlation]]
- [[concepts/descriptive-statistics]]
- [[concepts/data-visualization]]

## Sources

- [[summaries/data-analysis-visualizations-python-04-chapter-1-introduction-to-data-science-with-python]]
- [[summaries/data-analysis-visualizations-python-10-chapter-7-data-visualization]]
