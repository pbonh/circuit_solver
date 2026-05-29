---
title: Epsilon Embedding Method
type: claim
id: concepts/epsilon-embedding-method
tags:
- ode
- dae
- singular-perturbation
- runge-kutta
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

The ε-embedding method applies an implicit Runge–Kutta or multistep formula to the full [[concepts/singular-perturbation-problem]] y' = f(y, z), ε z' = g(y, z) and then sets ε = 0, producing a method directly applicable to the limiting index-1 [[concepts/differential-algebraic-equation]]. Originally developed in Hairer–Lubich–Roche (1988); Griepentrog–März call the resulting RK scheme IRK(DAE).

## How It Works

For an IRK method with invertible coefficient matrix A, setting ε = 0 in the discretised stage equations gives a system that can be reduced via the substitution w_{ij} = (A^{−1})_{ij}: the algebraic part collapses to g(Y_i, Z_i) = 0 at each stage, while the differential part is the usual IRK step on y. The output approximation y_{n+1} = y_n + h ∑ b_i f(Y_i, Z_i) and z_{n+1} = ∑ d_i Z_i (or Z_s for [[concepts/stiffly-accurate-method]]s) inherits the order of the underlying IRK on the smooth components. For stiffly accurate methods (a_{si} = b_i) the ε-embedding approach and the direct [[concepts/state-space-form]] method coincide.

## Key Parameters

- Underlying IRK or LMS method.
- Invertibility of A (the coefficient matrix).
- Stiffly accurate flag.

## When To Use

- Solving index-1 DAEs by reusing existing IRK code (RADAU5, RODAS) with ε set to zero.
- Mass-matrix DAEs M u' = φ(u) with constant (possibly singular) M.
- Singular-perturbation problems where one wants the same solver to handle ε = 0 and ε > 0 transparently.

## Risks & Pitfalls

- The z-component achieves only stage-order convergence for non-stiffly-accurate methods.
- For methods with |R(∞)| > 1, the algebraic z-component diverges; check L-stability first.
- Inconsistent initial conditions on z (z_0 ≠ G(y_0)) produce a delta-function-like initial transient that the method must absorb.

## Related Concepts

- [[concepts/singular-perturbation-problem]]
- [[concepts/state-space-form]]
- [[concepts/index-1-dae]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/order-reduction]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
