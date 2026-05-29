---
title: One-Leg Method
type: claim
id: concepts/one-leg-method
tags:
- ode
- numerical-integration
- multistep
- stiff
- nonlinear
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

A one-leg method is Dahlquist's (1975) nonlinear-stable reformulation of a [[concepts/linear-multistep-methods|linear multistep]] method. Given an LMS pair (ρ, σ), the one-leg companion is ∑_i α_i y_{n+i} = h f(x̄, ∑_i β_i y_{n+i}), where the right-hand side is evaluated at the *single* "leg" point x̄ = ∑ β_i x_{n+i} with the *single* state ∑ β_i y_{n+i} instead of a linear combination of f-values.

## How It Works

The one-leg form evaluates f only once per step (at x̄), unlike the underlying LMS which evaluates f at every history point. Dahlquist's equivalence theorem: a one-leg method is contractive in a quadratic Lyapunov norm ‖·‖_G (with G symmetric positive definite) on any nonlinear problem satisfying the [[concepts/one-sided-lipschitz-condition]] iff the underlying LMS pair (ρ, σ) is A-stable. This is [[concepts/g-stability]] — the nonlinear analogue of A-stability for multistep methods. Riesz–Herglotz provides the equivalence proof. One-leg methods are the canonical theoretical vehicle for nonlinear-multistep convergence proofs; in practice the underlying LMS form is preferred for variable-step implementation, but the one-leg form is what makes nonlinear theory work.

## Key Parameters

- LMS pair (ρ, σ).
- Leg coefficients β_i (often β_i = σ_i).
- G-matrix (symmetric positive definite Lyapunov weight).

## When To Use

- Nonlinear-stability proofs for multistep methods on stiff problems.
- Theoretical underpinning of G-stability and the [[concepts/multiplier-technique]].
- Method-design constraint: a method is desirable iff its one-leg companion is G-stable.

## Risks & Pitfalls

- The one-leg form has its own variable-step subtleties; equivalence with the underlying LMS holds only at constant step.
- The Lyapunov matrix G is method-dependent and not always easy to compute.

## Related Concepts

- [[concepts/linear-multistep-methods]]
- [[concepts/g-stability]]
- [[concepts/a-stability]]
- [[concepts/multiplier-technique]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/contractivity]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
