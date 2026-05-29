---
title: Silicon-on-Insulator (SOI)
type: claim
id: claim-silicon-on-insulator
tags:
- semiconductor
- device-physics
- mosfet
- substrate
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt
confidence:
  base: 0.65
---

## Definition

Silicon-on-insulator (SOI) is a substrate technology in which a thin silicon device layer is separated from a thick handle wafer by a buried oxide (BOX) layer. Sze & Ng (3rd ed., Sect. 6.5.4) distinguish SOI from Thin-Film Transistors by noting that "the top silicon layer of an SOI wafer is high-quality single-crystalline material that is suitable for high-performance and high-density integrated [circuits]". The book groups variants by insulator: silicon-on-oxide (most popular), silicon-on-sapphire (SOS, Al₂O₃ substrate), silicon-on-zirconia (SOZ, ZrO₂ substrate), and silicon-on-nothing (air gap).

## How It Works

Sze lists four manufacturing routes: SIMOX (separation by implantation of oxygen — high-dose O implant followed by high-temperature anneal to form buried SiO₂); wafer bonding plus etchback (bond two wafers, one with an oxidized layer, then thin the top); lateral epitaxial growth of silicon over an oxide layer from a seed opening; and laser recrystallization of amorphous silicon deposited on oxide. A MOSFET fabricated on SOI may be partially depleted (PD-SOI) or fully depleted (FD-SOI) depending on device-layer thickness; FD-SOI eliminates the floating-body effect and provides excellent short-channel control.

## Key Parameters

- Device-layer thickness t_Si and buried-oxide thickness t_BOX.
- Back-gate bias capability (in FD-SOI, back-gate provides additional Vt control).

## When To Use

- High-speed digital logic with reduced power.
- Radiation-hardened, single-event-upset-resistant designs.
- RF circuits requiring high-Q passives and low substrate loss.

## Risks & Pitfalls

- Floating-body history effects in PD-SOI cause hysteresis — visible as the "kink effect": when a floating body lacks a substrate tie (Rsub = ∞), the output curves "show sudden rise of I_D with V_D ... referred to as kink effect" (Sze Sect. 6.4.4 / 6.5.4).
- In severe cases the substrate current induces source-drain breakdown via the parasitic n-p-n bipolar action (book Eq. 107: `V_BDs = V_BDx (1 - α_npn)^(1/n)`).
- Self-heating is more severe than in bulk Si due to the insulating BOX ("worse heat conduction because of the oxide layer").
- Cost premium over bulk Si; "potentially inferior material properties" depending on fabrication route.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/finfet]]
- [[concepts/short-channel-effects]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
