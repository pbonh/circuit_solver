---
title: MOSFET (Metal-Oxide-Semiconductor Field-Effect Transistor)
type: claim
id: concepts/mosfet
tags:
- semiconductor
- device-physics
- mosfet
- device-model
- digital
- analog
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/00-preface.txt
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt
confidence:
  base: 0.85
  source_count: 2
  contradicted: false
  effective: 0.884
  inputs_hash: 48379ae317c01c10
---

## Definition

A MOSFET is a four-terminal (gate, source, drain, body) field-effect transistor in which a metal (or polysilicon) gate is separated from a doped semiconductor channel by a thin insulator (typically SiO2). A voltage applied to the gate capacitively modulates the channel's carrier density and thereby the source-drain conductance.

## How It Works

The gate-oxide-semiconductor stack is an MIS capacitor. Above a threshold gate voltage Vt an inversion layer forms at the semiconductor surface, providing a conductive channel. The drain current depends on Vgs and Vds and on channel mobility, gate-oxide capacitance, channel length, and width. Operates as an enhancement or depletion-mode device, and as n-channel or p-channel.

## Key Parameters

- Threshold voltage Vt.
- Gate-oxide thickness t_ox and capacitance C_ox.
- Channel length L, width W, mobility mu.
- Subthreshold slope and on/off current ratio.
- Body-effect coefficient, drain-induced barrier lowering (DIBL) for short channels.

## When To Use

- The dominant transistor for digital ICs (CMOS logic, microprocessors, SRAM, DRAM).
- Analog amplifiers and switches, especially at moderate frequencies.
- Power devices (LDMOS, vertical power MOSFETs) and nonvolatile memory (floating-gate, flash).

## Risks & Pitfalls

- Short-channel effects (DIBL, punch-through, velocity saturation) require careful scaling.
- Gate-oxide tunneling and reliability limit thickness scaling.
- Process variation, random dopant fluctuation, and line-edge roughness produce yield issues at advanced nodes.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/p-n-junction]]
- [[concepts/semiconductor-device]]
- [[concepts/carrier-mobility]]

## Sources

- [[summaries/graphs-in-vlsi-04-1-introduction]]
- [[summaries/sze-physics-semiconductor-devices-00-preface]]
- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
