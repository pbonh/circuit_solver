---
title: B-Stability
type: claim
id: concepts/b-stability
tags:
- ode
- numerical-integration
- stiff
- stability
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

Butcher's (1975) B-stability is the nonlinear analogue of [[concepts/a-stability]]. A Runge–Kutta method is B-stable if, whenever the right-hand side satisfies the [[concepts/one-sided-lipschitz-condition]] (f(x, y) − f(x, z), y − z) ≤ 0 in an inner-product norm, the numerical iterates obey ‖y_{n+1} − z_{n+1}‖ ≤ ‖y_n − z_n‖ for every step size h > 0 and every pair of starting values.

## How It Works

The one-sided Lipschitz bound is the contraction property of the *continuous* flow (Dahlquist 1975); a B-stable method preserves this contraction at the discrete level. Butcher showed that B-stability follows from [[concepts/algebraic-stability]]: b_i ≥ 0 plus the matrix M = BA + A^T B − bb^T non-negative definite. Hundsdorfer–Spijker (1981) proved the converse for S-irreducible methods via Kirszbraun's Lipschitz-extension theorem and Schoenberg's geometric argument. Gauss, Radau IA, Radau IIA, and Lobatto IIIC methods are algebraically stable and therefore B-stable; Lobatto IIIA and IIIB are not. The [[concepts/error-growth-function]] φ_B(x) measures the precise contraction rate.

## Key Parameters

- One-sided Lipschitz constant ν of f.
- Inner-product / norm choice on the state space.
- Algebraic-stability test matrix M = BA + A^T B − bb^T.
- Weights b_i ≥ 0.

## When To Use

- Nonlinear stiff problems (chemical kinetics, dissipative reaction–diffusion systems).
- Long-time integration where contractivity must hold globally.
- Theoretical analysis to establish [[concepts/b-convergence]] (stiffness-independent error bounds).

## Risks & Pitfalls

- Requires a *one-sided* Lipschitz bound; ordinary Lipschitz constants are too pessimistic on stiff problems.
- Algebraic stability is not enough for [[concepts/lobatto-iiia-method]] and [[concepts/lobatto-iiib-method]]; they are A-stable but not B-stable.
- B-stability does not by itself imply small constants in the global error — pair with B-convergence and stage-order analysis.

## Related Concepts

- [[concepts/algebraic-stability]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/contractivity]]
- [[concepts/error-growth-function]]
- [[concepts/a-stability]]
- [[concepts/b-convergence]]
- [[concepts/an-stability]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
