---
title: Threshold Factor
type: claim
id: concepts/threshold-factor
tags:
- ode
- numerical-integration
- stability
- nonlinear
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

The threshold factor R_0 (Spijker 1985; Bolley–Crouzeix; Kraaijevanger 1986) of a Runge–Kutta method in the ℓ^∞ or ℓ^1 norm is the largest r ≥ 0 such that the method is contractive whenever −h ν ≤ r, where ν is the [[concepts/one-sided-lipschitz-condition]] constant. Equivalently, it is the radius of contractivity in those maximum-type norms.

## How It Works

In the Euclidean norm, [[concepts/algebraic-stability]] gives contractivity for all h > 0 whenever ν ≤ 0 — so the "threshold" is infinite. In ‖·‖_∞ and ‖·‖_1, the situation is more restrictive: the method contracts only for sufficiently small h ν, and R_0 is finite. Kraaijevanger (1986) showed R_0 is determined by absolute monotonicity (Bernstein 1928) of the auxiliary function (1 − r z) R(z) at z = 0 — connecting [[concepts/absolutely-monotonic-function]] theory to nonlinear stability in maximum norms.

## Key Parameters

- Norm choice (‖·‖_∞ or ‖·‖_1).
- Stability function R(z) and its absolute-monotonicity radius.
- Method coefficients (A, b).

## When To Use

- Contractivity / monotonicity preservation in advection or PDE solvers using ‖·‖_∞ or ‖·‖_1.
- Choosing time-stepping methods for [[concepts/method-of-lines]] discretisations with maximum-norm monotonicity requirements (e.g., TVD / SSP schemes).
- Comparing methods that are equally B-stable but have very different contraction in non-Euclidean norms.

## Risks & Pitfalls

- R_0 = 0 for many methods that are perfectly B-stable in ‖·‖_2; ℓ^∞ contractivity is a strictly stronger requirement.
- The bound −h ν ≤ R_0 is sharp but conservative on most practical problems.

## Related Concepts

- [[concepts/contractivity]]
- [[concepts/b-stability]]
- [[concepts/absolutely-monotonic-function]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/algebraic-stability]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
