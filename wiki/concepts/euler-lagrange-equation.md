---
title: "Euler–Lagrange Equation"
type: concept
tags: [mechanical, lagrangian, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The Euler–Lagrange equation for a Lagrangian L(q, q̇, t) is d/dt (∂L/∂q̇) − ∂L/∂q = 0, the necessary condition for a trajectory q(t) to be a stationary point of the action functional ∫ L dt. For constrained systems with g(q) = 0, the equation generalises to d/dt(∂L/∂q̇) − ∂L/∂q = G(q)^T λ where λ is the Lagrange multiplier.

## How It Works

Euler–Lagrange equations are the foundation of analytical mechanics. For mechanical systems L = T − V with T = (1/2) q̇^T M(q) q̇ kinetic and V(q) potential, the equation becomes M(q) q̈ = f(q, q̇) − G(q)^T λ — the standard [[concepts/constrained-mechanical-system]] form used in Hairer–Wanner VII. The Hamiltonian formulation comes from the Legendre transform p = ∂L/∂q̇, H(q, p) = p^T q̇ − L. Numerical integration of Euler–Lagrange equations is the central topic of Chapter VII Section 8 (symplectic methods on the constraint manifold).

## Key Parameters

- Lagrangian L(q, q̇, t).
- Configuration space dimension dim(q).
- Constraint functions g_i(q) and their multipliers λ_i.

## When To Use

- Foundational formulation of mechanical systems.
- Setting up multibody / robot / vehicle models.
- Variational integrators (discrete Euler–Lagrange equations).

## Risks & Pitfalls

- Symbolic derivation of d/dt(∂L/∂q̇) − ∂L/∂q is non-trivial for complex Lagrangians.
- Naïve numerical discretisation can violate the symplectic structure; prefer variational / symplectic integrators.

## Related Concepts

- [[concepts/constrained-mechanical-system]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/lagrange-multiplier]]
- [[concepts/symplectic-method]]
- [[concepts/index-3-dae]]
- [[concepts/control-problem-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
