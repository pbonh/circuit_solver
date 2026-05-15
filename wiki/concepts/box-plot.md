---
title: "Box Plot"
type: concept
tags: [python, visualization, statistics, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt"]
confidence: high
---

## Definition

A box plot is a graphical summary of a continuous variable based on five statistics: minimum, first quartile, median, third quartile, and maximum. Whiskers extend to the extreme values; points outside the whiskers are plotted as outliers. The chapter uses Seaborn's `sns.boxplot(...)`.

## How It Works

The box spans the interquartile range (Q1 to Q3) with a line at the median. Whiskers reach the smallest and largest non-outlier values. Outliers appear as individual marks. Box plots can be drawn for a continuous variable alone or split by a categorical variable to compare groups.

## Key Parameters

- Variable(s) to plot
- Optional categorical grouping
- Outlier threshold (typically 1.5 × IQR)

## When To Use

- Comparing distributions across groups
- Quickly spotting skewness and outliers
- Reporting summary statistics visually

## Risks & Pitfalls

- Hides multi-modality
- Sensitive to sample size — small groups produce misleading boxes
- Whisker definitions vary across libraries

## Related Concepts

- [[concepts/data-visualization]]
- [[concepts/descriptive-statistics]]
- [[entities/seaborn]]

## Sources

- [[summaries/python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries]]
