---
title: Missing Data Handling
type: claim
id: concepts/missing-data-handling
tags:
- data-analysis
- pandas
- data-cleaning
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

Missing data handling is the set of techniques used to detect and treat missing values (NaN, NA, null) in a dataset prior to analysis or modeling. The book lists five common strategies: forward fill, backward fill, dropping missing values, replacing with a generic value, and replacing NaN with a scalar.

## How It Works

In Pandas the canonical methods are `df.fillna(value)` to substitute a constant, `df.fillna(method='pad')` (forward fill) or `'bfill'` (backward fill) to propagate adjacent values, and `df.dropna()` to remove rows or columns that contain missing entries. The right choice depends on whether the data is missing completely at random, missing at random, or missing not at random.

## Key Parameters

- Strategy: drop vs. impute
- Imputation source: constant, statistic, neighbor, model-based
- Axis: row-wise vs. column-wise drop

## When To Use

- Cleaning real-world datasets before exploratory or inferential analysis
- Preparing training data for ML models that cannot tolerate NaN
- Streaming/time-series pipelines where forward fill carries the last known value

## Risks & Pitfalls

- Dropping rows can bias the sample if missingness is not random
- Mean/median imputation reduces variance and weakens correlations
- Forward fill on time series can mask outages or trends

## Related Concepts

- [[concepts/data-analysis]]
- [[concepts/descriptive-statistics]]

## Sources

- [[summaries/data-analysis-visualizations-python-04-chapter-1-introduction-to-data-science-with-python]]
- [[summaries/data-analysis-visualizations-python-06-chapter-3-data-collection-structures]]
- [[summaries/data-analysis-visualizations-python-08-chapter-5-data-gathering-and-cleaning]]
- [[summaries/data-analysis-visualizations-python-09-chapter-6-data-exploring-and-analysis]]
