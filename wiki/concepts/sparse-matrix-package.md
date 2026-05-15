---
title: "Sparse Matrix Package (Vlach & Singhal Appendix E)"
type: concept
tags: [sparse-matrix, foundational, software, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/01-preface.txt", "raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt"]
confidence: medium
---

## Definition

A sparse-matrix solver package supplied as Appendix E of *Computer Methods for Circuit Analysis and Design*. It is designed for structurally symmetric matrices and assumes that pivoting for numerical accuracy is not required. Parts of the code execute orders of magnitude faster than contemporary circuit-analysis-package implementations.

## How It Works

- Stores only nonzero entries of the network matrix.
- Performs LU factorization and forward/back substitution using a sparse data structure tuned to structurally symmetric circuit matrices.
- Pivots only for structural reasons (zero-on-diagonal), not for numerical reasons.
- Companion to the FORTRAN network analysis program in Appendix D.

## Key Parameters

- Matrix structural sparsity pattern (assumed structurally symmetric).
- Pivoting strategy (none required for accuracy — strong assumption).
- Storage scheme (typically linked-list or row/column index based, as is conventional in circuit sparse codes).

## When To Use

- Linear system solves arising from nodal, modified nodal, or tableau formulations where the matrix is sparse and structurally symmetric.
- Educational demonstration of sparse techniques inside a real circuit-analysis program.

## Risks & Pitfalls

- The no-numerical-pivoting assumption can fail on ill-conditioned matrices or on certain modified-nodal stamps that introduce indefinite diagonals; not safe for arbitrary linear systems.
- Less sophisticated than production sparse packages, though competitive on the specific class of circuit matrices it targets.

## Related Concepts

- [[concepts/sparse-matrix-methods]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/tableau-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-01-preface]]
- [[summaries/computer-methods-circuit-analysis-design-25-appendix-e-sparse-matrix-solver]]
