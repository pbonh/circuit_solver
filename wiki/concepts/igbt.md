---
title: "Insulated-Gate Bipolar Transistor (IGBT)"
type: concept
tags: [semiconductor, device-physics, power-device, mosfet, bjt, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/00-preface.txt"]
confidence: low
---

## Definition

An insulated-gate bipolar transistor is a hybrid power device that combines an input MOSFET (voltage-controlled gate) with an output bipolar transistor (high current density, low on-state voltage drop). It offers MOSFET-like ease of gate drive while delivering BJT-like conduction characteristics.

## How It Works

The MOSFET section turns on the base of an internal BJT, which then carries the bulk of the drain-to-source current via minority-carrier injection. Conductivity modulation in the drift region keeps on-state voltage low. Turn-off is slower than a pure MOSFET because stored minority charge must be removed.

## Key Parameters

- Breakdown voltage and on-state voltage drop V_CE(sat).
- Switching times and turn-off tail current.
- Safe operating area (SOA) for hard switching.

## When To Use

- Medium-to-high voltage, medium-frequency power switching: motor drives, induction heating, power supplies, traction inverters.

## Risks & Pitfalls

- Switching losses from minority-carrier storage limit frequency.
- Latch-up of the parasitic thyristor under fault conditions.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/thyristor]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-00-preface]]
- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
