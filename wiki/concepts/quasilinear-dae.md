---
title: Quasilinear DAE
type: claim
id: concepts/quasilinear-dae
tags:
- dae
- modeling
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

A quasilinear DAE is a system C(y) y' = f(y) in which the mass matrix C(y) depends on the state but has *constant rank* m < n (full dimension). When m = n the system is an ordinary ODE; when m < n it is a DAE whose effective rank-deficiency depends nonlinearly on the state.

## How It Works

The constant-rank assumption gives a smooth left annihilator T_2(y) of C(y); the consistency condition T_2(y) f(y) = 0 must hold along solutions. Lemma 6.1 (Hairer–Wanner) gives a local existence / uniqueness theorem under the additional condition that (B' / (T_2 f)') is invertible (with B' = [C, T_2 f]). Lemma 6.2 shows the perturbed pencil C(y) + λ(f'(y_0) − f̄(y_0, y_0')) is invertible for small λ ≠ 0 — the key technical step for applying [[concepts/rosenbrock-method]] or [[concepts/extrapolation-method]] formulas. Examples: the moving-finite-element method (K. Miller–R.N. Miller 1981) on [[concepts/burgers-equation]] (C singular near inflection points); reduced-order models of multibody systems.

## Key Parameters

- Mass matrix C(y) and its rank.
- Left annihilator T_2(y).
- Consistency manifold {y : T_2(y) f(y) = 0}.
- Smoothness of C as a function of y.

## When To Use

- Moving-mesh / adaptive-mesh PDE methods.
- Reduced-order modelling of constrained systems.
- Applications where the natural physical formulation has a state-dependent mass.

## Risks & Pitfalls

- The constant-rank assumption can fail at isolated points; behaviour near a rank drop is singular and may require regularisation.
- The semi-explicit transformation y' = z, 0 = C(y) z − f(y) gives an index-1 DAE that can be passed to standard solvers; for direct quasilinear treatment, use LIMEX-style linearly-implicit Euler with (I − h C(y_0)^{−1} f'(y_0)) k_1 = f(y_0) approximations.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-1-dae]]
- [[concepts/linearly-implicit-euler]]
- [[concepts/moving-finite-elements]]
- [[concepts/burgers-equation]]
- [[entities/limex]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
