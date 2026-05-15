---
title: "Conjugate Gradient Method"
type: concept
tags: [algorithm, linear-algebra, sparse-matrix, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt"]
confidence: high
---

## Definition

The Conjugate Gradient (CG) method (Hestenes and Stiefel, 1952) is an iterative algorithm for solving the linear system A x = b when A is symmetric positive definite (SPD). CG minimizes the quadratic form f(x) = (1/2) x^T A x − b^T x via successive line searches along A-conjugate directions.

## How It Works

Starting from an initial guess, CG computes the residual r_0 = b − A x_0 and direction p_0 = r_0. Each iteration k advances x_k = x_{k-1} + α_k p_{k-1}, with α_k chosen to minimize the quadratic in that direction; new direction p_k is chosen A-conjugate to all previous directions. CG converges in at most n iterations in exact arithmetic, often far fewer with good preconditioning.

## Key Parameters

- Matrix size and condition number.
- Preconditioner.
- Stopping tolerance on residual norm.

## When To Use

- Solving SPD linear systems too large for direct factorization.
- Inner solver in multigrid and domain-decomposition schemes.
- Graph Laplacian systems (after grounding).

## Risks & Pitfalls

- Fails or converges slowly for ill-conditioned matrices without preconditioning.
- Requires SPD; non-SPD systems need BICG, BICGSTAB, or GMRES.

## Related Concepts

- [[concepts/krylov-subspace-method]]
- [[concepts/preconditioning]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/sparse-matrix]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
