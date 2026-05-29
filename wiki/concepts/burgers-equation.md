---
title: Burgers' Equation
type: claim
id: concepts/burgers-equation
tags:
- pde
- ode
- benchmark
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

Burgers' equation (Burgers 1948) is the scalar nonlinear PDE u_t + u u_x = ν u_{xx}: an advection-diffusion equation with a quadratic nonlinearity. Viscous Burgers (ν > 0) develops thin shock-like fronts and is a 1D model of compressible Navier–Stokes; inviscid Burgers (ν = 0) develops genuine shocks in finite time.

## How It Works

Steepening fronts develop wherever ∂u/∂x is negative; the diffusion term smooths them into thin layers of width O(ν). Numerically, Burgers is a touchstone for shock-capturing schemes (TVD / ENO / WENO) and for [[concepts/moving-finite-elements]] (MFE adapts the mesh into the front). In Hairer–Wanner the [[concepts/quasilinear-dae]] formulation arises when MFE is applied: the mass matrix C(y) bundles node positions and solution values and becomes near-singular at the front. The Cole–Hopf transformation u = −2ν φ_x/φ converts viscous Burgers into the heat equation φ_t = ν φ_{xx}, giving exact analytical solutions for benchmark comparison.

## Key Parameters

- Viscosity ν > 0.
- Front width O(ν) for steady solutions.
- Time scale for shock formation O(1/max(−u_x)).

## When To Use

- Benchmark for nonlinear PDE numerics, especially shock-capturing or moving-mesh methods.
- Stress test for [[concepts/moving-finite-elements]] / [[concepts/quasilinear-dae]] solvers (LIMEX).
- Pedagogical example of regularised conservation laws.

## Risks & Pitfalls

- Inviscid limit (ν → 0) requires entropy-stable / TVD discretisations; naive central differences fail.
- Mesh adaptation near the front is essential; uniform-grid methods need O(1/ν) points to resolve the layer.

## Related Concepts

- [[concepts/moving-finite-elements]]
- [[concepts/quasilinear-dae]]
- [[concepts/method-of-lines]]
- [[concepts/brusselator]]
- [[entities/limex]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
