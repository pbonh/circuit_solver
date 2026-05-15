---
title: "Floating-Gate Memory"
type: concept
tags: [semiconductor, device-physics, mosfet, memory, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: medium
---

## Definition

A floating-gate memory cell is a MOSFET with an extra polysilicon gate completely surrounded by insulator and located between the channel and the control gate. Charge stored on the floating gate shifts the apparent threshold voltage by Q_fg / C_cg, allowing nonvolatile binary (or multi-level) storage. Flash memory, EPROM, and EEPROM use this principle.

## How It Works

The floating gate stores electrons injected by Fowler-Nordheim tunneling or by channel hot-electron injection during program. The stored charge raises the cell Vt; readout senses I_D at a fixed V_CG. Erase removes electrons (or injects holes) by F-N tunneling. Endurance is limited by oxide degradation from repeated tunneling stress; retention is limited by SILC and detrapping over years to decades.

## Key Parameters

- Programmed and erased Vt levels (separation determines reading margin).
- Tunnel oxide thickness and quality (typically 8-10 nm for endurance > 10^5 cycles).
- Coupling ratio between control gate and floating gate.

## When To Use

- NAND flash mass storage (USB drives, SSDs, smartphones).
- NOR flash for code storage with random-access read.
- EEPROM for small reprogrammable nonvolatile storage.

## Risks & Pitfalls

- Endurance/retention trade-off intensifies with oxide scaling.
- Charge loss at high temperature limits hot-environment use.
- 3-D NAND structures emerged to extend density when planar scaling stalled.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/fowler-nordheim-tunneling]]
- [[concepts/mis-capacitor]]
- [[concepts/dielectric-breakdown]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
