---
title: "Index Reduction"
type: concept
tags: [dae, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

Index reduction is the technique of differentiating algebraic constraints of a higher-index DAE to produce an equivalent system of lower index, ideally index 1 so it can be passed to a standard ODE solver. For a [[concepts/constrained-mechanical-system]] q' = u, M u' = f − G^T λ, g(q) = 0, differentiating g once gives the velocity-level constraint G u = 0 (index 2), and differentiating twice gives the acceleration-level constraint G u' + Ġ u = 0 (index 1).

## How It Works

After reducing to index 1, the system is amenable to RADAU5 / RODAS / DASSL. The price is the *[[concepts/drift-off]]* phenomenon: the original constraint g(q) = 0 (and intermediate ones) is no longer enforced numerically — small round-off / discretisation errors accumulate and produce g(q(t)) ≈ O(t^2), G(q(t)) u(t) ≈ O(t). Remedies for the drift include [[concepts/baumgarte-stabilization]] (replace g̈ = 0 by g̈ + 2α ġ + β² g = 0), [[concepts/projection-method-dae]] (project back to the constraint manifold after each step), and [[concepts/ggl-formulation]] (augment with extra multiplier μ to satisfy both g and ġ). [[concepts/overdetermined-dae]] formulations append all constraint levels at once and resolve them by least squares (Führer–Leimkuhler).

## Key Parameters

- Number of differentiations performed.
- Drift-off rate after reduction.
- Stabilisation parameters α, β (Baumgarte).
- Projection frequency / GGL multiplier.

## When To Use

- Multibody dynamics with index-3 constraints.
- Singular-perturbation problems where the reduced system is the index-1 limit.
- Optimal control with state and adjoint constraints.

## Risks & Pitfalls

- Drift-off without stabilisation — the manifold is left after few steps.
- Stabilisation parameters need tuning; too small leaves drift, too large stiffens the system.
- Naïve differentiation propagates noise; analytic / symbolic differentiation is preferable when feasible.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/index-of-a-dae]]
- [[concepts/differentiation-index]]
- [[concepts/drift-off]]
- [[concepts/baumgarte-stabilization]]
- [[concepts/projection-method-dae]]
- [[concepts/ggl-formulation]]
- [[concepts/overdetermined-dae]]
- [[concepts/derivative-array]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
