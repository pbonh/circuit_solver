---
title: 'Prototyping Python Dashboards — Chapter 5: Our First Dashboard'
type: source
id: source-prototyping-python-dashboards-09-chapter-5-our-first-dashboard
kind: derived-summary
tags:
- python
- dashboard
- plotly
- dash
- css
- regression
- visualization
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/09-chapter-5-our-first-dashboard.txt
---

## Key Points

- The first working dashboard splits code across three files: `atads.py` (top-level Dash app), `atads_layout.py` (widgets), and `atads_figures.py` (data + chart construction), plus a CSS file for screen layout.
- `atads.py` mirrors a standard Dash skeleton: instantiate `app = dash.Dash()`, define `app.layout()`, register callbacks, and call `app.run()`.
- The `atads_layout` class exposes one method per widget (`dropdown_airports`, `dropdown_use_variable`, year checkboxes, radio buttons for raw/smoothed/poly toggles); each method returns a Dash component with a unique `id` referenced by callbacks.
- The `atads_figures` class handles data and chart construction. Initialization reads CSV airport files (PHX is the default), normalizes column names, builds a `ydecimal` date column for plotting time series, and constructs a `var_dict` mapping user-friendly menu labels to internal column names using the `zip()` idiom.
- Data is loaded via pandas `read_csv()` with year filtering via `isin()`; data is concatenated into `self.df` for the airport list under study.
- `update_mainchart()` is the central chart-building method invoked by callbacks; it conditionally adds raw, smoothed, and polynomial traces, restricts curve-fit display to the first two airports for clarity, and finishes with watermark/border/title/x-axis range enhancements.
- Smoothing is performed via a rolling-window average; the x-coordinate is shifted by half the window width to correct the offset introduced by `rolling()`.
- Curve fitting uses NumPy's `poly.polyfit()` (coefficients) and `poly.polyval()` (curve values); the chapter explains converting coefficients from absolute-year reference to relative-year reference (`year_min`) so equation strings remain intuitive (e.g., `y = 800 + 10*t` instead of `y = -19200 + 10*t`).
- Equation strings for chart annotations are built with careful sign/whitespace formatting and prepended with airport names.
- CSS controls the side-by-side panel layout via `grid-template-rows`/`grid-template-columns`; classNames (`banner`, `parameter_selections`, `chart`) on `html.Div` elements bind to CSS blocks (`.banner{}`, etc.) and reserve rows/cells for later panels.

## Relevant Concepts

- [[concepts/dashboard]] — the artifact this chapter completes in v1.
- [[entities/dash]] — framework wrapping the callbacks.
- [[entities/plotly]] — graphics library producing the chart traces.
- [[concepts/callback]] — the mechanism wiring widgets to figure updates.
- [[concepts/object-oriented-design]] — drives the three-class split.
- [[concepts/regression]] — polynomial curve fitting on time series.
- [[concepts/polynomial]] — linear and quadratic curve families displayed on chart.
- [[concepts/smoothing]] — rolling-window mean to suppress short-term noise.
- [[concepts/css-grid]] — CSS layout system positioning widgets and chart.
- [[entities/pandas]] — `read_csv`, `isin`, `rolling`, `iloc` all used.
- [[concepts/dataframe]] — the core internal data structure (`self.df`).
- [[concepts/time-series]] — the data form being plotted via `ydecimal`.
- [[entities/atads-dataset]] — the data source.
- [[entities/numpy]] — provides `poly.polyfit`/`poly.polyval` for curve fitting.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 5 — Our First Dashboard
- File path: raw/PrototypingPythonDashboards/_txt/09-chapter-5-our-first-dashboard.txt
- Author: Padraig Houlahan
