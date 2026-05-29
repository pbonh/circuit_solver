---
title: Metal-Insulator-Semiconductor (MIS) Capacitor
type: claim
id: concepts/mis-capacitor
tags:
- semiconductor
- device-physics
- mosfet
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A metal-insulator-semiconductor capacitor is the three-layer stack consisting of a metal (or heavily doped polysilicon) gate, a thin insulating dielectric (most commonly SiO2 in silicon technology), and a doped semiconductor body. It is the gate stack of every MOSFET and the storage element of floating-gate nonvolatile memory.

## How It Works

A voltage applied between the gate and the substrate creates an electric field across the insulator, modulating the semiconductor surface between accumulation, depletion, and inversion regimes. In inversion, a thin layer of minority carriers forms at the semiconductor surface, providing the conducting channel of a MOSFET. The capacitance versus gate voltage (C-V) curve is the canonical diagnostic for oxide quality, interface-trap density, and doping profile.

## Key Parameters

- Oxide thickness t_ox and dielectric permittivity (oxide capacitance C_ox).
- Flatband voltage V_FB and threshold voltage Vt.
- Substrate doping and interface-trap density D_it.

## When To Use

- As the gate stack of MOSFETs and the storage node of floating-gate memory.
- As an experimental vehicle for evaluating new gate dielectrics and interface quality.

## Risks & Pitfalls

- Oxide reliability: time-dependent dielectric breakdown (TDDB) limits scaling.
- Interface states and fixed oxide charges shift Vt and degrade mobility.
- Polysilicon-gate depletion adds an effective oxide thickness penalty.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/semiconductor-device]]
- [[concepts/poisson-equation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-05-part-ii-device-building-blocks]]
- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
