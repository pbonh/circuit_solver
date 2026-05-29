---
title: Order Reduction
type: claim
id: claim-order-reduction
tags:
- ode
- numerical-integration
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

Order reduction (Prothero–Robinson 1974) is the phenomenon that the *effective* order of a numerical method on stiff problems collapses from its classical order p to the [[concepts/stage-order]] q = min(p, max η in C(η)). The global error behaves like h^q rather than h^p for stiff modes, even though h^p convergence is recovered on the nonstiff limit.

## How It Works

On the linear test problem y' = λ(y − φ(x)) + φ'(x) with very negative Re λ (Prothero–Robinson), the local error decomposes into a smooth-component piece of order p and a stiff-component piece of order q. As h |λ| → ∞ only the order-q piece survives, because the stage equations cannot match the smooth derivatives beyond their stage-order accuracy. The same effect appears on nonlinear stiff problems satisfying [[concepts/one-sided-lipschitz-condition]] and on [[concepts/singular-perturbation-problem]]s / [[concepts/differential-algebraic-equation]]s, where the algebraic component sees only stage-order convergence. Implicit Gauss methods (p = 2s, q = s) suffer the strongest reduction; [[concepts/lobatto-iiia-method]] and [[concepts/lobatto-iiib-method]] (p = 2s − 2, q = s) also lose order; collocation Radau IIA (p = 2s − 1, q = s) recovers superconvergence in the differential component but not the algebraic.

## Key Parameters

- Classical order p.
- Stage order q.
- Method's [[concepts/butcher-simplifying-assumptions]] C(η), D(ζ).
- Stiffness scale h |λ|.

## When To Use

- Predicting effective convergence behaviour of an IRK method on stiff problems.
- Choosing between methods of equal classical order but different stage order.
- Designing Rosenbrock methods that avoid order reduction (Σ b_i ω_{ij} α_j = 1 condition).

## Risks & Pitfalls

- The observed order on a stiff problem can mislead method comparisons if h is reduced enough that one moves into the nonstiff regime mid-experiment.
- Order reduction is not the same as instability; the method still converges, just slowly.
- [[concepts/dense-output]] can suffer additional order loss at the boundary-layer regions.

## Related Concepts

- [[concepts/stage-order]]
- [[concepts/b-convergence]]
- [[concepts/butcher-simplifying-assumptions]]
- [[concepts/runge-kutta-method]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/differential-algebraic-equation]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
