---
title: Regression
type: claim
id: claim-regression
tags:
- statistics
- modeling
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/08-chapter-4-planning-the-dashboard-prototype.txt
confidence:
  base: 0.85
---

## Definition

Regression is the set of statistical techniques for fitting a parameterized curve (often a low-order polynomial) to observed data, choosing parameters that minimize the distance between the fitted curve and the data points.

## How It Works

For a chosen functional form (linear `y = a0 + a1*t`, quadratic `y = a0 + a1*t + a2*t^2`, etc.), regression solves for the coefficients that minimize an error metric (typically the sum of squared residuals). In Python, `numpy.polynomial.polynomial.polyfit()` returns the coefficient array, and `polyval()` reconstructs y-values from the coefficients across a query x-axis.

## Key Parameters

- Order of the polynomial (1 = linear, 2 = quadratic, ...)
- Data points (x, y arrays)
- Choice of reference origin (the book rebases coefficients from year-0 to year_min for readability)
- Optional weights

## When To Use

- Smoothing or summarizing a time series into a trend line
- Forecasting modest extrapolations from a stable trend
- Communicating an "annual growth rate" with a single slope number
- Comparing trends between similar datasets (peer airports)

## Risks & Pitfalls

- High-order polynomials fit noise and overfit; interpretation becomes fraught
- Extrapolation beyond data range is risky
- Coefficients computed in absolute-time (year 0 A.D.) reference are unintuitive; rebase to a local origin
- Trend lines can mask important short-period patterns; pair with smoothing or spectra

## Related Concepts

- [[concepts/polynomial]]
- [[concepts/linear-regression]]
- [[concepts/time-series]]
- [[concepts/smoothing]]
- [[entities/numpy]]

## Sources

- [[summaries/prototyping-python-dashboards-08-chapter-4-planning-the-dashboard-prototype]]
- [[summaries/prototyping-python-dashboards-09-chapter-5-our-first-dashboard]]
- [[summaries/prototyping-python-dashboards-15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis]]
