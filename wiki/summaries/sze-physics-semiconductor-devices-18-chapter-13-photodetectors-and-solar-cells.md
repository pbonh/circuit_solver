---
title: 'Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 13: Photodetectors
  and Solar Cells'
type: source
id: summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells
kind: publication
tags:
- semiconductor
- device-physics
- photonic
- p-n-junction
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt
---

## Key Points

- The chapter develops semiconductor photodetectors -- devices that convert optical signals into electrical currents -- and solar cells -- devices that convert solar irradiance into electrical power. Photodetector families covered: photoconductors, p-n / p-i-n / heterojunction / Schottky / MSM photodiodes, avalanche photodiodes (APD), phototransistors, CCD and CMOS image sensors, and quantum-well infrared photodetectors (QWIP). Solar cells: silicon, GaAs, multi-junction, thin-film.
- Common figures of merit for photodetectors: quantum efficiency eta, responsivity R = eta q lambda / (h c) (A/W), dark current, noise-equivalent power (NEP), specific detectivity D* = sqrt(A B) / NEP, bandwidth, and uniformity.
- Photoconductor: bulk semiconductor with two ohmic contacts; photogenerated carriers reduce its resistance. Gain = tau / t_tr can exceed unity. Limited by carrier-lifetime-bandwidth product.
- Photodiodes: reverse-biased junctions in which photo-generated carriers in the depletion region are swept out as a photocurrent. p-i-n photodiodes use a wide intrinsic absorption region to maximize quantum efficiency at the cost of transit time; heterojunction photodiodes use a transparent wide-gap window layer to reduce surface recombination; Schottky photodiodes work in UV through visible (transparent thin metal layer).
- Avalanche photodiode (APD): adds reverse-biased multiplication region after absorption region to amplify photocurrent by impact ionization, raising signal above thermal noise. Excess noise factor F(M) depends on the electron/hole ionization-rate ratio k = alpha_p/alpha_n; smaller k gives lower noise (Si is best, InGaAs/InP APDs use separate absorption and multiplication regions).
- Phototransistor: a BJT (or HBT) whose base is illuminated; current gain multiplies the photocurrent. Lower bandwidth and higher noise than APDs but no high-voltage bias needed.
- Charge-coupled device (CCD): an array of MOS capacitors whose stored minority-carrier charge is transferred sequentially by clocked gate voltages to a sense node. Used as image sensors and analog shift registers. Quantum efficiency, dynamic range, dark current, charge-transfer efficiency, and noise floor are key.
- CMOS image sensors: pixel-level readout using MOSFETs (3T, 4T pixel architectures with pinned photodiode); now dominate consumer imaging due to lower power, on-chip integration, and rolling-shutter advantages.
- MSM (metal-semiconductor-metal) photodetector: interdigitated Schottky contacts on a semiconductor surface; very low capacitance enables wide bandwidth (>50 GHz) for telecom.
- QWIP (quantum-well infrared photodetector): intersubband absorption in a multiple-quantum-well stack used for mid- and long-wave IR imaging; lower detectivity than HgCdTe but mature manufacturable III-V process.
- Solar cells: an open-circuit voltage V_oc set by the photo-generated quasi-Fermi-level separation; short-circuit current J_sc set by absorbed photon flux. Fill factor FF and efficiency eta = P_max / P_in are the key metrics. The detailed-balance (Shockley-Queisser) limit for a single junction at 1 sun is ~33% for Eg near 1.4 eV.
- Photocurrent and spectral response: external quantum efficiency depends on absorption profile, minority-carrier collection length, and surface recombination.
- Device configurations: Si planar, screen-printed bulk Si solar cell; high-efficiency PERC, IBC, and HJT cells; thin-film amorphous Si, CdTe, CIGS; multi-junction GaInP/GaInAs/Ge for space.

## Relevant Concepts

- [[concepts/photodiode]]
- [[concepts/avalanche-photodiode]]
- [[concepts/solar-cell]]
- [[concepts/charge-coupled-device]]
- [[concepts/p-i-n-diode]]
- [[concepts/photoconductor]]
- [[concepts/quantum-well-infrared-photodetector]]
- [[concepts/p-n-junction]]
- [[concepts/heterojunction]]
- [[concepts/impact-ionization]]

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 13 — Photodetectors and Solar Cells
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
