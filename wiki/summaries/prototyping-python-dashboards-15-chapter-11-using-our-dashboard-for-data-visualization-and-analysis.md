---
title: "Prototyping Python Dashboards — Chapter 11: Using Our Dashboard for Data Visualization and Analysis"
type: summary
tags: [python, visualization, spectrum, fft, time-series, analysis, modeling]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis.txt"]
confidence: high
---

## Key Points

- The chapter shifts focus from building the dashboard to using it to extract insight from the ATADS data, treating the techniques as transferable to any time-series project.
- Airport type and geography drive different traffic patterns: LAX is stable, while DEN and JFK show weather-driven dips along the East Coast; spectrum and trend tools quantify these signatures.
- Tourism-driven airports like Grand Canyon (GCN) show pre/post-pandemic activity gaps invisible without graphical comparison.
- Spectra applied to airshow data (e.g., Oshkosh OSH, Flagstaff FLG) reveal a ~362-day annual peak (not exactly 365 because annual events shift to align with weekends).
- A single-day event (Flagstaff airshow) produces a clean annual spectrum peak, while a weeklong event (Oshkosh) shows a cluttered peak structure due to overlap with seasonal background traffic.
- Phoenix Sky Harbor's spectrum reveals 7-day, 3.5-day, and 2.3-day periodicities (corresponding to once, twice, and three times per week).
- Northern airports like Juneau (JNU) show strong seasonality (warm-weather tourism) with weak day-of-week sensitivity in their weekly histograms.
- Modeling time-series patterns from synthetic Python lists is demonstrated: build a base seasonal sinusoid (`yb`) and add a flat fly-in block (`yf`) of amplitude `a2` over 5 days; controlling `a1`/`a2` reproduces qualitatively the observed spectrum behavior.
- When the fly-in spike (`a2 = 1500`) is 15x larger than the seasonal background, the spectrum develops the cluttered peak set seen at OSH; when comparable (`a2 = 150`), only the annual peak dominates.
- Suggested student/journalist project ideas: document growth/congestion trends, characterize airport type (hub vs. tourist), study impact of storms/hurricanes/fires/9-11/COVID-19, identify peer airports, build forecast models from trend coefficients, develop synthetic models matching observed spectra.

## Relevant Concepts

- [[concepts/dashboard]] — the analysis platform exercised in this chapter.
- [[concepts/spectrum]] — frequency-domain view used to find weekly/seasonal/event periodicities.
- [[concepts/fft]] — Fourier transform underlying the spectrum.
- [[concepts/time-series]] — the data type the chapter explores.
- [[concepts/data-visualization]] — driving philosophy: charts reveal hidden phenomena.
- [[concepts/regression]] — trend slopes used for forecasting.
- [[concepts/modeling]] — building synthetic data series to match observed spectra.
- [[entities/atads-dataset]] — data source for the case studies.

## Source Metadata

- Source type: book chapter
- Book title: Prototyping Python Dashboards for Scientists and Engineers
- Chapter: 11 — Using Our Dashboard for Data Visualization and Analysis
- File path: raw/PrototypingPythonDashboards/_txt/15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis.txt
- Author: Padraig Houlahan
