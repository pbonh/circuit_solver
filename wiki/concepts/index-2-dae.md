---
title: Index-2 DAE
type: claim
id: concepts/index-2-dae
tags:
- dae
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

An index-2 DAE is a semi-explicit system y' = f(y, z), 0 = g(y) in which the algebraic constraint g depends only on the differential variable y (not on z), and g_y f_z is invertible. Two differentiations of g are required to extract the underlying ODE; the first produces the [[concepts/hidden-constraint]] g_y f(y, z) = 0.

## How It Works

The hidden constraint must hold along solutions even though it is not explicit in the original formulation; consistent initialisation requires (y_0, z_0) with both g(y_0) = 0 and g_y(y_0) f(y_0, z_0) = 0. Numerical schemes: [[concepts/runge-kutta-method]] with the ε-embedding approach gives Y_n = O(h^p), Z_n = O(h^{q+1}) for [[concepts/stiffly-accurate-method]]s; non-stiffly-accurate methods diverge in z. [[concepts/runge-kutta-collocation|Collocation]] at Radau IIA points achieves superconvergence O(h^{2s−1}) in y and O(h^s) in z. BDF achieves O(h^p) in y and O(h^{p−1}) in z after k initialisation steps. [[concepts/projected-runge-kutta]] restores the position-level constraint after each step. [[concepts/ggl-formulation]] augments the system with an extra multiplier μ to satisfy both g and ġ.

## Key Parameters

- Constraint g depending only on y.
- Hidden constraint g_y f.
- g_y f_z invertible.
- Stage order q of the integrator.

## When To Use

- Velocity-level constraints in [[concepts/constrained-mechanical-system]]s after one [[concepts/index-reduction]] step.
- Optimal control problems with state-only constraints.
- Reaction networks with conserved quantities depending only on concentrations.

## Risks & Pitfalls

- z-component converges at stage order, not classical order — see [[concepts/order-reduction]].
- Inconsistent initial values produce O(1) errors that take many steps to damp.
- Drift-off in the constraint manifold without projection or stabilisation.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-of-a-dae]]
- [[concepts/hidden-constraint]]
- [[concepts/index-1-dae]]
- [[concepts/index-3-dae]]
- [[concepts/projected-runge-kutta]]
- [[concepts/ggl-formulation]]
- [[concepts/radau-iia-method]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
