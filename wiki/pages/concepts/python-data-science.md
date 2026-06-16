---
title: Python Data Science Stack
type: concept
slug: python-data-science
created: 2026-06-16
updated: 2026-06-16
summary: The core Python libraries for data analysis — pandas, NumPy, matplotlib, seaborn — and the data pipeline patterns (gather, clean, explore, visualize) for scientific and engineering data.
tags: [python, pandas, numpy, matplotlib, seaborn, data-science, eda]
sources: [data-analysis-visualizations-python, python-data-analysts-toolkit]
status: active
---

# Python Data Science Stack

The Python data science ecosystem is built on a small set of interoperable libraries. NumPy provides fast array operations; pandas provides labeled, heterogeneous tabular structures; matplotlib provides publication-quality plotting; seaborn adds statistical visualization. Together they support end-to-end data pipelines from raw files to analyzed, visualized results.

## Core Libraries

| Library | Primary Use | Key API |
|---|---|---|
| NumPy | n-dimensional array math | `ndarray`, broadcasting, `linalg` |
| pandas | Tabular data analysis | `DataFrame`, `Series`, `groupby`, `merge` |
| matplotlib | Base plotting | `Figure`, `Axes`, `pyplot` |
| seaborn | Statistical visualization | `distplot`, `boxplot`, `jointplot`, `heatmap` |
| Plotly/Dash | Interactive web charts | `go.Figure`, `dash.Dash`, `dcc.Graph` |
| SciPy | Scientific algorithms | `signal`, `fft`, `linalg`, `optimize` |

## pandas: Key Operations

**Data structures**: `Series` (1D labeled), `DataFrame` (2D table), both support label-based (`loc`) and position-based (`iloc`) indexing.

**Data cleaning**: `isnull()`, `dropna()`, `fillna()`, `astype()`, `duplicated()`, `drop_duplicates()`.

**Reshape**: `groupby()` → split-apply-combine; `pivot_table()`, `melt()`, `stack()`/`unstack()`.

**Combine**: `merge()` (SQL-style joins), `concat()` (vertical/horizontal stacking).

**I/O**: `read_csv()`, `read_json()`, `read_html()`, `read_excel()`, `read_parquet()`, `to_csv()`.

## Data Pipeline Pattern

```
Raw data (CSV, JSON, log, waveform)
  → Load (pd.read_csv / custom parser)
  → Clean (missing values, type conversion, dedup)
  → Explore (describe, groupby, correlation)
  → Analyze (statistics, aggregation, join)
  → Visualize (matplotlib / seaborn / plotly)
  → Report / Dashboard
```

## Circuit Simulation Application

SPICE/Spectre output files → parse into DataFrame (columns: time/frequency, voltage/current nodes) → groupby by simulation sweep parameter → compute metrics (gain, phase margin, noise figure) → plot waveform families or histograms → dashboard for yield analysis.

## Related concepts and entities

- [[data-analysis-tooling]] - parent topic
- [[circuit-simulation]] - primary domain for scientific Python in this wiki
