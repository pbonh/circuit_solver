---
title: "Silicon-on-Insulator (SOI)"
type: concept
tags: [semiconductor, device-physics, mosfet, substrate, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: low
---

## Definition

Silicon-on-insulator (SOI) is a substrate technology in which a thin silicon device layer is separated from a thick handle wafer by a buried oxide (BOX) layer. Devices built on SOI enjoy reduced parasitic capacitance, complete dielectric isolation between adjacent transistors, and reduced soft-error rates.

## How It Works

SOI wafers are produced by SIMOX (oxygen implantation), wafer bonding plus etchback, or Smart-Cut (hydrogen-induced separation). A MOSFET fabricated on SOI may be partially depleted (PD-SOI) or fully depleted (FD-SOI) depending on the device-layer thickness; FD-SOI eliminates the floating-body effect and provides excellent short-channel control.

## Key Parameters

- Device-layer thickness t_Si and buried-oxide thickness t_BOX.
- Back-gate bias capability (in FD-SOI, back-gate provides additional Vt control).

## When To Use

- High-speed digital logic with reduced power.
- Radiation-hardened, single-event-upset-resistant designs.
- RF circuits requiring high-Q passives and low substrate loss.

## Risks & Pitfalls

- Floating-body history effects in PD-SOI cause hysteresis.
- Self-heating is more severe than in bulk Si due to the insulating BOX.
- Cost premium over bulk Si.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/finfet]]
- [[concepts/short-channel-effects]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
