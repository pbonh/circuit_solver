---
title: "Data Wrangling"
type: concept
tags: [data-analysis, pandas, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/08-chapter-4-descriptive-data-analysis-basics.txt"]
confidence: high
---

## Definition

Data wrangling (also called munging) is the process of transforming raw data into a structure suitable for analysis. It is the most time-consuming step of the descriptive analysis workflow — analysts spend roughly 80% of their time here.

## How It Works

Wrangling comprises three sub-activities:
1. Tidying — mapping variables to columns and observations to rows.
2. Cleansing — removing or substituting missing values, fixing data types, eliminating duplicates, dealing with outliers, removing filler characters.
3. Enrichment — adding new derived columns and joining with external sources.

Python's Pandas is the canonical tool for this work.

## Key Parameters

- Source format (CSV, Excel, JSON, SQL, etc.)
- Missing-value handling strategy (drop, fill, interpolate)
- Type-coercion rules
- Join keys for enrichment

## When To Use

- Whenever raw data is unfit for direct analysis or modeling
- As a prerequisite to visualization and statistical testing
- When integrating multiple data sources

## Risks & Pitfalls

- Aggressive imputation distorts downstream statistics
- Silent dtype coercions can lose precision
- Over-cleaning may discard meaningful signal

## Related Concepts

- [[concepts/data-analysis]]
- [[concepts/exploratory-data-analysis]]
- [[concepts/data-levels]]
- [[entities/pandas]]

## Sources

- [[summaries/python-data-analysts-toolkit-08-chapter-4-descriptive-data-analysis-basics]]
- [[summaries/python-data-analysts-toolkit-10-chapter-6-prepping-your-data-with-pandas]]
- [[summaries/python-data-analysts-toolkit-12-chapter-8-data-analysis-case-studies]]
