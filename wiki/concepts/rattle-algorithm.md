---
title: RATTLE Algorithm
type: claim
id: concepts/rattle-algorithm
tags:
- mechanical
- symplectic
- molecular-dynamics
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

RATTLE (Andersen 1983) is the velocity-completed extension of the [[concepts/shake-algorithm]]. After SHAKE's position-projection, RATTLE adds a velocity-projection step using a second Lagrange multiplier μ to enforce the velocity-level constraint G(q_{n+1}) u_{n+1} = 0. The result is a second-order symplectic-on-manifold integrator for [[concepts/constrained-hamiltonian-system]]s that respects both g(q) = 0 and its first derivative.

## How It Works

After SHAKE produces (q_{n+1}, q_{n+1}^*), RATTLE computes the velocity u_{n+1/2} ≈ (q_{n+1} − q_n)/h and adjusts it by μ via G(q_{n+1}) M^{−1} G(q_{n+1})^T μ = G(q_{n+1}) u_{n+1/2}. The updated u_{n+1} = u_{n+1/2} − M^{−1} G(q_{n+1})^T μ then exactly satisfies the velocity constraint. RATTLE is symplectic on the full constraint manifold M = {g = 0, G u = 0}, second-order accurate, and the molecular-dynamics standard whenever bond *and* bond-angle constraints are needed.

## Key Parameters

- Position-projection tolerance (inherited from SHAKE).
- Velocity-projection tolerance.
- Two multiplier solves per step (one for SHAKE, one for the velocity step).

## When To Use

- Molecular dynamics with constraints, especially when velocity-coupled forces (e.g. SETTLE for water) are present.
- Long-time symplectic-on-manifold integration of constrained mechanical systems.
- Any simulation where SHAKE alone leaves a constraint-velocity drift.

## Risks & Pitfalls

- Second-order only; the [[concepts/lobatto-iiia-iiib-pair]] gives higher-order alternatives.
- Iterative velocity-projection convergence depends on the conditioning of G M^{−1} G^T.
- Adaptive step destroys symplecticity.

## Related Concepts

- [[concepts/shake-algorithm]]
- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/projected-runge-kutta]]
- [[concepts/projection-method-dae]]
- [[concepts/lobatto-iiia-iiib-pair]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
