---
title: Perturbation Index
type: claim
id: claim-perturbation-index
tags:
- dae
- classification
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

The perturbation index pi of a DAE (Hairer–Lubich–Roche 1989, "HLR89") is the smallest non-negative integer m such that for every perturbation δ(x) of the equation and every Δy_0 of the initial conditions, the perturbed solution ŷ satisfies ‖ŷ(x) − y(x)‖ ≤ C (‖Δy_0‖ + ‖δ‖_∞ + ‖δ'‖_∞ + … + ‖δ^{(m−1)}‖_∞) on a compact interval.

## How It Works

The perturbation index quantifies *sensitivity* to perturbations: index 1 means errors in the equation propagate at the same scale (good); higher indices mean each unit of perturbation produces errors scaling with derivatives of the perturbation, which is bad numerically (round-off and discretisation error always have non-zero derivative content). Lubich (1989) showed that for the implicit-form DAE M(y) y' = f(y) the perturbation and differentiation indices can differ arbitrarily — Campbell–Gear's nilpotent-Jordan example (Eq. 1.32 in Hairer–Wanner) makes this concrete with di = 2 and pi = 1. Numerical-method choice is best guided by perturbation index because round-off enters as a perturbation.

## Key Parameters

- Number of derivatives of δ that appear in the error bound.
- Compact interval of definition.
- Constant C in the bound.

## When To Use

- Predicting numerical sensitivity of a DAE.
- Choosing solver tolerance: high pi requires tighter tolerance to keep the derivative norms small.
- Theoretical convergence analysis (HLR89 theorems use pi explicitly).

## Risks & Pitfalls

- pi can be lower than di (sensitivity is benign even when many differentiations are needed); also higher (rare).
- The bound is asymptotic; explicit constants C are problem-dependent.

## Related Concepts

- [[concepts/index-of-a-dae]]
- [[concepts/differentiation-index]]
- [[concepts/index-of-nilpotency]]
- [[concepts/index-1-dae]]
- [[concepts/index-2-dae]]
- [[concepts/index-3-dae]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
