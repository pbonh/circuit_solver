---
title: Reduced System
type: claim
id: concepts/reduced-system
tags:
- ode
- dae
- singular-perturbation
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The reduced system of a [[concepts/singular-perturbation-problem]] y' = f(y, z), ε z' = g(y, z) is the limit ε → 0: y' = f(y, z), 0 = g(y, z). It is an index-1 [[concepts/differential-algebraic-equation]] whose differential variable is y and whose algebraic constraint is g(y, z) = 0.

## How It Works

When g_z is invertible the algebraic equation can be solved (locally) for z = G(y) via the [[concepts/implicit-function-theorem]], yielding the equivalent ODE in [[concepts/state-space-form]] y' = f(y, G(y)). The slow / fast decomposition lets the reduced system describe the *outer* solution of the SPP — the smooth dynamics outside [[concepts/boundary-layer]]s. Vasil'eva's asymptotic expansion theorem (1963) shows the full SPP solution differs from the reduced + boundary-layer composite by O(ε^{N+1}) in the smooth regime, under μ(g_z) ≤ −1.

## Key Parameters

- Slow variable y, algebraic z.
- Constraint manifold M = {(y, z) : g(y, z) = 0}.
- Implicit map G : y ↦ z.

## When To Use

- Asymptotic / boundary-layer analysis of SPPs.
- Constructing numerical schemes that integrate only the slow variable.
- Bridge between SPP theory and index-1 DAE theory.

## Risks & Pitfalls

- The reduced system is valid only where g_z is invertible — at points where it fails (turning points) the reduction breaks down.
- It omits boundary-layer dynamics; initial conditions for the reduced system must be consistent with g(y_0, z_0) = 0.

## Related Concepts

- [[concepts/singular-perturbation-problem]]
- [[concepts/state-space-form]]
- [[concepts/index-1-dae]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/asymptotic-expansion]]
- [[concepts/boundary-layer]]
- [[concepts/implicit-function-theorem]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
