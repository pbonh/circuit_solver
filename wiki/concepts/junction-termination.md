---
title: "Junction Termination"
type: concept
tags: [semiconductor, device-physics, power-device, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/15-chapter-11-thyristors-and-power-devices.txt"]
confidence: medium
---

> Sze & Ng Chapter 11 cites the foundational termination references — Cornu, Schweitzer & Kuhn (1974) on "Double Positive Beveling: A Better Edge Contour for High Voltage Devices" and Davies & Gentry (1964) on "Control of Electric Field at the Surface of p-n Junctions" — but does not treat the design rules in the main text. The general-knowledge content below is consistent with those references and the implied power-device requirements throughout Sect. 11.

## Definition

Junction termination is the set of design techniques used at the edges of a p-n junction to prevent premature breakdown due to field crowding. Without termination, the curvature of the junction at its physical edge enhances the electric field and causes breakdown well below the planar (1-D) value. Termination techniques include beveling (mesa devices), guard rings (planar devices), field plates, and junction-termination extensions (JTE).

## How It Works

A planar (1-D) junction supports its full theoretical breakdown voltage only where the field is uniform. At edges or curved regions, the field is enhanced; the local critical field is reached at lower applied voltage. Termination spreads the field by introducing additional depleted volume, lower-doped regions, or floating guard rings that pick up some of the voltage drop and reduce local field crowding.

## Key Parameters

- Termination width (must be comparable to depletion width).
- Doping of JTE region or field-stop.
- Surface passivation quality.

## When To Use

- All high-voltage power devices: thyristors, IGBTs, power MOSFETs, fast-recovery diodes.
- High-Q microwave varactors.

## Risks & Pitfalls

- Surface charge on passivation drifts the breakdown voltage over time (humidity sensitivity).
- Termination consumes die area, increasing cost.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/avalanche-breakdown]]
- [[concepts/depletion-region]]
- [[concepts/thyristor]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
