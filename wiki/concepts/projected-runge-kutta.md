---
title: "Projected Runge–Kutta"
type: concept
tags: [dae, runge-kutta, mechanical, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A projected Runge–Kutta method (Ascher–Petzold 1991; Hairer–Wanner) is a standard RK integrator for an index-reduced DAE followed by an explicit *projection* step that restores the original constraint after each integration step. The projection solves min ‖q̃ − q_{n+1}‖_M^2 subject to g(q̃) = 0 (or the velocity-level analogue).

## How It Works

The integrator alternates a regular RK step (treating the index-1 or index-2 reduced system) with a projection step (a small QP / Newton solve) that maps the result back to the constraint manifold. For [[concepts/index-2-dae]] systems this recovers the position-level constraint that the reduced index-1 system would otherwise lose; for [[concepts/index-3-dae]] [[concepts/constrained-mechanical-system]]s the projection cost is modest because the constraint Jacobian and mass matrix are already factored as part of the RK Newton step. Convergence: the differential variable retains the RK's classical order; the algebraic variable retains stage-order convergence. Projection does not break L-stability, B-stability, or symplecticity (the projection is itself symplectic when done with the [[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]] pattern).

## Key Parameters

- Underlying RK method order p.
- Projection norm (mass-matrix or Euclidean).
- Projection tolerance.

## When To Use

- Index-2 / index-3 DAE solvers where constraint accuracy matters.
- Long-time multibody and molecular dynamics.
- Symplectic-friendly integration on constraint manifolds.

## Risks & Pitfalls

- Without projection, the reduced-system integrator drifts off the constraint manifold ([[concepts/drift-off]]).
- Projection costs roughly one extra Newton solve per step — acceptable when the integration step already has Jacobian-factor amortisation.
- Projection error must be tighter than discretisation error to avoid order reduction.

## Related Concepts

- [[concepts/projection-method-dae]]
- [[concepts/runge-kutta-method]]
- [[concepts/drift-off]]
- [[concepts/index-reduction]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/ggl-formulation]]
- [[concepts/half-explicit-method]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
