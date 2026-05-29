---
title: Error Constant
type: claim
id: claim-error-constant
tags:
- ode
- numerical-integration
- error-analysis
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

The error constant C of a numerical method of order p is the leading coefficient of the local truncation error: τ(x; h) = C h^{p+1} y^{(p+1)}(x) + O(h^{p+2}). It is the single scalar (or sometimes vector) that measures how accurate the method is at its claimed order.

## How It Works

For a [[concepts/linear-multistep-methods]] method (ρ, σ), C is determined by ρ(e^h) − h σ(e^h) = C h^{p+1} + O(h^{p+2}). For [[concepts/runge-kutta-method]]s, C is the maximum over Butcher trees of the residual order-p+1 conditions. The error constant directly governs the step-size selection through err ≈ |C| h^{p+1} ‖y^{(p+1)}‖. Two methods of the same order p with error constants C_1 and C_2 require step sizes h_1 / h_2 ≈ (|C_2|/|C_1|)^{1/(p+1)} for equal local error — so a factor-of-32 difference in C drops the step size by 2 for p = 4.

## Key Parameters

- Method order p.
- Constant C (signed scalar).
- For multi-component RK methods, an error-constant *vector* (one per tree).

## When To Use

- Method comparison across families of equal order.
- Step-size prediction formulas inside adaptive codes.
- Method design: minimise |C| subject to stability constraints.

## Risks & Pitfalls

- A small |C| does not protect against [[concepts/order-reduction]] on stiff problems — only the [[concepts/stage-order]] does.
- Error constants depend on the chosen norm; sign and magnitude can shift across formulations.
- Trapezoidal rule has C = −1/12, the smallest |C| compatible with A-stability for an order-2 LMS — Dahlquist's extremal.

## Related Concepts

- [[concepts/peano-kernel]]
- [[concepts/dahlquist-barrier]]
- [[concepts/trapezoidal-rule]]
- [[concepts/linear-multistep-methods]]
- [[concepts/runge-kutta-method]]
- [[concepts/order-reduction]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
