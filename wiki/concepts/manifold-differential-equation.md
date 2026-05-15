---
title: "Manifold Differential Equation"
type: concept
tags: [dae, mechanical, geometric, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: medium
---

## Definition

A manifold differential equation is an ODE y' = f(y) whose solutions are required to lie on a smooth submanifold M ⊂ ℝ^n. Tangency f(y) ∈ T_y M for all y ∈ M ensures that solutions starting on M stay on M. Equivalently, the system can be written as a DAE with explicit constraint c(y) = 0 defining M and right-hand side f tangent to the constraint manifold.

## How It Works

Constrained Hamiltonian / Lagrangian systems are the prototypical manifold ODEs: the constraint manifold M = {(q, p) : g(q) = 0, G(q) H_p = 0} carries the constrained Hamiltonian flow. Numerical integration on M requires either: (i) parametrising M locally (e.g. by [[concepts/tangent-space-parametrization]] or [[concepts/generalized-coordinate-partitioning]]) and integrating in the reduced space; (ii) integrating in ℝ^n and projecting back to M after each step ([[concepts/projection-method-dae]]); or (iii) using a manifold-aware integrator that respects the geometry intrinsically (Lie-group integrators, [[concepts/lobatto-iiia-iiib-pair]]). [[concepts/backward-error-analysis-manifolds]] explains long-time near-preservation of M by [[concepts/symplectic-integrator]]s.

## Key Parameters

- Manifold M and its dimension dim M.
- Codimension (= number of constraints).
- Tangency condition f(y) ∈ T_y M.

## When To Use

- Constrained mechanical / Hamiltonian systems.
- Lie-group dynamics (rotations, SE(3)).
- Reduction of geometric / topological invariants in numerical integration.

## Risks & Pitfalls

- Local parametrisations are valid only on coordinate patches; integration across patches needs chart-switching.
- Projection methods cost extra Newton iterations.
- Lie-group integrators have their own zoo (RK-MK, Munthe-Kaas) that differ from Euclidean RK.

## Related Concepts

- [[concepts/differential-algebraic-equation]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/constrained-mechanical-system]]
- [[concepts/projection-method-dae]]
- [[concepts/tangent-space-parametrization]]
- [[concepts/generalized-coordinate-partitioning]]
- [[concepts/symplectic-method]]
- [[concepts/backward-error-analysis-manifolds]]
- [[concepts/state-space-form]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
