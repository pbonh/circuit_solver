---
title: "Contractivity"
type: concept
tags: [ode, numerical-integration, stability, nonlinear, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A flow is contractive (in a chosen norm) if any two solutions y(·) and z(·) of the same ODE satisfy ‖y(x) − z(x)‖ ≤ ‖y(x_0) − z(x_0)‖ for all x ≥ x_0. A numerical method is contractive if its iterates obey ‖y_{n+1} − z_{n+1}‖ ≤ ‖y_n − z_n‖ for every step size h > 0 whenever the underlying continuous flow is contractive — i.e., whenever the right-hand side satisfies the [[concepts/one-sided-lipschitz-condition]] with constant ν ≤ 0.

## How It Works

Continuous contractivity in an inner product follows from (f(x, y) − f(x, z), y − z) ≤ ν‖y − z‖^2 with ν ≤ 0 (Dahlquist 1975). Discrete contractivity for Runge–Kutta methods follows from [[concepts/algebraic-stability]]; for one-leg multistep methods from [[concepts/g-stability]]. In maximum and ℓ^1 norms, contractivity is governed by the [[concepts/threshold-factor]] of Spijker (1985) / Bolley–Crouzeix and the [[concepts/absolutely-monotonic-function]] characterisation due to Kraaijevanger (1986). The numerical [[concepts/error-growth-function]] φ_B(x) gives the sharp contraction rate.

## Key Parameters

- Norm choice (inner-product, maximum, ℓ^1).
- One-sided Lipschitz constant ν.
- Threshold factor of the method in the chosen norm.

## When To Use

- Long-time stability proofs for nonlinear stiff problems.
- Justifying step-size-independent error bounds on dissipative systems.
- Monotonicity-preservation in advection or hyperbolic SSP / TVD time stepping.

## Risks & Pitfalls

- Contractivity is norm-dependent; methods contractive in ‖·‖_2 may not be contractive in ‖·‖_∞.
- A method only [[concepts/a-stability]] is not necessarily contractive on nonlinear problems.
- Strong contractivity (with rate exp(νh) for ν < 0) requires stronger assumptions than mere non-expansion.

## Related Concepts

- [[concepts/b-stability]]
- [[concepts/algebraic-stability]]
- [[concepts/g-stability]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/threshold-factor]]
- [[concepts/absolutely-monotonic-function]]
- [[concepts/error-growth-function]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
