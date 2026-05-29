---
title: Method of Lines
type: claim
id: claim-method-of-lines
tags:
- pde
- ode
- numerical-integration
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

The method of lines (MOL) discretises a PDE in space (by finite differences, finite elements, finite volumes, or spectral methods) while leaving time continuous, producing a large system of ODEs (or DAEs, if boundary conditions are algebraic) to be integrated by a time-stepping scheme. For a parabolic PDE u_t = L u with L an elliptic spatial operator, the semi-discrete system u̇ = L_h u inherits a Jacobian whose eigenvalues span O(h_x^{−2}) — making the ODE stiff.

## How It Works

After space discretisation the system is integrated by a time-marching method appropriate to its stiffness: stiff [[concepts/implicit-runge-kutta]] (RADAU5), [[concepts/gear-bdf|BDF]], [[concepts/rosenbrock-method]] (RODAS), or stabilised explicit [[concepts/chebyshev-method]] (RKC, ROCK). Convergence theorems (Lubich's Theorem 7.10) cover the parabolic case via [[concepts/holomorphic-semigroup]] arguments, and order-reduction shows up as the [[concepts/order-reduction]] of the time integrator at boundary-layer transitions. MOL connects PDE numerics to the entire stiff ODE machinery — circuit simulation tools that handle distributed elements (transmission lines, lossy interconnects) do MOL internally.

## Key Parameters

- Space discretisation parameter h_x.
- Time step h_t.
- Spatial-Jacobian conditioning / spectrum.

## When To Use

- Parabolic PDE (heat, diffusion–reaction, Navier–Stokes incompressible).
- Hyperbolic PDE with appropriate explicit time integrators.
- Coupled multi-physics where the spatial operator can be discretised separately.
- Lossy / dispersive interconnect modelling in circuit simulation.

## Risks & Pitfalls

- Stiffness scales as h_x^{−2} for parabolic problems; implicit time integration is essential.
- Spatial discretisation introduces numerical diffusion / dispersion; verify with grid refinement.
- Order reduction can dominate when boundary conditions are time-dependent or singular.

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/gear-bdf]]
- [[concepts/chebyshev-method]]
- [[concepts/holomorphic-semigroup]]
- [[concepts/order-reduction]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/brusselator]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
