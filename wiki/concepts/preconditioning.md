---
title: Preconditioning
type: claim
id: concepts/preconditioning
tags:
- algorithm
- linear-algebra
- sparse-matrix
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Preconditioning transforms a linear system A x = b into an equivalent system M^{-1} A x = M^{-1} b (left preconditioning) where M ≈ A is an easily-invertible matrix. The transformed system has a better condition number, dramatically accelerating Krylov-subspace solver convergence.

## How It Works

Common preconditioners include: diagonal (Jacobi), incomplete LU (ILU) and incomplete Cholesky (IC), sparse approximate inverse (SPAI), multigrid (when used as a preconditioner inside Krylov), and physics-based block factorizations. The trade-off is between the cost of constructing and applying M^{-1} and the reduction in iteration count.

## Key Parameters

- Sparsity threshold for ILU/IC factorization.
- Drop tolerance.
- Spectral properties of A.

## When To Use

- Always: virtually no iterative solver on a real-world VLSI matrix is competitive without preconditioning.

## Risks & Pitfalls

- ILU breakdown on indefinite matrices.
- Memory cost of fill-in.
- Wrong preconditioner can slow convergence below unpreconditioned baseline.

## Related Concepts

- [[concepts/krylov-subspace-method]]
- [[concepts/conjugate-gradient-method]]
- [[concepts/multigrid-method]]
- [[concepts/sparse-matrix]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
