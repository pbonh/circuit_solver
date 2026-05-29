---
title: Gauss Method
type: claim
id: claim-gauss-method
tags:
- ode
- numerical-integration
- runge-kutta
- symplectic
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

The s-stage Gauss method is the [[concepts/collocation-method]] whose nodes are the s roots of the shifted Legendre polynomial P_s(2t − 1) on [0, 1]. It has classical order 2s — the highest order achievable by any s-stage Runge–Kutta method — and is A-stable, B-stable (algebraically stable), [[concepts/symplectic-method]], and time-symmetric.

## How It Works

Gauss quadrature exactness on polynomials of degree 2s − 1 lifts to order 2s for the ODE integrator, satisfying B(2s), C(s), D(s). The [[concepts/stability-function]] is the (s, s) diagonal Padé approximation to e^z, which is A-stable for every s but not L-stable (R(∞) = (−1)^s). Symplecticity of the s-stage Gauss method follows from M = BA + A^T B − bb^T = 0, a deeper algebraic consequence of Legendre orthogonality (Lasagni 1988, Sanz-Serna 1988). The [[concepts/stage-order]] is s, so on stiff problems the effective order reduces from 2s to s.

## Key Parameters

- Number of stages s (any positive integer).
- Nodes c_i = (1 + ξ_i)/2 with ξ_i the Legendre roots.
- Order p = 2s, stage order q = s.
- R(∞) = (−1)^s.

## When To Use

- Highest-order one-step integration on smooth problems.
- Hamiltonian / mechanical systems where symplecticity matters (paired with [[concepts/symplectic-integrator]] theory).
- Theoretical optimality benchmarks for IRK methods.

## Risks & Pitfalls

- Not L-stable; stiff transients are not damped.
- Severe [[concepts/order-reduction]] on stiff problems (effective order s, not 2s).
- The fully coupled (sn) × (sn) system is expensive for large dim(y); SDIRK or Rosenbrock is often preferred in practice.

## Related Concepts

- [[concepts/collocation-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/symplectic-method]]
- [[concepts/pade-approximation]]
- [[concepts/radau-iia-method]]
- [[concepts/algebraic-stability]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
