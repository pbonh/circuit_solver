---
title: Pade Approximation
type: claim
id: claim-pade-approximation
tags:
- math
- numerical
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/13-chapter-10-numerical-laplace-transform-inversion.txt
confidence:
  base: 0.65
---

## Definition

A Pade approximation R_{N,M}(z) = P_N(z) / Q_M(z) of a function f(z) is a rational function whose first N+M+1 Taylor coefficients match those of f. For f(z) = e^z, closed-form expressions exist (Eq. 10.1.6 in Vlach & Singhal): P_N(z) = sum_{i=0..N} ((M+N-i)! N! / ((M+N)! i! (N-i)!)) z^i, with Q_M(z) defined similarly.

## How It Works

The Pade table for e^z lists R_{N,M} for various (N, M). For Vlach NILT, M > N + 2 is required so that R_{N,M} has at least two more poles than zeros (otherwise the contour integral at infinity does not vanish). For M not much larger than N, poles of R_{N,M} are all simple and lie in the right half-plane.

The accuracy of the inversion formula depends on the choice of (N, M) and on t. Higher (N, M) gives higher order — Vlach's published tables go up to M = 12.

## Key Parameters

- M, N (degrees of denominator and numerator).
- Position of poles z_i and residues K_i of R_{N,M}(z) (tabulated).

## When To Use

- Numerical Laplace transform inversion (Vlach method).
- Approximating other transcendental functions.
- Model-order reduction.

## Risks & Pitfalls

- High-order Pade approximations can have poles near the imaginary axis, causing instability.
- The choice of (N, M) involves a trade-off between accuracy and pole-location stability.

## Related Concepts

- [[concepts/numerical-laplace-transform-inversion]]
- [[concepts/laplace-transform]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion]]
- [[summaries/computer-methods-circuit-analysis-design-23-appendix-c-special-complex-integration-of-a-rational-function]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
