---
title: Generalized Coordinate Partitioning
type: claim
id: claim-generalized-coordinate-partitioning
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

Generalized coordinate partitioning (Wehage–Haug 1982) is a local parametrisation of the constraint manifold of a [[concepts/constrained-mechanical-system]] by splitting the generalised coordinates q = (q_d, q_a) into *dependent* and *independent* groups so that g(q_d, q_a) = 0 can be solved for q_d = h(q_a). The independent coordinates q_a parameterise the manifold locally.

## How It Works

The splitting is chosen so that the partial Jacobian ∂g/∂q_d is invertible — the [[concepts/implicit-function-theorem]] then gives the local map q_d = h(q_a) and the reduced ODE q_a'' = (M_aa − M_ad h_a)^{−1} (f_a − M_ad q_d'' + …) is a plain ODE in q_a. The partitioning may need to be changed as the trajectory moves (when ∂g/∂q_d becomes ill-conditioned), giving a *chart-switching* algorithm. Used in multibody-dynamics codes (DADS, ADAMS) as an alternative to projection or GGL approaches.

## Key Parameters

- Partition (q_d, q_a) of the coordinate vector.
- Invertibility threshold for ∂g/∂q_d (triggers chart switch).
- Dimension of reduced ODE = dim(q) − rank(G).

## When To Use

- Reduced-coordinate multibody dynamics.
- Real-time mechanical simulation where projection cost matters.
- Theoretical state-space-form construction (Potra–Rheinboldt 1990).

## Risks & Pitfalls

- Chart switching is implementation-heavy.
- Singular configurations break the partitioning; detection and fallback strategies are needed.
- Less robust than projection-based methods for systems with redundant constraints.

## Related Concepts

- [[concepts/state-space-form]]
- [[concepts/tangent-space-parametrization]]
- [[concepts/manifold-differential-equation]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/implicit-function-theorem]]
- [[concepts/index-reduction]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
