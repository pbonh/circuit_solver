---
title: "Symbolic Factorization"
type: concept
tags: [sparse-matrix, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt"]
confidence: high
---

## Definition

Symbolic factorization is a preprocessing step that determines the nonzero pattern of the LU factors of a sparse matrix without computing numerical values. It is run once per matrix structure and is amortized over many numeric factorizations.

## How It Works

Given the chosen ordering, symbolic factorization simulates the elimination process on the nonzero pattern, recording all fill-ins. Output is the nonzero pattern of L and U, which is used to:
- Allocate storage for the numeric factors.
- Pre-generate indices for interpretive sparse code.
- Optionally compile machine code that performs the numeric factorization directly without runtime indexing.

Similarly, symbolic solution analyzes the sparsity of the RHS and the desired output set to determine which entries of z and x must be computed during forward and back substitution.

## Key Parameters

- Permutation (ordering) used.
- Number of fill-ins produced — determines factor storage.
- Output: ordered lists of (i, j) positions where L_ij or U_ij is nonzero.

## When To Use

- Whenever the matrix structure is fixed and many numerical factorizations are required (typical in circuit simulation: AC sweep, transient time stepping, Newton iteration).
- As input to interpretive or compiled sparse-factorization code.

## Risks & Pitfalls

- Predicted fill-in may underestimate the true fill if numerical pivoting changes the actual pivot sequence (threshold partial pivoting can introduce dynamic fills).
- Mismatch between predicted and actual sparsity patterns requires fallback to dynamic storage.

## Related Concepts

- [[concepts/sparse-matrix-methods]]
- [[concepts/reordering]]
- [[concepts/fill-in]]
- [[concepts/forward-back-substitution]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-25-appendix-e-sparse-matrix-solver]]
