---
title: Singular Perturbation Problem
type: claim
id: claim-singular-perturbation-problem
tags:
- ode
- dae
- stiff
- singular-perturbation
- asymptotic
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

A singular perturbation problem (SPP) is a coupled ODE system y' = f(y, z), ε z' = g(y, z) in which the small parameter ε > 0 multiplies the derivative of the *fast* variable z. As ε → 0 the system degenerates to a [[concepts/differential-algebraic-equation]] of index 1: y' = f(y, z), 0 = g(y, z), called the [[concepts/reduced-system]]. Singularity of the limit is in the *order* of the ODE (it drops from second to first), not in any regularity sense.

## How It Works

When g_z is invertible (Assumption 1.7 in Hairer–Wanner) the algebraic constraint g(y, z) = 0 defines z = G(y) via the [[concepts/implicit-function-theorem]], giving the limit [[concepts/state-space-form]] y' = f(y, G(y)). For ε > 0 small, the smooth solution admits an [[concepts/asymptotic-expansion]] in powers of ε (Vasil'eva 1963); a [[concepts/boundary-layer]] term in η = (x − x_0)/ε is added when the initial z(x_0) is not consistent with g(y(x_0), z) = 0. Classical examples: [[concepts/van-der-pol-equation]] in Liénard coordinates (Dorodnicyn 1947), chemical kinetics with fast / slow reactions, [[concepts/method-of-lines]] discretisations of parabolic PDE. Numerical solution uses [[concepts/epsilon-embedding-method]] (apply IRK to full SPP, then ε → 0) or directly the index-1 [[concepts/state-space-form]].

## Key Parameters

- Small parameter ε > 0 (with ε → 0 the singular limit).
- Slow variable y, fast variable z.
- Jacobian g_z (must be invertible for index-1 SPP).
- Logarithmic norm μ(g_z) ≤ −1 (stability hypothesis for boundary layers).

## When To Use

- Multi-scale dynamical systems with widely separated time constants.
- Reaction–diffusion problems where the diffusive term sets the slow scale.
- Theoretical bridge between stiff ODEs and DAEs.

## Risks & Pitfalls

- Inconsistent initial conditions trigger [[concepts/boundary-layer]] transients; numerical methods must handle them or suffer order reduction.
- Even stiffly-accurate IRK methods reduce in order on the algebraic component (see [[concepts/stage-order]] / [[concepts/order-reduction]]).
- The singular limit may itself be singular (g_z(y, z) not invertible on the solution manifold) — these are higher-index problems.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/reduced-system]]
- [[concepts/asymptotic-expansion]]
- [[concepts/boundary-layer]]
- [[concepts/epsilon-embedding-method]]
- [[concepts/state-space-form]]
- [[concepts/index-1-dae]]
- [[concepts/order-reduction]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
