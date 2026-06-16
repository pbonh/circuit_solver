---
title: Semiconductor Physics
type: topic
slug: semiconductor-physics
created: 2026-06-16
updated: 2026-06-16
summary: The physical principles underlying semiconductor devices — energy bands, carrier transport, junction physics — that form the basis of all SPICE device models.
tags: [semiconductor, device-physics, mosfet, pn-junction, vlsi]
sources: [physics-of-semiconductor-devices-3rd-ed]
status: active
---

# Semiconductor Physics

The field encompassing energy band theory, carrier statistics and transport, and the physics of semiconductor junctions and device structures. Every SPICE device model (diode, MOSFET, BJT) is a numerical approximation to the underlying equations in this domain.

## Overview

- Energy bands and bandgap determine which materials conduct: Si (1.12 eV gap), GaAs (1.42 eV), Ge (0.67 eV)
- Carrier concentration at equilibrium: Fermi-Dirac statistics, intrinsic concentration n_i, doping-controlled n and p
- Transport: drift (μE), diffusion (D∇n), continuity equations, Hall effect, high-field velocity saturation
- Device building blocks: [[pn-junction]], metal-semiconductor contacts (Schottky), MOS capacitor
- Transistor families: [[mosfet-physics]] (dominant in VLSI), bipolar (BJT/HBT), JFET/MESFET/MODFET

## Entities and concepts in this topic

- [[pn-junction]] - fundamental building block; basis of SPICE diode model
- [[mosfet-physics]] - core transistor for VLSI; basis of BSIM/SPICE MOSFET models
- [[circuit-simulation]] - uses device physics via SPICE models
- [[spice-simulation]] - device models are physics approximations
- [[physics-of-semiconductor-devices-3rd-ed]] - Sze & Ng 3rd ed.; the authoritative device physics reference

## Open threads

- How do modern FinFET/GAA physics deviate from classical MOSFET models — and how do BSIM-CMG models capture this?
- Quantum confinement effects in sub-10nm nodes: when does semiclassical transport fail?
- Connection between semiconductor sensor physics and circuit-level interface modeling
