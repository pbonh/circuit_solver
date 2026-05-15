---
title: "Tableau Formulation"
type: concept
tags: [foundational, analog, dc, ac, transient, sparse-matrix, graph, netlist, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt"]
confidence: medium
---

## Definition

The tableau (sparse-tableau) formulation expresses the entire set of network equations — KCL, KVL, and branch constitutive relations — as a single large sparse algebraic-differential system. Variables include all node voltages and all branch currents and branch voltages.

## How It Works

Three blocks make up the tableau:
1. KCL: A * i_b = 0, where A is the reduced incidence matrix and i_b are branch currents.
2. KVL: v_b = A^T v_n, relating branch voltages to node voltages.
3. Branch constitutive equations: f(v_b, i_b, t, parameters) = 0.

The result is one tall sparse system, far larger than nodal or MNA, but the sparsity is extremely regular. With a good sparse solver and ordering, the cost is competitive.

## Key Parameters

- Number of branches *b* (sets a major dimension).
- Number of nodes *n*.
- Sparsity pattern: extremely sparse, structurally regular.
- For nonlinear circuits, the branch block is the source of nonlinearity.

## When To Use

- Educational / theoretical clarity — tableau is the most direct expression of the network equations.
- Implementations that benefit from a single, uniform treatment of all branches.
- Foundations for two-graph formulations and other general approaches discussed in Vlach & Singhal Chapter 4.

## Risks & Pitfalls

- Much larger matrix than nodal or MNA, so demands a strong sparse solver to be competitive.
- Less common in production simulators than MNA.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/sparse-matrix-methods]]
- [[concepts/kirchhoff-current-law]]
- [[concepts/kirchhoff-voltage-law]]
- [[concepts/incidence-matrix]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
