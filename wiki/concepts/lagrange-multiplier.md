---
title: Lagrange Multiplier
type: claim
id: concepts/lagrange-multiplier
tags:
- optimization
- mechanical
- dae
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

A Lagrange multiplier λ is an auxiliary variable introduced to enforce an algebraic constraint g(q) = 0 in a dynamical or optimisation problem. In a [[concepts/constrained-mechanical-system]] M q̈ = f − G^T λ, λ is the *generalised force* required to keep the trajectory on the constraint manifold {g(q) = 0}; in optimisation problems it is the dual variable / KKT multiplier.

## How It Works

For a constrained Lagrangian L̃ = L − λ^T g, stationarity gives the unconstrained dynamics plus the constraint: M q̈ + G^T λ = f, g = 0. The multiplier is solved from the equation obtained by differentiating g twice along the trajectory: G M^{−1} G^T λ = G M^{−1} f − Ġ q̇ (assuming G M^{−1} G^T invertible). The vector λ is the *constraint reaction force* in mechanical terms, the dual / shadow-price in optimisation terms, and the costate / adjoint in optimal-control terms. In numerical DAE codes, λ is solved as a stage variable in each step alongside the differential variables.

## Key Parameters

- Multiplier vector λ (one per constraint).
- Constraint Jacobian G.
- Mass / metric matrix M.

## When To Use

- Constrained mechanical systems.
- Constrained optimisation (KKT conditions).
- Optimal control (Pontryagin's minimum principle).
- Constrained variational principles.

## Risks & Pitfalls

- Multipliers can become very large near singular constraint configurations.
- Computing λ requires solving a linear system at each step; conditioning depends on G M^{−1} G^T.
- For [[concepts/ggl-formulation]] systems a *second* multiplier μ appears, enforcing the velocity-level constraint.

## Related Concepts

- [[concepts/constrained-mechanical-system]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/euler-lagrange-equation]]
- [[concepts/index-3-dae]]
- [[concepts/ggl-formulation]]
- [[concepts/control-problem-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
