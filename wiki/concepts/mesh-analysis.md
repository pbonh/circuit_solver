---
title: "Mesh Analysis"
type: concept
tags: [foundational, analog, well-established, graph]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt"]
confidence: high
---

## Definition

Mesh analysis writes KVL around each independent mesh (window) of a planar network, producing a system ZI = E in which I is the vector of circulating mesh currents and Z the mesh-impedance matrix. There are b - n independent meshes, where b is the number of branches and n the number of ungrounded nodes.

## How It Works

Inspection rules dual to nodal analysis:
- Z_ii = sum of impedances around mesh i.
- Z_ij = - sum of impedances common to meshes i and j (assuming meshes have consistent orientation).
- E_i = sum of voltage rises in mesh i from independent sources.

A symbolic stamp z * ((e_j - e_k)(e_j - e_k)^T) inserts a branch impedance between two meshes.

## Key Parameters

- Number of independent meshes b - n.
- Mesh orientation (typically all clockwise).
- Restriction to planar networks.

## When To Use

- Hand calculations on simple planar networks.
- Pedagogical illustration of duality with nodal analysis.

## Risks & Pitfalls

- Inapplicable to nonplanar networks (e.g., K_5, K_{3,3} interconnects).
- Planarity testing and automatic mesh extraction algorithms are complex; mesh analysis is rarely used in CAD software.

## Related Concepts

- [[concepts/kirchhoff-voltage-law]]
- [[concepts/nodal-analysis]]
- [[concepts/tableau-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
