---
title: "Pendulum Equation (DAE Form)"
type: concept
tags: [mechanical, dae, benchmark, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The pendulum equation in DAE form is the [[concepts/index-3-dae]] description of a planar pendulum: a point mass m at position q = (x, y) suspended from the origin by a massless rod of length ℓ. The equations are q' = u, m u' = (0, −m g)^T − G(q)^T λ, 0 = x² + y² − ℓ², where G(q) = (2x, 2y) is the constraint Jacobian. It is the simplest non-trivial constrained mechanical system and the canonical pedagogical example for higher-index DAEs.

## How It Works

The constraint g(q) = x² + y² − ℓ² = 0 confines the mass to a circle. Differentiating once gives the velocity-level constraint G u = 2x u_x + 2y u_y = 0 (tangency); differentiating again gives the acceleration-level constraint, from which λ can be solved as a Lagrange multiplier. Numerical experiments compare half-explicit, projected RK, BDF + projection, and symplectic methods (the [[concepts/lobatto-iiia-iiib-pair]] preserves the energy almost exactly while drift-control methods slowly bleed energy).

## Key Parameters

- Mass m, length ℓ, gravity g.
- Initial angle and angular velocity.
- Period ≈ 2π √(ℓ/g) for small oscillations.

## When To Use

- Pedagogical illustration of index-3 DAE and constraint handling.
- Quick test of new DAE / symplectic integrators.
- Demonstration of [[concepts/drift-off]] and stabilisation techniques.

## Risks & Pitfalls

- Simple enough that some methods (Baumgarte with bad parameters) look acceptable on it but fail on real multibody problems.
- For long-time integration, only symplectic-on-manifold methods preserve the energy.

## Related Concepts

- [[concepts/index-3-dae]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/multibody-system]]
- [[concepts/symplectic-method]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/lagrange-multiplier]]
- [[concepts/drift-off]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
