---
title: Pivoting (Partial and Full)
type: claim
id: claim-pivoting
tags:
- foundational
- numerical
- well-established
- sparse-matrix
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.85
---

## Definition

Pivoting is the row (and possibly column) reordering of a matrix performed during Gaussian elimination or LU factorization to avoid zero or small pivots. Two main strategies:
- Partial pivoting: at step k, swap row k with the row containing the largest |a_ik| in the current column k.
- Full pivoting: search the entire remaining submatrix for the largest |a_ij|; both row and column permutations are required, which changes the variable order.

## How It Works

After choosing a pivot, elimination/factorization proceeds normally. Partial pivoting yields a permutation matrix P such that P A = L U. Full pivoting yields P A Q = L U with both row and column permutations.

In sparse circuit matrices, the criterion shifts: rather than accuracy, the chief concern is fill-in minimization. Sparse codes often combine threshold partial pivoting (accept any pivot above a threshold fraction of the largest in its column) with sparsity-driven ordering.

## Key Parameters

- Strategy (partial, full, threshold, none).
- Threshold (for threshold pivoting, typically 0.01 to 0.1).
- For sparse matrices: trade-off between numerical accuracy and fill-in.

## When To Use

- Whenever the matrix is not strictly diagonally dominant or symmetric positive definite.
- Always recommended in general-purpose factorization routines.
- For sparse circuit matrices: combine numerical thresholding with minimum-degree / minimum-fill ordering.

## Risks & Pitfalls

- Without pivoting, a zero pivot halts the algorithm and a near-zero pivot amplifies round-off catastrophically.
- Full pivoting is expensive and changes variable ordering; partial pivoting is usually adequate.
- Aggressive sparsity-preserving pivot choices may compromise numerical accuracy on ill-conditioned matrices.

## Related Concepts

- [[concepts/lu-decomposition]]
- [[concepts/gaussian-elimination]]
- [[concepts/reordering]]
- [[concepts/sparse-matrix-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
