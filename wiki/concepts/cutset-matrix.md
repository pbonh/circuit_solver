---
title: Cutset Matrix (Q)
type: claim
id: concepts/cutset-matrix
tags:
- foundational
- graph
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The basic cutset matrix Q is an n x b matrix encoding the n basic cuts of an (n+1)-node oriented graph with respect to a chosen tree. Row i corresponds to the cut associated with twig i; column j is the j-th edge. Entry Q_ij is +1 if edge j is in the cut with the same orientation as twig i, -1 if opposite, and 0 if edge j is not in the cut.

## How It Works

KCL takes the matrix form Q i = 0. Partitioning Q with twigs first: Q = [1 | Q_c]. The chord currents i_c are independent and the twig currents are i_t = -Q_c i_c. Together: i = B^T i_c, where B = [-Q_c^T | 1] is the loopset matrix.

The orthogonality relation B Q^T = 0 holds between cutset and loopset matrices, so either suffices.

## Key Parameters

- Tree choice (Q depends on tree).
- Rank of Q is n.
- Q_c block has size n x (b - n).
- With recommended twigs-first numbering, the left block of Q is the identity.

## When To Use

- Deriving topological nodal formulation Y V = J via partitioned A_a I = 0.
- Constructing state-variable equations.
- Theoretical proofs about network structure.

## Risks & Pitfalls

- A different tree gives a different Q; care needed for cross-comparison.
- Q is generally dense even when the resulting admittance matrix is sparse.

## Related Concepts

- [[concepts/tree-cotree]]
- [[concepts/loopset-matrix]]
- [[concepts/incidence-matrix]]
- [[concepts/orthogonality-relations]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
