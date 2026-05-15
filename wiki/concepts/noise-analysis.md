---
title: "Noise Analysis"
type: concept
tags: [analog, noise, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Noise analysis is a variation of [[concepts/ac-analysis]] that computes the output spectral density of a circuit driven by the intrinsic noise sources of its devices — thermal, shot, flicker (1/f), and similar — linearized around the DC operating point. Like AC, it produces a frequency-domain answer for a linear time-invariant model.

## How It Works

For each frequency point, the simulator sums the contributions of every device's noise sources, each propagated through the small-signal transfer function from the source's location to the user-specified output. The result is an output noise power spectral density and, when an input source is named, an equivalent input-referred noise. Per-device contributions are typically reported as a breakdown so the designer can see which devices dominate.

## Key Parameters

- Frequency sweep
- Output node (and reference)
- Input source for input-referred noise computation
- Per-device noise-model parameters (KF/AF for flicker, thermal coefficient, etc.)

## When To Use

- Estimating SNR and noise figure for amplifiers, filters, ADCs (at the AC-suitable portions)
- Designing low-noise input stages and choosing operating points for noise–power trade-offs

## Risks & Pitfalls

- Same fundamental limitations as [[concepts/ac-analysis]]: cannot model frequency conversion (mixers), noise folding (samplers), or noise modulation by a large periodic LO. Use PNoise / cyclostationary noise analysis for those.
- 1/f noise models depend strongly on device-model parameters; small differences in KF/AF translate into large changes in low-frequency noise predictions.

## Related Concepts

- [[concepts/ac-analysis]]
- [[concepts/small-signal-analysis]]
- [[concepts/dc-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
