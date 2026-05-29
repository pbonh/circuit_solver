---
title: Orthogonality Relations (B Q^T = 0)
type: claim
id: concepts/orthogonality-relations
tags:
- foundational
- graph
- analog
- well-established
- math
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

For an oriented graph with chosen tree, the basic cutset matrix Q and basic loopset matrix B (with columns ordered consistently) satisfy the orthogonality relation B Q^T = 0 (equivalently Q B^T = 0). This is a fundamental topological identity, related to (and a consequence of) Tellegen's theorem.

## How It Works

Partitioning Q = [1 | Q_c] and B = [B_t | 1], the relation B Q^T = 0 gives B_t + Q_c^T = 0, hence B_t = -Q_c^T. Therefore either matrix can be recovered from the other:
- B = [-Q_c^T | 1].
- Q = [1 | -B_t^T].

This means only one of Q or B needs to be stored or computed in software.

## Key Parameters

- Sign of orientation: must be consistent in computing both matrices.
- Tree choice: orthogonality holds for any tree.

## When To Use

- Theoretical proofs of equivalence between nodal and loop formulations.
- Reducing storage in topological algorithms (compute Q, derive B by transposition).
- Deriving Tellegen's theorem and the adjoint network.

## Risks & Pitfalls

- Sign conventions must be uniform across both matrices.
- Care required when augmented matrices (with sources) are partitioned.

## Related Concepts

- [[concepts/cutset-matrix]]
- [[concepts/loopset-matrix]]
- [[concepts/tellegen-theorem]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
