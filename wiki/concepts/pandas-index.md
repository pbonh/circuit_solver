---
title: "Pandas Index"
type: concept
tags: [python, pandas, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/10-chapter-6-prepping-your-data-with-pandas.txt"]
confidence: high
---

## Definition

The Pandas `Index` is an immutable, hashable container of labels attached to the rows or columns of a Series/DataFrame. It accelerates lookup, enables label alignment in arithmetic, and supports many specialized subtypes (`RangeIndex`, `Int64Index`, `Float64Index`, `IntervalIndex`, `CategoricalIndex`, `DateTimeIndex`, `PeriodIndex`, `TimedeltaIndex`, `MultiIndex`).

## How It Works

Indexes back hash-table lookups so retrieving a row by label is O(1) on average — the chapter shows a 1.66 ms linear scan dropping to 281 μs once a column becomes the index. Indexes can be customized via the `index` parameter, set from a column via `set_index`, or reset via `reset_index`. They are immutable once created. Set operations (`union`, `difference`, `symmetric_difference`) work across indexes.

## Key Parameters

- Index subtype (`RangeIndex`, `DateTimeIndex`, etc.)
- Single vs. `MultiIndex` (hierarchical)
- Whether labels are unique

## When To Use

- Repeated lookups by key
- Time-series alignment
- Hierarchical groupings

## Risks & Pitfalls

- Duplicate labels can cause ambiguous selections
- Mutating an index requires creating a new one
- Resetting an index moves the old labels into a column

## Related Concepts

- [[concepts/pandas-series]]
- [[concepts/pandas-dataframe]]

## Sources

- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
