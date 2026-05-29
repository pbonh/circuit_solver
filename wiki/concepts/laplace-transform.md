---
title: Laplace Transform
type: claim
id: claim-laplace-transform
tags:
- foundational
- analog
- transient
- ac
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/04-chapter-1-fundamental-concepts.txt
confidence:
  base: 0.85
---

## Definition

The Laplace transform maps a time function f(t) to a function F(s) of a complex variable s = sigma + j omega via F(s) = ∫_{0-}^{infty} f(t) e^{-st} dt. It converts linear time-invariant differential equations into algebraic equations in s.

## How It Works

Key formulas used by Vlach and Singhal:
- L{u(t)} = 1/s (unit step).
- L{delta(t)} = 1 (Dirac impulse).
- L{df/dt} = sF(s) - f(0-) (used to handle initial conditions on L and C).

For circuit elements, the constitutive equations become algebraic in s, defining impedance Z(s) and admittance Y(s). Initial conditions on capacitors and inductors are represented as equivalent independent sources.

## Key Parameters

- Domain of convergence (right of the rightmost pole of F(s)).
- Choice of one-sided (Laplace) versus two-sided (Fourier-like) transform.
- s = sigma + j omega; setting s = j omega recovers the Fourier-frequency response.

## When To Use

- Solving linear time-invariant networks symbolically.
- Reducing transient ODEs to algebraic equations.
- Defining network functions, poles, and zeros.
- Bridge to numerical Laplace transform inversion (Chapter 10) for time-domain response.

## Risks & Pitfalls

- Inapplicable directly to nonlinear or time-varying circuits.
- Inverse transform requires care with distributional terms (impulses).
- Pole/residue computation is ill-conditioned for repeated or nearly-repeated poles.

## Related Concepts

- [[concepts/impedance-admittance]]
- [[concepts/poles-and-zeros]]
- [[concepts/numerical-laplace-transform-inversion]]
- [[concepts/partial-fraction-expansion]]
- [[concepts/dirac-impulse]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-04-chapter-1-fundamental-concepts]]
- [[summaries/computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion]]
- [[summaries/computer-methods-circuit-analysis-design-21-appendix-a-laplace-transforms]]
- [[summaries/computer-methods-circuit-analysis-design-22-appendix-b-partial-fraction-decomposition-of-rational-functions]]
- [[summaries/computer-methods-circuit-analysis-design-23-appendix-c-special-complex-integration-of-a-rational-function]]
