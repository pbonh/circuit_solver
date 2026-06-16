---
title: Runge-Kutta Methods
type: concept
slug: runge-kutta-methods
created: 2026-06-16
updated: 2026-06-16
summary: One-step multi-stage ODE integration methods, both explicit (non-stiff) and implicit (stiff); Radau IIA is the gold standard for stiff problems and index-1 DAEs.
tags: [numerical-methods, runge-kutta, radau, sdirk, stiff-ode, dae]
sources: [solving-ode-ii-stiff-dae]
status: active
---

# Runge-Kutta Methods

Runge-Kutta methods advance the ODE solution from t_n to t_{n+1} by computing s intermediate "stage" values k_i, then combining them into the next step. Described by a Butcher tableau (A matrix, b vector, c vector). Order p means the method is exact for polynomials up to degree p.

## Explicit vs. Implicit

| Type | A matrix | Cost | Stiff? |
|---|---|---|---|
| ERK (explicit) | Strictly lower triangular | s function evaluations | Not suitable (unstable) |
| DIRK (diagonally implicit) | Lower triangular | s sequential linear solves | Moderate stiffness |
| SDIRK | Lower triangular, all diagonal entries equal | 1 LU + s back-solves | Moderate stiffness |
| IRK (fully implicit) | Full | 1 block LU (s×n block) | Severe stiffness |

## Key Implicit Methods for Stiff Problems

### Gauss Methods
- Collocation at Gauss-Legendre points; order 2s; A-stable but not L-stable (R(∞) ≠ 0)
- Symplectic (preserve quadratic invariants)
- Not stiffly accurate; algebraically stable

### Radau IIA (RADAU5)
- Collocation at Radau quadrature points; order 2s-1 (s=3 → order 5)
- L-stable, stiffly accurate (R(∞) = 0), algebraically stable
- B-convergence of order 2s-1 on nonlinear stiff problems
- **RADAU5** (Hairer & Wanner Fortran code): the gold-standard solver for stiff ODE and index-1 DAE
- Implements simplified Newton on block stage equations with W-transformation to reduce to real arithmetic

### Lobatto Methods
- IIIA/IIIB pair: symplectic; used in SHAKE/RATTLE for constrained Hamiltonian systems
- IIIC: strongly S-stable; algebraically stable

### Rosenbrock Methods
- Semi-implicit: linearize ODE at each step, avoid iterative nonlinear solve
- One Jacobian per step; exact Jacobian → Rosenbrock; approximate → W-methods
- RODAS: 4th-order stiffly accurate; recommended for index-1 DAE with mild stiffness
- Order reduction occurs on very stiff problems (less severe than IRK)

## Butcher Tableau and Order Conditions

The order conditions are derived via rooted trees and elementary differentials. B-series theory provides a unified framework. Simplifying assumptions (C1, D1 conditions) reduce the number of conditions to check for collocation methods.

## Connection to Circuit Simulation

- SPICE's Trapezoidal Rule is equivalent to Lobatto IIIA (2-stage, order 2) — its marginal stability and ringing are explained by its non-L-stability
- Backward Euler in SPICE is the 1-stage Radau IIA (L-stable, order 1)
- RADAU5 would offer superior accuracy per step for circuit DAEs; not yet standard in EDA tools
- The simplified Newton iteration within RADAU5 is analogous to SPICE's per-timepoint NR

## Related concepts and entities

- [[stiff-ode-methods]] - broader category including Rosenbrock, extrapolation
- [[bdf-methods]] - multistep alternative
- [[differential-algebraic-equations]] - primary application of IRK methods
- [[integration-methods]] - SPICE-level integration methods (BE = Radau IIA order 1, TR = Lobatto IIIA)
- [[newton-raphson]] - nonlinear solver within IRK stage computation
