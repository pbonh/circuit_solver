---
title: "Constrained Hamiltonian System"
type: concept
tags: [dae, mechanical, hamiltonian, symplectic, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A constrained Hamiltonian system is a Hamiltonian dynamical system with holonomic constraints: q' = H_p, p' = −H_q − G(q)^T λ, 0 = g(q), where H(q, p) is the Hamiltonian and λ the [[concepts/lagrange-multiplier]] enforcing the position-level constraint g(q) = 0. It is an [[concepts/index-3-dae]] and the natural Hamiltonian counterpart of the Lagrangian [[concepts/constrained-mechanical-system]].

## How It Works

Theorem VII.8.1 (Hairer–Wanner): the flow of a constrained Hamiltonian system is *symplectic on the constraint manifold* M = {(q, p) : g(q) = 0, G(q) H_p(q, p) = 0} — the symplectic 2-form is preserved when restricted to M, even though the unconstrained Hamiltonian flow is not symplectic in (q, p, λ). Numerical [[concepts/symplectic-integrator]]s for the constrained system must preserve this restricted symplecticity. The first-order symplectic method (Eq. 8.8) uses position projection after each step; second-order [[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]] integrate molecular dynamics while preserving symplectic structure; Jay's (1994/96) [[concepts/lobatto-iiia-iiib-pair]] gives high-order (2s − 2) symplectic methods on M. [[concepts/composition-method]]s (Yoshida 1990, Reich 1996) build arbitrarily-high-order symplectic methods.

## Key Parameters

- Hamiltonian H(q, p).
- Constraint g(q).
- Constraint Jacobian G(q).
- Multiplier λ.

## When To Use

- Molecular dynamics with bond-length constraints.
- Celestial mechanics with reduction to centre-of-mass frame.
- Long-time integration of constrained Hamiltonian models.

## Risks & Pitfalls

- Non-symplectic integrators drift in energy over long times — disqualifying for long Hamiltonian integration.
- Symplecticity on the constraint manifold requires special projection / integrator structure.
- [[concepts/backward-error-analysis-manifolds]] explains the long-term near-conservation but only under symplectic preservation.

## Related Concepts

- [[concepts/constrained-mechanical-system]]
- [[concepts/index-3-dae]]
- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/composition-method]]
- [[concepts/manifold-differential-equation]]
- [[concepts/backward-error-analysis-manifolds]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
