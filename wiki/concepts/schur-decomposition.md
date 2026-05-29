---
title: Schur Decomposition (Matrix Partitioning)
type: claim
id: claim-schur-decomposition
tags:
- linear-algebra
- hierarchical
- sparse-matrix
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/12-8-hierarchical-analysis-methods.txt
confidence:
  base: 0.65
---

## Definition

For a partitioned matrix `M = [[A11, A12], [A21, A22]]` with `A11` invertible, the Schur complement of `A11` is `S = A22 - A21 A11^{-1} A12`. Schur decomposition uses this to reduce a block linear system to one of smaller size on the un-eliminated variables.

## How It Works

Eliminate `x1` from `A11 x1 + A12 x2 = 0` (zero RHS in subblock 1) by `x1 = -A11^{-1} A12 x2`, then substitute into the second block equation to obtain `(A22 - A21 A11^{-1} A12) x2 = b2`. In symbolic circuit analysis, `Y2 = A21 A11^{-1} A12` is built one entry at a time using a DDD per cofactor; the result is the multi-port admittance stamp of the eliminated subcircuit.

## Key Parameters

- Choice of internal variable block (`x1`); ideally these are interior nodes with no output coupling.
- Sparsity-preserving partition order.

## When To Use

- Block-by-block hierarchical symbolic analysis.
- Symbolic model-order reduction by interior-node elimination.

## Risks & Pitfalls

- `A11^{-1}` involves divisions, which propagate into the symbolic expressions.
- Fill-in in the Schur complement can be substantial.
- Cancellation re-emerges in the eliminated symbolic expressions.

## Related Concepts

- [[concepts/symbolic-stamp]]
- [[concepts/hierarchical-symbolic-analysis]]
- [[concepts/gaussian-elimination]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
