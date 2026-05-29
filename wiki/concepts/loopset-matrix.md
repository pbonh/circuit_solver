---
title: Loopset Matrix (B)
type: claim
id: concepts/loopset-matrix
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

The basic loopset matrix B is a (b - n) x b matrix encoding the b - n basic loops of an oriented graph relative to a chosen tree. Each row corresponds to one chord; entry B_ij is +1 if edge j is traversed by loop i in the loop's direction, -1 if opposite, and 0 if edge j is not in loop i.

## How It Works

KVL takes the matrix form B v = 0. Partitioning B with twigs first and chords last: B = [B_t | 1]. The orthogonality relation B Q^T = 0 yields B_t = -Q_c^T, so B = [-Q_c^T | 1].

Independent loop variables are chord currents i_c. Edge currents reconstruct as i = B^T i_c. Topological loop formulation: Z I_c = E_s with Z = B Z_b B^T and E_s = -B_E E_p, where Z_b is the diagonal block of branch impedances.

## Key Parameters

- Tree choice (B depends on tree).
- Rank of B is b - n.
- B_t block has size (b - n) x n.

## When To Use

- Topological loop formulation of network equations, applicable to nonplanar networks (unlike mesh analysis).
- Theoretical proofs leveraging the duality between cuts and loops.
- Loop-based small-signal analysis.

## Risks & Pitfalls

- Z = B Z_b B^T is often dense, making loop formulation less attractive than nodal for large networks.
- Different tree choices produce different B matrices but equivalent solutions.

## Related Concepts

- [[concepts/tree-cotree]]
- [[concepts/cutset-matrix]]
- [[concepts/orthogonality-relations]]
- [[concepts/topological-loop-formulation]]
- [[concepts/mesh-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
