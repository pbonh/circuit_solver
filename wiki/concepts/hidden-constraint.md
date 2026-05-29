---
title: Hidden Constraint
type: claim
id: concepts/hidden-constraint
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

A hidden constraint of a DAE is an algebraic relation that holds along solutions but is not stated explicitly in the original formulation — it appears only after differentiating an existing constraint. For the index-2 DAE y' = f(y, z), 0 = g(y), one differentiation gives the hidden constraint g_y(y) f(y, z) = 0.

## How It Works

Hidden constraints encode the *velocity-level* compatibility of the system: they say that the trajectory must stay tangent to the constraint manifold {g = 0}. Without them, consistent initialisation is impossible; with them, the underlying ODE on the constraint manifold becomes well-posed. For [[concepts/index-3-dae]] mechanical systems q' = u, M u' = f − G^T λ, g(q) = 0, *two* hidden constraints appear: the velocity-level G u = 0 and the acceleration-level G u' + Ġ u = 0. Numerical methods that ignore hidden constraints suffer [[concepts/drift-off]]; methods that enforce them ([[concepts/ggl-formulation]], [[concepts/projection-method-dae]]) maintain the constraint manifold over long times.

## Key Parameters

- Order of differentiation that exposes the hidden constraint.
- Constraint Jacobian rank.
- Consistent / inconsistent initial conditions.

## When To Use

- Diagnosing solvability of higher-index DAEs.
- Constructing consistent initial conditions.
- Designing index-reduction schemes that preserve all hidden constraints.

## Risks & Pitfalls

- Forgetting a hidden constraint at consistent initialisation gives a trajectory that drifts even before discretisation error enters.
- Numerical drift-off (under inexact arithmetic) violates hidden constraints first; projection methods catch this.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-2-dae]]
- [[concepts/index-3-dae]]
- [[concepts/index-reduction]]
- [[concepts/drift-off]]
- [[concepts/projection-method-dae]]
- [[concepts/ggl-formulation]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
