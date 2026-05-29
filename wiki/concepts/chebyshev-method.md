---
title: Chebyshev Method
type: claim
id: concepts/chebyshev-method
tags:
- ode
- numerical-integration
- runge-kutta
- stiff
- parabolic
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

A Chebyshev method (or stabilised explicit Runge–Kutta) is an explicit s-stage RK method whose [[concepts/stability-function]] R(z) is built from a damped, shifted Chebyshev polynomial so that the [[concepts/stability-region]] stretches far along the negative real axis. The interval [−ℓ_s, 0] inside the stability region scales as ℓ_s ≈ 2(s^2 − 1)/3 or larger, much better than the O(s) length of a standard explicit RK method.

## How It Works

Setting R(z) = T_s(1 + z/s^2) (shifted Chebyshev) gives a stability interval of length 2 s^2 but the boundary is fragile: tiny imaginary perturbations leave the region. Damping by a small factor η ∈ (0, 1) — R(z) = (1 − η + η T_s(1 + z(2 − η)/s^2))/T_s(1 − η) — yields a narrow strip around the negative real axis that *is* robust, at the cost of ≈ 20% reduction in the real-axis stability length. Specific Chebyshev codes: Lebedev's DUMKA (damped Chebyshev order 2 and 3), van der Houwen–Sommeijer's RKC (order 2, second-derivative version), Abdulle–Medovikov's ROCK4 (order 4 stabilised RK). The number of stages s is chosen at runtime based on the current spectral radius estimate of h J.

## Key Parameters

- Number of stages s (varies per step).
- Damping factor η (≈ 0.05 typical).
- Stability-interval length ℓ_s ≈ 2 (s^2 − 1)/3.
- Order p (typically 2, 3, or 4).

## When To Use

- Large-dimension mildly stiff problems with eigenvalues clustered near the negative real axis.
- [[concepts/method-of-lines]] for parabolic PDE where one wants to avoid LU factorisation.
- Diffusion-dominated reaction–diffusion systems (e.g. [[concepts/brusselator]]) at moderate stiffness.

## Risks & Pitfalls

- Imaginary-axis eigenvalues (e.g. from advection or hyperbolic terms) escape the damped strip; Chebyshev methods are not suitable for highly oscillatory problems.
- Very high s (s > 50) accumulates round-off; recursion-formula evaluations matter.
- Order is limited (rarely exceeds 4 in practical Chebyshev families).

## Related Concepts

- [[concepts/explicit-runge-kutta]]
- [[concepts/stability-region]]
- [[concepts/method-of-lines]]
- [[concepts/stability-function]]
- [[entities/dumka]]
- [[entities/rkc]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
