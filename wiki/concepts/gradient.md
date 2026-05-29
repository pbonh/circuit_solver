---
title: Gradient
type: claim
id: claim-gradient
tags:
- optimization
- foundational
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/18-chapter-15-introduction-to-optimization-theory.txt
confidence:
  base: 0.85
---

## Definition

The gradient of a scalar function F(x) of n variables is the n-vector nabla F = [dF/dx_1, ..., dF/dx_n]^T. It points in the direction of steepest increase. The downhill (descent) direction is -nabla F. A direction s gives descent if -s^T nabla F > 0.

## How It Works

In CAD, the gradient of an objective function F(x) over design variables x is computed by the adjoint sensitivity method of Chapter 6: one adjoint solve per output gives sensitivities to all n parameters at the cost of two LU-factor solves total (plus inner products).

All efficient optimization algorithms (steepest descent, conjugate gradient, quasi-Newton, Newton) use the gradient as their primary input. Without an analytical or adjoint-computed gradient, finite-difference approximations are noisy and unreliable.

## Key Parameters

- Number of variables n.
- Tolerance on gradient norm (for convergence check).
- Computation method (analytic, adjoint, finite difference).

## When To Use

- Steepest descent, conjugate gradient, all gradient-based optimization.
- Line-search Wolfe conditions.
- Constraint qualification in constrained optimization.

## Risks & Pitfalls

- Finite-difference gradient has step-size trade-off (truncation vs. roundoff).
- Adjoint sensitivities require care with output choice and dependency tracking.
- Non-smooth objective functions break gradient methods.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/sensitivity-analysis]]
- [[concepts/adjoint-method]]
- [[concepts/transpose-system-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-18-chapter-15-introduction-to-optimization-theory]]
