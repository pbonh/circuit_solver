---
title: Daniel–Moore Conjecture
type: claim
id: claim-daniel-moore-conjecture
tags:
- ode
- numerical-integration
- stability
- order-bound
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

The Daniel–Moore conjecture (Daniel & Moore 1970, proved by Wanner–Hairer–Nørsett 1978 via [[concepts/order-star]] theory): an A-stable s-stage Runge–Kutta method or s-derivative multistep method has order p ≤ 2s. Equality requires the error constant to satisfy (−1)^s C ≥ s! s! / ((2s)! (2s + 1)!), and the diagonal (s, s) [[concepts/pade-approximation]] of e^z achieves the bound (Gauss methods).

## How It Works

The order-star proof counts "in" fingers of the rational stability function R(z) at the origin (p + 1 fingers for an order-p method) and shows that an A-stable rational function with no finger crossing the imaginary axis cannot have p > 2s without a topological contradiction. The proof generalises to the [[concepts/general-linear-method]] class: any A-stable s-stage method with s poles representing numerical work has order at most 2s. The conjecture's resolution is one of the celebrated successes of order-star theory in Hairer–Wanner.

## Key Parameters

- Number of stages s (or derivatives, in the multistep generalisation).
- Method order p (bound is p ≤ 2s).
- Error constant C.

## When To Use

- Establishing maximum-order targets when designing new IRK / general-linear methods.
- Confirming optimality of Gauss methods (s stages, order 2s, A-stable).
- Theoretical baseline for the cost-vs-order trade-off in stiff integration.

## Risks & Pitfalls

- The 2s bound is for *A-stable* methods; non-A-stable methods can have higher order.
- For [[concepts/explicit-runge-kutta]] the analogous Butcher bound is much stricter (no A-stable explicit method exists; order is bounded by ≈ s for large s).
- The order-star proof is qualitative; quantitative error constants require additional analysis.

## Related Concepts

- [[concepts/order-star]]
- [[concepts/a-stability]]
- [[concepts/pade-approximation]]
- [[concepts/gauss-method]]
- [[concepts/dahlquist-barrier]]
- [[concepts/general-linear-method]]
- [[concepts/property-c]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
