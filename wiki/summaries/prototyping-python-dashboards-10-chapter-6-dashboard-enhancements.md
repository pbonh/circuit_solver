---
title: "Prototyping Python Dashboards — Chapter 6: Dashboard Enhancements"
type: summary
tags: [python, dashboard, plotly, dash, fft, spectrum, smoothing, visualization]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/10-chapter-6-dashboard-enhancements.txt"]
confidence: high
---

## Key Points

- The enhanced dashboard adds a banner, two instruction panels, monthly and weekday histograms, and a spectrum panel — eight panels total across four rows.
- New static panels (banner, instructions, spectrum instructions) only need new `atads_layout` methods with `className=` settings and matching CSS class blocks; no callbacks are required.
- Dynamic panels (histograms, spectrum) also need new methods in `atads_figures`, new callback `Output()` entries, and new return values from `update_dashboard()`.
- The monthly histogram uses `go.Box()` and replaces numeric tickvals with month-letter ticktext; it is restricted to the most recently selected year to avoid confusion.
- The weekday histogram flattens seasonal effects by subtracting a 21-day smoothed copy of the active variable from itself, so the visible day-of-week deviation is the residual.
- The spectrum chart applies NumPy's `fft.fft()` to the active variable's array after subtracting its mean (to avoid a large DC spike); resulting amplitudes (`a_vals`) are paired with a separately computed frequency list (`fq_list`) whose period reciprocals (`p_vals`) are displayed on cursor hover.
- The spectrum reveals weekly (1/7 ≈ 0.144/day), twice-weekly, and three-times-weekly periodicities in airport traffic — periods/frequencies are reciprocals.
- The chapter introduces a quantification scheme separating short-term and long-term variations: apply a 9-day smoothing filter to isolate seasonal-like background (`S09`), subtract from raw to isolate weekly variations; apply a 31-day filter (`S31`) for purely seasonal. Standard deviations `stdv09` and `stdv31` summarize each scale (amplitude ≈ 3σ).
- A toy demonstration: a period-5 sawtooth [101..105 repeating] under a 5-element smoothing filter flattens to a constant 103, and subtracting yields the pure ±2 triangle wave.
- The author deliberately labels stats as `stdv09`/`stdv31` rather than weekly/monthly so window-size choices are explicit and comparable across researchers.
- Comparing ANC and JFK for 2019: similar weekly variation (`stdv09` 99 vs 75) but ANC's seasonal `stdv31` of 222 dwarfs JFK's 52, capturing Anchorage's much stronger annual cycle.

## Relevant Concepts

- [[concepts/dashboard]] — the artifact being progressively enhanced.
- [[entities/dash]] — framework integrating the new panels.
- [[entities/plotly]] — graphics library providing `go.Box()` and `go.Scatter()`.
- [[concepts/fft]] — Fast Fourier Transform used to compute the spectrum.
- [[concepts/spectrum]] — frequency-domain view of the time series.
- [[concepts/smoothing]] — rolling-window filters used to isolate scales.
- [[concepts/standard-deviation]] — used to quantify weekly/seasonal scales.
- [[concepts/time-series]] — the data type to which all enhancements apply.
- [[concepts/css-grid]] — controls placement of the new panels.
- [[concepts/callback]] — multi-output callbacks drive multiple panel updates.
- [[entities/numpy]] — provides `fft.fft()` for the spectrum chart.
- [[entities/atads-dataset]] — data source revealing weekly/seasonal patterns.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 6 — Dashboard Enhancements
- File path: raw/PrototypingPythonDashboards/_txt/10-chapter-6-dashboard-enhancements.txt
- Author: Padraig Houlahan
