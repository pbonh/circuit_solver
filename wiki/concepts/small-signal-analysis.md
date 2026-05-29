---
title: Small-Signal Analysis
type: claim
id: claim-small-signal-analysis
tags:
- analog
- ac
- noise
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

Small-signal analysis is the umbrella term for analyses that assume the stimulus is small enough to be treated as a perturbation of a known operating point. The circuit is linearized about that operating point, and the resulting linear (time-invariant or periodically time-varying) model is solved directly in the frequency domain or for a specified excitation.

## How It Works

A bias-point analysis (typically [[concepts/dc-analysis]], or a periodic steady-state analysis for time-varying small-signal) establishes the linearization point. The simulator builds the small-signal Jacobian — the linearization of every device model — and solves the linear system as required: a single frequency in classical [[concepts/ac-analysis]], a frequency sweep, a noise summation in [[concepts/noise-analysis]], or a linear time-varying solve for PAC/PNoise around a periodic operating point.

## Key Parameters

- Operating-point definition (DC or periodic steady-state)
- Linearization Jacobian — derived from device-model derivatives
- Frequency sweep or specific excitation

## When To Use

- AC, noise, and transfer-function analyses around a DC operating point.
- PAC / PNoise / phase-noise analyses around a periodic operating point (frequency conversion, samplers, oscillators).
- Stability and compensation analysis where the loop can be reasonably approximated linearly.

## Risks & Pitfalls

- Validity is bounded by the linearization assumption. Large signals invalidate the model and require nonlinear time-domain analyses.
- The Jacobian is only as good as the device model; missing terms in the model translate directly into missing physics in the small-signal answer.

## Related Concepts

- [[concepts/ac-analysis]]
- [[concepts/noise-analysis]]
- [[concepts/dc-analysis]]
- [[concepts/transient-analysis]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
