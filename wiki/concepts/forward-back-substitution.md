---
title: Forward and Back Substitution
type: claim
id: claim-forward-back-substitution
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

After LU decomposition A = LU, the system A x = b is solved in two stages:
- Forward substitution: solve L z = b for z.
- Back substitution: solve U x = z for x.

Each stage costs ~n^2/2 operations in the dense case; in sparse implementations, the cost is much less and depends on the sparsity pattern of L and U.

## How It Works

Forward substitution (with L lower-triangular, L_ii != 0):
- z_1 = b_1 / l_11.
- z_i = (b_i - sum_{j<i} l_ij z_j) / l_ii for i = 2, ..., n.

Back substitution (with U unit upper triangular):
- x_n = z_n.
- x_i = z_i - sum_{j>i} u_ij x_j for i = n-1, ..., 1.

In sparse implementations: process the forward sub by columns and the back sub by rows; only positions whose z_i / x_i become nonzero need be computed. A symbolic substitution pass determines the pattern once for repeated solves.

## Key Parameters

- Sparsity of b (forward sub).
- Required output positions in x (back sub can stop early).
- Triangular structure of L and U.

## When To Use

- Every time a new RHS vector is solved with already-factored A — multiple-RHS solves (AC sweeps, transient steps, sensitivities).
- Symbolic precomputation when the structure of b and the set of desired outputs are fixed.

## Risks & Pitfalls

- Zero diagonal in L stops forward substitution.
- For sparse data structures, indexing overhead may dominate arithmetic for very small matrices.

## Related Concepts

- [[concepts/lu-decomposition]]
- [[concepts/sparse-matrix-methods]]
- [[concepts/symbolic-factorization]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
