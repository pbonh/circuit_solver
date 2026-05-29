---
title: Collocation Method
type: claim
id: claim-collocation-method
tags:
- ode
- numerical-integration
- runge-kutta
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

A collocation method (Wright 1970) for ODEs seeks a polynomial u(x) of degree s on the step interval [x_n, x_{n+1}] satisfying u(x_n) = y_n and u'(x_n + c_i h) = f(x_n + c_i h, u(x_n + c_i h)) at s collocation nodes c_1 < … < c_s ∈ [0, 1]; the step output is y_{n+1} = u(x_{n+1}). Every collocation method is equivalent to a Runge–Kutta method whose A, b, c are determined by Lagrange interpolation on the nodes.

## How It Works

Choice of nodes determines the method family: Gauss–Legendre roots give the Gauss method (order 2s, A-stable); right-shifted Radau nodes give [[concepts/radau-iia-method]] (order 2s − 1, L-stable); Lobatto nodes (including 0 and 1) give [[concepts/lobatto-iiia-method]]. By construction, collocation methods automatically satisfy the [[concepts/butcher-simplifying-assumptions]] C(s), so their [[concepts/stage-order]] equals s; this gives them excellent [[concepts/b-convergence]] properties. The polynomial u(x) doubles as a free [[concepts/dense-output]] of order s. Lobatto IIIB and IIIC are *not* collocation methods (they enforce a discrete-derivative condition instead) — only IIIA is.

## Key Parameters

- Node count s and node positions c_i.
- Quadrature weights b_i from ∫ ℓ_i(t) dt.
- Order p of the method.
- Stage order q = s.

## When To Use

- High-order stiff integration via Gauss, Radau, or Lobatto IIIA collocation.
- Problems where the interpolating polynomial doubles as continuous output.
- Boundary-value problems (the standard COLNEW / COLSYS lineage).
- Differential–algebraic equations (Radau IIA collocation has superconvergence in y, stage-order convergence in z).

## Risks & Pitfalls

- High classical order does not protect against [[concepts/order-reduction]] on stiff problems; only the stage order matters.
- The polynomial output u(x) may interpolate with O(h^s) accuracy even when the endpoint output is O(h^{2s−1}) accurate (superconvergence loss in the interior).

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/gauss-method]]
- [[concepts/radau-iia-method]]
- [[concepts/lobatto-iiia-method]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/stage-order]]
- [[concepts/dense-output]]
- [[concepts/multistep-collocation]]
- [[concepts/runge-kutta-collocation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
