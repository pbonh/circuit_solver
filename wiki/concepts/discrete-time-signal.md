---
title: "Discrete-Time Signal"
type: concept
tags: [digital, foundational, well-established, math]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt"]
confidence: high
---

## Definition

A discrete-time signal is a sequence of values x(n) indexed by integer n, representing samples of a continuous-time signal at uniform intervals. Fundamental signals: unit step u(n), Dirac sequence delta(n), shifted delta(n - k). Any signal decomposes as x(n) = sum_k x(k) delta(n - k).

## How It Works

For a linear shift-invariant (LSI) system with impulse response h(n), the output to input w(n) is the convolution y(n) = sum_k w(k) h(n - k). Difference equations sum a_k y(n-k) = sum b_k w(n-k) describe causal LSI systems and are structurally similar to LMS integration formulas.

## Key Parameters

- Sample rate T (uniform interval between samples).
- Length of impulse response (finite or infinite).
- Aliasing if sample rate < 2 * max signal frequency (Nyquist).

## When To Use

- Digital signal processing.
- Switched-capacitor circuit analysis.
- Discrete-time control systems.

## Risks & Pitfalls

- Aliasing if undersampled.
- Quantization error in finite-precision representations.

## Related Concepts

- [[concepts/z-transform]]
- [[concepts/digital-network-analysis]]
- [[concepts/switched-capacitor-network]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
