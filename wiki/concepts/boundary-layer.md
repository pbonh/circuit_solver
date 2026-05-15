---
title: "Boundary Layer"
type: concept
tags: [ode, singular-perturbation, asymptotic, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

In a [[concepts/singular-perturbation-problem]] y' = f(y, z), ε z' = g(y, z) with inconsistent initial condition z_0 ≠ G(y_0), the boundary layer is a thin transient near x_0 in which z relaxes exponentially fast (rate ≈ 1/ε) to its slow-manifold value. In stretched coordinates ξ = (x − x_0)/ε the layer becomes O(1) wide; in physical x it is O(ε) wide.

## How It Works

The composite Vasil'eva (1963) expansion writes the solution as outer expansion + inner boundary-layer correction: z(x) = z̄(x) + ζ(ξ) with ζ → 0 as ξ → ∞ and ζ(0) = z_0 − G(y_0). The inner correction satisfies dζ/dξ = g(y(x_0), z̄(x_0) + ζ); under [[concepts/logarithmic-norm]] μ(g_z) ≤ −1 the correction decays like e^{−κ ξ} with κ > 0. For numerical methods, the boundary layer is the regime where [[concepts/order-reduction]] is strongest: smooth-component orders survive but algebraic / transient components only achieve stage-order convergence. [[concepts/extrapolation-method]]s with the *perturbed* asymptotic expansion (Deuflhard–Hairer–Zugck 1987) carry localised perturbation terms supported in the layer that survive limit-passage.

## Key Parameters

- Layer width ≈ ε.
- Inner / outer scaled variable ξ = (x − x_0)/ε.
- Decay rate κ ≥ −μ(g_z).
- Initial inconsistency Δz_0 = z_0 − G(y_0).

## When To Use

- Asymptotic analysis of SPPs and DAE limits.
- Diagnosing initial transients in stiff chemical / electronic / mechanical models.
- Designing dense-output schemes that don't amplify initial-layer perturbations (Hairer–Ostermann 1990).

## Risks & Pitfalls

- A boundary-layer transient can spoil the apparent order of a method until the layer dies out.
- Output points inside the layer need [[concepts/dense-output]] aware of the localised perturbation — naive interpolation can blow up.
- For methods with |R(∞)| > 1, the layer-induced error grows unboundedly.

## Related Concepts

- [[concepts/singular-perturbation-problem]]
- [[concepts/asymptotic-expansion]]
- [[concepts/perturbed-asymptotic-expansion]]
- [[concepts/order-reduction]]
- [[concepts/dense-output]]
- [[concepts/reduced-system]]

## Sources

- [[summaries/hairer-ode-ii-05-chapter-vi-singular-perturbation-problems]]
