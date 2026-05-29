---
title: MESFET (Metal-Semiconductor Field-Effect Transistor)
type: claim
id: concepts/mesfet
tags:
- semiconductor
- device-physics
- rf
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/10-chapter-7-jfets-mesfets-and-modfets.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A MESFET is a field-effect transistor with a Schottky-barrier gate directly contacting the active channel, dispensing with an insulator. It is the workhorse III-V device for microwave power amplifiers and was the high-frequency leader before MODFET / HEMT structures took over.

## How It Works

The Schottky barrier provides a depletion region in the channel whose width is modulated by gate bias. Pinch-off occurs when the gate depletion meets the substrate or buffer layer. Compared to MOSFETs, MESFETs avoid the surface-state and oxide-trap problems but suffer from gate leakage at forward bias (limited to one Schottky-diode drop above the Fermi level).

## Key Parameters

- Schottky barrier height phi_B and ideality factor.
- Channel thickness and doping.
- Gate length L_g (sets f_T).
- Pinch-off voltage V_P.

## When To Use

- Microwave power amplifiers (cellular base stations, radar, MMICs).
- Low-noise amplifiers in the 1-20 GHz range.
- Phase shifters and switches in phased-array systems.

## Risks & Pitfalls

- Gate-leakage limits maximum positive V_GS.
- Backgating from substrate traps causes drift in GaAs MESFETs (mitigated by buffer-layer or LT-grown GaAs).

## Related Concepts

- [[concepts/schottky-barrier]]
- [[concepts/modfet]]
- [[concepts/jfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
