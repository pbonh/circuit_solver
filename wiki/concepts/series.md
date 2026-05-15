---
title: "Pandas Series"
type: concept
tags: [python, pandas, dataframe, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/05-chapter-1-working-with-python.txt"]
confidence: high
---

## Definition

A Pandas Series is a one-dimensional labeled array. It behaves like a list but supports both integer and label-based indexing, holds a name, and underlies each column of a DataFrame.

## How It Works

Series are constructed from lists, dictionaries, or scalars. Access is by label (`s.loc[label]`) or position (`s.iloc[i]`); the `index` attribute exposes labels. Series support vector arithmetic with scalars and with other Series of matching index, enabling expressive column operations on DataFrames.

## Key Parameters

- Index (the label axis)
- Name (passed when constructing or via `s.name = ...`)
- Dtype (numeric, object/string, datetime, etc.)

## When To Use

- Holding a single column extracted from a DataFrame
- Performing element-wise arithmetic with broadcasting
- Building filter masks (`s > value`) for DataFrame row selection
- Aggregating with `.sum()`, `.mean()`, `.std()`

## Risks & Pitfalls

- Confusing Series output (`df['col']`) with a Python list — wrap with `list(s)` if a plain list is required
- Auto-alignment by index can silently produce NaNs when combining mis-indexed Series
- Implicit dtype inference can yield surprising mixed-object dtypes

## Related Concepts

- [[concepts/dataframe]]
- [[concepts/list]]
- [[entities/pandas]]
- [[concepts/python]]

## Sources

- [[summaries/prototyping-python-dashboards-05-chapter-1-working-with-python]]
