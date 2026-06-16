---
title: Stiff ODE Methods
type: concept
slug: stiff-ode-methods
created: 2026-06-16
updated: 2026-06-16
summary: Numerical methods designed for stiff ODEs where stability, not accuracy, bounds the timestep — primarily implicit Runge-Kutta (Radau), SDIRK, Rosenbrock, and BDF families.
tags: [numerical-methods, stiff-ode, runge-kutta, radau, rosenbrock, circuit-simulation]
sources: [solving-ode-ii-stiff-dae, computer-methods-circuit-analysis-design]
status: active
---

# Stiff ODE Methods

A stiff ODE is one where some components decay much faster than others, forcing small timesteps for stability with explicit methods even when accuracy would permit large steps. Stiffness arises naturally in circuit simulation (wide range of RC time constants), chemical kinetics, diffusion, and mechanical systems with constraints.

## Stability Concepts

| Property | Definition |
|---|---|
| A-stability | Stability region contains the entire left half-plane |
| L-stability | A-stable + R(∞) = 0 (damps infinitely stiff components) |
| A(α)-stability | Stable for arguments in a wedge of angle α around the negative real axis |
| B-stability | Bounded error growth for nonlinear monotone problems (one-sided Lipschitz) |
| Algebraic stability | Butcher matrix condition M = BA + A^T B - bb^T ≥ 0; implies B-stability |

**Second Dahlquist barrier**: No linear multistep method (BDF, Adams) can be A-stable beyond order 2. Implicit RK methods have no such barrier.

## Method Families

### Implicit Runge-Kutta (IRK)
- **Gauss methods**: Superconvergent (order 2s for s stages), A-stable but not L-stable; not stiffly accurate
- **Radau IIA**: L-stable, stiffly accurate (R(∞) = 0), algebraically stable, B-convergence of order 2s-1; RADAU5 is the production code for stiff ODE/index-1 DAE
- **Lobatto IIIA/B/C**: Symplectic pairs; Lobatto IIIC is strongly S-stable
- Implementation: simplified Newton on stage equations; W-transformation for block-diagonal linear systems

### SDIRK (Singly Diagonally Implicit RK)
- One LU factorization per step (diagonal elements equal); cheaper than full IRK
- Stiffly accurate SDIRK: R(∞) = 0; robust for index-1 DAE
- Good for moderate stiffness; less accurate than Radau per work unit on very stiff problems

### Rosenbrock / W-Methods
- Linearize ODE at beginning of each step; require one Jacobian evaluation per step
- No iterative nonlinear solve (one linear system solve per stage)
- RODAS: 4th-order stiffly-accurate Rosenbrock; recommended for non-stiff-to-moderate-stiff problems
- W-methods allow approximate/inexact Jacobian (good for large sparse systems)
- Order reduction on very stiff problems (less B-convergent than Radau)

### Extrapolation (SEULEX)
- Richardson extrapolation on linearly implicit midpoint rule
- Variable order; dense output; good for smooth problems
- Requires smoothing for stiff problems; less efficient than Radau on very stiff

## Choosing a Method

| Stiffness | Circuit analog | Recommended |
|---|---|---|
| Mild | Wide but not extreme RC spread | RODAS (Rosenbrock) |
| Moderate | Typical transistor circuits | SDIRK, BDF2-4 |
| Severe | Multi-scale circuits, index-1 DAE | RADAU5 |
| Very severe + DAE | Mechanical or coupled circuit-field | RADAU5 with mass matrix |

## Connection to Circuit Simulation

- [[integration-methods]] in SPICE: BE (BDF1), TR (marginally stable), G2 (BDF2)
- SPICE/Spectre use BDF-type methods; RADAU5 could offer better accuracy per step
- Modified Nodal Analysis (MNA) yields index-1 [[differential-algebraic-equations]]
- Simplified Newton within IRK stages corresponds to SPICE's inner NR iteration

## Related concepts and entities

- [[integration-methods]] - the subset used in SPICE (BE, TR, G2)
- [[differential-algebraic-equations]] - DAEs solved by these methods
- [[bdf-methods]] - multistep complement to one-step stiff methods
- [[newton-raphson]] - the nonlinear solver within IRK step evaluation
- [[circuit-simulation]] - primary application domain
