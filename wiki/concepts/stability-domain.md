---
title: "Stability Domain"
type: concept
tags: [ode, numerical-integration, stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The stability domain of an integration method is the closed set in the complex plane on which the linear-stability criterion holds. Used interchangeably with [[concepts/stability-region]] in Hairer–Wanner; the convention "domain" emphasises the geometric set, "region" emphasises the boundary curve.

## How It Works

For a one-step method, S = {z ∈ ℂ : |R(z)| ≤ 1} where R is the [[concepts/stability-function]]. The boundary, given by |R(z)| = 1, is the level set of a rational function; for [[concepts/explicit-runge-kutta]] it is a closed polynomial level curve, for [[concepts/implicit-runge-kutta]] and Rosenbrock methods it can be unbounded. Stiff codes pick h so the product hλ for every Jacobian eigenvalue λ lies inside S.

## Key Parameters

- Boundary topology (bounded vs. unbounded).
- Imaginary-axis interval (longest [-iy, iy] ⊂ S).
- Real-axis interval (longest [-x, 0] ⊂ S).
- Sector half-angle α for which S ⊇ S_α.

## When To Use

- Step-size selection (mostly for explicit methods on mildly stiff problems).
- Visual classification of methods via stability-region plots.
- [[concepts/chebyshev-method]] design: stretch S along the negative real axis to integrate parabolic discretisations explicitly.

## Risks & Pitfalls

- A large S is no guarantee against [[concepts/order-reduction]] on stiff problems.
- Linear-stability domain analysis ignores nonlinear contractivity; pair with [[concepts/b-stability]].
- For variable coefficient or non-normal Jacobians, spectrum-based arguments can mislead — use [[concepts/kreiss-matrix-theorem]].

## Related Concepts

- [[concepts/stability-region]]
- [[concepts/stability-function]]
- [[concepts/dahlquist-test-equation]]
- [[concepts/chebyshev-method]]
- [[concepts/a-stability]]
- [[concepts/order-star]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
