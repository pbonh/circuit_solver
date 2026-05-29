---
title: SDBDF Method
type: claim
id: concepts/sdbdf-method
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

A second-derivative backward differentiation formula (SDBDF) augments the BDF skeleton ∑_i α_i y_{n+i} = h β f_{n+k} with a second-derivative term h^2 γ g_{n+k}, where g = f' = f_x + f_y f. It is a member of the [[concepts/enright-method]] family that reaches higher orders than ordinary BDF while preserving stiff stability.

## How It Works

For k = 1, …, 9 the SDBDF formulas are derived by matching the truncation error to order k + 2; SDBDF goes up to order 11 (k = 9 stiffly stable). The g term is evaluated either analytically (closed-form chain-rule) or by finite differences. The [[concepts/a-alpha-stability]] sector is larger than for BDF of the same order, partially compensating for the extra cost of f'.

## Key Parameters

- Step count k (order = k + 2 typically).
- Coefficients (α_i, β, γ).
- Cost of f' evaluation per step.
- Stability sector angle α.

## When To Use

- Stiff problems where higher than BDF6 order is required.
- Applications with analytic Jacobian (cheap f' via chain rule).
- Theoretical comparison of barrier-circumventing methods.

## Risks & Pitfalls

- Inaccurate f' degrades the effective order.
- Variable-order / variable-step implementation is more involved than BDF.
- For very high orders (10, 11) the error constant becomes large and step-size restrictions tighten.

## Related Concepts

- [[concepts/enright-method]]
- [[concepts/gear-bdf]]
- [[concepts/linear-multistep-methods]]
- [[concepts/extended-bdf-method]]
- [[concepts/blended-multistep-method]]
- [[concepts/dahlquist-barrier]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
