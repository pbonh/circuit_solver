---
title: Split-Apply-Combine
type: claim
id: claim-split-apply-combine
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

Split-apply-combine is a data-analysis pattern formalized by Hadley Wickham: split data into independent groups, apply a function to each group, then combine the results. In Pandas it is implemented by `groupby` followed by `agg`, `transform`, `apply`, or `filter`.

## How It Works

`df.groupby(col)` produces a `GroupBy` object — effectively a dictionary of subframes keyed by group label. `agg` reduces each group to a scalar (or row), `transform` returns a same-shape result, `apply` is most flexible (any-shape output), and `filter` drops groups not satisfying a predicate. Aggregations include `sum`, `mean`, `min`, `max`, `std`, `var`, `count`, `size`, `first`, `last`, `describe`, `nth`.

## Key Parameters

- Grouping column(s)
- Aggregating column(s)
- Aggregation function or dict mapping columns to functions

## When To Use

- Computing per-category summaries
- Imputing missing values per group
- Filtering categories by aggregate properties

## Risks & Pitfalls

- High-cardinality grouping keys cause many small groups
- Forgetting `as_index=False` when later operations expect flat output
- Misusing `apply` where `transform` is sufficient

## Related Concepts

- [[concepts/pandas-dataframe]]
- [[concepts/data-wrangling]]
- [[concepts/aggregate-statistics]]

## Sources

- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
- [[summaries/python-data-analysts-toolkit-12-chapter-8-data-analysis-case-studies]]
