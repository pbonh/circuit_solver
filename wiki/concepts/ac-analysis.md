---
title: AC Analysis
type: claim
id: concepts/ac-analysis
tags:
- analog
- ac
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

AC analysis is a sinusoidal small-signal frequency-domain analysis that computes the steady-state response of a circuit linearized around its [[concepts/dc-analysis]] operating point. The simulator assumes the stimulus is a single small sinusoid (so the response is also a single sinusoid at the same frequency) and reports magnitude and phase as a function of frequency.

## How It Works

After finding the DC operating point, the simulator computes the small-signal Jacobian (the linearized device models) and solves the resulting linear time-invariant (LTI) system in the frequency domain by sweeping ω, one complex linear solve per frequency point. The output is a transfer function: H(jω) = V_out(jω) / V_in(jω).

## Key Parameters

- Frequency sweep range and density (linear, decade, octave)
- AC source magnitude (canonically 1 unit because the response is linear in the input)
- DC bias point definition (set by the immediately preceding DC operating-point analysis or by a user-specified bias condition)

## When To Use

For circuits that operate close to a DC equilibrium where the signal stays small enough that linearization is faithful. Canonical applications:
- Amplifier gain, bandwidth, and phase margin
- Continuous-time filters (lowpass, bandpass)
- Compensation analysis of feedback loops

## Risks & Pitfalls

- AC analysis fundamentally cannot model frequency conversion, intermodulation distortion, or noise folding because it works on a single-frequency LTI model. It is therefore **unsuitable** for:
  - Mixers, oscillators, VCOs
  - Samplers, sample-and-holds, switched-capacitor and switched-current filters
  - Chopper-stabilized amplifiers, frequency multipliers/dividers, phase detectors, parametric amplifiers, detectors
- For circuits that operate around a large periodic stimulus rather than a DC point, periodic small-signal analyses (PAC, PNoise) or harmonic balance are required.
- The DC operating point must be correct; large-signal effects (distortion, clipping) are invisible to AC.

## Related Concepts

- [[concepts/dc-analysis]]
- [[concepts/noise-analysis]]
- [[concepts/small-signal-analysis]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/transient-analysis]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
