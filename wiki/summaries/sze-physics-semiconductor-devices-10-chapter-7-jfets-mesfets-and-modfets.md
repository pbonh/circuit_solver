---
title: 'Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 7: JFETs, MESFETs,
  and MODFETs'
type: source
id: source-sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets
kind: derived-summary
tags:
- semiconductor
- device-physics
- rf
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/10-chapter-7-jfets-mesfets-and-modfets.txt
---

## Key Points

- The chapter covers three field-effect transistor families that complement the MOSFET (Chapter 6) by using a p-n junction (JFET), a Schottky contact (MESFET), or a modulation-doped heterojunction (MODFET / HEMT) as the gate-channel structure.
- JFET (junction field-effect transistor): conducting channel between source and drain is pinched off by the depletion region of a reverse-biased p-n junction gate; depletion-mode device, normally-on, turned off by negative V_GS (for n-channel). Used mainly for low-noise input stages and power devices today.
- MESFET (metal-semiconductor FET): gate is a Schottky barrier directly on the active channel, eliminating the need for a gate dielectric. Operates well in GaAs and other III-V where high mobility and low parasitic capacitance make microwave performance excellent.
- I-V characteristics for JFET / MESFET in the gradual-channel approximation: linear region for V_DS small, saturation when the depletion region pinches off the channel at the drain end; saturation current I_DSS at V_GS = 0 and pinch-off voltage V_P set the device.
- Arbitrary doping profiles and enhancement-mode operation: heavily doped buried channels for short-pinch-off-voltage power devices; thin lightly doped channels for enhancement-mode logic.
- Microwave performance: cutoff frequency f_T = g_m / (2 pi C_GS) and maximum oscillation frequency f_max; GaAs MESFETs achieve f_T in tens to hundreds of GHz; relevant noise sources include channel thermal noise, gate leakage noise, and induced gate noise.
- Device structures: recessed-gate, T-gate, mushroom-gate MESFETs; ion-implanted vs. epitaxial channels.
- MODFET / HEMT: a heterojunction between a wider-gap doped barrier (e.g., n-AlGaAs) and an undoped narrower-gap channel (GaAs) creates a 2-DEG at the interface. Spatial separation of dopants from channel eliminates ionized-impurity scattering, giving very high low-temperature mobility and very high f_T.
- MODFET I-V: similar gradual-channel equations to MOSFET but with C_2DEG set by the heterojunction band offset rather than an oxide; threshold determined by Schottky barrier height plus heterojunction conduction-band offset.
- Advanced MODFET structures: pseudomorphic HEMT (pHEMT) on InGaAs/AlGaAs; metamorphic and lattice-matched InP-based HEMTs (lattice-matched InAlAs/InGaAs); InP HEMTs achieve f_T > 500 GHz.

## Relevant Concepts

- [[concepts/jfet]] — p-n-junction-gated FET.
- [[concepts/mesfet]] — Schottky-gated FET.
- [[concepts/modfet]] — modulation-doped heterojunction FET (HEMT).
- [[concepts/two-dimensional-electron-gas]] — channel in MODFET / HEMT.
- [[concepts/schottky-barrier]] — gate structure for MESFET.
- [[concepts/heterojunction]] — basis of MODFET.
- [[concepts/p-n-junction]] — gate structure for JFET.
- [[entities/gallium-arsenide]] — workhorse MESFET / MODFET substrate.

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 7 — JFETs, MESFETs, and MODFETs
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/10-chapter-7-jfets-mesfets-and-modfets.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
