---
title: "Junction Field-Effect Transistor (JFET)"
type: concept
tags: [semiconductor, device-physics, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/10-chapter-7-jfets-mesfets-and-modfets.txt"]
confidence: medium
---

## Definition

A junction field-effect transistor (JFET) is a three-terminal voltage-controlled current source in which the conducting channel between source and drain is modulated by the depletion region of a reverse-biased p-n junction gate. JFETs are normally-on (depletion-mode); applying a reverse V_GS pinches off the channel.

## How It Works

The channel is a doped slab between two opposite-doped gate regions. At V_GS = 0 the channel is mostly open; as |V_GS| (reverse bias) increases, the gate depletion regions expand into the channel until they meet at pinch-off, V_GS = -V_P. For small V_DS the device is a voltage-controlled resistor; for larger V_DS the channel pinches off at the drain end and the current saturates.

## Key Parameters

- Pinch-off voltage V_P (and equivalent saturation current I_DSS at V_GS = 0).
- Transconductance g_m in saturation.
- Gate leakage (very low because gate is reverse-biased p-n junction).
- Channel doping and thickness.

## When To Use

- Low-noise analog amplifier input stages (very high input impedance, low input current).
- Series-pass elements in voltage regulators.
- Power JFETs (vertical structure) in audio amplifiers.

## Risks & Pitfalls

- Always depletion-mode in conventional Si processes; needs negative gate-source supply to turn off.
- Lower frequency response than MESFETs / MODFETs.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/mesfet]]
- [[concepts/mosfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
