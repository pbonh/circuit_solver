---
title: "Lobatto IIIC Method"
type: concept
tags: [ode, numerical-integration, runge-kutta, stiff, dae, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

The s-stage Lobatto IIIC method is the algebraically-stable variant in the Lobatto family on Lobatto nodes. It has classical order 2s − 2 and stage order s − 1, and is A-stable, L-stable, [[concepts/algebraic-stability|algebraically stable]], B-stable, and [[concepts/stiffly-accurate-method]].

## How It Works

Lobatto IIIC enforces an extra constraint a_{i1} = b_1 for all i (and corresponding c_1 = 0, c_s = 1) so the first column equals b_1 𝟙. This makes A non-singular (unlike IIIB), gives R(∞) = 0 (unlike IIIA), and produces a non-negative definite M = BA + A^T B − bb^T (unlike both IIIA and IIIB). The trade-off is that stage order is s − 1, not s, so [[concepts/b-convergence]] order is s − 1. For s ≥ 3, Hairer–Wanner observe α_0(A^{−1}) = 0 (the standard coercivity coefficient vanishes), but Liu & Kraaijevanger (1988) proved IRK stage uniqueness still holds whenever μ(f_y) ≤ 0.

## Key Parameters

- Number of stages s ≥ 2.
- Nodes c_1 = 0, c_s = 1.
- Order 2s − 2, stage order s − 1.
- A-, L-, B-, algebraically stable.

## When To Use

- Stiff problems where B-stability is required (nonlinear dissipative ODEs).
- Index-2 [[concepts/differential-algebraic-equation]]s (Lobatto IIIC is stiffly accurate, supporting DAE convergence theorems).
- Theoretical analysis of algebraically-stable Lobatto families.

## Risks & Pitfalls

- Lower stage order than [[concepts/radau-iia-method]] (s − 1 vs. s), so larger order-reduction effect.
- α_0(A^{−1}) = 0 for s ≥ 3 means the standard uniqueness bound is vacuous; rely on the Liu–Kraaijevanger result.

## Related Concepts

- [[concepts/lobatto-iiia-method]]
- [[concepts/lobatto-iiib-method]]
- [[concepts/runge-kutta-method]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/algebraic-stability]]
- [[concepts/stiffly-accurate-method]]
- [[concepts/radau-iia-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
