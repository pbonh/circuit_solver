---
title: DataFrame
type: claim
id: claim-dataframe
tags:
- pandas
- dataframe
- data-analysis
- python
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/06-chapter-3-data-collection-structures.txt
confidence:
  base: 0.85
---

## Definition

A DataFrame is a two-dimensional, size-mutable, heterogeneous tabular data structure with labeled axes (rows and columns), provided by the Pandas library. It is the workhorse object for nearly all Python data analysis: each column is a Series, and columns can hold different data types.

## How It Works

DataFrames are constructed from many sources — dicts of Series, dicts of ndarrays or lists, structured/record arrays, lists of dicts, dicts of tuples (yielding a multi-index), or other DataFrames. Internally each column is a typed array. Columns are accessed with `df[col]`, rows by label with `df.loc[label]` and by integer location with `df.iloc[i]`. Boolean masks like `df[df['C'] > 7]` filter rows. Arithmetic operations broadcast across aligned indexes; mismatches produce NaN.

## Key Parameters

- `data` — source of values (dict, ndarray, list, Series, DataFrame)
- `index` — row labels
- `columns` — column labels (and ordering)
- `dtype` — coerce all columns to a single type
- `copy` — whether to copy input data

## When To Use

- Loading and analyzing CSV, JSON, SQL, Excel data
- Aligning multiple data sources by label
- Group-by, pivot, and time-series transformations
- Producing tabular input for plotting and modeling libraries

## Risks & Pitfalls

- Silent NaN production when indexes don't align in operations
- Memory blow-up on large datasets without dtype care
- Chained indexing assignments (`df[df.a > 0]['b'] = ...`) cause SettingWithCopyWarning and unreliable updates
- Mixed dtypes per column lead to object arrays and slow numerical performance

## Related Concepts

- [[concepts/python]]
- [[concepts/data-analysis]]
- [[concepts/missing-data-handling]]

## Sources

- [[summaries/data-analysis-visualizations-python-06-chapter-3-data-collection-structures]]
- [[summaries/data-analysis-visualizations-python-08-chapter-5-data-gathering-and-cleaning]]
- [[summaries/data-analysis-visualizations-python-09-chapter-6-data-exploring-and-analysis]]
- [[summaries/prototyping-python-dashboards-05-chapter-1-working-with-python]]
- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables]]
