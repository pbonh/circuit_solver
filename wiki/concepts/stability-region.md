---
title: Stability Region
type: claim
id: claim-stability-region
tags:
- ode
- numerical-integration
- stability
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

The (linear) stability region of a numerical ODE method is the subset of the complex plane in which the dimensionless product z = hλ produces a non-growing numerical solution of the [[concepts/dahlquist-test-equation]]. For a one-step method, S = {z ∈ ℂ : |R(z)| ≤ 1}; for a [[concepts/linear-multistep-methods]] method with first and second characteristic polynomials ρ, σ, S = {μ ∈ ℂ : all roots ζ of ρ(ζ) − μσ(ζ) = 0 satisfy |ζ| ≤ 1, with simple roots on the boundary}.

## How It Works

S is bounded by the [[concepts/root-locus-curve]] — for one-step methods the level set |R(z)| = 1; for multistep methods the image of {|ζ| = 1} under μ = ρ(ζ)/σ(ζ). Time-step selection in stiff codes pins hλ_max inside S for all Jacobian eigenvalues. Different stability adjectives label different containments: [[concepts/a-stability]] = S ⊇ ℂ^−; [[concepts/a-alpha-stability]] = S ⊇ sector S_α; [[concepts/ao-stability]] = S ⊇ (−∞, 0); [[concepts/l-stability]] adds R(∞) = 0.

## Key Parameters

- Boundary curve (level set or root locus).
- Imaginary-axis interval (governs purely oscillatory stability).
- Negative-real-axis interval (governs damped real spectra).
- Behaviour of R(z) as |z| → ∞.

## When To Use

- Step-size selection for explicit methods: h ≤ min hλ-distance to ∂S over all Jacobian eigenvalues.
- Classifying methods (A, A(α), A0, L, stiff-stable).
- Comparing stability properties via [[concepts/order-star]] or [[concepts/property-c]].

## Risks & Pitfalls

- Stability region characterises only linear scalar behaviour; nonlinear problems need [[concepts/b-stability]] / [[concepts/algebraic-stability]].
- For nonsymmetric Jacobians the spectrum alone is not enough — pseudo-spectra and norm bounds (see [[concepts/kreiss-matrix-theorem]], [[concepts/von-neumann-theorem]]) matter.
- A method may have a large S but still suffer [[concepts/order-reduction]] on stiff problems.

## Related Concepts

- [[concepts/stability-function]]
- [[concepts/stability-domain]]
- [[concepts/root-locus-curve]]
- [[concepts/a-stability]]
- [[concepts/l-stability]]
- [[concepts/a-alpha-stability]]
- [[concepts/dahlquist-test-equation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
