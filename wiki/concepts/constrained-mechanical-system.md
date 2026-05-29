---
title: Constrained Mechanical System
type: claim
id: claim-constrained-mechanical-system
tags:
- dae
- mechanical
- lagrangian
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

A constrained mechanical system is a Lagrangian dynamical system with holonomic constraints g(q) = 0. Its equations of motion (Eq. 1.46 in Hairer–Wanner VII) are q' = u, M(q) u' = f(q, u) − G(q)^T λ, 0 = g(q), where M(q) is the (symmetric positive definite) mass matrix, f(q, u) the applied forces, G(q) = ∂g/∂q the constraint Jacobian, and λ the [[concepts/lagrange-multiplier]] enforcing g(q) = 0. The system is [[concepts/index-3-dae]] in this position-level form.

## How It Works

Three equivalent formulations are used in practice: index 3 (position level: g(q) = 0), index 2 (velocity level: G u = 0), and index 1 (acceleration level: G u' + Ġ u = 0). The matrix G(q) M(q)^{−1} G(q)^T must be invertible (the "non-degenerate constraint" condition). Numerical methods include [[concepts/half-explicit-method]]s (Hairer–Lubich–Roche 1989, Brasey–Hairer 1993), [[concepts/projected-runge-kutta]] methods, BDF + index reduction + projection, [[concepts/baumgarte-stabilization]] for drift control, and symplectic methods ([[concepts/lobatto-iiia-iiib-pair]], [[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]]) for [[concepts/constrained-hamiltonian-system]]s. The benchmark "Andrews [[concepts/squeezer-mechanism]]" is a 7-body, 6-constraint test problem with three loops; the stiff variant adds stiff springs.

## Key Parameters

- Mass matrix M(q).
- Constraint Jacobian G(q).
- Lagrange multipliers λ (one per constraint).
- Index level (3 / 2 / 1).

## When To Use

- Multibody dynamics (vehicles, robots, biomechanics).
- Lagrangian mechanics with explicit constraints.
- Industrial mechanism-design simulation.

## Risks & Pitfalls

- Index-reduction without stabilisation causes [[concepts/drift-off]].
- Stiff variants with mixed time scales need stiff DAE solvers (RADAU5, RODAS).
- Singular configurations (loss of constraint regularity) require special treatment.

## Related Concepts

- [[concepts/index-3-dae]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/lagrange-multiplier]]
- [[concepts/euler-lagrange-equation]]
- [[concepts/half-explicit-method]]
- [[concepts/projected-runge-kutta]]
- [[concepts/ggl-formulation]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/projection-method-dae]]
- [[concepts/multibody-system]]
- [[concepts/squeezer-mechanism]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
