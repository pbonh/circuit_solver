---
title: Crout Algorithm (LU Variant)
type: claim
id: claim-crout-algorithm
tags:
- foundational
- numerical
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.65
---

## Definition

The Crout algorithm is a specific variant of LU decomposition (A = L U, U unit-upper-triangular) that processes columns of L and rows of U alternately. It is implemented in the CROUT FORTRAN subroutine in Fig. 2.5.1 of Vlach & Singhal Chapter 2.

## How It Works

For each k = 1, 2, ..., n:
- Compute column k of L: l_ik = a_ik - sum_{m<k} l_im u_mk for i = k, ..., n.
- Compute row k of U: u_kj = (a_kj - sum_{m<k} l_km u_mj) / l_kk for j = k+1, ..., n.

The Crout (and equivalent row-by-row LUROW) form has a tighter innermost loop than the Gaussian (LUG) form, requiring one fewer array reference per iteration. This makes it faster than LUG, and amenable to assembly-level optimization with high-precision accumulation of the running sum T.

## Key Parameters

- Matrix size n.
- In-place overwriting of A (L below the diagonal, U above, unit diagonal of U not stored).

## When To Use

- Dense or sparse LU solves where the inner loop dominates runtime.
- When high-precision accumulation in the inner-product is desired.

## Risks & Pitfalls

- Requires nonzero diagonal pivots; pivoting must be added for stability.
- Slightly more complex bookkeeping than the Gaussian elimination form for novices.

## Related Concepts

- [[concepts/lu-decomposition]]
- [[concepts/gaussian-elimination]]
- [[concepts/forward-back-substitution]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
