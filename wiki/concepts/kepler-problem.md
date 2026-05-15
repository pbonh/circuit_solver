---
title: "Kepler Problem"
type: concept
tags: [mechanical, hamiltonian, benchmark, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The Kepler problem is the two-body central-force gravitational system: a point mass moving under the inverse-square attraction H(q, p) = ‖p‖²/2 − 1/‖q‖. Equations of motion: q' = p, p' = −q/‖q‖³. Closed-form Keplerian orbits (ellipses, parabolas, hyperbolas) make it the gold-standard test problem for [[concepts/symplectic-method]]s and long-time-energy-preserving integrators.

## How It Works

The Hamiltonian H, total angular momentum L = q × p, and (in 2D / 3D) the Laplace–Runge–Lenz vector are *first integrals* — exact constants of the motion. A perfect numerical integrator would preserve all three exactly. Non-symplectic methods (RK4, classical multistep) drift in energy linearly with time; symplectic methods (Verlet, [[concepts/composition-method]]s, [[concepts/lobatto-iiia-iiib-pair]]) preserve a *nearby* H̃ exactly, so the energy oscillates around the true value but does not drift secularly (Fig. 2.3 in Hairer–Wanner VII). This near-conservation is explained by [[concepts/backward-error-analysis-manifolds]]: a symplectic step is the exact flow of a modified Hamiltonian H̃ = H + O(h^p).

## Key Parameters

- Initial conditions (eccentricity e ∈ [0, 1) gives closed elliptical orbits).
- Period T = 2π / (1 − e²)^{3/2}.
- First integrals H, L (and the Laplace–Runge–Lenz vector in dimensions ≥ 2).

## When To Use

- Long-time symplectic-integrator benchmarks.
- Demonstration of [[concepts/backward-error-analysis]] and conservation properties.
- Pedagogical example of integrable Hamiltonian systems.

## Risks & Pitfalls

- Near pericentre (close approach to the origin) the dynamics speed up by orders of magnitude; adaptive step control is needed even with symplectic methods.
- Adaptive step destroys symplecticity unless one uses Levi-Civita / Kustaanheimo–Stiefel regularisation.

## Related Concepts

- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/composition-method]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/backward-error-analysis]]
- [[concepts/backward-error-analysis-manifolds]]
- [[concepts/manifold-differential-equation]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
