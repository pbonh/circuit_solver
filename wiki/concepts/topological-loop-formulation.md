---
title: Topological Loop Formulation
type: claim
id: claim-topological-loop-formulation
tags:
- foundational
- graph
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/06-chapter-3-graph-theoretic-formulation-of-network-equations.txt
confidence:
  base: 0.85
---

## Definition

The topological loop formulation derives loop equations Z I_c = E_s using the augmented loopset matrix B_a and branch constitutive relations Z_b I_b = V_b. Z = B Z_b B^T is the loop impedance matrix and E_s = -B_E E_p the source-voltage vector.

## How It Works

After converting all current sources to voltage sources, partition B_a = [B_E | B] (sources | passive). KVL on the passive block yields B V + B_E E = 0. Using the constitutive Z_b I = V and the current relation I = B^T I_c (with I_c the independent chord currents), the result is:
- B Z_b B^T I_c = -B_E E.
- Z = B Z_b B^T (the loop impedance matrix).
- E_s = -B_E E (the right-hand side).

Currents in voltage sources I_E are recovered after I_c is known via I_E = B_E^T I_c.

## Key Parameters

- B is (b - n) x b_p; B_E is (b - n) x m.
- Z_b is diagonal of branch impedances.
- Tree choice (different trees give different but equivalent Z).

## When To Use

- Loop-based analysis of nonplanar networks (unlike mesh analysis which requires planarity).
- When the number of loops b - n is small compared to the number of nodes.

## Risks & Pitfalls

- The loop matrix Z is often dense even when the network is sparse, making this formulation rarely competitive for large CAD problems.
- Source conversions (current → voltage) must be carefully tracked.

## Related Concepts

- [[concepts/loopset-matrix]]
- [[concepts/topological-nodal-formulation]]
- [[concepts/mesh-analysis]]
- [[concepts/tree-cotree]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-06-chapter-3-graph-theoretic-formulation-of-network-equations]]
