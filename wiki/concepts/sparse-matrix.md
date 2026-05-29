---
title: Sparse Matrix
type: claim
id: claim-sparse-matrix
tags:
- sparse-matrix
- well-established
- linear-algebra
- vlsi
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.85
---

## Definition

A sparse matrix is a matrix in which most entries are zero, allowing specialized storage formats (CSR, CSC, COO, LIL) and algorithms that exploit the sparsity pattern for dramatically reduced memory and computation compared with dense methods.

## How It Works

Sparse matrices arise naturally in VLSI circuit analysis because each node typically connects to only a handful of neighbors, so the conductance matrix has O(|V|+|E|) nonzeros rather than |V|^2. Direct solvers (sparse LU, sparse Cholesky) use fill-reducing orderings (AMD, METIS-based nested dissection). Iterative solvers (CG, GMRES) only need the matrix-vector product, which is O(nnz).

## Key Parameters

- Number of nonzeros (nnz).
- Bandwidth or fill-in after factorization.
- Symmetry / positive-definiteness.
- Conditioning.

## When To Use

- Circuit simulation matrices.
- Discretized PDEs (finite element/difference).
- Graph Laplacians.
- Machine learning feature matrices and recommender systems.

## Risks & Pitfalls

- Storage format choice critically affects performance for different access patterns.
- Fill-in during direct factorization can negate sparsity benefits without ordering.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/laplacian-matrix]]
- [[concepts/preconditioning]]
- [[concepts/domain-decomposition]]

## Related Decisions

- [[decisions/0002-hybrid-sparse-direct-solver-backend-russell-faer|ADR-0002]] — Chooses pure-Rust `russell` and `faer` backends over FFI-based sparse solvers.

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
