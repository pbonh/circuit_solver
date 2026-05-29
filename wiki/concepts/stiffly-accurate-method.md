---
title: Stiffly Accurate Method
type: claim
id: claim-stiffly-accurate-method
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

A Runge–Kutta method is stiffly accurate (Prothero–Robinson 1974) if its last row of A equals b^T and c_s = 1: a_{si} = b_i for i = 1, …, s. The step output y_{n+1} then coincides with the last internal stage Y_s, and the [[concepts/stability-function]] automatically satisfies R(∞) = 0.

## How It Works

Stiff accuracy combines two benefits: (i) it forces L-stability whenever the method is A-stable, because R(∞) = 0 follows from b^T = e_s^T A; (ii) on index-1 DAEs and [[concepts/singular-perturbation-problem]]s the last stage equation is exactly the algebraic-constraint equation, so y_{n+1} satisfies the algebraic part automatically. This is why [[concepts/radau-iia-method]], stiffly-accurate variants of [[concepts/sdirk-method]] (Hairer–Wanner SDIRK4), and Rosenbrock-DAE methods like RODAS are all stiffly accurate by design. For Rosenbrock methods the analogous condition is a_{si} + γ_{si} = b_i together with α_s = 1.

## Key Parameters

- Last-row condition a_{si} = b_i.
- c_s = 1.
- R(∞) = 0 automatic consequence.
- For Rosenbrock, also α_s = 1.

## When To Use

- Any IRK method intended for index-1 [[concepts/differential-algebraic-equation]]s.
- Singular-perturbation problems where the algebraic limit ε = 0 must be respected.
- Methods that need automatic L-stability without separately enforcing R(∞) = 0.

## Risks & Pitfalls

- The condition forces one specific stage to be the output, removing some design freedom.
- Stage order is unaffected — stiffly accurate methods still suffer [[concepts/order-reduction]] in the differential variable.
- Without stiff accuracy, an IRK method's z-component on index-2 DAEs diverges when R(∞) > 1.

## Related Concepts

- [[concepts/l-stability]]
- [[concepts/radau-iia-method]]
- [[concepts/sdirk-method]]
- [[concepts/rosenbrock-method]]
- [[concepts/differential-algebraic-equation]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/stability-function]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
