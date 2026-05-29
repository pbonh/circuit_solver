---
title: Predictor-Corrector Methods
type: claim
id: claim-predictor-corrector
tags:
- transient
- numerical-integration
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt
confidence:
  base: 0.85
---

## Definition

A predictor-corrector method combines an explicit predictor formula (to generate an initial estimate of x_{n+1}) with an implicit corrector formula (iterated until convergence). Vlach & Singhal use forward Euler as predictor and backward Euler or trapezoidal as corrector.

## How It Works

1. Predictor: x_{n+1}^{(0)} = x_n + h x'_n (forward Euler).
2. Corrector iteration: x_{n+1}^{(k+1)} = x_n + h x'_{n+1}^{(k)} (backward Euler) or x_n + (h/2)(x'_{n+1}^{(k)} + x'_n) (trapezoidal).
3. Stop when |x_{n+1}^{(k+1)} - x_{n+1}^{(k)}| < epsilon, then advance.

In nonlinear circuit simulation, Newton-Raphson replaces fixed-point iteration for faster convergence.

## Key Parameters

- Predictor formula (typically same order or one lower than corrector).
- Corrector formula (A-stable for stiff systems).
- Maximum iterations per step.
- Convergence tolerance epsilon.

## When To Use

- Any implicit-method time-stepping scheme.
- Foundational pattern for SPICE-style transient analysis.
- Estimate of local truncation error (predictor - corrector difference) → step-size control.

## Risks & Pitfalls

- Fixed-point iteration converges only for small enough h (contraction condition); Newton-Raphson is more robust.
- Initial estimate quality affects iteration count.
- Predictor instability does not break corrector stability, but a bad predictor wastes iterations.

## Related Concepts

- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
