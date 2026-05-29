---
title: Fourier Analysis
type: claim
id: claim-fourier-analysis
tags:
- analog
- transient
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.85
---

## Definition

In SPICE-class circuit simulators, Fourier analysis computes the Fourier-series coefficients of a steady-state, periodic transient waveform. It is run as a post-processing step on a [[concepts/transient-analysis]] result, evaluated over one period after the circuit has settled.

## How It Works

The transient waveform is sampled at the simulator's adaptive timepoints, **linearly interpolated** onto an equally-spaced grid over one period, and a discrete Fourier transform (DFT) is applied. The number of harmonics requested by the user dictates the grid density.

## Key Parameters

- Fundamental period (must be set correctly or harmonics are aliased into nonsense)
- Number of harmonics
- `tstep`, `tmax`, `reltol` — all control the underlying transient timestep, which directly limits Fourier resolution

## When To Use

- Computing harmonic distortion (THD, HD2, HD3) of amplifiers and filters
- Estimating spectral content of oscillator outputs after settling

## Risks & Pitfalls

- **Resolution is dominated by interpolation error.** Linear interpolation of an ideal cosine on 50 points generates spurs as high as -54 dB — this directly caps Fourier resolution at 40-60 dB with default settings even though the underlying transient waveform is far more accurate.
- SPICE compounds the issue with poorly-chosen tstep/tmax defaults and non-obvious control mechanisms.
- Achieving 120 dB resolution (which users routinely want for distortion analysis) requires manually tightening `tstep`, `tmax`, and `reltol` aggressively — see Kundert's *The Designer's Guide to SPICE and Spectre*.
- Other failure modes: incomplete settling before the analysis window, wrong period selected, aliasing, simulator noise floor.
- For genuine spectral analysis around periodic operating points, use harmonic balance or shooting+PSS instead of transient-plus-Fourier.

## Related Concepts

- [[concepts/transient-analysis]]
- [[concepts/local-truncation-error]]
- [[concepts/ac-analysis]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
