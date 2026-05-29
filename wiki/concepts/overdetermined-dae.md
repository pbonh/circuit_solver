---
title: Overdetermined DAE
type: claim
id: claim-overdetermined-dae
tags:
- dae
- mechanical
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

An overdetermined DAE formulation (Eq. 2.25 in Hairer–Wanner Chapter VII; Führer 1988) appends *all* differentiated forms of the constraint — position level, velocity level, and acceleration level — to the original DAE and solves the resulting overdetermined system by least squares with Lagrange multipliers (Führer–Leimkuhler 1991).

## How It Works

Instead of choosing one index level (and accepting [[concepts/drift-off]] from the others), an overdetermined formulation requires g(q) = 0, G u = 0, *and* G u' + Ġ u = 0 simultaneously, fitting them by minimising a weighted least-squares residual. The discrete system is consistent on the original manifold (where all three constraints hold simultaneously) and the least-squares solver returns the unique closest point when round-off perturbs them. The approach is closely related to [[concepts/ggl-formulation]] but with extra constraint levels and a different discretisation. Campbell's unstructured-higher-index approach (1989) builds the [[concepts/derivative-array]] system and computes an underlying ODE via QR factorisation.

## Key Parameters

- Number of differentiated constraint levels appended.
- Weighting in the least-squares functional.
- Linear-system size after augmentation.

## When To Use

- Stress-test multibody problems where drift control is essential.
- Index-determination algorithms (Campbell-style derivative arrays).
- Systems with redundant constraints.

## Risks & Pitfalls

- The augmented linear system can be much larger than the original; cost per step grows.
- Weighting choice affects numerical conditioning.
- Less mature implementation lineage than projection or Baumgarte; fewer canned codes.

## Related Concepts

- [[concepts/index-reduction]]
- [[concepts/ggl-formulation]]
- [[concepts/projection-method-dae]]
- [[concepts/derivative-array]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/drift-off]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
