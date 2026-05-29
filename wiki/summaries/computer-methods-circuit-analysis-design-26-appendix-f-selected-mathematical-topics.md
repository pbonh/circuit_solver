---
title: 'Computer Methods for Circuit Analysis and Design — Appendix F: Selected Mathematical
  Topics'
type: source
id: summaries/computer-methods-circuit-analysis-design-26-appendix-f-selected-mathematical-topics
kind: publication
tags:
- foundational
- math
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/26-appendix-f-selected-mathematical-topics.txt
---

## Key Points

- F.1 Vector spaces: span, basis, dimensionality, linear independence/dependence. The least-cardinality spanning set is a basis; its size is the dimension. n basis vectors generate an n-dimensional space.
- F.2 Matrices and sets of equations:
  - Row space and column space have the same dimensionality = rank of A.
  - System Ax = b has a unique solution iff A is nonsingular (rank = n).
  - For rank r < n, the consistency condition is rank[A|b] = rank[A]; if consistent, there is an (n-r)-dimensional family of solutions.
- F.3 Determinants: defined by Laplace expansion over n! permutations. Sign by permutation parity. Generalized to block matrices when submatrices are square. The rank of A equals the order of the largest submatrix with nonzero minor. |A| != 0 iff A is full rank (nonsingular).
- F.4 Norms: a vector norm ||x|| is nonnegative, scales as ||cx|| = |c| ||x||, and satisfies the triangle inequality. Three common vector norms:
  - ||x||_1 = sum |x_i|.
  - ||x||_2 = sqrt(sum |x_i|^2).
  - ||x||_infinity = max |x_i|.
- Induced matrix norms:
  - ||A||_1 = max column sum of |a_ij|.
  - ||A||_2 = sqrt(max eigenvalue of A*A) — the spectral norm.
  - ||A||_infinity = max row sum of |a_ij|.
- Singular values: positive square roots of eigenvalues of A*A. The 2-norm equals the largest singular value.
- F.5 Errors in solution: perturbations Ab in b cause errors Ax = A^{-1} Ab with ||Ax|| <= ||A^{-1}|| ||Ab||. Condition number kappa(A) = ||A|| ||A^{-1}|| amplifies relative errors. Solving Ax = b with finite precision can yield meaningless results when kappa(A) is large.

## Relevant Concepts

- [[concepts/vector-space]] — Basic linear-algebra prerequisite.
- [[concepts/condition-number]] — Error-amplification factor.
- [[concepts/matrix-norm]] — Generalized matrix size measure.
- [[concepts/singular-values]] — Spectral characterization of A*A.

## Source Metadata

- Source type: book appendix
- Book title: *Computer Methods for Circuit Analysis and Design*
- Chapter: Appendix F — Selected Mathematical Topics
- File path: `raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/26-appendix-f-selected-mathematical-topics.txt`
- Authors: Jiri Vlach, Kishore Singhal
