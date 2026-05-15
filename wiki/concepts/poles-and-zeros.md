---
title: "Poles and Zeros"
type: concept
tags: [foundational, analog, ac, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt"]
confidence: high
---

## Definition

For a rational network function F(s) = N(s)/D(s), the zeros are the roots of N(s) and the poles are the roots of D(s). They are plotted in the complex s-plane (zeros as circles, poles as crosses); together with the leading constant K, they completely specify F(s).

## How It Works

Poles characterize the network's natural modes — a pole at p contributes a term proportional to e^{p t} in the transient response. Complex-conjugate pole pairs produce damped sinusoids. Repeated poles produce polynomial multipliers (t^{m-1} e^{p t}). Zeros affect the relative weight of these modes in the output.

A network with all poles in the open left half-plane is stable. Poles on the imaginary axis indicate marginal stability or sustained oscillation. Poles in the right half-plane indicate instability.

## Key Parameters

- Coordinates of each pole p_i = gamma_i + j delta_i and zero z_i = alpha_i + j beta_i.
- Multiplicity (a pole of order m contributes m terms).
- Number of poles m and zeros n; the difference m - n governs high-frequency rolloff.

## When To Use

- Stability analysis: checking that all poles have negative real parts.
- Transient response prediction.
- Filter design.
- Sensitivity analysis (pole/zero sensitivities are discussed later in Chapter 7).

## Risks & Pitfalls

- Numerical pole computation from polynomial coefficients is ill-conditioned for high orders or clustered poles.
- Pole/zero cancellation hides internal dynamics that may be uncontrollable or unobservable.
- For very high orders, discrete-frequency response sampling may miss narrow resonances (motivating pole/zero analysis as a complement to AC sweeps).

## Related Concepts

- [[concepts/network-function]]
- [[concepts/laplace-transform]]
- [[concepts/amplitude-phase-group-delay]]
- [[concepts/partial-fraction-expansion]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
- [[summaries/computer-methods-circuit-analysis-design-22-appendix-b-partial-fraction-decomposition-of-rational-functions]]
- [[summaries/computer-methods-circuit-analysis-design-23-appendix-c-special-complex-integration-of-a-rational-function]]
