---
title: FinFET
type: claim
id: claim-finfet
tags:
- semiconductor
- device-physics
- mosfet
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt
confidence:
  base: 0.65
---

## Definition

A FinFET is a three-dimensional MOSFET in which the channel is a narrow vertical silicon fin gated on multiple sides (typically three). Sze & Ng (3rd ed., Sect. 6.5.5 "Three-Dimensional Structures") describe the family as MOSFETs built on an "ultra-thin layer such that the body is fully depleted under the whole bias range" with "a surround gate structure that encloses the body layer from at least two sides" (Fig. 38). The horizontal variant of Fig. 38a — current parallel to the wafer surface with vertical sidewall channels — is the FinFET; the vertical variant of Fig. 38b shares the surround-gate / thin-body topology but routes current vertically.

## How It Works

Source and drain regions are formed at the two ends of a silicon fin. A gate dielectric wraps around the top and sides of the fin; the gate electrode covers it. Because the channel is thin (~10 nm) and the gate surrounds it, the gate has greater effective coupling than in planar bulk MOSFETs. Multiple fins per device set the effective W.

## Key Parameters

- Fin height H_fin (sets W_eff = 2 H_fin + W_fin per fin).
- Fin width W_fin and pitch.
- Gate-stack EOT.
- Channel doping (usually low, since SCEs are controlled geometrically).

## When To Use

- Logic MOSFETs at and below the 22 nm node.
- Low-leakage, high-density digital ICs.

## Risks & Pitfalls

- Layout discretization: device width comes in fin-sized quanta.
- Self-heating is more severe than in planar bulk due to thermal isolation from the substrate.
- Variability from fin-width and line-edge roughness.
- Sze notes (Sect. 6.5.5) the *fabrication* challenges that are intrinsic to the topology: "the majority or all of the channel surface is on a vertical wall ... presents great challenges in achieving a smooth channel surface from etching and growth or deposition of gate dielectrics on these surfaces. Formation of the source/drain junction is no longer trivial by means of ion implantation. Salicide formation will also be much more difficult."

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/short-channel-effects]]
- [[concepts/dennard-scaling]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
