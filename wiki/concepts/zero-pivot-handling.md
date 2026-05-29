---
title: Zero Pivot Handling in Sparse Factorization
type: claim
id: concepts/zero-pivot-handling
tags:
- sparse-matrix
- numerical
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/11-chapter-8-large-change-sensitivity-and-related-topics.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

When sparse LU factorization encounters a zero (or very small) pivot at step i, the standard approach is to repivot — but in sparse codes with a fixed ordering, repivoting is expensive. Vlach & Singhal's Section 8.4 technique adds unit entries e_i e_i^T to the matrix at each zero-pivot step, factor the modified matrix, then recover the original solution via a rank-modification correction.

## How It Works

If pivots are zero at steps i_1, ..., i_m, the modified matrix A_m = A + P P^T where P = [e_{i_1}, ..., e_{i_m}]. Factor A_m normally. Then solve the original A x = b via:
1. F_hat = P^T A_m^{-1} P (m x m), b_hat = P^T A_m^{-1} b.
2. Solve (-I + F_hat) z = b_hat.
3. Recover x: A_m x = b - P z.

If (-I + F_hat) is singular, A itself is singular.

## Key Parameters

- Number of zero pivots m.
- Cost of preprocessing: m forward/back substitutions on A_m.
- Per-query cost: m x m solve and one forward/back substitution.

## When To Use

- Sparse direct solvers when the chosen ordering produces occasional zero pivots at certain frequencies (e.g., at poles in symbolic analysis).
- As an alternative to full repivoting in fixed-pattern sparse codes.

## Risks & Pitfalls

- Numerical stability: small but nonzero pivots may also need this treatment with threshold pivoting.
- Cost scales with the number of zero pivots; many of them defeat the purpose.

## Related Concepts

- [[concepts/large-change-sensitivity]]
- [[concepts/low-rank-matrix-update]]
- [[concepts/sparse-matrix-methods]]
- [[concepts/pivoting]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-11-chapter-8-large-change-sensitivity-and-related-topics]]
