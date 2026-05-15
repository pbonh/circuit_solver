---
title: "Standard Deviation"
type: concept
tags: [statistics, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/10-chapter-6-dashboard-enhancements.txt"]
confidence: high
---

## Definition

Standard deviation (σ) is a measure of the spread of a set of values around their mean. For approximately Gaussian data, ~99% of samples lie within ±3σ of the mean, making 3σ a useful proxy for signal amplitude.

## How It Works

For a dataset {x_i}, the variance is the mean of `(x_i − mean)^2` and σ is its square root. Implementations: NumPy's `np.std()`, pandas' `Series.std()`. The book uses two windowed standard deviations (`stdv09` from data minus a 9-day smoothed signal, and `stdv31` from a 31-day smoothed signal) to quantify weekly and seasonal variation scales separately. Although the data is skewed (not strictly Gaussian), σ is still useful as a comparative scale metric.

## Key Parameters

- Population (ddof=0) vs. sample (ddof=1) divisor
- Series or window over which σ is computed
- Pre-filtering (subtracting a smoothed copy) to isolate a specific scale

## When To Use

- Summarizing variability of a measurement
- Quantifying signal amplitude when paired with smoothing-based isolation
- Comparing volatility across peer datasets (airports, sites, sensors)
- Triggering anomaly detection thresholds (e.g., |x − mean| > 3σ)

## Risks & Pitfalls

- Heavy-tailed or skewed data invalidates the ±3σ interpretation
- Mixing population and sample formulas yields slightly different numbers
- Reporting σ without specifying the window or filter makes it impossible to compare results across analysts
- Outliers inflate σ disproportionately

## Related Concepts

- [[concepts/smoothing]]
- [[concepts/time-series]]
- [[concepts/probability]]
- [[concepts/descriptive-statistics]]

## Sources

- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
