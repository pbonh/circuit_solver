---
title: Branch Stamping
type: claim
id: claim-branch-stamping
tags:
- foundational
- netlist
- analog
- well-established
- sparse-matrix
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/07-chapter-4-general-formulation-methods.txt
confidence:
  base: 0.85
---

## Definition

Branch stamping is the element-by-element procedure of contributing entries to the system matrices (G, C) and source vector (W) during MNA assembly. Each element type has a fixed stamp template that maps its terminals to specific matrix positions and signs.

## How It Works

For a two-terminal admittance y between nodes j and k, the stamp adds +y at (j,j) and (k,k); -y at (j,k) and (k,j). For an ideal voltage source between nodes j and k with value E, an extra row m+1 is appended with entries 1 at (m+1, j), -1 at (m+1, k), and an extra column with 1 at (j, m+1), -1 at (k, m+1); W(m+1) = E. Similar templates exist for all ideal elements (Fig. 4.4.1 of Vlach & Singhal).

The assembly is a single scan over the netlist, applying the appropriate template to each element. Frequency-independent terms go into G; capacitor/inductor s-coefficients go into C. The total system at frequency s is T = G + s C.

## Key Parameters

- Element type → stamp template.
- Element values → numerical entries.
- Node numbering — maps element pins to matrix indices.
- Grounded node → omit row/column (one node is reference).

## When To Use

- Implementing a SPICE-style simulator.
- Adding new element types to an existing simulator.
- Manual MNA matrix assembly during pedagogy.

## Risks & Pitfalls

- Sign errors in stamps are common and produce wrong answers silently.
- For two-port elements, four entries can be required; missing one causes singular matrices.
- Stamping switches (F=0 open, F=1 short) requires combined templates to enable runtime toggling.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/two-graph-modified-nodal]]
- [[concepts/nodal-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
