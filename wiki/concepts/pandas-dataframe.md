---
title: "Pandas DataFrame"
type: concept
tags: [python, pandas, dataframe, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/10-chapter-6-prepping-your-data-with-pandas.txt"]
confidence: high
---

## Definition

A Pandas `DataFrame` is a two-dimensional labeled table — values plus a row index and a column index — built atop NumPy arrays. It is the central data structure for tabular analysis in Python.

## How It Works

Construct via `pd.DataFrame(...)` from dicts of columns, lists of Series, NumPy arrays, or lists of tuples; load from disk via `pd.read_csv`, `pd.read_excel`, `pd.read_json`, `pd.read_html`. The chapter covers add/remove of rows and columns, label vs. positional indexing (`loc`, `iloc`, `at`, `iat`), Boolean filtering, the `query` method, dtype changes via `astype`, and split-apply-combine with `groupby`.

## Key Parameters

- Row index and column index objects
- `dtype` per column
- `inplace=True` for mutating operations
- `axis=0`/`1` distinguishing row vs. column operations

## When To Use

- Any tabular data analysis in Python
- ETL prior to modeling, plotting, or statistics
- Interactive exploration in Jupyter

## Risks & Pitfalls

- `SettingWithCopyWarning` from chained indexing
- Mixing `loc`/`iloc` semantics
- Large DataFrames exceeding memory; use `category` dtype where possible

## Related Concepts

- [[concepts/pandas-series]]
- [[concepts/pandas-index]]
- [[concepts/data-wrangling]]
- [[concepts/split-apply-combine]]
- [[concepts/tidy-data]]
- [[entities/pandas]]

## Sources

- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
