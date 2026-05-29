---
title: One-Sided Lipschitz Condition
type: claim
id: claim-one-sided-lipschitz-condition
tags:
- ode
- numerical-integration
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

A right-hand side f(x, y) satisfies a one-sided Lipschitz condition (in an inner-product norm) with constant ν ∈ ℝ on a domain D if (f(x, y) − f(x, z), y − z) ≤ ν ‖y − z‖^2 for all (x, y), (x, z) ∈ D. Unlike the ordinary Lipschitz constant L (which controls the full operator norm of f_y), ν can be very negative for stiff problems whose Jacobian is strongly dissipative, even when L is huge.

## How It Works

The classical Lipschitz constant L bounds ‖f(x, y) − f(x, z)‖ ≤ L ‖y − z‖ and grows with the largest eigenvalue magnitude of f_y. The one-sided version uses the *real part* of the inner product and equals the [[concepts/logarithmic-norm]] μ(f_y) for linear systems, so ν ≤ μ(f_y) ≤ Re(spectrum(f_y)). For a stiff problem with eigenvalues −10^6 ± i, L ≈ 10^6 but ν ≈ −10^6, giving contraction at rate exp(νh). This is the right notion for proving [[concepts/contractivity]], [[concepts/b-stability]], [[concepts/b-convergence]], and any stiffness-independent error bound.

## Key Parameters

- Inner product (often weighted to match a Lyapunov function).
- Constant ν (can be negative, zero, or positive).
- Domain D where the bound holds.

## When To Use

- Nonlinear stability proofs on stiff ODEs.
- Establishing the hypothesis for B-stability / algebraic-stability / G-stability theorems.
- [[concepts/coercivity-coefficient]] estimates for the existence of IRK solutions: ν combines with α_0(A^{−1}) into the bound h ν < α_0(A^{−1}).

## Risks & Pitfalls

- Inner-product choice matters: the constant changes with the norm; a strict bound in ‖·‖_2 may fail in ‖·‖_∞.
- ν is not the spectral abscissa — for non-normal Jacobians ν can be much larger than max Re λ.
- Establishing ν is application-specific (often via physical-energy arguments).

## Related Concepts

- [[concepts/logarithmic-norm]]
- [[concepts/b-stability]]
- [[concepts/g-stability]]
- [[concepts/contractivity]]
- [[concepts/coercivity-coefficient]]
- [[concepts/algebraic-stability]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
