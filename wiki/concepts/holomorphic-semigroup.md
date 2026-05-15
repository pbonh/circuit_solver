---
title: "Holomorphic Semigroup"
type: concept
tags: [ode, pde, functional-analysis, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

A holomorphic (analytic) semigroup is a strongly continuous semigroup {S(t)}_{t ≥ 0} on a Banach space that extends to a holomorphic map of a sector {z ∈ ℂ : |arg z| < θ} into bounded operators. Equivalently, the generator A is *sectorial*: its spectrum lies in a sector arg(λ − ω) ∈ [π − θ, π + θ] and the resolvent (λ − A)^{−1} satisfies a Hille–Yosida-type bound ‖(λ − A)^{−1}‖ ≤ M / |λ − ω| outside the sector.

## How It Works

Sectorial generators arise from elliptic differential operators (Laplacian, second-order self-adjoint operators with positive coefficients); the resulting semigroup is the solution operator of the corresponding parabolic PDE. In Hairer–Wanner Chapter V, the holomorphic-semigroup hypothesis is the abstract setting for proving uniform convergence of stiff multistep methods (Lubich 1988–91, Theorem 7.10) on parabolic [[concepts/method-of-lines]] discretisations: the resolvent estimate combines with the [[concepts/discrete-variation-of-constants]] formula and [[concepts/kreiss-matrix-theorem]] decay to give O(h^p) global error independent of the spatial discretisation parameter.

## Key Parameters

- Sector half-angle θ (typically θ ∈ (0, π/2]).
- Type ω (exponential growth bound).
- Resolvent constant M.

## When To Use

- Convergence theory for stiff time discretisations of parabolic PDEs.
- Functional-analytic setting for stiffness-uniform error bounds.
- Theoretical justification for the *order reduction* model e_m = O(|λ|^{−1} h^{p−1}) on Prothero–Robinson-type systems.

## Risks & Pitfalls

- Hyperbolic / wave-equation operators are not sectorial; their semigroups are merely C_0, not holomorphic, and the convergence theorems do not apply.
- The constant M and angle θ are PDE-specific; bounding them rigorously can be non-trivial.

## Related Concepts

- [[concepts/method-of-lines]]
- [[concepts/discrete-variation-of-constants]]
- [[concepts/kreiss-matrix-theorem]]
- [[concepts/order-reduction]]
- [[concepts/singular-perturbation-problem]]
- [[concepts/logarithmic-norm]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
