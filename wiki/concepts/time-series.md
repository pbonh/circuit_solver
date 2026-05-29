---
title: Time Series
type: claim
id: claim-time-series
tags:
- data
- foundational
- modeling
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/08-chapter-4-planning-the-dashboard-prototype.txt
confidence:
  base: 0.85
---

## Definition

A time series is a sequence of measurements indexed by time. Daily airport operation counts, hourly weather readings, and monthly sales totals are all time-series data; the book's ATADS dataset records daily operation totals for each US airport.

## How It Works

A time series is typically stored as two parallel arrays: a time axis (or datetime index) and value(s). Analyzing a time series uses methods like trend regression, smoothing filters, decomposition into seasonal/trend/residual components, and spectral analysis (FFT) to identify periodicities. The book introduces a `ydecimal` column that converts year+month+day into a fractional year so daily data can be plotted on a continuous numeric axis.

## Key Parameters

- Sampling rate / frequency
- Time-axis representation (datetime, decimal year, integer index)
- Stationarity (whether mean/variance change over time)
- Seasonality and cyclic structure
- Missing-data policy

## When To Use

- Monitoring data evolution over time
- Trend identification and forecasting
- Detecting periodicities (weekly, seasonal, annual)
- Comparing peer systems' temporal behavior

## Risks & Pitfalls

- Boundary effects when smoothing (filter windows produce NA near edges)
- Mixing different sampling rates between series
- Confusing date-time formats across data sources
- Treating skewed business data with Gaussian statistical assumptions

## Related Concepts

- [[concepts/smoothing]]
- [[concepts/spectrum]]
- [[concepts/fft]]
- [[concepts/regression]]
- [[concepts/standard-deviation]]

## Sources

- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis]]
