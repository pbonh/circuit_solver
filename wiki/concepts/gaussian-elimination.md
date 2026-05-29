---
title: Gaussian Elimination
type: claim
id: concepts/gaussian-elimination
tags:
- foundational
- numerical
- well-established
- math
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Gaussian elimination is a direct algorithm for solving the n x n linear system Ax = b. By successively adding multiples of one equation to another, it reduces A to upper-triangular form; back substitution then recovers x.

## How It Works

At step k:
- Divide row k by a_kk (the pivot).
- For each i > k, subtract a_ik times the new row k from row i to zero a_ik.

The algorithm requires approximately n^3/3 operations (multiplications + additions) for the elimination, plus ~n^2/2 for the back substitution. Each step requires the pivot a_kk to be nonzero.

The "Gaussian form" of LU decomposition (subroutine LUG in Fig. 2.5.1) performs LU implicitly by leaving the L multipliers in place of the eliminated entries.

## Key Parameters

- Matrix size n.
- Pivoting strategy (none, partial, or full).
- Choice of pivot column k at each step.

## When To Use

- Small to moderate dense linear systems.
- Educational presentation of direct solvers.

## Risks & Pitfalls

- A zero pivot halts the algorithm; small pivots cause large round-off.
- Without pivoting, the method can be numerically unstable.
- Dense n^3/3 cost is prohibitive for circuit matrices of practical size; sparse LU is far preferred.

## Related Concepts

- [[concepts/lu-decomposition]]
- [[concepts/pivoting]]
- [[concepts/cramers-rule]]
- [[concepts/sparse-matrix-methods]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
