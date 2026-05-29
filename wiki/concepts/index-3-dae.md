---
title: Index-3 DAE
type: claim
id: claim-index-3-dae
tags:
- dae
- mechanical
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

An index-3 DAE is typified by [[concepts/constrained-mechanical-system]]s of the form q' = u, M(q) u' = f(q, u) − G(q)^T λ, 0 = g(q), where g is the position-level constraint and G = ∂g/∂q. Three differentiations are needed to extract λ: g, ġ = G u, g̈ = G u' + (∂G/∂q · u) u, with G M^{−1} G^T invertible.

## How It Works

The constraint manifold M = {(q, u) : g(q) = 0, G(q) u = 0} carries the dynamics; λ is the [[concepts/lagrange-multiplier]] enforcing the constraint. Three formulations are equivalent: index-3 position level (q' = u, M u' + G^T λ = f, g(q) = 0); index-2 velocity level (replace g(q) = 0 by G u = 0); index-1 acceleration level (replace by G u' + (∂G/∂q · u) u = 0). The [[concepts/ggl-formulation]] keeps both g and ġ satisfied by introducing a second multiplier μ. Numerical schemes: [[concepts/half-explicit-method]]s (Hairer–Lubich–Roche 1989; Brasey–Hairer 1993), [[concepts/projected-runge-kutta]] methods, BDF + index reduction + projection, [[concepts/baumgarte-stabilization]] for drift control. Symplectic [[concepts/lobatto-iiia-iiib-pair]] preserves the constrained Hamiltonian structure.

## Key Parameters

- Mass matrix M(q).
- Constraint Jacobian G(q).
- Lagrange multiplier λ.
- Differentiation level (position / velocity / acceleration).

## When To Use

- Multibody dynamics (Andrews squeezer, vehicle simulation, biomechanics).
- Constrained robotics / contact dynamics.
- Lagrangian / Hamiltonian mechanics with holonomic constraints.

## Risks & Pitfalls

- Naive [[concepts/index-reduction]] to index 1 introduces [[concepts/drift-off]] of the constraint by O(t^2) — projection or Baumgarte stabilisation required.
- Stage order matters even more than at lower indices; methods of stage order < 2 lose substantial accuracy on λ.
- Symplectic integration on the constraint manifold requires special method pairs (SHAKE, RATTLE, Lobatto IIIA-IIIB).

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-of-a-dae]]
- [[concepts/index-2-dae]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/half-explicit-method]]
- [[concepts/projected-runge-kutta]]
- [[concepts/ggl-formulation]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/lagrange-multiplier]]
- [[concepts/lobatto-iiia-iiib-pair]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
