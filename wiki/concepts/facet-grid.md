---
title: "Facet Grid"
type: concept
tags: [python, visualization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt"]
confidence: medium
---

## Definition

A facet grid is a multi-panel plot where the data is split by one or more categorical variables — rows, columns, or hues — and each panel shows the same plot type for its subset. The chapter uses Seaborn's `FacetGrid` (with `.map(...)`) and high-level wrappers `lmplot` and `catplot`.

## How It Works

`g = sns.FacetGrid(df, col=..., row=..., hue=...)` creates a grid object; `g.map(plot_func, *columns)` draws the plot in each panel. Wrappers like `lmplot` (regression) and `catplot` (categorical) combine FacetGrid with a specific plot kind.

## Key Parameters

- `row`, `col`, `hue` conditioning variables
- `height` and `aspect` controlling panel size
- Plot function and its arguments via `.map`

## When To Use

- Comparing conditional distributions across categories
- Exploring high-dimensional data slice by slice
- Communicating subgroup differences clearly

## Risks & Pitfalls

- Too many panels make the grid unreadable
- Different y-axis ranges per panel can mislead
- Small subgroups give noisy panels

## Related Concepts

- [[concepts/data-visualization]]
- [[entities/seaborn]]

## Sources

- [[summaries/python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries]]
