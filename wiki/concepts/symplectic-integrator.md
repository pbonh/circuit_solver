---
title: Symplectic Integrator
type: claim
id: concepts/symplectic-integrator
tags:
- hamiltonian
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

A symplectic integrator is a numerical method whose step map preserves the symplectic structure of the underlying Hamiltonian flow. The terms "symplectic integrator" and [[concepts/symplectic-method]] are interchangeable in the Hairer–Wanner usage; this page focuses on the *family* of integrators classified by structure (RK, partitioned, composition, splitting).

## How It Works

The symplectic-integrator zoo:
- **Symplectic Euler**: the partitioned (q^{n+1} = q^n + h H_p(q^{n+1}, p^n), p^{n+1} = p^n − h H_q(q^{n+1}, p^n)) — order 1.
- **Störmer–Verlet (leapfrog)**: explicit order-2 symplectic for separable H = T(p) + V(q); used everywhere in molecular dynamics.
- **Gauss IRK family**: symplectic for any number of stages s, order 2s.
- **Partitioned Lobatto IIIA–IIIB**: high-order symplectic methods for separable Hamiltonians and (with constraints) constrained Hamiltonian systems.
- **[[concepts/composition-method]]s**: build higher-order symplectic methods by composing a basic symplectic step with carefully chosen time-step weights (Yoshida 1990).
- **[[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]]**: symplectic-on-manifold methods for constrained Hamiltonian systems.

## Key Parameters

- Order p.
- Explicit / implicit / partitioned.
- Constraint-aware or not.
- Cost per step.

## When To Use

- Long-time Hamiltonian integration where energy drift is unacceptable.
- Molecular dynamics, celestial mechanics, plasma physics.
- Reversible integrators for time-reversible problems.

## Risks & Pitfalls

- Variable step sizes typically break symplecticity.
- The error per step is not smaller; the *long-time* energy error is what improves.
- Constrained Hamiltonian systems need symplectic-on-manifold variants, not generic symplectic methods.

## Related Concepts

- [[concepts/symplectic-method]]
- [[concepts/composition-method]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]
- [[concepts/gauss-method]]
- [[concepts/backward-error-analysis]]
- [[concepts/backward-error-analysis-manifolds]]
- [[concepts/manifold-differential-equation]]
- [[concepts/kepler-problem]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
