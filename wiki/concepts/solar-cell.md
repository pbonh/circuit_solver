---
title: Solar Cell
type: claim
id: claim-solar-cell
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
confidence:
  base: 0.65
---

## Definition

A solar cell is a large-area semiconductor p-n (or heterojunction or quantum-junction) device that converts incident solar irradiance into electrical power. It operates as a photodiode in the fourth-quadrant of its I-V curve, where the device delivers power to an external load.

## How It Works

Photons with h*nu > Eg create electron-hole pairs that are separated by the built-in field of a p-n junction. The collected photocurrent J_sc shifts the diode I-V curve downward; the open-circuit voltage V_oc = (n kT / q) ln(J_sc / J_0 + 1) is the maximum voltage at which the diffusion current cancels the photocurrent. Efficiency eta = J_sc V_oc FF / P_in, where FF is the fill factor (typically 0.8 for good cells). The Shockley-Queisser detailed-balance limit gives ~33% for a single junction near Eg = 1.4 eV at one-sun AM1.5.

## Key Parameters

- Short-circuit current J_sc and open-circuit voltage V_oc.
- Fill factor FF (limited by series resistance and shunt leakage).
- Efficiency eta.
- Spectral response and absorption coefficient alpha(lambda).
- Minority-carrier diffusion length L (must exceed cell thickness for high collection).

## When To Use

- Terrestrial photovoltaics (rooftop, utility scale, mobile).
- Space power generation (multi-junction cells for spectrum splitting).
- Building-integrated photovoltaics (thin-film, transparent variants).

## Risks & Pitfalls

- Series resistance from grid contacts, shunt leakage at the cell edge.
- Light-induced degradation in amorphous silicon (Staebler-Wronski).
- Temperature reduces V_oc by ~2 mV/K.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/heterojunction]]
- [[concepts/photodiode]]
- [[concepts/bandgap]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
