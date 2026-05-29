---
title: Projection Method (DAE)
type: claim
id: concepts/projection-method-dae
tags:
- dae
- mechanical
- numerical-integration
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

A projection method for DAEs restores the constraint by projecting the numerical solution onto the constraint manifold after each integration step. For a [[concepts/constrained-mechanical-system]] with g(q) = 0, the standard projection solves min_q̃ ‖q̃ − q_{n+1}‖_M^2 subject to g(q̃) = 0 — a small QP per step — and replaces q_{n+1} by q̃. A companion velocity-level projection enforces G(q̃) u_{n+1} = 0.

## How It Works

The Lagrange-multiplier formulation gives the augmented linear system [M, G^T; G, 0] (Δq, μ) = (q_{n+1} − q̃, 0) — exactly the same structure as the index-3 step itself but solved once per step at the *output* point. Hairer–Wanner Eq. 2.10 (position projection) and Eq. 2.11 (velocity projection) detail the algorithm. Numerical experiments in Section VII.2 show that velocity-stabilisation projection alone is as effective as combined position + velocity projection — i.e. correcting ġ = G u to be exactly zero on the manifold is enough; the position drift is automatically bounded because the velocity is on the manifold.

## Key Parameters

- Projection norm (often the mass-matrix norm ‖·‖_M).
- Position vs. velocity projection (or both).
- QP / Newton tolerance for the projection.

## When To Use

- Long-time multibody dynamics where constraint accuracy matters.
- Symplectic-method post-projection in the [[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]].
- Validation of [[concepts/index-reduction]] schemes.

## Risks & Pitfalls

- Extra cost per step ≈ one constraint Newton solve.
- Projection moves the solution off the integration trajectory; very high-order integrators lose order if the projection error is sloppy.
- Velocity projection may not be enough on very stiff problems — combined position + velocity is the safe default.

## Related Concepts

- [[concepts/drift-off]]
- [[concepts/index-reduction]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/ggl-formulation]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/projected-runge-kutta]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
