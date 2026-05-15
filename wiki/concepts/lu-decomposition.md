---
title: "LU Decomposition (Triangular Factorization)"
type: concept
tags: [foundational, numerical, well-established, sparse-matrix]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt"]
confidence: high
---

## Definition

LU decomposition factors a square matrix A as the product A = L U of a lower triangular matrix L and an upper unit-triangular matrix U (Crout/Doolittle conventions vary on which factor carries the unit diagonal). It is the workhorse direct solver for circuit-analysis linear systems.

## How It Works

Crout algorithm (Eqs. 2.5.9, 2.5.10 in Vlach & Singhal):
- l_ik = a_ik - sum_{m<k} l_im u_mk for i >= k (process column k of L).
- u_kj = (a_kj - sum_{m<k} l_km u_mj) / l_kk for j > k (process row k of U).

Forward substitution L z = b and back substitution U x = z each cost ~n^2/2 operations; total is ~n^3/3 + n^2 for one solve. Compared to plain Gaussian elimination, LU decomposition allows:
- Cheap multiple-RHS solves.
- Cheap transpose solves A^T x = e for sensitivity computation.
- determinant = product of l_kk.

In sparse form, operation counts and storage grow approximately linearly with n when good orderings are used.

## Key Parameters

- Matrix size n.
- Sparsity pattern and chosen ordering.
- Pivoting strategy.
- Symmetric variant: A = U^T D U cuts cost and storage almost in half.

## When To Use

- Any linear system to be solved repeatedly with different RHS vectors (typical of AC sweeps, transient steps, Newton iterations).
- Sensitivity computations requiring A^T solves.
- Circuit simulators of all kinds (DC, AC, transient).

## Risks & Pitfalls

- A zero pivot l_kk halts the factorization; some pivoting is mandatory for general matrices.
- For sparse circuit matrices, naive pivoting can drastically increase fill-in.
- Numerical instability if pivots are tiny without threshold pivoting.

## Related Concepts

- [[concepts/gaussian-elimination]]
- [[concepts/crout-algorithm]]
- [[concepts/forward-back-substitution]]
- [[concepts/sparse-matrix-methods]]
- [[concepts/pivoting]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
