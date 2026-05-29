---
title: Kirchhoff Voltage Law (KVL)
type: claim
id: claim-kirchhoff-voltage-law
tags:
- foundational
- analog
- well-established
- graph
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.85
---

## Definition

Kirchhoff's voltage law states that the sum of voltage drops around any closed loop of a network is zero. It expresses conservation of energy in a quasi-static circuit.

## How It Works

For each independent loop (mesh in a planar network), sum_{branches in the loop} V_b = 0 with signs determined by the loop traversal direction. KVL is the basis of mesh analysis. The relation v_b = A^T v_n linking branch voltages to node voltages is the matrix form of KVL using the reduced incidence matrix A.

## Key Parameters

- Number of independent loops = b - n where b = branches, n = ungrounded nodes.
- Loop choice (meshes for planar networks; spanning-tree-based fundamental loops in general).

## When To Use

- Mesh analysis of planar networks (mostly pedagogical or hand calculation).
- Tableau formulation, where KVL is one of three equation blocks.
- Verification of computed solutions.

## Risks & Pitfalls

- Mesh formulation is restricted to planar networks; nonplanar ones require generalized loop analysis.
- Loop orientation choices must be consistent.

## Related Concepts

- [[concepts/kirchhoff-current-law]]
- [[concepts/mesh-analysis]]
- [[concepts/tableau-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
