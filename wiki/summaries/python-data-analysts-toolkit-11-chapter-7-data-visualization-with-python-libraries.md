---
title: 'Python Data Analyst''s Toolkit — Chapter 7: Data Visualization with Python
  Libraries'
type: source
id: source-python-data-analysts-toolkit-11-chapter-7-data-visualization-with-python-libraries
kind: derived-summary
tags:
- python
- visualization
- data-analysis
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt
---

## Key Points

- Three Python libraries are introduced for visualization: Matplotlib (the foundation), Pandas (built on Matplotlib), and Seaborn (built on Matplotlib).
- Common plot types covered: bar chart, histogram, box plot, pie chart, scatter plot, heat map — each suited to specific data levels.
- Matplotlib has two APIs: the stateful `pyplot` interface (MATLAB-like) and the object-oriented interface using `figure` and `axes` objects. The chapter recommends the object-oriented approach for control and customization.
- Stateful workflow: `plt.plot`, `plt.xlim`, `plt.xlabel`, `plt.title`, etc. Object-oriented workflow: `fig, ax = plt.subplots(...)`, then `ax.plot`, `ax.set_xlim`, `ax.set_xlabel`, etc.
- The OO approach for multi-panel figures: `fig = plt.figure(figsize=(w,h))`, `ax1 = fig.add_subplot(211)` for top row, `ax2 = fig.add_subplot(212)` for bottom, then plot/label each.
- Pandas wraps Matplotlib via `DataFrame.plot(kind=...)`; switching `kind` between `scatter`, `hist`, `pie`, etc. illustrates polymorphism (one method, many forms). Pandas requires wide/aggregated data and does not auto-aggregate (use `value_counts` first for pie charts).
- Seaborn changes Matplotlib defaults for nicer aesthetics and auto-aggregation. Requires tidy (long) data; supports multi-variable visualization. Common functions: `boxplot`, `kdeplot`, `violinplot`, `countplot`, `heatmap`, `FacetGrid`, `regplot`, `lmplot`, `stripplot`, `swarmplot`, `catplot`, `pairplot`, `jointplot`.
- Box plots summarize a continuous variable's five-number summary (min, Q1, median, Q3, max) and outliers via whiskers.
- KDE plots visualize a continuous variable's probability distribution; violin plots merge box and KDE.
- Heatmaps visualize correlation matrices (`df.corr()`); `annot=True` adds numeric values, `cmap` chooses color palettes.
- Facet grids spread plots across row/col/hue parameters; `lmplot` and `catplot` combine regression/strip plots with facet grids.
- `pairplot` shows bivariate relationships across every pair of variables in a DataFrame; `jointplot` shows two variables plus their marginal distributions.
- The `%matplotlib inline` magic command renders plots inline in Jupyter notebooks.
- "axes" in Matplotlib refers to the subplot/plot area, not the x or y axis.

## Relevant Concepts

- [[concepts/data-visualization]] — the umbrella concept developed here.
- [[concepts/matplotlib-interfaces]] — stateful vs. object-oriented APIs.
- [[concepts/box-plot]] — five-number summary visualization.
- [[concepts/scatter-plot]] — correlation visualization for two continuous variables.
- [[concepts/heatmap-visualization]] — color-coded correlation matrix.
- [[concepts/kernel-density-estimate]] — smooth distribution estimate.
- [[concepts/facet-grid]] — multi-panel conditional plots.
- [[concepts/polymorphism]] — Pandas `plot(kind=...)` is the chapter's worked example.
- [[entities/matplotlib]] — base library.
- [[entities/seaborn]] — high-level statistical plotting library.
- [[entities/pandas]] — plotting via DataFrame methods.

## Source Metadata

- Source type: book chapter
- Book title: Python Data Analyst's Toolkit
- Chapter: 7 — Data Visualization with Python Libraries
- File path: raw/PythonDataAnalystsToolkit/_txt/11-chapter-7-data-visualization-with-python-libraries.txt
- Author: Gayathri Rajagopalan
