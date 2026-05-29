---
title: Implicit Runge–Kutta
type: claim
id: claim-implicit-runge-kutta
tags:
- ode
- numerical-integration
- runge-kutta
- stiff
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

A Runge–Kutta method is implicit if its coefficient matrix A is not strictly lower triangular — i.e. at least one stage depends on its own value (or on a later stage). The stage equations Y_i = y_n + h ∑_j a_{ij} f(Y_j) form a coupled nonlinear system of size s · dim(y) and must be solved at each step (typically by [[concepts/simplified-newton-iteration]]).

## How It Works

Implicit IRK methods reach much higher orders for a given stage count than explicit methods: Gauss with s stages has order 2s, [[concepts/radau-iia-method]] has order 2s − 1, [[concepts/lobatto-iiic-method]] has order 2s − 2. Their [[concepts/stability-function]] is rational (R(z) = det(I − zA + z𝟙b^T)/det(I − zA)) and can be A-stable, L-stable, or both; this makes them the canonical choice for stiff and DAE problems. Practical implementations (RADAU5, SDIRK4) reduce the cost of the dense Newton system: SDIRK uses a single repeated γ on the diagonal so one LU per step suffices; Radau IIA uses an eigenvalue decomposition of A^{−1} into one real eigenvalue and a complex pair, giving a real n × n plus a complex n × n solve instead of the full (3n) × (3n).

## Key Parameters

- Number of stages s.
- Tableau (A, b, c); shape of A (DIRK, SDIRK, full).
- Order p, stage order q, R(∞).
- Newton-solver Jacobian reuse rate.

## When To Use

- Stiff ODEs and DAEs (index 1; index 2 with [[concepts/stiffly-accurate-method]] structure).
- Singular-perturbation and reaction–diffusion problems.
- High-accuracy long-time integration where step size is set by accuracy, not stability.

## Risks & Pitfalls

- The nonlinear solver dominates cost on stiff problems; reusing LU and Jacobian factors is essential.
- High classical order ≠ high stage order — see [[concepts/order-reduction]].
- Lobatto IIIA/IIIB are A-stable but not L- or B-stable; consult the family-specific page before selecting.
- For very large dim(y), dense Newton scales as O(s^3 n^3); switch to Krylov-based variants or [[concepts/rosenbrock-method]] (linearly implicit) when n is huge.

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/sdirk-method]]
- [[concepts/dirk-method]]
- [[concepts/gauss-method]]
- [[concepts/radau-iia-method]]
- [[concepts/lobatto-iiic-method]]
- [[concepts/simplified-newton-iteration]]
- [[concepts/stiffly-accurate-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
