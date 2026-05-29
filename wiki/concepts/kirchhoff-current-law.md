---
title: Kirchhoff Current Law (KCL)
type: claim
id: concepts/kirchhoff-current-law
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
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Kirchhoff's current law states that the algebraic sum of currents leaving (or entering) any node of a network is zero. It expresses conservation of electric charge at each node.

## How It Works

For each node i, sum_{branches j incident to i} I_{ij} = 0, where the sign of each current depends on its reference direction relative to the node. KCL is the basis of nodal analysis: applied at each ungrounded node, it produces n equations in n unknown node voltages (for an (n+1)-node network with one node grounded as reference).

## Key Parameters

- Number of nodes n (one designated as ground reference).
- Branch incidence relations encoded in the reduced incidence matrix A.
- Sign convention: currents flowing away from a node have positive sign in the standard convention.

## When To Use

- Forming nodal-admittance equations in any LTI circuit.
- Modified nodal analysis (extension with extra branch currents).
- Tableau formulation (one of three constituent equation blocks).

## Risks & Pitfalls

- Mistakes in reference direction lead to sign errors in the matrix.
- For nodes coincident with ideal voltage sources, plain KCL leaves the node current indeterminate, motivating MNA.

## Related Concepts

- [[concepts/kirchhoff-voltage-law]]
- [[concepts/nodal-analysis]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/tableau-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
