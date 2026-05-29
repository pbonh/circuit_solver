---
title: Active Network Analysis (Hand Method)
type: claim
id: claim-active-network-analysis
tags:
- analog
- well-established
- pedagogy
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
confidence:
  base: 0.65
---

## Definition

A set of inspection rules for hand-analysis of active networks containing voltage sources, VVTs, and ideal operational amplifiers. By exploiting the constraints those ideal elements impose, the size of the matrix system can be reduced to the minimum number of unknown node voltages.

## How It Works

Rules (Section 4.5 of Vlach & Singhal):
1. Insert known voltages (from independent voltage sources) directly into the diagram.
2. Denote resistors by conductances G_i = 1/R_i.
3. Write KCL only at nodes not connected to any voltage source (independent or dependent).
4. For an ideal OPAMP, the two input terminals are at equal potential (virtual short); if one is grounded, the other is at zero. Do NOT write KCL at the OPAMP's output node.

After applying these rules, only a small number of KCL equations remain, often producing 2x2 or 3x3 systems for circuits that would yield 6x6 or 7x7 MNA matrices.

## Key Parameters

- Number of voltage sources (each removes a node from the KCL set).
- Number of OPAMPs (each adds a virtual-short equality and removes one KCL).
- Network topology.

## When To Use

- Hand analysis of small active filters.
- Verification of computer-generated MNA results on small examples.
- Building intuition for what variables really matter in an active network.

## Risks & Pitfalls

- Hard to systematize for general element types — primarily a hand-calculation technique.
- Does not give branch currents directly (must be back-computed).
- Less suited to computer implementation; the two-graph MNA is the systematic version.

## Related Concepts

- [[concepts/operational-amplifier]]
- [[concepts/two-graph-modified-nodal]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
