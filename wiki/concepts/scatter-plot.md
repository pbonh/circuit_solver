---
title: "Scatter Plot"
type: concept
tags: [python, visualization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt"]
confidence: high
---

## Definition

A scatter plot displays values of two continuous variables as points in a 2-D plane, revealing whether they are correlated. The chapter shows Pandas' `df.plot(kind='scatter', x=..., y=...)` and Seaborn's `regplot`/`lmplot`/`jointplot` variants that add regression layers.

## How It Works

Each observation contributes one point at (x, y). Patterns — linear, monotonic, clustered, none — emerge from point density. Adding hue, size, or marker shape can encode additional categorical or continuous variables.

## Key Parameters

- x and y variables
- Optional hue/size/style encoding
- Point alpha for handling overplotting

## When To Use

- Checking for linear or non-linear relationships
- Spotting outliers in two-variable data
- As a base layer for regression overlays

## Risks & Pitfalls

- Overplotting in dense datasets hides structure
- Misleading correlation when data range is restricted
- Hidden confounders behind apparent associations

## Related Concepts

- [[concepts/data-visualization]]
- [[concepts/correlation]]

## Sources

- [[summaries/python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries]]
