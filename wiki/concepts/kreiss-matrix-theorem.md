---
title: "Kreiss Matrix Theorem"
type: concept
tags: [linear-algebra, numerical-integration, stability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/solving_ordinary_differential_equations_ii/_txt/"]
confidence: high
---

## Definition

Kreiss's matrix theorem (Kreiss 1962) relates power-boundedness of a matrix A to a resolvent condition: ‖A^n‖ ≤ M for all n ≥ 0 if and only if there exists C > 0 such that ‖(z I − A)^{−1}‖ ≤ C/(|z| − 1) for every |z| > 1. The constant M can be taken proportional to k · C for k × k matrices (LeVeque–Trefethen 1984 sharp form).

## How It Works

The forward direction (power-bounded ⟹ resolvent bound) follows from the Neumann series. The hard direction (resolvent bound ⟹ power-bounded) was Kreiss's original contribution; LeVeque–Trefethen gave the linear-in-k bound. In numerical-ODE theory the theorem replaces eigenvalue-only stability arguments with resolvent / pseudo-spectral bounds, essential for non-normal matrices where the spectrum can sit deep inside the unit disk while ‖A^n‖ grows large (transient growth). For [[concepts/linear-multistep-methods]] in Chapter V, Kreiss's theorem applied to the companion matrix C(μ) gives uniform decay estimates on the [[concepts/discrete-variation-of-constants]] resolvent r_j(μ).

## Key Parameters

- Power-bound M.
- Resolvent-bound constant C.
- Matrix dimension k.

## When To Use

- Stability proofs for stiff multistep methods on non-normal linear systems.
- Discrete-resolvent decay estimates underpinning convergence proofs for parabolic PDE method-of-lines.
- Theoretical companion to [[concepts/von-neumann-theorem]] and pseudo-spectral analysis.

## Risks & Pitfalls

- The bound is sharp in k for general matrices; replace by spectral / pseudo-spectral arguments when more structure is available.
- The constant C in the resolvent condition is geometry-dependent; computing it can be involved.

## Related Concepts

- [[concepts/von-neumann-theorem]]
- [[concepts/logarithmic-norm]]
- [[concepts/discrete-variation-of-constants]]
- [[concepts/holomorphic-semigroup]]
- [[concepts/multiplier-technique]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
