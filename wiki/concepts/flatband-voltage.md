---
title: Flatband Voltage
type: claim
id: concepts/flatband-voltage
tags:
- semiconductor
- device-physics
- mosfet
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The flatband voltage V_FB of an MIS capacitor is the gate voltage at which the semiconductor energy bands are flat from bulk to surface (zero surface band bending). V_FB = phi_MS - Q_eff / C_ox, where phi_MS is the work-function difference between the gate and the semiconductor and Q_eff = Q_f + Q_m + Q_ot + integral of Q_it captures all effective oxide and interface charges referred to the metal-oxide interface.

## How It Works

For a clean ideal MIS structure, V_FB = phi_MS. Real structures have nonzero Q_f, Q_m, Q_ot, Q_it that shift V_FB negative (for the typical positive oxide charge in Si-SiO2). V_FB is extracted from C-V data by computing the depletion capacitance C_D at flatband from the semiconductor doping and finding the gate voltage where C(V) equals the series combination of C_ox and C_D(flatband).

## Key Parameters

- Work-function difference phi_MS (depends on gate material; poly-Si doping; metal gate material).
- Fixed oxide charge Q_f, mobile-ion charge Q_m, trap charge Q_ot, interface-trap charge Q_it.

## When To Use

- C-V analysis to extract oxide quality, doping, and threshold voltage.
- Setting MOSFET threshold by gate-work-function engineering.

## Risks & Pitfalls

- Mobile ions (Na+) drift under bias-temperature stress and change V_FB over time (instability).
- For non-uniform doping, the depletion capacitance at flatband must be computed self-consistently.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/threshold-voltage]]
- [[concepts/oxide-charge]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
