---
title: "Tangent Space Parametrization"
type: concept
tags: [dae, mechanical, geometric, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

A tangent-space parametrisation (Potra–Rheinboldt 1990) of a [[concepts/manifold-differential-equation]] near a point y_0 ∈ M is a local chart that identifies the manifold with an open neighbourhood of the origin in the tangent space T_{y_0} M ≃ ℝ^{dim M}. Combined with a retraction map back to M, it lets standard ODE integrators operate in the reduced (dim M)-dimensional Euclidean space.

## How It Works

Choose a basis B = (b_1, …, b_d) of T_{y_0} M (e.g. by QR / SVD of the constraint Jacobian G). The chart is η ↦ y_0 + B η + R(η) where R is a higher-order correction (the *retraction*) chosen to land on M. The reduced ODE η' = (BᵀM B)^{−1} Bᵀ M f̃(y_0 + B η + R(η)) is a plain ODE in η, integrated by any standard method. Chart switching (re-anchoring at the current y_n) is needed when η leaves the validity ball of the current parametrisation. This is the geometric analogue of [[concepts/generalized-coordinate-partitioning]] (which picks a coordinate subset rather than a tangent basis).

## Key Parameters

- Tangent basis B and reference point y_0.
- Retraction R(η) (often quadratic correction toward M).
- Chart-switch threshold.

## When To Use

- Geometric integration on constraint manifolds.
- Lie-group integrators (the tangent space is the Lie algebra).
- Multibody dynamics codes that prefer reduced-coordinate formulations.

## Risks & Pitfalls

- Chart switching is delicate; bad heuristics cause discontinuities in η.
- Retraction must be at least order p − 1 to preserve a method of order p.
- For high-dimensional manifolds the basis computation can dominate cost.

## Related Concepts

- [[concepts/manifold-differential-equation]]
- [[concepts/state-space-form]]
- [[concepts/generalized-coordinate-partitioning]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/symplectic-integrator]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
