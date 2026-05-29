---
title: Extended BDF Method
type: claim
id: claim-extended-bdf-method
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
  base: 0.65
---

## Definition

Cash's Extended BDF (EBDF, 1980) and Modified EBDF (MEBDF, 1983) are multistep schemes for stiff problems that augment the BDF formula with one "super-future" point at x_{n+k+1} computed by a separate predictor. The composite is solved by a three-stage predictor–corrector–corrector scheme: (i) BDF predicts y_{n+k}^*, (ii) the super-future predictor produces y_{n+k+1}^*, (iii) the modified formula uses both to compute y_{n+k}.

## How It Works

The super-future point gives the corrector access to a one-step-ahead derivative estimate, which acts like a virtual order-boost similar to [[concepts/enright-method]]'s second-derivative term but using only f-evaluations. EBDF/MEBDF reach order p with A-stability up to p = 4 and stiff stability up to p = 9. Implementation requires nesting two predictor stages within one step, raising the cost over plain BDF; the trade-off is good stability margins and modest extra storage compared to the second-derivative families.

## Key Parameters

- Step count k (order p = k + 1).
- Predictor / corrector coefficients.
- Super-future point evaluation at x_{n+k+1}.

## When To Use

- Stiff problems where extending past BDF6 is needed.
- Code lineages preferring f-evaluations over f'-evaluations (no analytic Jacobian).
- Theoretical study of barrier-circumventing predictor–corrector designs.

## Risks & Pitfalls

- Three stages per step ≈ 3× the cost of plain BDF.
- Variable step is intricate.
- Stability sector still narrows for very high orders.

## Related Concepts

- [[concepts/gear-bdf]]
- [[concepts/enright-method]]
- [[concepts/sdbdf-method]]
- [[concepts/blended-multistep-method]]
- [[concepts/predictor-corrector-method]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
