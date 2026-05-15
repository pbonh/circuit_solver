---
title: "Control Problem DAE"
type: concept
tags: [dae, optimal-control, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

A control-problem DAE (Eq. 1.39 in Hairer–Wanner VII) arises when Pontryagin's minimum principle is applied to an optimal-control problem min ∫ φ(y, u) dx subject to y' = f(y, u): the necessary conditions form a DAE in (y, v, u), where v is the adjoint / costate. Specifically y' = f(y, u), v' = −f_y^T v − φ_y^T, 0 = B^T v + φ_u with B = f_u.

## How It Works

The algebraic equation 0 = B^T v + φ_u is the *stationarity condition* of the Hamiltonian H = φ(y, u) + v^T f(y, u) with respect to the control u: ∂H/∂u = 0. The system's index depends on the regularity of D = φ_uu (singular Hessian) and B^T C B (control-Jacobian rank): index 1 if D is invertible, index 2 if D is singular but B^T D' B is, and so on. Boundary conditions split between y at the initial time and v at the final time, making the problem a *boundary-value* DAE — standard shooting / collocation BVP codes apply with appropriate index handling.

## Key Parameters

- State dim(y), control dim(u), costate dim(v) = dim(y).
- Hessian D = φ_uu (invertibility ⇒ index 1).
- Control Jacobian B = f_u (rank).

## When To Use

- Trajectory optimisation in aerospace, robotics, chemical-process control.
- Theoretical analysis of nonlinear optimal-control problems.
- DAE / BVP solver benchmarking with constraint-rich systems.

## Risks & Pitfalls

- Variable index along the trajectory (bang–bang controls switch between active sets, each with its own index).
- Singular arcs require special handling — naive collocation fails.
- Direct collocation can suffer order reduction due to the algebraic component.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-of-a-dae]]
- [[concepts/euler-lagrange-equation]]
- [[concepts/lagrange-multiplier]]
- [[concepts/index-2-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
