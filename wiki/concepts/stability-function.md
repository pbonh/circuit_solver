---
title: "Stability Function"
type: concept
tags: [ode, numerical-integration, stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

For a one-step method applied to the [[concepts/dahlquist-test-equation]] y' = λy, the stability function R(z) is the scalar rational (or polynomial) propagator such that y_{n+1} = R(hλ) y_n. For an s-stage Runge–Kutta method with tableau (A, b, c), R(z) = 1 + z b^T (I − z A)^{−1} 𝟙. For explicit RK, A is strictly lower triangular and R(z) is a polynomial of degree s; for implicit RK, R(z) is a rational function with denominator det(I − zA).

## How It Works

R(z) is the building block of all linear-stability analysis: the [[concepts/stability-region]] is S = {z : |R(z)| ≤ 1}; A-stability is S ⊇ ℂ^−; L-stability adds R(∞) = 0. For Rosenbrock and W-methods, R(z) is again rational with the same denominator structure. Padé approximants R_{kj}(z) to e^z are the optimal-order rational approximations and characterise the highest-accuracy A-stable cases: Ehle's conjecture (proved by Wanner–Hairer–Nørsett 1978 via [[concepts/order-star]]) says R_{kj} is A-stable iff k ≤ j ≤ k + 2.

## Key Parameters

- Degree (numerator, denominator) for the rational form.
- Value R(∞) — zero means [[concepts/l-stability]].
- Order of contact p with e^z at the origin: R(z) − e^z = O(z^{p+1}).
- Padé indices (k, j).

## When To Use

- Computing or plotting the [[concepts/stability-region]] of a method.
- Comparing methods via order stars and rational-approximation theory.
- Constructing new methods with prescribed stability behaviour (Padé-based IRK, Rosenbrock).

## Risks & Pitfalls

- R(z) governs scalar linear behaviour only; nonlinear problems require [[concepts/b-stability]] / [[concepts/algebraic-stability]].
- High |R(∞)| means stiff transients are damped slowly; check for L-stability when stiff modes must die fast.
- For [[concepts/explicit-runge-kutta]], R is a polynomial that grows for large |z|, so the stability region is bounded — explicit methods cannot be A-stable.

## Related Concepts

- [[concepts/stability-region]]
- [[concepts/stability-domain]]
- [[concepts/dahlquist-test-equation]]
- [[concepts/a-stability]]
- [[concepts/l-stability]]
- [[concepts/pade-approximation]]
- [[concepts/order-star]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
