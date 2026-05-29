---
title: G-Stability
type: claim
id: claim-g-stability
tags:
- ode
- numerical-integration
- multistep
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
---

## Definition

G-stability (Dahlquist 1975/76) is the nonlinear-multistep analogue of [[concepts/a-stability]]. A [[concepts/one-leg-method]] is G-stable if, on every problem satisfying the [[concepts/one-sided-lipschitz-condition]] in an inner-product norm, the discrete iterates contract in a quadratic Lyapunov norm ‖·‖_G with G a symmetric positive-definite matrix on the state history (Y_n, Y_{n−1}, …, Y_{n−k+1}).

## How It Works

Dahlquist's equivalence theorem: a one-leg method is G-stable iff its underlying LMS pair (ρ, σ) is A-stable. The proof uses the Riesz–Herglotz characterisation of Re(ρ(ζ)/σ(ζ)) > 0 for |ζ| > 1, recasting it as a quadratic Lyapunov inequality for the state history. G-stability is the right discrete analogue of continuous contractivity for multistep methods, just as [[concepts/b-stability]] is for one-step methods. The Lyapunov matrix G can be computed explicitly via the Riesz–Herglotz construction and gives the sharp contraction rate.

## Key Parameters

- LMS pair (ρ, σ).
- Lyapunov matrix G (k × k, symmetric positive definite).
- One-sided Lipschitz constant ν of the right-hand side.

## When To Use

- Nonlinear-stability proofs for stiff multistep methods.
- Long-time integration where contractive behaviour matters.
- Theoretical bridge between A-stability (linear) and B-stability (RK nonlinear).

## Risks & Pitfalls

- Strictly requires the one-leg form; variable-step implementations of the underlying LMS may not preserve the G-contractivity.
- The G-matrix is method-specific and not a uniform constant across LMS families.
- G-stability does not imply [[concepts/b-convergence]] without additional stage-order analysis (multistep stage order is not the same notion as for RK).

## Related Concepts

- [[concepts/one-leg-method]]
- [[concepts/a-stability]]
- [[concepts/b-stability]]
- [[concepts/linear-multistep-methods]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/contractivity]]
- [[concepts/multiplier-technique]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
