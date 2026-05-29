---
title: B-Convergence
type: claim
id: concepts/b-convergence
tags:
- ode
- numerical-integration
- stiff
- convergence
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

A numerical method is B-convergent of order p* if its global error on any nonlinear ODE satisfying the [[concepts/one-sided-lipschitz-condition]] with constant ν is bounded by C · h^{p*}, with the constant C *independent* of the stiffness of the problem (Frank, Schneid, Ueberhuber 1981). B-convergence is the nonlinear stiffness-uniform analogue of classical convergence.

## How It Works

Classical convergence theorems give bounds that scale with the Lipschitz constant L = max ‖f_y‖ — uselessly large for stiff problems. B-convergence replaces L with the one-sided constant ν, which can be negative on dissipative systems. Theorem 15.3 (Hairer–Wanner) states an algebraically stable method of [[concepts/stage-order]] q is B-convergent of order q (not the classical p > q). This explains [[concepts/order-reduction]] rigorously: convergence happens at the stage-order rate, not the classical rate, when stiffness is large. [[concepts/lobatto-iiib-method]] cannot be B-convergent because A is singular and the local error blows up; [[concepts/radau-iia-method]] and [[concepts/lobatto-iiic-method]] are B-convergent of order s.

## Key Parameters

- Method order p (classical).
- Stage order q (sets the B-convergence order).
- One-sided Lipschitz constant ν.
- Algebraic stability matrix M.

## When To Use

- Comparing methods for stiff nonlinear problems where classical bounds are vacuous.
- Establishing the correct rate of convergence for IRK methods on [[concepts/method-of-lines]] discretisations.
- Method-design criterion: maximise stage order to maximise B-convergence order.

## Risks & Pitfalls

- B-convergence order ≤ stage order ≤ classical order; the gap can be wide (Gauss s-stage: p = 2s, q = s).
- For [[concepts/rosenbrock-method]]s the analogue requires the additional condition Σ b_i ω_{ij} α_j = 1.
- B-convergence does not by itself control error constants; pair with quantitative bounds.

## Related Concepts

- [[concepts/order-reduction]]
- [[concepts/stage-order]]
- [[concepts/algebraic-stability]]
- [[concepts/b-stability]]
- [[concepts/one-sided-lipschitz-condition]]
- [[concepts/radau-iia-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
