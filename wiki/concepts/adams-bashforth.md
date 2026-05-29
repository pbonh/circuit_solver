---
title: Adams-Bashforth Method
type: claim
id: concepts/adams-bashforth
tags:
- transient
- numerical-integration
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The Adams-Bashforth method is a family of explicit linear multistep formulae for ODEs. The k-step Adams-Bashforth formula uses only past derivative values to extrapolate the solution: x_{n+k} = x_{n+k-1} + h sum_{j=1..k} beta_j x'_{n+k-j}. The second-order version is x_{n+k} = x_{n+k-1} + (h/2)(3 x'_{n+k-1} - x'_{n+k-2}).

## How It Works

Derived by integrating a Lagrange interpolating polynomial fit to the past derivative values. The order equals the number of past derivative samples (= k). All coefficients are positive linear combinations of binomial terms.

Used as predictors paired with Adams-Moulton correctors in classical predictor-corrector codes.

## Key Parameters

- k (number of past samples).
- Order p = k.
- Step size h.

## When To Use

- Predictor stage in Adams predictor-corrector codes.
- Non-stiff ODE problems where high explicit order is acceptable.

## Risks & Pitfalls

- Stability region shrinks with order; high-order Adams-Bashforth is unstable for moderate h on stiff problems.
- Self-starting requires k initial values; usually a lower-order method bootstraps.

## Related Concepts

- [[concepts/adams-moulton]]
- [[concepts/linear-multistep-methods]]
- [[concepts/predictor-corrector]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
