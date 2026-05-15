---
title: "Symplectic Method"
type: concept
tags: [hamiltonian, mechanical, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A symplectic method (also "symplectic integrator") for a Hamiltonian system q' = H_p, p' = −H_q is a numerical scheme whose one-step map Φ_h preserves the symplectic 2-form ω = ∑ dp_i ∧ dq_i — i.e. (Φ_h)^* ω = ω. Equivalently, the Jacobian DΦ_h satisfies (DΦ_h)^T J (DΦ_h) = J where J = (0, I; −I, 0).

## How It Works

Symplecticity is a geometric (volume-preserving + more) invariant of Hamiltonian flows; preserving it discretely gives long-time energy near-conservation without secular drift ([[concepts/kepler-problem]] example). Classical symplectic methods: symplectic Euler, Störmer–Verlet (order 2, leapfrog), the Gauss IRK family ([[concepts/gauss-method]]) which is symplectic for *all* s, and partitioned methods like the [[concepts/lobatto-iiia-iiib-pair]] for separable Hamiltonians. For [[concepts/constrained-hamiltonian-system]]s the integrator must be symplectic *on the constraint manifold*: [[concepts/shake-algorithm]], [[concepts/rattle-algorithm]], and Jay's high-order Lobatto IIIA-IIIB pairs accomplish this. The deep explanation is via [[concepts/backward-error-analysis]] / [[concepts/backward-error-analysis-manifolds]]: a symplectic method of order p is the exact flow of a modified Hamiltonian H̃ = H + O(h^p), so the *modified* energy is conserved exactly while the true energy oscillates around it.

## Key Parameters

- Method order p.
- Whether symplecticity holds for the full RK or only for partitioned variants.
- Constraint compatibility (symplectic on M for constrained problems).

## When To Use

- Long-time integration of Hamiltonian / mechanical systems.
- Molecular dynamics with bond constraints.
- Celestial mechanics, plasma physics, statistical mechanics.

## Risks & Pitfalls

- Adaptive step sizing destroys symplecticity; use special regularisations or fixed h.
- Symplectic methods are not magically more accurate per step — they trade per-step error for long-time energy stability.
- Implicit symplectic methods (Gauss) cost more per step than non-symplectic ones; explicit methods are usually partitioned (Verlet).

## Related Concepts

- [[concepts/symplectic-integrator]]
- [[concepts/composition-method]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]
- [[concepts/gauss-method]]
- [[concepts/backward-error-analysis]]
- [[concepts/backward-error-analysis-manifolds]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/kepler-problem]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
