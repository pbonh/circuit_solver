---
title: Stage Order
type: claim
id: concepts/stage-order
tags:
- ode
- numerical-integration
- runge-kutta
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The stage order of a Runge–Kutta method is the largest integer q such that the simplifying assumption C(q) — ∑_j a_{ij} c_j^{k−1} = c_i^k / k for k = 1, …, q — holds. Equivalently, q is the order to which each internal stage Y_i agrees with the exact solution at x_n + c_i h. Always q ≤ p, the classical order of the method.

## How It Works

C(η) controls the order of interpolation built into each stage. A method's effective order on stiff problems and [[concepts/differential-algebraic-equation]]s is governed by q, not by p, because stiff/algebraic components only see the stage interpolant — this is the precise mechanism of [[concepts/order-reduction]] and [[concepts/b-convergence]]. For Gauss methods of s stages, p = 2s but q = s; for [[concepts/radau-iia-method]] p = 2s − 1, q = s; for [[concepts/lobatto-iiic-method]] p = 2s − 2, q = s − 1 (Lobatto IIIC has stage order s − 1, not s, because c_1 = 0 forces a row constraint). [[concepts/sdirk-method]]s and [[concepts/dirk-method]]s typically have q = 1, which is why they reduce dramatically on stiff problems.

## Key Parameters

- C(η) assumption order η — the stage-order quantity.
- D(ζ) — companion left-handed condition for high p.
- B(p) — quadrature order.

## When To Use

- Predicting B-convergence rate for stiff / DAE problems.
- Designing collocation methods whose stage order matches their classical order on smooth problems.
- Choosing between methods of equal classical order — higher stage order is preferable for stiff use.

## Risks & Pitfalls

- High stage order requires more stages or specific node placement.
- A high p, low q method (Gauss) is excellent on smooth problems but reduces drastically on stiff.

## Related Concepts

- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/order-reduction]]
- [[concepts/b-convergence]]
- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/radau-iia-method]]
- [[concepts/gauss-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
