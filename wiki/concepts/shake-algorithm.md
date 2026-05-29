---
title: SHAKE Algorithm
type: claim
id: concepts/shake-algorithm
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

SHAKE (Ryckaert–Ciccotti–Berendsen 1977) is a constraint-handling extension of the Störmer–Verlet leapfrog scheme for molecular dynamics. After a Verlet step, SHAKE iteratively projects the new positions back onto the constraint manifold {g(q) = 0} by solving the constraint equations with respect to the Lagrange multipliers λ.

## How It Works

The unconstrained Verlet step gives q_{n+1}^* = 2 q_n − q_{n−1} + h² M^{−1} f(q_n). SHAKE then iteratively solves G(q_n) M^{−1} G(q_n)^T λ = g(q_{n+1}^* − h² M^{−1} G^T λ) / h², updating q_{n+1} = q_{n+1}^* − h² M^{−1} G(q_n)^T λ until g(q_{n+1}) ≈ 0. The method is symplectic on the constraint manifold (proved by Leimkuhler–Skeel 1994), second-order accurate, and the standard for biomolecular simulations with bond-length and bond-angle constraints. Its velocity-completed cousin is the [[concepts/rattle-algorithm]] (Andersen 1983), which additionally enforces the velocity-level constraint.

## Key Parameters

- Position-projection tolerance.
- Number of iteration cycles per step.
- Mass matrix M (typically diagonal for atoms).
- Constraint Jacobian G(q).

## When To Use

- Molecular dynamics with bond / angle constraints.
- Bead–spring polymer simulations.
- Long-time mechanical simulation where symplectic energy preservation matters.

## Risks & Pitfalls

- Iterative solver may converge slowly when constraints are stiff or highly coupled.
- Does not enforce velocity-level constraint — use [[concepts/rattle-algorithm]] when that matters.
- Second-order only; for higher order use the [[concepts/lobatto-iiia-iiib-pair]] or [[concepts/composition-method]]s.

## Related Concepts

- [[concepts/rattle-algorithm]]
- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/projected-runge-kutta]]
- [[concepts/projection-method-dae]]
- [[concepts/lobatto-iiia-iiib-pair]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
