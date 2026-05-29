---
title: Krylov Subspace Method
type: claim
id: concepts/krylov-subspace-method
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

Krylov subspace methods are a family of iterative linear-system solvers that build approximate solutions in the Krylov subspace K_k(A, r_0) = span{r_0, A r_0, A^2 r_0, ..., A^{k-1} r_0}. Members include CG (SPD), BICG, BICGSTAB, MINRES, and GMRES (general systems).

## How It Works

Each iteration expands the Krylov subspace by one matrix-vector product. The method selects an approximate solution x_k ∈ x_0 + K_k(A, r_0) that minimizes some norm of the residual (least-squares for GMRES, A-norm for CG). Restarted variants (GMRES(m)) cap memory at m vectors.

## Key Parameters

- Subspace dimension / restart length.
- Preconditioner.
- Tolerance on residual norm.

## When To Use

- Any large sparse linear system where direct factorization is too costly.
- Black-box scenarios where only matrix-vector multiplication is available.

## Risks & Pitfalls

- Memory growth in non-restarted GMRES.
- Stagnation without preconditioning.
- Non-symmetric methods can be sensitive to round-off.

## Related Concepts

- [[concepts/conjugate-gradient-method]]
- [[concepts/preconditioning]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/sparse-matrix]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
