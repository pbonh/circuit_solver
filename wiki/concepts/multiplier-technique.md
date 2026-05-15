---
title: "Multiplier Technique"
type: concept
tags: [ode, numerical-integration, multistep, stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

The multiplier technique (Nevanlinna–Odeh 1981) is an analytical trick for proving convergence of [[concepts/a-alpha-stability|A(α)-stable]] linear multistep methods on nonlinear stiff problems. Instead of taking the inner product of the error recursion with Δy_m, one takes the inner product with ∑_j μ_{m−j} Δy_j for a rational *multiplier* μ(ζ); the modified scheme (ρ̃, σ̃) = (ρ τ, σ ς)/x is required to be A-stable.

## How It Works

The point of the multiplier is that A(α)-stability of the *original* method is not strong enough for direct G-stability arguments, but A-stability of a multiplier-modified scheme *is*. Nevanlinna–Odeh's key result: BDF formulas of orders k = 2..6 admit such η-multipliers, so global error bounds for one-sided Lipschitz problems satisfying a multiplier-modified contractivity condition (Hairer–Wanner Eq. V.8.22) can be derived for the full BDF family — extending the rigorous nonlinear convergence theory past the order-2 [[concepts/dahlquist-barrier]]. The technique combines with [[concepts/discrete-variation-of-constants]] and [[concepts/kreiss-matrix-theorem]] arguments to handle holomorphic-semigroup / sectorial-operator problems.

## Key Parameters

- Multiplier polynomial μ(ζ) of degree ≤ k.
- Modified pair (ρ̃, σ̃).
- Multiplier-modified one-sided Lipschitz hypothesis on f.

## When To Use

- Proving nonlinear convergence for BDF on stiff or parabolic problems.
- Theoretical convergence analysis where direct G-stability is unavailable.
- Method-design step: certify a new A(α)-stable LMS method by exhibiting a multiplier.

## Risks & Pitfalls

- The required multiplier-modified contractivity condition is restrictive — not every nonlinear stiff problem satisfies it.
- Existence of a valid μ is non-trivial; BDF7+ admits none, mirroring its zero-instability.

## Related Concepts

- [[concepts/g-stability]]
- [[concepts/gear-bdf]]
- [[concepts/a-alpha-stability]]
- [[concepts/discrete-variation-of-constants]]
- [[concepts/kreiss-matrix-theorem]]
- [[concepts/holomorphic-semigroup]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
