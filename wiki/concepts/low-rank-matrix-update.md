---
title: "Low-Rank Matrix Update (Sherman-Morrison-Woodbury)"
type: concept
tags: [foundational, numerical, well-established, math]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt"]
confidence: medium
---

## Definition

A low-rank matrix update modifies a matrix A by adding a rank-m product: A' = A + U V^T. If A^{-1} is already known, the Sherman-Morrison-Woodbury formula gives A'^{-1} = A^{-1} - A^{-1} U (I + V^T A^{-1} U)^{-1} V^T A^{-1} — requiring only the inversion of an m x m matrix instead of refactoring A'.

## How It Works

In Vlach & Singhal Chapter 8, perturbations enter as T_0 + P delta Q^T where delta is m x m and P, Q are n x m. The Woodbury form gives:
T^{-1} = T_0^{-1} - T_0^{-1} P (delta^{-1} + Q^T T_0^{-1} P)^{-1} Q^T T_0^{-1}.

The matrix F = Q^T T_0^{-1} P (size m x m) and the resulting (delta^{-1} + F) m x m system are the central computational objects. Once T_0 is LU-factored once and F is computed in m+1 forward/back substitutions, repeated perturbation queries are cheap.

## Key Parameters

- Rank m of the update.
- Number of preprocessing forward/back substitutions: m+1.
- Cost of per-query small-system solve: O(m^3).

## When To Use

- Repeated solves with closely related matrices.
- Iterative refinement in nonlinear solvers (quasi-Newton).
- Large change sensitivity in circuit simulation.

## Risks & Pitfalls

- Numerical stability of the small (delta^{-1} + F) system depends on the original matrix's conditioning.
- For very high-rank updates, refactoring may be cheaper.

## Related Concepts

- [[concepts/large-change-sensitivity]]
- [[concepts/zero-pivot-handling]]
- [[concepts/lu-decomposition]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
