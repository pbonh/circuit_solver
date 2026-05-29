---
title: Condition Number
type: claim
id: concepts/condition-number
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
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The condition number of a matrix A is kappa(A) = ||A|| ||A^{-1}||. It measures the amplification of relative errors in solving Ax = b. If b has relative error eps_b, then x has relative error at most kappa(A) * eps_b. The 2-norm condition number is the ratio of the largest to smallest singular value of A.

## How It Works

For finite-precision arithmetic, the achievable accuracy in x is approximately kappa(A) * machine epsilon. A matrix with kappa(A) ~ 10^16 in double precision yields essentially no significant digits. Common rules of thumb:
- kappa < 10^4: well-conditioned, expect at least 12 significant digits.
- 10^4 < kappa < 10^10: moderately ill-conditioned.
- kappa > 10^10: severely ill-conditioned; use higher precision or reformulation.

For circuit matrices, scaling (Section 1.8 of the book) reduces kappa by bringing element values to comparable magnitudes.

## Key Parameters

- Norm used (1-, 2-, infinity-norm).
- Singular value spread (for 2-norm: kappa = sigma_max / sigma_min).
- Matrix scale.

## When To Use

- Diagnosing numerical accuracy of linear solves.
- Comparing alternative network scalings.
- Setting tolerance for iterative refinement.

## Risks & Pitfalls

- Computing kappa exactly requires A^{-1} or SVD — expensive. Estimators (LAPACK CONDEST) are preferred.
- High condition number doesn't always mean the LU solution is wrong; it just means the bound on the error is loose.

## Related Concepts

- [[concepts/vector-space]]
- [[concepts/matrix-norm]]
- [[concepts/singular-values]]
- [[concepts/network-scaling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-26-appendix-f-selected-mathematical-topics]]
