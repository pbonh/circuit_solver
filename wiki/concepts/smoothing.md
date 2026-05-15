---
title: "Smoothing"
type: concept
tags: [data, time-series, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/09-chapter-5-our-first-dashboard.txt"]
confidence: high
---

## Definition

Smoothing is the application of a window-based filter (typically a moving average) to a time series, suppressing fluctuations shorter than the window size while preserving longer-term patterns.

## How It Works

A smoothing filter computes each output element as the mean (or weighted combination) of the original elements within a fixed window centered on that index. In pandas this is `series.rolling(window=N).mean()`. The book uses a 30-day window to reveal seasonal trends and combines two windows (9-day and 31-day) to separate weekly from seasonal variations. The filter introduces NA values near the boundaries where the window extends past the data and produces a half-window offset that must be corrected with `.shift()` to align with the raw series.

## Key Parameters

- Window size (controls which scale of variation is suppressed)
- Window shape (uniform mean, weighted, exponential)
- Boundary handling (NA padding, reflection, edge replication)
- Alignment / offset compensation

## When To Use

- Suppressing noise to reveal long-term trends
- Isolating short-term variations by subtracting smoothed from raw
- Computing scale-specific standard deviations
- Preparing chart traces that overlay raw and trend

## Risks & Pitfalls

- Boundary NAs propagate into downstream analysis
- Half-window offset corrupts time alignment if uncorrected
- Choosing window sizes that are multiples of a real periodicity can suppress signals you wanted to keep
- Aggressive smoothing destroys event-driven dips (e.g., storms, pandemics)

## Related Concepts

- [[concepts/time-series]]
- [[concepts/standard-deviation]]
- [[concepts/spectrum]]
- [[concepts/fft]]
- [[entities/pandas]]

## Sources

- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
