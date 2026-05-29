---
title: Data Levels (Nominal / Ordinal / Interval / Ratio)
type: claim
id: claim-data-levels
tags:
- statistics
- data-analysis
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/08-chapter-4-descriptive-data-analysis-basics.txt
confidence:
  base: 0.85
---

## Definition

Data levels classify variables by what operations are meaningful on them. The four standard levels are nominal, ordinal, interval, and ratio (with nominal/ordinal categorized as categorical and interval/ratio as continuous).

## How It Works

- Nominal: unordered categories (color, gender, ID). Operations: counting, mode.
- Ordinal: ordered categories (grades, satisfaction). Operations: median, percentiles, counting.
- Interval: ordered numeric without a true zero (Celsius temperature, year). Operations: addition, subtraction, mean, SD.
- Ratio: ordered numeric with a true zero (age, height, fare). Operations: all arithmetic including ratios.

Chart choice follows: bar/pie for nominal/ordinal, histogram for continuous, box plot for one categorical vs. one continuous, scatter for two continuous, clustered/stacked bars for two categorical.

## Key Parameters

- Existence of a true zero
- Whether order is defined
- Whether arithmetic on values is meaningful

## When To Use

- Before deciding on summary statistics for a variable
- Before selecting a chart type
- Before running statistical tests with type-specific assumptions

## Risks & Pitfalls

- Treating numeric codes (e.g., PassengerId) as quantitative
- Assuming interval data has a meaningful ratio
- Computing means on ordinal scales

## Related Concepts

- [[concepts/descriptive-statistics]]
- [[concepts/data-visualization]]
- [[concepts/data-wrangling]]

## Sources

- [[summaries/python-data-analysts-toolkit-08-chapter-4-descriptive-data-analysis-basics]]
