---
title: Absolutely Monotonic Function
type: claim
id: concepts/absolutely-monotonic-function
tags:
- ode
- numerical-integration
- mathematical-tool
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

A function φ defined on an interval I ⊂ ℝ is absolutely monotonic if every derivative φ^{(k)}(x) ≥ 0 for x ∈ I and k ≥ 0 (Bernstein 1928). The radius of absolute monotonicity of a function at a point a is the supremum of r > 0 such that φ is absolutely monotonic on [a − r, a].

## How It Works

Bernstein's characterisation links non-negative Taylor coefficients about a base point to a global non-negativity / contractivity property. Kraaijevanger (1986) used it to show that the contraction radius of a Runge–Kutta method's [[concepts/stability-function]] R in the ℓ^∞ or ℓ^1 norm equals the radius of absolute monotonicity of (1 − r z) R(z) at z = 0. This is the key step in computing the method's [[concepts/threshold-factor]] R_0 and underlies SSP (strong-stability-preserving) coefficient theory of Shu–Osher (1988) and later authors.

## Key Parameters

- Base point a (typically 0 or a method-specific normalisation).
- Radius of absolute monotonicity r.
- Non-negativity of Taylor coefficients in the expansion about a.

## When To Use

- Computing SSP / threshold-factor coefficients of explicit and implicit RK methods.
- Stability theory in maximum / sum norms for stiff or hyperbolic PDE time stepping.
- Construction of optimal-radius methods (e.g. SSPRK3, SDIRK families).

## Risks & Pitfalls

- The base point matters; absolute monotonicity at 0 does not imply it at other shifts.
- The condition is restrictive — many methods with excellent ‖·‖_2 properties have zero radius in ‖·‖_∞.

## Related Concepts

- [[concepts/threshold-factor]]
- [[concepts/contractivity]]
- [[concepts/stability-function]]
- [[concepts/b-stability]]
- [[concepts/runge-kutta-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
