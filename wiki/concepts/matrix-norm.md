---
title: "Matrix Norm"
type: concept
tags: [foundational, math, numerical, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/26-appendix-f-selected-mathematical-topics.txt"]
confidence: high
---

## Definition

A matrix norm ||A|| is a real nonnegative number satisfying ||cA|| = |c| ||A||, ||A + B|| <= ||A|| + ||B||, and ||AB|| <= ||A|| ||B||. The induced (operator) norm is ||A|| = max_{||x||=1} ||Ax||. Three commonly used induced norms:
- ||A||_1 = max over columns of sum of |a_ij| (max column sum).
- ||A||_2 = sqrt(max eigenvalue of A^* A) = max singular value (spectral norm).
- ||A||_infinity = max over rows of sum of |a_ij| (max row sum).

## How It Works

For Ax = b with perturbation in b: ||Delta x|| <= ||A^{-1}|| ||Delta b||, so the relative error is bounded by kappa(A) ||Delta b|| / ||b||. Choice of norm matters for the constant in these bounds but the qualitative conclusion (well- vs. ill-conditioned) is consistent across norms.

## Key Parameters

- Choice of vector norm (induces matrix norm).
- Computational complexity: 1- and infinity-norms are O(n^2); 2-norm requires eigenvalues of A^* A (O(n^3)).

## When To Use

- Error analysis of linear solves.
- Stopping criteria in iterative methods.
- Defining convergence in matrix sequences.

## Risks & Pitfalls

- Different norms give different specific bounds; report consistently.
- 2-norm is expensive; 1 or infinity is preferred for runtime estimation.

## Related Concepts

- [[concepts/vector-space]]
- [[concepts/condition-number]]
- [[concepts/singular-values]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-26-appendix-f-selected-mathematical-topics]]
