---
title: Spline Approximation (for Device Nonlinearities)
type: claim
id: concepts/spline-approximation
tags:
- device-model
- numerical
- well-established
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Spline approximation replaces an exact nonlinear device equation (e.g., the Shockley diode equation) with a piecewise-polynomial interpolant whose values and derivatives are inexpensive to evaluate. Vlach and Singhal recommend this in Chapter 11 to circumvent the computational cost and the error-prone derivation of analytic derivatives required by Newton-Raphson iteration.

## How It Works

A grid of (x, f(x), f'(x)) samples is precomputed from the original analytic model. At simulation time, the spline subroutine returns f and f' at any query point by polynomial evaluation on the bracketing interval. Cubic Hermite splines and B-splines are typical choices.

## Key Parameters

- Spline order (cubic Hermite is the most common).
- Number/distribution of knots (logarithmic for diode currents, etc.).
- Continuity (C^1 minimum for Newton convergence; C^2 desirable).

## When To Use

- Whenever evaluating a nonlinear device function or its derivative is expensive or error-prone (e.g., compound exponentials, complicated empirical models).
- When user-defined models are needed: a curve fit can be supplied as data without requiring analytic derivatives.

## Risks & Pitfalls

- A poor knot placement can cause Newton iterations to oscillate.
- C^0-only splines yield discontinuous Jacobians and break convergence.
- Extrapolation outside the fitted range can produce unphysical values.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/device-modeling]]
- [[concepts/macromodeling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
