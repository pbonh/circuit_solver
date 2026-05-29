---
title: Brusselator
type: claim
id: concepts/brusselator
tags:
- ode
- pde
- stiff
- benchmark
- reaction-diffusion
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

The Brusselator (Prigogine–Lefever 1968) is a two-species autocatalytic chemical-reaction model exhibiting limit-cycle oscillations and (with diffusion) Turing patterns. The ODE form is u̇ = A + u^2 v − (B + 1) u, v̇ = B u − u^2 v. The PDE form adds diffusion: u_t = D_1 ∂_xx u + (kinetics), v_t = D_2 ∂_xx v + (kinetics).

## How It Works

For 1 < B < 1 + A^2 the ODE has a single unstable fixed point and a stable limit cycle. The PDE on a finite interval with appropriate D_1, D_2 develops Turing instabilities — spatial patterns from a uniform initial state. Discretising the PDE in space by finite differences gives a stiff ODE system whose stiffness scales as (Δx)^{−2}; this is the canonical [[concepts/method-of-lines]] benchmark in Hairer–Wanner. The 1D Brusselator on N spatial points with second-derivative finite differences is *the* standard test problem for stabilised explicit ([[concepts/chebyshev-method]], RKC, ROCK4) and implicit (RADAU5, RODAS, BDF) stiff codes.

## Key Parameters

- Rate constants A, B.
- Diffusion coefficients D_1, D_2.
- Spatial grid size Δx (sets stiffness for the MOL system).
- System size N after discretisation.

## When To Use

- Stiff-method benchmarking with controllable stiffness ratio.
- Pedagogical example of pattern-forming reaction–diffusion.
- Testing implicit / Rosenbrock codes against stabilised explicit codes on parabolic problems.

## Risks & Pitfalls

- Stiffness grows quadratically with N — code timing comparisons must scale carefully.
- The PDE solution becomes spatially complex; verify with grid refinement.

## Related Concepts

- [[concepts/method-of-lines]]
- [[concepts/chebyshev-method]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/stiff-circuit]]
- [[concepts/burgers-equation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
