---
title: Spectral Analysis of Switched-Capacitor Networks
type: claim
id: claim-sc-spectral-analysis
tags:
- switched-capacitor
- ac
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/17-chapter-14-digital-and-switched-capacitor-networks.txt
confidence:
  base: 0.65
---

## Definition

Spectral analysis of switched-capacitor networks computes the output frequency spectrum given a continuous-time input. Because SC networks are periodically time-varying (period T_clock), the output spectrum contains components at the input frequency plus aliases at f_clock +/- f_input and integer multiples thereof.

## How It Works

For input w(t) = exp(j 2 pi f t):
- Output y(t) = sum_n H_n(f) exp(j 2 pi (f + n f_clock) t).
- H_0(f) is the desired-band transfer function.
- H_{n != 0}(f) are aliases.

For f_input << f_clock/2, the alias contributions are small and the SC filter approximates a continuous-time filter with H(f) ≈ H_0(f).

In Vlach & Singhal Section 14.8, the spectral computation reduces to solving the per-phase LTI equations and combining the phase outputs via the Fourier series of the clock waveform.

## Key Parameters

- Input frequency f.
- Clock frequency f_clock.
- Phase duty cycle.
- Anti-aliasing pre-filter (continuous-time, on the input).

## When To Use

- SC filter design verification.
- Aliasing prediction and pre-filter sizing.
- Noise analysis of SC circuits.

## Risks & Pitfalls

- Aliases near the band of interest can degrade filter performance.
- The convergence of the alias sum may be slow at high f.

## Related Concepts

- [[concepts/switched-capacitor-network]]
- [[concepts/clock-phase-formulation]]
- [[concepts/digital-network-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
