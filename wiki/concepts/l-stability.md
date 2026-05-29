---
title: L-Stability
type: claim
id: claim-l-stability
tags:
- ode
- numerical-integration
- stiff
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

L-stability (Ehle, 1969) strengthens [[concepts/a-stability]] by additionally requiring R(∞) = 0, where R(z) is the method's [[concepts/stability-function]]. An L-stable method damps the most stiff modes (|hλ| → ∞) to zero in a single step, instead of leaving them at any nonzero |R(∞)| ≤ 1 a merely A-stable method would tolerate.

## How It Works

For the [[concepts/dahlquist-test-equation]] y' = λy, the propagator is R(hλ). A-stability guarantees |R(z)| ≤ 1 on the left half-plane but allows |R(−∞)| up to 1; the trapezoidal rule gives R(∞) = −1 and so leaves stiff transients oscillating with no decay. L-stable methods (backward Euler R(z) = 1/(1−z), SDIRK methods with appropriate γ, Radau IIA, RODAS) make R(∞) = 0 by construction — either via stiffly-accurate rows (a_{si} = b_i) or by stationing zeros of the numerator of the Padé rational approximant. For implicit RK methods, [[concepts/stiffly-accurate-method]] automatically implies R(∞) = 0 and L-stability whenever A-stability holds.

## Key Parameters

- Stability function R(z) = 1 + zb^T (I − zA)^{−1} 𝟙 evaluated at z = ∞.
- Whether b^T = e_s^T A (the stiffly accurate condition).
- Order of contact with e^z at the origin (for accuracy) versus order of contact with 0 at infinity (for damping).

## When To Use

- Very stiff problems with transients that must be damped after a single step (chemical kinetics with disparate rates, [[concepts/method-of-lines]] for parabolic PDE).
- Index-1 [[concepts/differential-algebraic-equation]] and [[concepts/singular-perturbation-problem]] solvers: stiffly-accurate (hence L-stable) IRK methods are essential because the algebraic component is set by R(∞).

## Risks & Pitfalls

- L-stability is not enough on its own — pair with [[concepts/b-stability]] / [[concepts/algebraic-stability]] for nonlinear problems.
- Over-damping fast oscillations may smear physically meaningful high-frequency content; trapezoidal-like methods are better when accuracy on lightly damped modes matters.
- For Lobatto IIIA/IIIB, A-stability holds but R(∞) ≠ 0, so they are *not* L-stable.

## Related Concepts

- [[concepts/a-stability]]
- [[concepts/stability-function]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/radau-iia-method]]
- [[concepts/sdirk-method]]
- [[concepts/rosenbrock-method]]
- [[concepts/backward-euler]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
