---
title: Silicide
type: entity
id: entities/silicide
tags:
- semiconductor
- device-physics
- materials
- contact
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt
---

## Overview

Silicides are intermetallic compounds of silicon with a metal (e.g., TiSi₂, NiSi, CoSi₂, PtSi, WSi₂). Per Sze & Ng (Sect. 3.2.3): "a technology for making controllable Schottky barrier contacts has been developed in which a chemical reaction between the metal and the underlying silicon is induced to form silicides. The formation of metal silicides by solid-solid metallurgical reaction provides more reliable and reproducible Schottky barriers, because the interface chemical reactions are well defined and can be maintained under good control." Sze's Fig. 9 shows the empirical correlation between barrier height on n-Si and silicide eutectic temperature. Table 4 lists `φ_Bn` for representative silicides on n-type Si — e.g., CoSi (0.68 V, cubic, forming temp 400°C, melting 1460°C); CoSi₂ (0.64 V, cubic, forming 450°C, melting 1326°C); CrSi₂ (0.57 V, hexagonal, forming 450°C, melting 1475°C); rare-earth disilicides (DySi₂ 0.37 V, ErSi₂ 0.39 V, GdSi₂ 0.37 V).

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
