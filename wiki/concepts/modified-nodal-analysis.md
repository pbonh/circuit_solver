---
title: Modified Nodal Analysis (MNA)
type: claim
id: claim-modified-nodal-analysis
tags:
- analog
- sparse-matrix
- foundational
- netlist
- dc
- ac
- transient
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/AdvancedSymbolicAnalysisForVLSISystems/_txt/04-1-introduction.txt
confidence:
  base: 0.85
---

## Definition

Modified Nodal Analysis is the standard matrix formulation used by SPICE-class simulators, in which node-voltage KCL equations are augmented with branch-current variables and constitutive equations for elements that cannot be expressed in admittance form (voltage sources, inductors, controlled sources, etc.).

## How It Works

For a network with `n` nodes and `m` extra branches, MNA produces a system `[G + sC] x = b` where `x` stacks node voltages and extra branch currents. Element stamps are added to the matrix; the structure is sparse and indefinite (not necessarily symmetric positive-definite). Symbolic analysis operates on the same matrix but keeps element values as variables instead of numbers.

## Key Parameters

- Element stamp catalog (R, L, C, V, controlled sources, nullors, etc.).
- Numbering / variable order (affects fill-in for LU and BDD/DDD size for symbolic).
- Choice between full MNA and reduced/compressed variants.

## When To Use

- Any SPICE-style numerical AC/DC/transient analysis.
- As the substrate for DDD-style symbolic expansion of circuit determinants.

## Risks & Pitfalls

- MNA matrices are not symmetric; some solvers require unsymmetric routines.
- Naive symbolic determinant expansion of MNA is factorial in size — compact graph representations are essential.

## Related Concepts

- [[concepts/determinant-decision-diagram]]
- [[concepts/symbolic-analysis]]
- [[concepts/two-graph-method]]
- [[concepts/nullor]]

## Sources

- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-04-1-introduction]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-05-2-symbolic-analysis-techniques-in-a-nutshell]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-08-4-determinant-decision-diagrams]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-09-5-ddd-implementation]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-10-6-generalized-two-graph-theory]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-12-8-hierarchical-analysis-methods]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-13-9-symbolic-nodal-analysis-of-analog-circuits-using-nullors]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-16-11-performance-bound-analysis-of-analog-circuits-considering-process-variations]]
- [[summaries/advanced-symbolic-analysis-for-vlsi-systems-17-12-statistical-parallel-monte-carlo-analysis-on-gpus]]
- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-07-chapter-4-general-formulation-methods]]
- [[summaries/computer-methods-circuit-analysis-design-09-chapter-6-computer-generation-of-sensitivities]]
- [[summaries/computer-methods-circuit-analysis-design-24-appendix-d-program-for-network-analysis]]
- [[summaries/graphs-in-vlsi-04-1-introduction]]
- [[summaries/graphs-in-vlsi-06-3-graphs-in-vlsi-circuits-and-systems]]
- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
- [[summaries/graphs-in-vlsi-17-b-uniqueness-based-on-boundary-conditions]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
