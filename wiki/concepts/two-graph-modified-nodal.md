---
title: Two-Graph Modified Nodal Formulation
type: claim
id: concepts/two-graph-modified-nodal
tags:
- foundational
- graph
- analog
- sparse-matrix
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The two-graph modified nodal formulation combines the redundancy-elimination of separate I- and V-graphs with the compact appending of additional rows/columns characteristic of MNA. It produces the smallest systematic formulation among those compared in Chapter 4 — e.g., 4x4 for a second-order active filter and 3x3 for a generalized impedance converter (matching the by-hand active-nodal solution).

## How It Works

For each admittance y:
- Find the I-graph edge (j_i → j_i') and the V-graph edge (j_v → j_v').
- Stamp +y at (j_i, j_v) and (j_i', j_v'); -y at (j_i, j_v') and (j_i', j_v).

For non-admittance elements (voltage source, gyrator, transformer, etc.), append one or two extra rows for the constitutive equations and matching extra columns for the introduced branch currents, following the patterns in Fig. 4.8.2 of Vlach & Singhal.

OPAMPs require no extra rows because the nullor pair imposes V_v = 0 and I_i = 0, fully handled by graph collapse/deletion.

## Key Parameters

- I-graph and V-graph adjacency information (e.g., (from, to) tables of the type in Eqs. 4.7.1 and 4.7.2).
- Required outputs (whether to retain certain currents/voltages affects collapse/deletion).
- Stamping rules per element type.

## When To Use

- Compact analog and switched-capacitor circuit simulators.
- Manual analysis of active networks where minimal matrix sizes are desired.
- Algorithms requiring symbolic analysis where matrix size dominates cost.

## Risks & Pitfalls

- Implementation in software requires careful management of the two graphs.
- Errors in collapse rules produce subtly wrong matrices.
- Less direct than single-graph MNA for adding new element types.

## Related Concepts

- [[concepts/two-graph-formulation]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/branch-stamping]]
- [[concepts/active-network-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
- [[summaries/computer-methods-circuit-analysis-design-17-chapter-14-digital-and-switched-capacitor-networks]]
- [[summaries/computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis]]
