---
title: Van der Pol Equation
type: claim
id: concepts/van-der-pol-equation
tags:
- ode
- stiff
- singular-perturbation
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

The van der Pol oscillator (Van der Pol 1922) is the second-order ODE y'' − μ (1 − y^2) y' + y = 0, modelling self-sustained oscillations in a triode amplifier (and many other nonlinear oscillators). The parameter μ > 0 controls nonlinearity: μ ≪ 1 gives nearly sinusoidal oscillations, μ ≫ 1 gives stiff relaxation oscillations with fast switching between slow branches.

## How It Works

For large μ the equation has a slow manifold (y' = 0 / (1 − y^2) ≈ 0) where motion is slow, joined by fast jumps when the manifold loses stability. The Liénard transformation y, z = y/μ + (y^3/3 − y)/μ converts the equation to the [[concepts/singular-perturbation-problem]] form y' = μ z − y, ε z' = −y where ε = 1/μ^2 (Dorodnicyn 1947). The reduced system (ε = 0) gives a discontinuous "relaxation" cycle; for ε > 0 the smooth limit cycle has [[concepts/boundary-layer]]s at the fast jumps. Hairer–Wanner use it as the canonical SPP benchmark — stiff at large μ, mildly stiff at moderate μ, nonstiff at small μ.

## Key Parameters

- Nonlinearity parameter μ.
- Stiffness ratio ε = 1/μ^2 in the Liénard form.
- Period of the limit cycle ≈ (3 − 2 ln 2) μ for large μ.

## When To Use

- Benchmark for stiff ODE / SPP integrators (RADAU5, DASSL, DOPRI5 comparisons).
- Pedagogical example of relaxation oscillations.
- Test case for [[concepts/automatic-stiffness-detection]] (transitions between stiff and nonstiff regimes within one period).

## Risks & Pitfalls

- Fast jumps require small time steps in the layer; adaptive control essential.
- For very large μ (≈ 10^6) only stiffly accurate IRK / Rosenbrock methods are practical.
- DOPRI5 fails dramatically; the result is a standard "explicit-method-on-stiffness" cautionary tale.

## Related Concepts

- [[concepts/singular-perturbation-problem]]
- [[concepts/boundary-layer]]
- [[concepts/asymptotic-expansion]]
- [[concepts/stiff-circuit]]
- [[concepts/automatic-stiffness-detection]]
- [[concepts/brusselator]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
