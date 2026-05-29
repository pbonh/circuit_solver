---
title: Join Types (Inner / Outer / Left / Right)
type: claim
id: claim-join-types
tags:
- python
- pandas
- data-analysis
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/10-chapter-6-prepping-your-data-with-pandas.txt
confidence:
  base: 0.85
---

## Definition

Joins combine two tables by matching keys. Four join semantics — left, right, outer, inner — govern which rows survive. Pandas exposes joins through `merge` (column-based, defaults to inner), `join` (index-based, defaults to left), and `concat` (axis-based, defaults to outer).

## How It Works

- Left: every row from the left table; unmatched right rows are NaN.
- Right: every row from the right table; unmatched left rows are NaN.
- Outer (full): every row from either table; missing matches are NaN.
- Inner: only matching rows from both tables.

The chapter contrasts `concat`, `append`, `join`, and `merge` for combining DataFrames and shows the `how` parameter (or `join=` for `concat`) for switching semantics.

## Key Parameters

- Join keys (`on`, `left_on`, `right_on`)
- `how` (or `join`) parameter
- Suffixes for overlapping column names

## When To Use

- Enriching a fact table with lookup data (left join)
- Combining mutually exclusive lists (outer)
- Filtering to common keys (inner)

## Risks & Pitfalls

- Cartesian product explosions when keys are non-unique
- Silent NaN introduction in left/outer joins
- Column-name collisions without suffixes

## Related Concepts

- [[concepts/pandas-dataframe]]
- [[concepts/data-wrangling]]

## Sources

- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
