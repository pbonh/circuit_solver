---
title: "Hierarchical Matrix (H-Matrix)"
type: concept
tags: [algorithm, linear-algebra, sparse-matrix, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt"]
confidence: medium
---

## Definition

A hierarchical matrix (H-matrix), introduced by Hackbusch, is a data-sparse representation that recursively partitions a matrix into a cluster tree of blocks. Blocks identified as low-rank (typically off-diagonal) are stored in factored form A_{i,j} = M N^T with M, N ∈ R^{p×k}, k ≪ p; full-rank (diagonal) blocks are stored densely.

## How It Works

Top-down clustering subdivides rows and columns; for each pair of clusters, if the corresponding block has effective rank below a threshold, it is compressed via SVD or adaptive cross approximation. Recursion stops when blocks reach minimum size or full rank. Matrix-vector products and approximate LU decomposition then run in O(n (log n)^2) time, vs O(n^3) for direct dense LU.

## Key Parameters

- Minimum block size m_min.
- Rank threshold k_min.
- Admissibility condition (geometric or algebraic).

## When To Use

- Large dense matrices arising in boundary-element methods, electromagnetic simulation, partial-element-equivalent-circuit (PEEC) extraction.
- Power-supply layout analysis with low-rank far-field interaction.
- Three-dimensional IC thermal analysis with million-point granularity.

## Risks & Pitfalls

- Approximation error bounds depend on rank threshold and admissibility.
- Memory overhead for the cluster tree.

## Related Concepts

- [[concepts/sparse-matrix]]
- [[concepts/domain-decomposition]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
