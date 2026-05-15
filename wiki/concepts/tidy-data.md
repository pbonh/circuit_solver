---
title: "Tidy Data"
type: concept
tags: [python, pandas, data-analysis, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/10-chapter-6-prepping-your-data-with-pandas.txt"]
confidence: high
---

## Definition

Tidy data is a structural convention by Hadley Wickham: each column is a variable, each row is an observation, and each table contains a single observational unit. Tidy form facilitates downstream analysis and visualization.

## How It Works

Untidy data violates these rules — for example, variables stored as column headers, multiple variables in one column, or multiple observational units in one table. Pandas converts to long (tidy) form via `stack` or `melt`, and back to wide form via `unstack` or `pivot`. Tidying differs from cleansing: tidying restructures the shape; cleansing fixes values.

## Key Parameters

- `id_vars` (preserved columns in `melt`)
- `value_vars`, `var_name`, `value_name`
- `pivot(index, columns, values)`

## When To Use

- Whenever data must feed a Pandas/seaborn/ggplot-style pipeline
- Before applying group-by aggregations
- When variables are scattered across columns

## Risks & Pitfalls

- Pivoting on a non-unique key raises errors
- Over-aggressive melting loses categorical structure
- Confusing tidiness with cleanliness

## Related Concepts

- [[concepts/data-wrangling]]
- [[concepts/pandas-dataframe]]
- [[concepts/split-apply-combine]]

## Sources

- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
