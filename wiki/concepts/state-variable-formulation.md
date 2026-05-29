---
title: State Variable Formulation
type: claim
id: claim-state-variable-formulation
tags:
- foundational
- analog
- transient
- well-established
- graph
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt
confidence:
  base: 0.85
---

## Definition

The state-variable formulation expresses a network's dynamic behavior as a first-order ODE system sX = AX + BW with output equation Y = CX + DW. The state vector X comprises the network's energy-storage variables — typically tree-capacitor voltages and chord-inductor currents.

## How It Works

A normal tree is selected (voltage sources first, then as many capacitors as possible, then resistors, completing with inductors as needed). The Q matrix of the network is written. KCL and KVL combine into:
- [i_t ; v_c] = [-Q_c ; Q_c^T] [v_t ; i_c]

with the partition matching tree (capacitor + voltage source) and cotree (inductor + current source). Branch constitutive equations are substituted; algebraic variables (resistor currents/voltages) are eliminated; the result is sM X = A X + B W, where M may be singular. Further processing reaches the normal form sX = AX + BW.

## Key Parameters

- Choice of normal tree.
- Network must permit a tree containing all capacitors (otherwise excess elements exist).
- State dimension n (equal to the number of energy-storage elements minus the number of degenerate constraints).

## When To Use

- Theoretical analysis of dynamical-system properties: stability, controllability, observability.
- Educational illustration of how circuit dynamics map to standard control-theory form.

## Risks & Pitfalls

- Elimination of algebraic equations is laborious and error-prone.
- Resulting matrices are often dense.
- Most modern circuit simulators use algebraic-differential formulations (modified nodal, tableau) directly, since they handle the algebraic constraints internally during numerical integration.

## Related Concepts

- [[concepts/normal-tree]]
- [[concepts/cutset-matrix]]
- [[concepts/algebraic-differential-equations]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/tableau-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
