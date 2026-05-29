---
title: Enright Method
type: claim
id: concepts/enright-method
tags:
- ode
- numerical-integration
- multistep
- stiff
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Enright's (1974) second-derivative multistep methods are generalised LMS schemes that include a second-derivative term: ∑_i α_i y_{n+i} = h ∑_i β_i f_{n+i} + h^2 γ_k g_{n+k}, where g = f' = f_x + f_y f. Including g lets the method circumvent the [[concepts/dahlquist-barrier]] and reach order k + 2 for a k-step formula.

## How It Works

The added g_{n+k} term carries information about the curvature of the solution, allowing A-stable methods up to order 4 (k = 1, 2) and stiffly stable methods up to order 9 (k = 7). The price is the need to evaluate (or approximate) f' — analytically when feasible, by finite differences otherwise. [[concepts/sdbdf-method]] (second-derivative BDF) extends the idea to higher orders (up to 11), and Cash's [[concepts/extended-bdf-method]] uses "super-future" points instead of g to similar effect.

## Key Parameters

- Step count k.
- Coefficients α_i, β_i, γ_k.
- Method order p = k + 2.
- Cost of evaluating f'.

## When To Use

- Stiff problems where f' is cheap to evaluate (analytical Jacobian + chain rule).
- Codes needing higher A-stable order than BDF can offer.
- Classroom-level study of multistep order barriers and their workarounds.

## Risks & Pitfalls

- Evaluating f' by finite differences is expensive and noisy.
- Variable-step implementation is more complex than for plain BDF.
- Stiff stability range still degrades with order.

## Related Concepts

- [[concepts/sdbdf-method]]
- [[concepts/extended-bdf-method]]
- [[concepts/gear-bdf]]
- [[concepts/linear-multistep-methods]]
- [[concepts/dahlquist-barrier]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
