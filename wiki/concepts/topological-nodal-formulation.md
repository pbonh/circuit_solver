---
title: Topological Nodal Formulation
type: claim
id: concepts/topological-nodal-formulation
tags:
- foundational
- graph
- analog
- sparse-matrix
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

The topological nodal formulation derives the nodal admittance equations YV_n = J_s using the augmented incidence matrix A_a of the oriented graph and the branch constitutive relations Y_b V_b = J_b. The result is identical to the inspection-based nodal formulation of Chapter 2 but is derived without reference to specific element types.

## How It Works

After converting all voltage sources to current sources, partition A_a = [A | A_s] (passive | source). KCL gives A I + A_s I_s = 0. KVL gives V = A^T V_n, equivalently V_b = A^T V_n for the passive block. Substituting Y_b V_b = I_p (passive branch currents) and using KCL:
- A Y_b A^T V_n = -A_s I_s.
- Y = A Y_b A^T (the nodal admittance matrix).
- J_s = -A_s I_s (the right-hand side).

The voltages across the current sources are recovered after V_n is known via V_s = A_s^T V_n.

## Key Parameters

- Dimensions: A is n x b_p (passive); A_s is n x m (sources).
- Y_b is a diagonal matrix of branch admittances.
- Number of source conversions required (Thevenin-Norton).

## When To Use

- Deriving the nodal matrix systematically for arbitrary networks.
- Software construction of Y from the netlist via A and Y_b separately.
- Proofs of properties such as Y = A Y_b A^T being symmetric when Y_b is symmetric.

## Risks & Pitfalls

- Storing A and Y_b separately uses more memory than direct element-by-element stamping; the formulation is primarily theoretical.
- The Thevenin-Norton source conversions must be tracked carefully.

## Related Concepts

- [[concepts/nodal-analysis]]
- [[concepts/incidence-matrix]]
- [[concepts/topological-loop-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
