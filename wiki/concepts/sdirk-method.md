---
title: SDIRK Method
type: claim
id: claim-sdirk-method
tags:
- ode
- numerical-integration
- runge-kutta
- stiff
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

A Singly Diagonally Implicit Runge–Kutta (SDIRK) method is a [[concepts/dirk-method]] with all diagonal entries equal: a_{ii} = γ for all i. The matrix I − h γ J then factorises once per step and is reused for every stage, giving per-step linear-algebra cost comparable to a single backward-Euler step.

## How It Works

A common γ collapses the order conditions to polynomial constraints in γ. Nørsett's order-3 SDIRK uses γ ≈ 0.4358665 (real root of γ^3 − 3γ^2 + 3γ/2 − 1/6 = 0) for A-stability; Hairer–Wanner SDIRK4 (the 5-stage, order 4(3) embedded code) uses γ = 1/4 with rational stage placements c'_2 = 1/2, c'_3 = 3/5, paired with a continuous third-order embedded estimator. With this single γ, the method is L-stable when the last row equals b^T ([[concepts/stiffly-accurate-method]]). Practical implementations exploit the shared LU by storing only one factorisation throughout the step.

## Key Parameters

- Common diagonal value γ.
- Number of stages s.
- Order p (classical), q (stage), p̂ (embedded estimator).
- L-stability flag (stiffly-accurate row?).

## When To Use

- Stiff problems where one LU per step is the cost budget.
- Codes that need a simple, robust DIRK family with embedded error estimation.
- Index-1 DAEs (use stiffly-accurate variant).

## Risks & Pitfalls

- Stage order q = 1 (or 2 for special families) means severe [[concepts/order-reduction]] on stiff problems.
- The value of γ is a delicate trade-off between A-stability, L-stability, and error constant.
- Not B-stable in general; the matrix M = BA + A^T B − bb^T is rarely non-negative definite for SDIRK.

## Related Concepts

- [[concepts/dirk-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/l-stability]]
- [[concepts/simplified-newton-iteration]]
- [[concepts/rosenbrock-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
