---
title: Data Aggregation
type: claim
id: claim-data-aggregation
tags:
- pandas
- data-analysis
- groupby
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/09-chapter-6-data-exploring-and-analysis.txt
confidence:
  base: 0.65
---

## Definition

Data aggregation is the process of summarizing groups of records into single values (counts, sums, means, standard deviations, custom statistics). In Pandas, aggregation typically follows a `groupby` that partitions the data, and applies one or more functions to each group.

## How It Works

`df.groupby(key)` produces a GroupBy object indexed by unique values of the key. Calling `.agg(func)` (or `.agg([f1, f2, ...])`) applies the function(s) to each group, returning a smaller DataFrame keyed by group. Related operations include `transform` (same-size result, useful for z-scoring per group) and `filter` (keeps or drops entire groups based on a predicate).

## Key Parameters

- Grouping key(s) — single column or list of columns
- Aggregation function(s) — built-in (`sum`, `mean`) or custom
- Whether to apply per-column or to the whole group

## When To Use

- Computing per-segment metrics (per-city sales, per-cohort retention)
- Creating summary tables for dashboards
- Producing features for downstream modeling (group-wise statistics)

## Risks & Pitfalls

- Aggregating without checking group sizes can hide imbalance
- Mean/std on small groups are noisy estimators
- Default behavior drops NaN groupby keys silently
- Using `apply` where `agg` or `transform` would be faster

## Related Concepts

- [[concepts/dataframe]]
- [[concepts/descriptive-statistics]]
- [[concepts/data-analysis]]

## Sources

- [[summaries/data-analysis-visualizations-python-09-chapter-6-data-exploring-and-analysis]]
- [[summaries/data-analysis-visualizations-python-11-chapter-8-case-studies]]
