---
title: "Silicide"
type: entity
tags: [semiconductor, device-physics, materials, contact, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt"]
confidence: low
---

## Overview

Silicides are intermetallic compounds of silicon with a metal (e.g., TiSi2, NiSi, CoSi2, PtSi, WSi2). They are formed by reaction between a deposited metal film and the underlying silicon during a thermal anneal. Silicides provide the low-resistance, thermally stable, and CMOS-compatible contacts used at MOSFET source, drain, and gate terminals and BJT emitter, base, and collector contacts.

## Characteristics

- Low specific contact resistivity (10^-7 to 10^-8 Ohm-cm^2) to heavily doped Si.
- Tunable barrier heights for use as the source/drain of Schottky-barrier MOSFETs (e.g., PtSi on n-Si gives phi_B ~ 0.85 eV).
- Lower sheet resistance than doped poly-Si gates; used in salicide ("self-aligned silicide") flows for both gate and source/drain.
- Thermal stability up to ~700-900 deg C depending on choice (CoSi2 and NiSi are common in scaled CMOS).

## Common Strategies

- Salicide self-aligned process: deposit metal, anneal to react only over exposed Si, etch unreacted metal.
- Co-evaporation and rapid thermal annealing to control phase formation.
- Strain engineering: NiSi alloyed with Pt to reduce agglomeration in narrow source/drain regions.

## Related Entities

- [[entities/silicon]]
- [[concepts/ohmic-contact]]
- [[concepts/schottky-barrier]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
