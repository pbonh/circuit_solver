---
title: Network Function
type: claim
id: claim-network-function
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

A network function is a Laplace-domain ratio relating an output (voltage or current) of a linear time-invariant network with no initial conditions to a single source input. Common types: input impedance Zin, input admittance Yin, voltage transfer Tv, current transfer Ti, transfer impedance Ztr, transfer admittance Ytr.

## How It Works

For a network with lumped elements, each network function is a rational function in s: F(s) = (sum a_i s^i) / (sum b_i s^i) = K * prod (s - z_i) / prod (s - p_i). The roots of the numerator are zeros; the roots of the denominator are poles. Substituting s = j omega gives the steady-state sinusoidal response.

## Key Parameters

- Type (impedance, admittance, transfer).
- Order of numerator n and denominator m.
- Pole and zero locations.
- Multiplicative constant K.

## When To Use

- Frequency-response analysis (Bode plots, |F(j omega)|, phase, group delay).
- Stability analysis via pole locations.
- Filter design — synthesizing functions with prescribed amplitude/phase behavior.

## Risks & Pitfalls

- Yin = 1/Zin holds, but Ytr ≠ 1/Ztr (mentioned explicitly in Example 1.9.1).
- Pole/zero cancellation can hide internal modes that may be unstable.
- Functions are well-defined only for zero initial conditions.

## Related Concepts

- [[concepts/poles-and-zeros]]
- [[concepts/amplitude-phase-group-delay]]
- [[concepts/laplace-transform]]
- [[concepts/impedance-admittance]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
- [[summaries/computer-methods-circuit-analysis-design-08-chapter-5-sensitivities]]
- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
