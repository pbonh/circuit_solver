---
title: "Schottky Barrier"
type: concept
tags: [semiconductor, device-physics, diode, metal-semiconductor, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt"]
confidence: medium
---

## Definition

A Schottky barrier is the potential barrier formed at the interface between a metal and a moderately doped semiconductor, resulting in a rectifying junction. The barrier height q phi_B is approximately the difference between the metal work function and the semiconductor electron affinity (modified by surface states).

## How It Works

Majority carriers (typically electrons in an n-type semiconductor) flow over the barrier by thermionic emission, giving rise to a diode I-V close to J = A* T^2 exp(-q phi_B/kT) [exp(qV/kT) - 1], with A* the effective Richardson constant. Unlike a p-n junction, conduction is by majority carriers, so reverse recovery is fast.

## Key Parameters

- Barrier height phi_B and ideality factor n.
- Effective Richardson constant A*.
- Series resistance and reverse-leakage current.

## When To Use

- Fast switching: rectifiers in switching power supplies, mixers, detectors.
- Logic clamps to prevent transistor saturation.
- Ohmic contacts are formed by heavily doping the semiconductor so that the barrier becomes thin enough to tunnel through.

## Risks & Pitfalls

- Barrier height is sensitive to surface preparation, interface states, and image-force lowering.
- Reverse leakage can be substantial compared to a p-n junction.
- Image-force barrier lowering at high reverse bias reduces breakdown.

## Related Concepts

- [[concepts/thermionic-emission]]
- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/p-n-junction]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-05-part-ii-device-building-blocks]]
- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
