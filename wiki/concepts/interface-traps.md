---
title: Interface Traps
type: claim
id: concepts/interface-traps
tags:
- semiconductor
- device-physics
- mosfet
- reliability
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

Interface traps (or interface states) are localized electronic states distributed in energy through the semiconductor bandgap at the semiconductor-insulator interface (most importantly Si-SiO2). Their density D_it has units cm^-2 eV^-1. Each trap can capture or emit carriers, charging or discharging in response to the surface Fermi level.

## How It Works

Interface traps stretch out the C-V curve (because each trap that charges as the surface potential sweeps through its energy adds capacitance C_it = q^2 D_it in parallel with C_D), degrade MOSFET subthreshold slope (every additional D_it electron must be charged before the inversion charge can grow), and reduce channel mobility through Coulomb scattering. They are typically reduced by post-metallization hydrogen anneals that passivate dangling Si-bonds, leaving final D_it ~ 1e10 cm^-2 eV^-1 in good thermal SiO2 on Si.

## Key Parameters

- Density D_it(E) (per area per energy).
- Capture cross sections sigma_n, sigma_p (typically 1e-14 - 1e-16 cm^2).
- Spatial distribution (interface vs. near-interface oxide).

## When To Use

- Measuring oxide quality: high-low frequency C-V, conductance, charge pumping, deep-level transient spectroscopy.
- Diagnosing MOSFET subthreshold slope degradation under stress.
- Reliability and bias-temperature-instability (BTI) modeling.

## Risks & Pitfalls

- Different measurement techniques sample different portions of the bandgap, leading to apparent disagreements.
- Hot-carrier and BTI stress create new interface traps that shift Vt over time.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/mosfet]]
- [[concepts/shockley-read-hall-recombination]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
