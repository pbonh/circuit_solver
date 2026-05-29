---
title: Dense Output
type: claim
id: claim-dense-output
tags:
- ode
- numerical-integration
- interpolation
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

Dense output is a piecewise-polynomial approximation u(x) of the ODE solution on the entire integration interval, valid between accepted step points and not just at them. The polynomial on each step [x_n, x_n + h] is constructed from the same internal information used to compute y_{n+1} (stage values and slopes), and its order p* is typically at or one below the method's classical order.

## How It Works

For [[concepts/collocation-method]]s the dense output is free: the collocation polynomial u(x) of degree s is the natural interpolant, with order s ≤ p*. For non-collocation methods (DOPRI5, RADAU5), extra evaluation effort or extra coefficients are needed: Shampine (1985) and Dormand–Prince (1986) construct continuous fifth-order extensions to DOPRI5 with a single extra function evaluation; Hairer–Wanner provide an order-3 continuous embedding for SDIRK4 that doubles as the embedded error estimator. For DAEs and [[concepts/singular-perturbation-problem]]s, Hairer–Ostermann's (1990) right-end Hermite interpolation construction avoids amplifying boundary-layer perturbations.

## Key Parameters

- Order p* of the dense-output polynomial.
- Extra coefficients / function evaluations per step.
- Continuity class (C^0, C^1) across step boundaries.

## When To Use

- Event-location / root-finding in ODE solvers.
- Output at user-prescribed times that do not match the integrator's adaptive step points.
- Visualisation and plotting where smooth curves are wanted.
- Pseudo-spectral / Galerkin coupling where solution values at quadrature points are needed.

## Risks & Pitfalls

- Dense-output order may be lower than the discrete order; expect O(h^{p*}) interpolation error, not O(h^p).
- For stiff and DAE problems, naive interpolation across boundary layers can amplify perturbations — use right-end Hermite construction (Hairer–Ostermann).
- Discontinuities in event functions must be handled by step-size restriction *and* dense-output evaluation.

## Related Concepts

- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/order-reduction]]
- [[concepts/boundary-layer]]
- [[concepts/extrapolation-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
