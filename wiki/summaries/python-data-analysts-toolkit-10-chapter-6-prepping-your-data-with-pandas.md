---
title: "Python Data Analyst's Toolkit — Chapter 6: Prepping Your Data with Pandas"
type: summary
tags: [python, pandas, dataframe, data-analysis, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/10-chapter-6-prepping-your-data-with-pandas.txt"]
confidence: high
---

## Key Points

- Pandas (created by Wes McKinney in 2008; name from "Panel Data") is built on Cython, integrates with NumPy/Matplotlib/SciPy, and is the de facto Python tool for data wrangling and analysis.
- Core data structures are the 1-D `Series` (values + index) and the 2-D `DataFrame` (values + row index + column index); both rely on hashable index objects implemented in NumPy.
- Series can be created from scalars, lists, dicts, ranges, random numbers, or with a custom `index` parameter; the constructor is `pd.Series(...)` (capital S).
- DataFrames can be built from dicts of columns, lists of Series, NumPy arrays, lists of tuples, or by importing CSV/Excel/JSON/HTML via `pd.read_csv`, `pd.read_excel`, `pd.read_json`, `pd.read_html`.
- Column manipulation: add via `df['col']=...`, `df.insert(...)`, `df.loc[:, 'col']=...`, or `pd.concat`; remove via `del`, `pop`, `drop(..., axis=1)`; rename with `rename` or by assigning to `df.columns`.
- Row manipulation: add via `append` (use `ignore_index=True`) or `pd.concat`; remove via Boolean filter or `drop`.
- Index types include `RangeIndex` (default), `Int64Index`, `Float64Index`, `IntervalIndex`, `CategoricalIndex`, `DateTimeIndex`, `PeriodIndex`, `TimedeltaIndex`, and `MultiIndex`. Indexes are immutable and dramatically speed up lookup (chapter shows 1.66 ms → 281 μs).
- Index alignment: arithmetic on Series aligns by index label; unmatched labels yield NaN. Index set operations: `union`, `difference`, `symmetric_difference`.
- dtypes include `object`, `int64`, `float64`, `datetime64`, and Pandas-specific `category` (huge memory savings — chapter shows ~93% reduction for a low-cardinality string column).
- Data selection: prefer `loc` (label-based) and `iloc` (position-based) indexers; `at`/`iat` for fast scalar access; `ix` is deprecated; the `[]` operator is for column or simple row selection only.
- Boolean indexing and the `query` method enable conditional row filtering with `&`, `|`, `~` operators.
- Date/time: `pd.Timestamp` (covers `datetime.date`/`time`/`datetime`), `pd.Timedelta` for durations, `pd.to_datetime` with format strings to parse strings.
- Grouping follows the split-apply-combine methodology by Hadley Wickham: `groupby` splits, aggregation functions (`sum`, `mean`, `median`, `count`, ...) apply, results recombine. The `agg`, `transform`, `apply`, and `filter` methods give different output shapes.
- Combining objects: `append` adds rows, `concat` adds rows or columns (default outer join), `join` aligns on indexes (default left join), `merge` aligns on common column names (default inner join).
- Tidy data principles (Hadley Wickham): each column is a variable, each row is an observation, each table contains one observational unit. `stack`/`melt` go wide-to-long; `unstack`/`pivot` go long-to-wide.
- Missing data tools: `isna`/`isnull`, `dropna`, `fillna` (with constants, forward/backward fill, or aggregates), and `interpolate` (linear).
- Duplicates: detect with `duplicated`; remove with `drop_duplicates` (`keep='first'` default).

## Relevant Concepts

- [[concepts/pandas-series]] — one-dimensional labeled array.
- [[concepts/pandas-dataframe]] — two-dimensional labeled table.
- [[concepts/pandas-index]] — hashable, immutable labels accelerating lookup.
- [[concepts/data-wrangling]] — cleaning and reshaping data, the chapter's central activity.
- [[concepts/split-apply-combine]] — Wickham's groupby methodology.
- [[concepts/tidy-data]] — Wickham's data-structure principles.
- [[concepts/missing-data-imputation]] — strategies for handling NaN values.
- [[concepts/join-types]] — inner, outer, left, right joins for combining tables.
- [[entities/pandas]] — the library covered in depth.
- [[entities/wes-mckinney]] — Pandas creator.
- [[entities/hadley-wickham]] — split-apply-combine and tidy-data author.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 6 — Prepping Your Data with Pandas
- File path: raw/PythonDataAnalystsToolkit/_txt/10-chapter-6-prepping-your-data-with-pandas.txt
- Author: Gayathri Rajagopalan
