---
title: MODFET / HEMT (Modulation-Doped FET)
type: claim
id: concepts/modfet
tags:
- semiconductor
- device-physics
- rf
- advanced
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

A modulation-doped FET (MODFET, also called high-electron-mobility transistor HEMT) is a heterojunction-based FET in which dopants are confined to a wide-gap barrier layer while the channel is an undoped narrower-gap layer. Electrons spill from the doped barrier into the channel where they form a two-dimensional electron gas (2DEG) with high mobility because of spatial separation from ionized donors.

## How It Works

An n-AlGaAs / undoped GaAs heterojunction confines electrons from the AlGaAs dopants in a triangular potential well at the GaAs side of the interface. A Schottky-barrier gate modulates the 2DEG density. Because the electrons travel through undoped GaAs, ionized-impurity scattering is essentially eliminated and the low-temperature mobility can exceed 10^6 cm^2/V-s; room-temperature mobility ~8000 cm^2/V-s. Pseudomorphic (pHEMT) variants use strained InGaAs channels on GaAs for higher electron density and mobility; lattice-matched InAlAs/InGaAs/InP HEMTs reach f_T above 500 GHz.

## Key Parameters

- Sheet density and mobility of the 2DEG.
- Conduction-band offset AE_c.
- Spacer thickness between donors and channel.
- Schottky barrier height of the gate.

## When To Use

- Microwave low-noise amplifiers (satellite TV LNBs, radio astronomy receivers).
- Power amplifiers at millimeter-wave frequencies (GaN HEMTs at 5G mm-wave bands).
- Mixers, frequency multipliers, and high-speed digital logic in III-V.

## Risks & Pitfalls

- Donor depletion (DX centers in AlGaAs) at low T causes persistent photoconductivity and threshold instability.
- GaN HEMT has surface-trap induced current collapse (mitigated by passivation, field plates).
- Lattice mismatch limits indium content in pseudomorphic channels.

## Related Concepts

- [[concepts/heterojunction]]
- [[concepts/two-dimensional-electron-gas]]
- [[concepts/mesfet]]
- [[concepts/schottky-barrier]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
