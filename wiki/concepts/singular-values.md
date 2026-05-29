---
title: Singular Values
type: claim
id: claim-singular-values
tags:
- foundational
- math
- numerical
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/26-appendix-f-selected-mathematical-topics.txt
confidence:
  base: 0.65
---

## Definition

The singular values sigma_i of a matrix A are the positive square roots of the eigenvalues of A^* A (or equivalently A A^*). They are real, nonnegative, and conventionally ordered as sigma_1 >= sigma_2 >= ... >= sigma_n >= 0. The 2-norm condition number is kappa_2(A) = sigma_1 / sigma_n.

## How It Works

The singular value decomposition (SVD) A = U Sigma V^* expresses A as a product of orthogonal matrices U, V and a diagonal matrix Sigma of singular values. SVD is the gold standard for:
- Numerical rank determination (count sigmas above a noise threshold).
- Least-squares solution (pseudoinverse).
- Low-rank approximation (Eckart-Young theorem).
- Condition-number computation.

For circuit matrices, sigma_n near zero indicates near-singular T (resonances, near-pole frequencies, or ill-defined operating points).

## Key Parameters

- Computational cost: O(n^3) via standard algorithms.
- Numerical tolerance for distinguishing zero from nonzero sigmas.

## When To Use

- Diagnosing matrix rank and conditioning.
- Computing pseudoinverses.
- Robust solution of nearly-singular systems.

## Risks & Pitfalls

- Computing all singular values is expensive; for n > 1000, iterative methods (Lanczos) are used.
- Tiny singular values may be artifacts of finite arithmetic rather than true near-singularity.

## Related Concepts

- [[concepts/vector-space]]
- [[concepts/condition-number]]
- [[concepts/matrix-norm]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-26-appendix-f-selected-mathematical-topics]]
