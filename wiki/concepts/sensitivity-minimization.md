---
title: Sensitivity Minimization (Robust Design)
type: claim
id: claim-sensitivity-minimization
tags:
- optimization
- sensitivity
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/20-chapter-17-design-by-minimization.txt
confidence:
  base: 0.65
---

## Definition

Sensitivity minimization selects element values to minimize the sum (or worst-case) of multiparameter sensitivities, producing a design that is robust to component tolerances. A typical objective is E = sum_j (S_{h_j}^F)^2 for sensitivities at a critical frequency.

## How It Works

The sensitivities S_{h_j}^F are computed by the adjoint method (Chapter 6). Their gradient with respect to the design variables (second-order sensitivities) is computed by Chapter 6 Section 6.6 techniques. SQP or Powell-type algorithms then minimize the objective.

Often combined with a primary mean-square objective to balance nominal performance against tolerance robustness. A weighted objective E = w_perf E_perf + w_sens E_sens balances the two goals.

## Key Parameters

- Sensitivity measure (sum of squares, worst case, weighted).
- Performance vs. sensitivity weight.
- Tolerance assumptions for each component type.

## When To Use

- IC design where capacitor matching is the primary tolerance.
- Yield-aware analog design.
- Critical-spec filter design.

## Risks & Pitfalls

- Minimizing sensitivities can degrade nominal performance.
- Higher-order sensitivities matter for large tolerances; first-order minimization may miss interactions.
- Trade-off requires careful weight selection.

## Related Concepts

- [[concepts/optimization-theory]]
- [[concepts/sensitivity-analysis]]
- [[concepts/multiparameter-sensitivity]]
- [[concepts/higher-order-sensitivity]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-20-chapter-17-design-by-minimization]]
