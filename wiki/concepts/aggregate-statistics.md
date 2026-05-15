---
title: "Aggregate Statistics"
type: concept
tags: [python, numpy, statistics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/09-chapter-5-working-with-numpy-arrays.txt"]
confidence: medium
---

## Definition

Aggregate statistics summarize an array (or a slice of it) into a single value or per-axis vector — including mean, variance, standard deviation, sum, cumulative sum, max, and min. NumPy exposes these as both ndarray methods and module-level functions.

## How It Works

`arr.mean()`, `arr.var()`, `arr.std()`, `arr.sum(axis=...)`, `arr.cumsum()`, and `arr.max()` operate over the whole array by default or along a chosen axis. They are implemented in compiled code and broadcast across dtypes.

## Key Parameters

- Aggregation axis
- Choice of biased vs. unbiased estimator (`ddof`)
- dtype upcasts to avoid overflow

## When To Use

- Quick exploratory statistics on a dataset
- Building features for downstream modeling
- Reducing high-dimensional data to summaries

## Risks & Pitfalls

- Default `ddof=0` differs from sample-variance convention
- Aggregating over NaNs without `nan*` variants silently propagates NaN
- Wrong axis collapses the wrong dimension

## Related Concepts

- [[concepts/descriptive-statistics]]
- [[concepts/ndarray]]

## Sources

- [[summaries/python-data-analysts-toolkit-09-chapter-5-working-with-numpy-arrays]]
