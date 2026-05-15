---
title: "Algebraic Multigrid (AMG)"
type: concept
tags: [algorithm, linear-algebra, sparse-matrix, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt"]
confidence: medium
---

## Definition

Algebraic Multigrid (AMG) generalizes geometric multigrid to systems without underlying spatial regularity. Coarse grids and inter-grid transfer operators are constructed purely from the structure of the matrix A, not from a geometric mesh.

## How It Works

AMG selects strong connections in the matrix graph (based on a threshold on |a_{ij}| / max |a_{ik}|), partitions nodes into coarse (C) and fine (F) sets, and constructs prolongation by interpolating each F-node from its strongly-connected C-neighbors. Restriction is typically the transpose of prolongation. AMG works well on symmetric M-matrices (e.g., Laplacians) and is the workhorse of fast circuit-simulation tools like PowerRush, which achieves linear complexity on 38-million-node circuits.

## Key Parameters

- Strength-of-connection threshold.
- Coarsening algorithm (Ruge-Stüben, smoothed aggregation).
- Number of levels.

## When To Use

- Power-grid analysis on irregular layouts.
- Graph Laplacian systems.
- As preconditioner inside Krylov methods on industrial-scale matrices.

## Risks & Pitfalls

- Setup cost can be substantial.
- Performance depends on matrix structure; degraded on highly anisotropic or asymmetric problems.

## Related Concepts

- [[concepts/multigrid-method]]
- [[concepts/preconditioning]]
- [[concepts/sparse-matrix]]
- [[concepts/laplacian-matrix]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
