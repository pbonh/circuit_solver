---
title: Amplitude, Phase, and Group Delay
type: claim
id: claim-amplitude-phase-group-delay
tags:
- foundational
- analog
- ac
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.85
---

## Definition

For a network function F(j omega) = A(omega) + j B(omega) evaluated on the imaginary axis:
- Amplitude: |F(j omega)| = sqrt(A^2 + B^2).
- Phase: phi(omega) = arctan(B/A).
- Group delay: tau(omega) = -d phi / d omega.

These are the canonical frequency-domain response characteristics of a linear network.

## How It Works

Closed-form expressions in terms of poles p_i = gamma_i + j delta_i and zeros z_i = alpha_i + j beta_i:
- |F(j omega)| = |K| * prod sqrt(alpha_i^2 + (omega - beta_i)^2) / prod sqrt(gamma_i^2 + (omega - delta_i)^2).
- phi(omega) = sum arctan((omega - beta_i)/alpha_i) - sum arctan((omega - delta_i)/gamma_i).
- tau(omega) = sum alpha_i / (alpha_i^2 + (omega - beta_i)^2) (analogous sums from poles, see Eq. 1.9.11).

## Key Parameters

- omega (frequency, rad/s).
- Pole/zero coordinates.
- K (multiplicative gain, sign affects phase by pi).

## When To Use

- Bode plots, filter characterization (pass-band ripple, stop-band attenuation).
- Phase linearity / group-delay flatness in linear-phase filters.
- Equalization and dispersion-compensation design.

## Risks & Pitfalls

- Frequency scaling: tau scales with omega_0; if omega is divided by omega_0, the scaled tau(omega_s) is omega_0 times larger.
- Phase wrapping conventions vary; some plots use unwrapped phase.
- Group delay diverges at peaks of |F| where d phi / d omega is steep.

## Related Concepts

- [[concepts/network-function]]
- [[concepts/poles-and-zeros]]
- [[concepts/network-scaling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
