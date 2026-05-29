---
title: Index-1 DAE
type: claim
id: claim-index-1-dae
tags:
- dae
- ode
- singular-perturbation
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

An index-1 DAE is a [[concepts/differential-algebraic-equation]] of the semi-explicit form y' = f(y, z), 0 = g(y, z) in which the Jacobian g_z is invertible everywhere on the solution manifold. The "1" refers to the [[concepts/differentiation-index]]: exactly one differentiation of g is required to extract the underlying ODE u' = φ(u).

## How It Works

Invertibility of g_z lets the [[concepts/implicit-function-theorem]] solve g(y, z) = 0 locally for z = G(y), reducing the system to the ODE y' = f(y, G(y)) on the constraint manifold (the [[concepts/state-space-form]]). At the discrete level, ε-embedding IRK applied to the limit ε = 0 of the corresponding [[concepts/singular-perturbation-problem]] gives O(h^p) convergence in y and O(h^{q+1}) in z (q = [[concepts/stage-order]]) under [[concepts/stiffly-accurate-method]]; for non-stiffly-accurate methods only min(p, q+1) order is achieved on z, and divergence if |R(∞)| > 1.

## Key Parameters

- Differential variable dim(y).
- Algebraic variable dim(z).
- Jacobian g_z (must be invertible).

## When To Use

- Chemical kinetics with conservation laws.
- Electric circuit analysis (modified nodal analysis with linear constitutive laws).
- Limit of [[concepts/singular-perturbation-problem]]s as ε → 0.

## Risks & Pitfalls

- Loss of invertibility of g_z signals an index jump to 2 or higher — check throughout the integration interval, not just initially.
- z-component convergence is governed by stage order, not classical order — see [[concepts/order-reduction]].
- Inconsistent initial conditions (z_0 ≠ G(y_0)) need either consistency projection or boundary-layer-aware initialisation.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-of-a-dae]]
- [[concepts/differentiation-index]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/state-space-form]]
- [[concepts/reduced-system]]
- [[concepts/index-2-dae]]
- [[concepts/stiffly-accurate-method]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
