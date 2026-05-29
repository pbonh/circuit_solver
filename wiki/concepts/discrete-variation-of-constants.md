---
title: Discrete Variation of Constants
type: claim
id: claim-discrete-variation-of-constants
tags:
- ode
- numerical-integration
- multistep
- convergence
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.65
---

## Definition

The discrete variation-of-constants formula (Crouzeix–Raviart 1976; Lubich 1988–91) expresses the global error of a multistep method on a linear problem as a convolution e_m = ∑_j r_{m−j}(μ) d_j, where r_j(μ) is the *discrete resolvent* — the j-th coefficient of (δ(ζ) − μ)^{−1} ζ^k / σ(ζ^{−1}) in its power-series expansion — and d_j is the local truncation error.

## How It Works

This is the discrete analogue of the continuous variation-of-constants formula y(x) = e^{Ax} y_0 + ∫ e^{A(x − s)} g(s) ds. Combined with [[concepts/kreiss-matrix-theorem]] decay estimates on r_j and assumptions of [[concepts/holomorphic-semigroup]] type on the Jacobian, it gives O(h^p) convergence for parabolic [[concepts/method-of-lines]] discretisations using A(α)-stable multistep methods. For the Prothero–Robinson stiff model problem the formula gives sharp order-reduction estimates e_m = O(|λ|^{−1} h^{p−1}) for sectorial linear systems.

## Key Parameters

- Discrete resolvent r_j(μ).
- LMS characteristic polynomials (ρ, σ).
- Sectorial / holomorphic-semigroup parameters of the spatial operator.

## When To Use

- Convergence proofs for stiff and parabolic time discretisations.
- Order-reduction analysis on Prothero–Robinson-type test problems.
- Sharp error estimates for [[concepts/gear-bdf|BDF]] on infinite-dimensional dissipative systems.

## Risks & Pitfalls

- The discrete resolvent decay estimates are subtle for non-A-stable methods; A(α)-stability + [[concepts/multiplier-technique]] is the practical sweet spot.
- The formula is most transparent on linear problems; nonlinear extensions require fixed-point arguments.

## Related Concepts

- [[concepts/kreiss-matrix-theorem]]
- [[concepts/holomorphic-semigroup]]
- [[concepts/multiplier-technique]]
- [[concepts/gear-bdf]]
- [[concepts/method-of-lines]]
- [[concepts/order-reduction]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
