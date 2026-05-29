---
title: Coercivity Coefficient
type: claim
id: claim-coercivity-coefficient
tags:
- ode
- numerical-integration
- stiff
- runge-kutta
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.65
---

## Definition

The coercivity coefficient α_0(A^{−1}) of a Runge–Kutta matrix A is the largest constant α such that (A^{−1} u, u) ≥ α ‖u‖^2 for every stage vector u in an inner-product norm. Introduced by Crouzeix–Raviart (1980) and refined by Dekker (1984), it controls existence, uniqueness, and perturbation bounds for the implicit-stage equations on stiff problems.

## How It Works

For an implicit RK method applied to a problem satisfying the [[concepts/one-sided-lipschitz-condition]] with constant ν, the simplified-Newton stage system has a unique solution under h ν < α_0(A^{−1}), and the perturbation estimate ‖Δg‖ ≤ ‖A^{−1}‖ / (α_0(A^{−1}) − h ν) · ‖δ‖ holds. Explicit values are tabulated for Gauss, Radau IA, Radau IIA, and Lobatto IIIC methods. For the Lobatto IIIC family with s ≥ 3 the coefficient vanishes (α_0 = 0), but Liu & Kraaijevanger (1988) proved uniqueness still holds for problems with μ(f_y) ≤ 0 by a different argument.

## Key Parameters

- Inner product on the stage space (often weighted by B = diag(b_i)).
- Coefficient matrix A and its inverse A^{−1}.
- Method-specific value α_0(A^{−1}) — tabulated in Hairer–Wanner IV.14.

## When To Use

- Proving existence and uniqueness of IRK stage solutions on stiff problems.
- Estimating perturbation sensitivity of [[concepts/simplified-newton-iteration]].
- Bounding the iteration error for inexact-Jacobian solvers ([[concepts/w-method]]).

## Risks & Pitfalls

- α_0 = 0 (Lobatto IIIC with s ≥ 3) means the standard inequality is vacuous; a finer analysis is needed.
- The coefficient depends on the chosen inner product; results don't transfer directly across norms.

## Related Concepts

- [[concepts/implicit-runge-kutta]]
- [[concepts/simplified-newton-iteration]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/b-stability]]
- [[concepts/lobatto-iiic-method]]
- [[concepts/w-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
