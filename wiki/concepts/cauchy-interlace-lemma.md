---
title: Cauchy's Interlace Lemma
type: claim
id: claim-cauchy-interlace-lemma
tags:
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt
confidence:
  base: 0.85
---

## Definition

Let A be a real symmetric n × n matrix and B an m × m principal submatrix (rows and columns indexed by the same subset S ⊂ [n]). Let λ_1 ≥ … ≥ λ_n be the eigenvalues of A and μ_1 ≥ … ≥ μ_m those of B. Then for i ∈ [m]:

  λ_i ≥ μ_i ≥ λ_{i + n - m}.

In particular, when m = n - 1, λ_1 ≥ μ_1 ≥ λ_2 ≥ … ≥ μ_{n-1} ≥ λ_n.

## How It Works

The eigenvalues of a principal submatrix interlace those of the parent. Geometric intuition: the submatrix optimizes the Rayleigh quotient over a smaller subspace, so its eigenvalues are sandwiched.

Application in the Sensitivity Conjecture (Huang 2019): for the {0, -1, +1}-matrix A_n with eigenvalues ±√n of equal multiplicity, any principal submatrix of size > 2^(n-1) has largest eigenvalue ≥ √n by interlacing. This bounds Δ(H) for induced subgraphs H of the hypercube Q_n.

## Key Parameters

- m = size of submatrix.
- The interlacing gap depends on n - m.

## When To Use

- Spectral graph theory (proving lower bounds on eigenvalues via subgraph analysis).
- Quadratic programming relaxations.
- Boolean function complexity.

## Risks & Pitfalls

- Requires real-symmetric matrices.
- "Principal submatrix" means same row/column set; non-principal submatrices need different tools.

## Related Concepts

- [[concepts/sensitivity]]
- [[concepts/hypercube]]
- [[concepts/adjacency-matrix]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
