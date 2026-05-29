---
title: Power Distribution Network
type: claim
id: claim-power-distribution-network
tags:
- vlsi
- power-integrity
- analog
- mixed-signal
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/03-about-the-authors.txt
confidence:
  base: 0.65
---

## Definition

A power distribution network (PDN) is the system of interconnect, vias, decoupling capacitors, and regulators that delivers supply voltages from board-level sources through package and on-chip metal layers to every active device on an integrated circuit.

## How It Works

A PDN is typically modeled as a multi-layer mesh graph of resistors (and inductors/capacitors for transient analysis) with current sources representing load circuits and voltage sources at supply pads. IR drop, dynamic noise, and impedance are analyzed using Laplacian-based methods (MNA), accelerated approximations (such as infinite-lattice models and the Infinity Mirror Technique), or domain decomposition. Designers iteratively adjust metal widths, via density, decoupling capacitor placement, and on-chip voltage regulator location to meet integrity targets.

## Key Parameters

- DC IR drop budget (small percentage of VDD).
- AC impedance vs. frequency profile.
- Number of metal layers dedicated to power.
- Decoupling capacitance (on-chip and off-chip).
- Voltage regulator placement and current capacity.

## When To Use

- All VLSI designs requiring sign-off on supply integrity.
- Board-level power planning (e.g., SPROUT) and on-chip power grid synthesis.

## Risks & Pitfalls

- Excessive IR drop degrades timing and noise margins.
- Resonances between package and on-chip inductance/capacitance can cause supply ringing.
- Electromigration risk in narrow power rails carrying high current density.

## Related Concepts

- [[concepts/ir-drop-analysis]]
- [[concepts/infinity-mirror-technique]]
- [[concepts/modified-nodal-analysis]]
- [[entities/sprout]]

## Sources

- [[summaries/graphs-in-vlsi-03-about-the-authors]]
- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
- [[summaries/graphs-in-vlsi-09-6-effective-resistance-of-truncated-infinite-mesh-structures]]
- [[summaries/graphs-in-vlsi-10-7-effective-resistance-of-finite-grids]]
- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
- [[summaries/graphs-in-vlsi-13-10-sprout-smart-power-routing-tool-for-board-level-exploration-and-prototyping]]
