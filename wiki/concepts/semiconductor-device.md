---
title: "Semiconductor Device"
type: concept
tags: [semiconductor, device-physics, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/00-preface.txt", "raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt"]
confidence: high
---

## Definition

A semiconductor device is an electronic component whose operation is based on controlled flow of charge carriers (electrons and holes) in a semiconductor material such as silicon, germanium, or III-V compound. Examples include p-n junction diodes, bipolar and field-effect transistors, MIS capacitors, light-emitting and laser diodes, photodetectors, solar cells, and various sensors.

## How It Works

A semiconductor's carrier density can be controlled by impurity doping (donors create n-type, acceptors create p-type), enabling regions of differing conductivity to be patterned in a single crystal. Junctions between such regions (or between a semiconductor and a metal, insulator, or dissimilar semiconductor) form built-in potential barriers that selectively block or transmit carriers under applied bias. Device operation is governed by Poisson's equation, drift-diffusion current, and continuity equations, augmented by recombination/generation, thermionic emission, tunneling, and impact-ionization processes.

## Key Parameters

- Bandgap Eg of the semiconductor and its doping levels (Nd, Na).
- Geometry: junction depths, channel length, oxide thickness.
- Material parameters: carrier mobility, diffusion constant, minority-carrier lifetime, effective Richardson constant.
- Operating regime: low-field linear, high-field saturation, breakdown, optical or thermal excitation.

## When To Use

- Whenever electronic signal amplification, switching, sensing, or photonic conversion is required.
- Choice of device family depends on signal domain (analog, digital, RF, optical), frequency, voltage/power level, and integration density.

## Risks & Pitfalls

- Parameter drift with temperature: bandgap, mobility, intrinsic carrier density all vary substantially with T.
- Reliability concerns: hot-carrier injection, electromigration, dielectric breakdown, gate-oxide tunneling.
- Process-variation sensitivity becomes severe as devices scale toward atomic dimensions.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/mosfet]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/schottky-barrier]]
- [[concepts/heterojunction]]
- [[concepts/energy-band-structure]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-00-preface]]
- [[summaries/sze-physics-semiconductor-devices-03-part-i-semiconductor-physics]]
- [[summaries/sze-physics-semiconductor-devices-20-appendix-a-list-of-symbols]]
