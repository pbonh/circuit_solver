---
title: Fill-In
type: claim
id: claim-fill-in
tags:
- sparse-matrix
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.85
---

## Definition

A fill-in is a matrix entry that was zero in the original sparse matrix A but becomes nonzero during LU factorization. In the elimination-graph view, a fill-in corresponds to a new edge between two previously non-adjacent neighbors of an eliminated vertex.

## How It Works

When vertex k is eliminated (pivot step k), any two distinct neighbors i, j of k that were not previously connected acquire an edge — corresponding to nonzeros at positions (i,j) and (j,i) of L+U. These positions had to store zero in A but now require storage and computation in L or U.

Fill-ins increase memory usage and operation count. The order of elimination strongly affects the total fill-in: minimum-degree, minimum-fill-in, and nested-dissection orderings all aim to keep fill-in small.

## Key Parameters

- Total fill-in count.
- Per-pivot fill-in count (used by minimum-fill-in algorithms).
- Sparsity pattern of L+U after factorization (the "factor pattern").

## When To Use

- As a target metric during sparse-ordering algorithms.
- For sizing memory in static-storage sparse solvers (symbolic factorization computes the worst-case fill pattern).

## Risks & Pitfalls

- A poor ordering can cause catastrophic fill-in (e.g., the "arrowhead" matrix becomes dense unless reordered).
- Numerical thresholding can cause unexpected fill-ins not predicted by symbolic factorization.

## Related Concepts

- [[concepts/reordering]]
- [[concepts/minimum-degree-ordering]]
- [[concepts/minimum-fill-in]]
- [[concepts/symbolic-factorization]]
- [[concepts/elimination-graph]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
