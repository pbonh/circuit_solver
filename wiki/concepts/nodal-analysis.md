---
title: Nodal Analysis
type: claim
id: concepts/nodal-analysis
tags:
- foundational
- analog
- dc
- ac
- netlist
- sparse-matrix
- well-established
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

Nodal analysis (the nodal-admittance formulation) writes KCL at each ungrounded node of a network containing R, L, C and current sources, producing a system YV = J with Y the nodal-admittance matrix, V the vector of node voltages, and J the right-hand side of source currents entering each node.

## How It Works

Two inspection rules build Y entry-by-entry:
- Y_ii = sum of admittances connected to node i.
- Y_ij = -(sum of admittances between nodes i and j).

For computer implementation, a single scan of the netlist stamps each two-terminal admittance y between nodes j and k via the rank-one update y * (e_j - e_k)(e_j - e_k)^T. Current sources contribute -J*(e_j - e_k) on the RHS.

When the network has only resistors, capacitors, inductors and current sources, Y is symmetric. With transducers (VCTs, transistors via hybrid-pi), Y is structurally symmetric but numerically asymmetric.

## Key Parameters

- Number of ungrounded nodes n.
- Number of branches b: at most n + 2b nonzeros in Y.
- Frequency or operating point at which the matrix is evaluated.

## When To Use

- AC, DC, and per-step transient analysis for circuits without ideal voltage sources or inductors (otherwise use MNA).
- Educational and small-circuit calculations.
- As the starting framework that MNA extends.

## Risks & Pitfalls

- Cannot handle ideal voltage sources or pure inductors without modification (motivates MNA).
- Indefinite nodal matrix (when reference node is not grounded) is singular; one node must be tied to ground.

## Related Concepts

- [[concepts/kirchhoff-current-law]]
- [[concepts/nodal-admittance-matrix]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/tableau-formulation]]
- [[concepts/sparse-matrix-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
