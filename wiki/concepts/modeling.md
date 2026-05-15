---
title: "Modeling"
type: concept
tags: [analysis, modeling, time-series]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis.txt"]
confidence: medium
---

## Definition

In the dashboard context, modeling is the construction of synthetic data series whose qualitative behavior reproduces features observed in real data, so that analysis tools (spectra, smoothing, trend lines) can be calibrated and interpreted.

## How It Works

The book builds a model of the Oshkosh fly-in traffic from two Python lists: a flat block (`yf`) representing the multi-day airshow burst at amplitude `a2`, and a year-long seasonal sinusoid (`yb`) at amplitude `a1`. Both are replicated to span the analysis period and added together. Adjusting `a1` and `a2` reproduces the cluttered-spectrum behavior observed at OSH (when the airshow dominates) versus a clean annual peak (when amplitudes are comparable). The model lives in the same `atads_figures.py` file as the data pipeline so dashboard tools can apply directly.

## Key Parameters

- Component signal amplitudes
- Component periodicities (annual sinusoid, weekly cycle, event-burst block)
- Background offset (zero vs. nonzero winter activity)
- Phase / timing offsets

## When To Use

- Calibrating the interpretation of dashboard spectra
- Testing analysis tools on data with known structure
- Demonstrating which features produce which spectral patterns
- Onboarding students to time-series intuition

## Risks & Pitfalls

- Models can mislead if their structure is taken too literally
- Modeling never substitutes for understanding the underlying domain
- Overly simple models miss multi-scale interactions

## Related Concepts

- [[concepts/time-series]]
- [[concepts/spectrum]]
- [[concepts/fft]]
- [[concepts/simulation]]

## Sources

- [[summaries/prototyping-python-dashboards-15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis]]
