---
title: Plotly
type: entity
id: entity-plotly
tags:
- python
- visualization
- interactive
- library
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/DataAnalysisAndVisualizationsPython/_txt/05-chapter-2-the-importance-of-data-visualization-in-business-intelligence.txt
---

## Overview

Plotly is a Python graphing library that produces interactive, publication-quality web visualizations. The chapter demonstrates Plotly for both online dashboards and offline use inside Jupyter notebooks, including 2D histogram-contour heatmaps and scatter plots.

## Characteristics

- HTML/JavaScript output rendered by plotly.js, allowing pan/zoom/hover
- Both online (cloud) and offline (`plotly.offline`) modes
- `graph_objs` API for declarative figure construction
- Strong support for streaming and real-time updates

## Common Strategies

- Use Plotly when interactivity matters more than static publication output
- Pair `init_notebook_mode(connected=True)` with `iplot` for inline notebook plots
- Export figures to PNG, HTML, or embed in dashboards
- Reach for higher-level Plotly Express for quick declarative charts

## Related Entities

- [[entities/matplotlib]]
- [[entities/seaborn]]
- [[entities/jupyter-notebook]]

## Sources

- [[summaries/data-analysis-visualizations-python-05-chapter-2-the-importance-of-data-visualization-in-business-intelligence]]
- [[summaries/prototyping-python-dashboards-04-introduction]]
- [[summaries/prototyping-python-dashboards-06-chapter-2-reactive-programming-with-plotly-and-dash]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-13-chapter-9-the-bts-t100-dataset-interacting-controls-and-tables]]
