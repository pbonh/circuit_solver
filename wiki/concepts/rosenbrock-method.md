---
title: Rosenbrock Method
type: claim
id: claim-rosenbrock-method
tags:
- ode
- numerical-integration
- runge-kutta
- stiff
- dae
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

A Rosenbrock (linearly implicit Runge–Kutta) method (Rosenbrock 1963) replaces the nonlinear stage system of an implicit RK method by *linear* systems of the form (I − h γ J) k_i = f(...) + h J ∑_{j<i} γ_{ij} k_j, where J = f_y(y_n) is the exact (or approximate) Jacobian and γ is a common diagonal coefficient. Each stage requires solving with the same matrix (I − h γ J), so one LU per step suffices.

## How It Works

Order conditions are formulated on Butcher trees with extra vertex-marking rules: β_{ij} = α_{ij} + γ_{ij}, with α the off-diagonal "explicit-RK-like" coefficients and γ the linearly-implicit coupling. The classical Hairer–Wanner construction yields GRK4T, GRK4A (Shampine 1982, Kaps–Rentrop 1979), the Veldhuizen variants, and the stiffly-accurate RODAS family (order 4 / 5 with embedded estimator) designed for [[concepts/differential-algebraic-equation]]s. A method is *stiffly accurate* for Rosenbrock when a_{si} + γ_{si} = b_i and α_s = 1, which forces R(∞) = 0 and makes the last stages equivalent to a [[concepts/simplified-newton-iteration]] step on the algebraic part. The companion [[concepts/w-method]] relaxes J to an arbitrary approximation A, but the order-condition tree set TW explodes; ROW methods are pragmatic compromises.

## Key Parameters

- Common diagonal γ; tableau α, γ (the two-tableau Rosenbrock form).
- Classical order p, embedded order p̂.
- A-stability / L-stability of the [[concepts/stability-function]].
- Stiffly-accurate flag for DAE applicability.

## When To Use

- Stiff ODEs where one Jacobian + one LU per step is the cost budget.
- Index-1 DAEs (use stiffly accurate Rosenbrock — RODAS).
- Embedded error estimation without nested Newton iterations.

## Risks & Pitfalls

- Order conditions require *exact* Jacobian (or a high-quality approximation); inaccurate J degrades effective order — switch to [[concepts/w-method]] when J cannot be evaluated precisely.
- Stage order is low (typically 1 or 2), causing [[concepts/order-reduction]] on stiff non-autonomous problems unless the extra Σ b_i ω_{ij} α_j = 1 condition is imposed.
- Numerical robustness depends on the choice of γ; classical recommendations are γ ∈ [0.25, 0.5].

## Related Concepts

- [[concepts/w-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/simplified-newton-iteration]]
- [[concepts/l-stability]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/order-reduction]]
- [[entities/rodas]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
