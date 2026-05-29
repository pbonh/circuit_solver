---
title: QZ Algorithm (Generalized Eigenvalue)
type: claim
id: claim-qz-algorithm
tags:
- foundational
- numerical
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/10-chapter-7-network-functions-in-the-frequency-domain.txt
confidence:
  base: 0.65
---

## Definition

The QZ algorithm (Moler & Stewart, 1973) solves the generalized eigenvalue problem (A - lambda B) x = 0 for square matrices A, B by unitary transformations Q and Z such that Q A Z and Q B Z are both upper triangular (Schur form). The diagonal entries provide pairs (alpha_i, beta_i) and the generalized eigenvalues are alpha_i / beta_i. EISPACK [6] provides reference FORTRAN code.

## How It Works

In network applications, T = sC + G; the generalized eigenvalue problem det(sC + G) = 0 yields the poles. The QZ algorithm computes (alpha_i, beta_i) such that det(sC + G) = product (alpha_i + beta_i s), revealing finite poles when beta_i != 0 and "poles at infinity" when beta_i = 0 (these are zero alpha_i contributing only to gain).

To find zeros: augment the system with one row and column representing the output via Cramer's rule, then apply QZ to the augmented T_M; the determinant equals N(s).

The 9th-order Cauer filter example produces 35 (alpha, beta) pairs; 26 have beta = 0, 9 give the actual poles.

## Key Parameters

- Matrix dimension n.
- Density of A and B (QZ uses dense computations; sparsity is lost).
- Numerical precision.

## When To Use

- Direct computation of poles and zeros from the system matrix without intermediate polynomial.
- Small networks (< ~50x50) where the cubic cost is acceptable.
- High-precision pole/zero computation.

## Risks & Pitfalls

- O(n^3) cost; not suitable for very large networks.
- Sparsity of original T is destroyed by the unitary transformations.
- Generalized eigenvalues at infinity require careful interpretation.

## Related Concepts

- [[concepts/poles-and-zeros]]
- [[concepts/symbolic-function-generation]]
- [[concepts/lu-decomposition]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-10-chapter-7-network-functions-in-the-frequency-domain]]
- [[summaries/computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis]]
