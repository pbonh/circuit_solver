---
title: State-Space Form
type: claim
id: claim-state-space-form
tags:
- ode
- dae
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

The state-space form of an index-1 DAE y' = f(y, z), 0 = g(y, z) (with g_z invertible) is the equivalent ordinary ODE y' = f(y, G(y)) obtained by eliminating z via the [[concepts/implicit-function-theorem]] map G(y) defined by g(y, G(y)) = 0. The "state space" is then the manifold of constraint-satisfying y-values, parametrised by y alone.

## How It Works

State-space form turns a [[concepts/differential-algebraic-equation]] into a plain ODE on a manifold, accessible to any non-DAE integrator. The catch is that G is only implicitly defined: each numerical step requires a nonlinear solve g(y_{n+1}, z_{n+1}) = 0 to recover z. For mechanical [[concepts/index-3-dae]] systems q' = u, M u' = f − G^T λ, g(q) = 0, the *local* state-space form uses [[concepts/generalized-coordinate-partitioning]] (Wehage–Haug 1982) or [[concepts/tangent-space-parametrization]] (Potra–Rheinboldt 1990) to set up a non-redundant parametrisation around the current state.

## Key Parameters

- Implicit map G : y ↦ z (no closed form in general).
- Cost of one nonlinear solve g(y, z) = 0.
- For higher-index DAEs, local-coordinate dimension on the constraint manifold.

## When To Use

- Index-1 DAEs and singular-perturbation problems where one wants to bypass DAE-specific solvers.
- Constrained mechanical systems with a stable parametrisation of the constraint manifold.
- Comparison baseline for DAE codes: a robust state-space ODE solver is the reference solution.

## Risks & Pitfalls

- The implicit equation may have multiple solutions; pick the branch consistent with initial conditions.
- For [[concepts/index-2-dae]] / [[concepts/index-3-dae]] problems the state-space dimension is dim(y) − rank(constraint Jacobian), often smaller than dim(y); local coordinates change as the state drifts.
- [[concepts/drift-off]] from the constraint manifold under inexact arithmetic; pair with [[concepts/projection-method-dae]] or [[concepts/baumgarte-stabilization]].

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-1-dae]]
- [[concepts/implicit-function-theorem]]
- [[concepts/generalized-coordinate-partitioning]]
- [[concepts/tangent-space-parametrization]]
- [[concepts/manifold-differential-equation]]
- [[concepts/projection-method-dae]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
