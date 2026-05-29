---
title: Lobatto IIIA–IIIB Pair
type: claim
id: concepts/lobatto-iiia-iiib-pair
tags:
- runge-kutta
- symplectic
- mechanical
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

The Lobatto IIIA–IIIB pair is a partitioned Runge–Kutta scheme using [[concepts/lobatto-iiia-method]] for the position-like variables and [[concepts/lobatto-iiib-method]] for the velocity-like variables, both on the same Lobatto nodes. Jay (1994/96) proved that the s-stage pair is symplectic on the constraint manifold of a constrained Hamiltonian system, with order 2s − 2 (Theorem 8.5 in Hairer–Wanner VII).

## How It Works

For a [[concepts/constrained-hamiltonian-system]] q' = H_p, p' = −H_q − G^T λ, g(q) = 0, the pair updates q via Lobatto IIIA (coefficients (b_i, a_{ij})) and p via Lobatto IIIB (coefficients (b̂_i, â_{ij})). Together they preserve a discrete symplectic 2-form *restricted to the constraint manifold M*. The 2-stage case (s = 2) recovers the trapezoidal-rule + symplectic-Euler pairing; the 3-stage case is the order-4 Jay pair used in long-time molecular and celestial-mechanics simulations. Coupled with [[concepts/composition-method]]s (Yoshida, Reich), arbitrarily high-order symplectic-on-manifold methods are achievable.

## Key Parameters

- Number of stages s ≥ 2.
- Order p = 2s − 2.
- Partition: q ↦ IIIA, p ↦ IIIB.
- Constraint-aware projection inside each stage.

## When To Use

- Long-time integration of constrained Hamiltonian systems (molecular dynamics with bond constraints).
- High-order symplectic-on-manifold integration where SHAKE / RATTLE order 2 is insufficient.
- Theoretical study of symplectic integration on constraint manifolds.

## Risks & Pitfalls

- Implementation is significantly more complex than unconstrained symplectic methods.
- Variable step destroys symplecticity.
- Lobatto IIIB has singular A — the velocity-stage equations need careful linear-algebra treatment.

## Related Concepts

- [[concepts/lobatto-iiia-method]]
- [[concepts/lobatto-iiib-method]]
- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/composition-method]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]
- [[concepts/runge-kutta-method]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
