---
title: "Spectrum"
type: concept
tags: [signal-processing, time-series, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PrototypingPythonDashboards/_txt/10-chapter-6-dashboard-enhancements.txt"]
confidence: high
---

## Definition

A spectrum is the frequency-domain representation of a time series. Each spectral coefficient indicates the amplitude with which a particular frequency contributes to the original signal; peaks identify dominant periodicities.

## How It Works

Given a uniformly sampled real-valued series of length N, the Discrete Fourier Transform (computed in practice via the FFT) produces N complex amplitudes; the magnitudes give the spectrum. The companion frequency array is derived separately from the number of samples and sampling interval. Subtracting the mean before the FFT removes the DC spike that would otherwise dominate the chart. Frequency and period are reciprocals: a peak at frequency 1/7 day corresponds to a period of 7 days.

## Key Parameters

- Sampling interval (sets max frequency = 1 / interval)
- Sample count (sets frequency resolution)
- Windowing function (to control spectral leakage; not used in the book's simple example)
- Mean-subtraction or detrending step

## When To Use

- Identifying weekly/seasonal/annual cycles in operations data
- Separating multiple superimposed periodic patterns
- Comparing the cycle structure of peer datasets
- Detecting unexpected periodicities (twice-weekly, three-times-weekly traffic)

## Risks & Pitfalls

- Aliasing if undersampled relative to the true frequency
- Leakage if the data range is not an integer multiple of the period of interest
- Edge artifacts when ranges include partial periods
- Spectra of skewed or event-driven data may interleave fly-in spikes with seasonal background, producing cluttered peak sets

## Related Concepts

- [[concepts/fft]]
- [[concepts/time-series]]
- [[concepts/smoothing]]
- [[concepts/standard-deviation]]
- [[entities/numpy]]

## Sources

- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis]]
