---
title: "FinFET"
type: concept
tags: [semiconductor, device-physics, mosfet, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: low
---

## Definition

A FinFET is a three-dimensional MOSFET in which the channel is a narrow vertical silicon fin gated on multiple sides (typically three). The thin, fully or near-fully depleted fin provides strong electrostatic control of the channel by the gate, suppressing short-channel effects at scaled gate lengths.

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

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/short-channel-effects]]
- [[concepts/dennard-scaling]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
