---
title: Missing Data Imputation
type: claim
id: claim-missing-data-imputation
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

Missing data imputation substitutes plausible values for missing entries (represented as `NaN` / `np.nan` in Pandas) instead of dropping rows. The chapter covers constant fills, forward/backward fills, central-tendency fills, and linear interpolation.

## How It Works

`df.isna()` flags missing entries; `df.dropna()` removes them; `df.fillna(value)` substitutes a constant or aggregate (e.g., column mean/median/mode); `df.fillna(method='ffill'|'bfill')` propagates adjacent values; `df.interpolate(method='linear')` estimates missing numeric entries using neighboring known points.

## Key Parameters

- Fill value or method
- Axis (`0` columns, `1` rows)
- `inplace=True` for in-place mutation
- Interpolation method (`linear`, `time`, `spline`, ...)

## When To Use

- Datasets with sparse but recoverable NaNs
- Time series with brief sensor dropouts
- Pre-modeling cleanup

## Risks & Pitfalls

- Imputing with the mean biases variance estimates
- Forward/backward fills can leak future or past information
- Linear interpolation may smooth over genuine discontinuities

## Related Concepts

- [[concepts/data-wrangling]]
- [[concepts/pandas-dataframe]]

## Sources

- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
- [[summaries/python-data-analysts-toolkit-12-chapter-8-data-analysis-case-studies]]
