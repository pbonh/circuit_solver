---
title: Zener Breakdown
type: claim
id: claim-zener-breakdown
tags:
- semiconductor
- device-physics
- p-n-junction
- breakdown
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.65
---

## Definition

Zener breakdown is the tunneling-driven reverse breakdown of a heavily doped p-n junction in which the depletion region is so narrow that band-to-band tunneling of valence-band electrons from the p-side to the conduction band of the n-side becomes appreciable at modest reverse bias. It dominates in junctions with V_BR < ~6 Eg/q (~5 V for Si); above that, avalanche typically dominates.

## How It Works

In a heavily doped p-n junction the depletion region can be only nanometers wide. Reverse bias displaces the bands so that filled valence-band states on the p-side line up energetically with empty conduction-band states on the n-side. Electrons tunnel through the narrow forbidden region directly, producing a sharp current onset at V_BR. Zener breakdown has a negative temperature coefficient (in contrast to avalanche's positive coefficient).

## Key Parameters

- Doping on both sides (must be very high).
- Bandgap (sets tunneling barrier height).
- Effective masses in conduction and valence bands.

## When To Use

- Voltage-reference Zener diodes (5-7 V Zener/avalanche compensation gives near-zero T coefficient).
- Modeling band-to-band tunneling leakage in heavily doped junctions of modern MOSFETs.

## Risks & Pitfalls

- "Zener diode" in practice often relies on avalanche breakdown for V > ~6 V; only sub-6 V devices are true Zener.
- Tunneling leakage in scaled junctions limits supply voltage reduction in CMOS.

## Related Concepts

- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/avalanche-breakdown]]
- [[concepts/p-n-junction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
