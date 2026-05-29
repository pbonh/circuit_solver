---
title: Fast Fourier Transform (FFT)
type: claim
id: concepts/fft
tags:
- signal-processing
- math
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PrototypingPythonDashboards/_txt/10-chapter-6-dashboard-enhancements.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The Fast Fourier Transform (FFT) is an efficient algorithm — O(N log N) instead of O(N²) — for computing the Discrete Fourier Transform of a finite, uniformly sampled sequence. It returns the complex amplitudes of the constituent frequencies of the input signal.

## How It Works

The FFT exploits the symmetric / divide-and-conquer structure of the DFT matrix to factor the transform into smaller transforms. NumPy exposes the FFT via `numpy.fft.fft(y_vals)`. The output is a complex array whose magnitudes give the spectrum amplitudes. The corresponding frequency array must be constructed separately from the sample count and the sampling interval. The book subtracts the mean before transforming so that a DC spike does not dominate the resulting spectrum chart.

## Key Parameters

- Input array length (powers of two are fastest but not required in NumPy)
- Sampling interval (used to construct the frequency axis)
- Detrending / mean-subtraction step
- Optional windowing (Hann, Hamming) to reduce leakage

## When To Use

- Computing spectra of time-series data
- Convolution via element-wise multiplication in the frequency domain
- Resampling and filtering operations
- Identifying periodic structure in noisy signals

## Risks & Pitfalls

- The FFT assumes uniform sampling; non-uniform spacing requires NUFFT or interpolation
- Aliasing if input is undersampled relative to the highest true frequency
- Spectral leakage on non-integer numbers of periods
- Care needed with the convention used for frequency normalization (cycles/sample vs. cycles/time)

## Related Concepts

- [[concepts/spectrum]]
- [[concepts/time-series]]
- [[entities/numpy]]

## Sources

- [[summaries/prototyping-python-dashboards-10-chapter-6-dashboard-enhancements]]
- [[summaries/prototyping-python-dashboards-15-chapter-11-using-our-dashboard-for-data-visualization-and-analysis]]
