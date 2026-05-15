---
title: "Data Analysis and Visualizations with Python — Chapter 7: Data Visualization"
type: summary
tags: [python, data-visualization, matplotlib, seaborn, pandas, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/DataAnalysisAndVisualizationsPython/_txt/10-chapter-7-data-visualization.txt"]
confidence: high
---

## Key Points

- Surveys Python visualization libraries: Pandas (simplest direct plotting), Seaborn (statistical charts with color), Bokeh (web-interactive), Pygal (vector/interactive), and Plotly (web-based interactive).
- Uses a Salaries dataset (rank, discipline, phd, service, sex, salary) as the running example across all plot types.
- Pandas direct plotting via the DataFrame `.plot` accessor: `.plot()` for line, `.plot.bar(title=..., color=...)`, `.plot.pie(autopct='%.2f')`, `.plot.box()`, `.plot.hist()`, `.plot(kind='scatter', x=..., y=..., s=0.9)`.
- Warns that mixing variables of very different magnitudes (e.g., salary vs phd vs service) hides the smaller-magnitude variables in line plots; recommends plotting comparable units together.
- Groupby + bar plot recipe: `dataset.groupby(['service']).sum().sort_values("salary", ascending=False)["salary"].plot.bar()` to visualize aggregated category totals.
- Seaborn strip plot: `sns.stripplot(x, y, data, jitter, hue)` — categorical scatter; can be combined with box plot for distributional context.
- Seaborn box plot: `sns.boxplot(x, y, data, whis, notch, palette)`, optionally layered with `sns.stripplot` or `sns.swarmplot(color='0.25')`; `hue='sex'` adds a secondary categorical axis.
- Seaborn swarm plot: like strip plot but prevents overlapping points; `dodge=True` separates hue categories side-by-side.
- Seaborn joint plot: `sns.jointplot(x, y, data, kind=...)` with kinds `'reg'` (regression), `'hex'` (hexbin), `'kde'` (kernel density); supports overlaying with `.plot_joint(sns.kdeplot, n_levels=6).plot_marginals(sns.rugplot)` and statistical functions like `stat_func=spearmanr`.
- Matplotlib line plotting: `plt.plot(x, y, label=...)`, `plt.xlabel`, `plt.ylabel`, `plt.title`, `plt.legend`, custom ticks via `plt.yticks([...], [labels])`, log scale via `plt.yscale('log')`, and grid via `plt.grid()`.
- Matplotlib bar chart: `plt.bar(x, heights, label=..., color='r')` with stacking by repeated calls.
- Matplotlib histogram: `plt.hist(values, bins=, histtype='bar', rwidth=, alpha=, color=, label=)` with `bins=` accepting a list of edges or an integer count; supports overlaying multiple histograms with `alpha` for transparency.
- Matplotlib scatter: `plt.scatter(x, y, label=..., color=..., marker='*', s=75)`.
- Matplotlib stack plot: `plt.stackplot(days, sleeping, eating, working, playing, colors=[...])` with manual legend stubs via `plt.plot([], [], color=..., label=...)`.
- Matplotlib pie: `plt.pie(slices, labels=..., colors=..., startangle=..., shadow=True, explode=(0,0,0.09,0), autopct='%1.1f%%')` for exploded sector slices.
- Exercises plot 500-row random temperatures across six cities, the Iris pairplot grouped by species, and Tips dataset visualizations using `FacetGrid` and `factorplot`.

## Relevant Concepts

- [[concepts/python]] — implementation language.
- [[concepts/data-visualization]] — central topic of the chapter.
- [[concepts/exploratory-data-analysis]] — plots are the front line of EDA.
- [[concepts/correlation]] — joint plots and pairplots visualize pairwise relationships.
- [[concepts/linear-regression]] — jointplot `kind='reg'` overlays a regression line.
- [[entities/matplotlib]] — primary low-level plotting library.
- [[entities/seaborn]] — high-level statistical visualization.
- [[entities/pandas]] — DataFrame `.plot` accessor.
- [[entities/numpy]] — numerical inputs and random data.
- [[entities/scipy]] — `scipy.stats.spearmanr` used in joint plots.

## Source Metadata

- Source type: book chapter
- Book title: Data Analysis and Visualizations with Python
- Chapter: 7 — Data Visualization
- File path: raw/DataAnalysisAndVisualizationsPython/_txt/10-chapter-7-data-visualization.txt
- Author: Ossama Embarak
