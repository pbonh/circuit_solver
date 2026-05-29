---
title: AN-Stability
type: claim
id: concepts/an-stability
tags:
- ode
- numerical-integration
- stiff
- stability
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

AN-stability extends [[concepts/a-stability]] from the scalar linear autonomous [[concepts/dahlquist-test-equation]] y' = λy to scalar linear *non-autonomous* problems y' = λ(x)y with Re λ(x) ≤ 0 on the integration interval. A Runge–Kutta method is AN-stable if, applied to this class of test equations, it produces |y_{n+1}| ≤ |y_n| for every choice of step h > 0 and every admissible λ(·).

## How It Works

Burrage–Butcher (1979) introduced AN-stability as a stepping stone between A-stability (constant λ) and [[concepts/b-stability]] (nonlinear contractivity). For an irreducible RK method AN-stability is equivalent to the matrix M = BA + A^T B − bb^T being non-negative definite together with b_i ≥ 0 — the [[concepts/algebraic-stability]] condition. AN-stability therefore sits between A-stability and B-stability in strength, and for many method families (Gauss, Radau IA, Radau IIA, Lobatto IIIC) all three coincide.

## Key Parameters

- Diagonal matrix B = diag(b_i) and the algebraic-stability test matrix M.
- Non-negativity of the weights b_i.
- Method irreducibility (no redundant stages).

## When To Use

- Analytical setting for time-varying linear stiff systems where μ(λ(x)) ≤ 0.
- Intermediate step when proving B-stability for a method family — show algebraic stability ⇒ AN-stability ⇒ B-stability under one-sided Lipschitz contractivity.

## Risks & Pitfalls

- AN-stability is strictly weaker than B-stability for reducible methods; check S-irreducibility before equating them.
- It does not control behaviour on nonlinear or coupled systems by itself.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/b-stability]]
- [[concepts/algebraic-stability]]
- [[concepts/dahlquist-test-equation]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/contractivity]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
