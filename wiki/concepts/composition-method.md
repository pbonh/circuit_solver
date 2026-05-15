---
title: "Composition Method"
type: concept
tags: [symplectic, numerical-integration, mechanical, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

A composition method (Yoshida 1990; Suzuki 1991) constructs a higher-order numerical scheme by composing a basic low-order method Φ_h with itself at carefully chosen sub-step sizes: Φ̃_h = Φ_{γ_k h} ∘ Φ_{γ_{k−1} h} ∘ … ∘ Φ_{γ_1 h}, where the γ_i are tuned to cancel low-order error terms.

## How It Works

For a symmetric basic method Φ_h (e.g. Störmer–Verlet), the simplest higher-order composition uses three sub-steps with γ_1 = γ_3 = 1/(2 − 2^{1/3}), γ_2 = −2^{1/3} γ_1, yielding a method of order 4. Iterating the construction gives orders 6, 8, … at cost 3^{k−1} basic steps for order 2k. The composition preserves all geometric properties of the basic method: symplecticity, time-reversibility, and (for constrained systems) the constraint manifold. Yoshida's coefficients are widely used in molecular dynamics and celestial mechanics; Reich (1996) extended the construction to constrained systems including [[concepts/constrained-hamiltonian-system]]s.

## Key Parameters

- Order 2k of the composed method.
- Number of sub-steps 3^{k−1}.
- Basic method Φ_h (symmetric, symplectic).
- Coefficient set γ_1, …, γ_{3^{k−1}} (negative entries appear past order 2).

## When To Use

- Long-time symplectic integration when high order is desired.
- Constrained Hamiltonian systems with [[concepts/shake-algorithm]] / [[concepts/rattle-algorithm]] base step.
- Special-function approximations (Magnus / Fer expansions).

## Risks & Pitfalls

- Negative sub-steps γ_i < 0 mean the integration runs *backward in time* on some sub-intervals — this is fine for autonomous systems but problematic for time-dependent forces with discontinuities.
- Higher orders require many basic-method evaluations per macro-step.
- Adaptive step sizing destroys symplecticity unless the composition coefficients are reweighted carefully.

## Related Concepts

- [[concepts/symplectic-method]]
- [[concepts/symplectic-integrator]]
- [[concepts/lobatto-iiia-iiib-pair]]
- [[concepts/shake-algorithm]]
- [[concepts/rattle-algorithm]]
- [[concepts/runge-kutta-method]]
- [[concepts/constrained-hamiltonian-system]]
- [[concepts/backward-error-analysis]]

## Sources

- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
