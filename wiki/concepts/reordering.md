---
title: "Reordering (Sparse Matrix Permutation)"
type: concept
tags: [sparse-matrix, foundational, numerical, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt"]
confidence: high
---

## Definition

Reordering is the choice of row and column permutations of a sparse matrix to minimize the number of fill-ins introduced during LU factorization. In circuit simulation, the reordering decision dominates the eventual cost of repeated factor-and-solve cycles.

## How It Works

For structurally symmetric matrices (as in nodal circuit analysis), reordering is constrained to symmetric row/column permutations so that L and U^T share the same nonzero pattern. Four strategies were compared in Vlach & Singhal Chapter 2 on a 17-node phase splitter:
1. No reordering: 68 fills, 377 ops.
2. Static fewest-nonzeros: 32 fills, 209 ops.
3. Minimum-degree: 14 fills, 147 ops.
4. Minimum local fill-in: 12 fills, 141 ops.

Reordering is typically performed once for a given matrix structure; the same permutation is reused for every numeric refactorization.

## Key Parameters

- Algorithm: no-reorder, fewest-nonzeros, minimum-degree, minimum-fill-in, nested dissection.
- Symmetric vs. asymmetric pivoting allowed.
- Numerical thresholds (when reordering is combined with threshold pivoting).

## When To Use

- For any sparse direct solve of a fixed structure with many right-hand sides.
- During the preprocessing phase of a circuit-simulation run.

## Risks & Pitfalls

- Aggressive sparsity-only reordering can yield unstable factorizations on ill-conditioned matrices.
- Minimum-fill-in is more expensive to compute than minimum-degree; the additional ordering cost may exceed the savings in factorization.

## Related Concepts

- [[concepts/minimum-degree-ordering]]
- [[concepts/minimum-fill-in]]
- [[concepts/fill-in]]
- [[concepts/elimination-graph]]
- [[concepts/sparse-matrix-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-25-appendix-e-sparse-matrix-solver]]
