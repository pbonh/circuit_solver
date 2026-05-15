---
title: "Decoupling Capacitor"
type: concept
tags: [vlsi, power-integrity, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt"]
confidence: medium
---

## Definition

A decoupling capacitor (decap) is a capacitor placed near a load circuit to supply transient current demand, suppress supply-voltage fluctuations, and reduce high-frequency noise on the power distribution network. Decaps are placed at multiple hierarchy levels: PCB, package, and on-die.

## How It Works

Decaps form a low-impedance path between supply and ground at frequencies above 1/(2π R_esr C). At lower frequencies, the supply network dominates; at higher frequencies, the decap shorts the noise. Optimal decap selection balances impedance reduction against area cost: on-die area is the most expensive (≈ 20× PCB cost), followed by package (≈ 4.5× PCB), then PCB.

## Key Parameters

- Capacitance value.
- Equivalent series resistance (ESR) and inductance (ESL).
- Insertion location (PCB, package, die).
- Area / cost per unit capacitance.

## When To Use

- Universally required in every clocked digital and analog IC.
- Allocation tuned during power-delivery exploration.

## Risks & Pitfalls

- Inductive resonance between package and on-die decaps can amplify certain frequencies.
- Over-provisioning wastes area; under-provisioning produces droops and overshoots.

## Related Concepts

- [[concepts/power-distribution-network]]
- [[concepts/power-delivery-exploration]]
- [[concepts/ir-drop-analysis]]
- [[concepts/voltage-regulator-placement]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
