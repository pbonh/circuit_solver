---
title: "Data Cleaning"
type: concept
tags: [data-cleaning, data-analysis, pandas, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/DataAnalysisAndVisualizationsPython/_txt/08-chapter-5-data-gathering-and-cleaning.txt"]
confidence: medium
---

## Definition

Data cleaning is the process of detecting and correcting errors, missing values, type inconsistencies, and noisy entries in raw data so the result is reliable for analysis and modeling. The chapter treats it as the second of five data-science stages, after acquisition and before exploration.

## How It Works

A cleaning pipeline detects missing or sentinel values (`isnull`, `notnull`), replaces or drops them (`fillna`, `dropna`, `replace`), normalizes column names (`rename`), reconciles encodings, converts types with custom converter functions, drops irrelevant columns/rows, and merges or concatenates with other sources to produce a single coherent dataset.

## Key Parameters

- Sentinel values to treat as missing (`na_values`)
- Imputation strategy (constant, forward fill, backward fill, model-based)
- Drop axis (row vs. column) and threshold
- Type coercion rules per column

## When To Use

- Always — between data acquisition and analysis
- Anytime a dataset combines multiple sources with inconsistent conventions
- Before training any model that cannot tolerate NaN or wrong types

## Risks & Pitfalls

- Imputation that biases the distribution
- Silent type coercion that masks real data errors
- Over-cleaning that destroys legitimate signal (e.g., legitimate outliers)
- Losing provenance — keep the raw data alongside the cleaned version

## Related Concepts

- [[concepts/missing-data-handling]]
- [[concepts/data-analysis]]
- [[concepts/data-extraction]]
- [[concepts/dataframe]]

## Sources

- [[summaries/data-analysis-visualizations-python-08-chapter-5-data-gathering-and-cleaning]]
- [[summaries/data-analysis-visualizations-python-11-chapter-8-case-studies]]
- [[summaries/prototyping-python-dashboards-07-chapter-3-working-with-online-data]]
- [[summaries/prototyping-python-dashboards-17-appendix-a-utilities-for-managing-atads-data]]
