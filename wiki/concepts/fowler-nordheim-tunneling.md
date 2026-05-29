---
title: Fowler-Nordheim Tunneling
type: claim
id: claim-fowler-nordheim-tunneling
tags:
- semiconductor
- device-physics
- mosfet
- tunneling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt
confidence:
  base: 0.65
---

## Definition

Fowler-Nordheim tunneling is the high-field tunneling of carriers from a metal or semiconductor through a triangular dielectric barrier into the dielectric's conduction band. In a MIS capacitor under strong bias, electrons tunnel from the cathode through the SiO2 conduction-band offset into the oxide, then drift through the oxide to the anode.

## How It Works

The current density follows J = (A E^2) exp(-B / E), with A and B constants set by the barrier height and effective electron mass in the oxide. The triangular-barrier WKB derivation predicts the exponential factor; the prefactor depends on supply density of states. Direct tunneling (rectangular barrier) takes over at thin oxides where the carrier exits before reaching the oxide conduction band.

## Key Parameters

- Barrier height phi_B at the cathode (3.1 eV for electrons at Si/SiO2).
- Oxide field E (typically 6-10 MV/cm for measurable F-N current).
- Effective electron mass in the oxide (~0.5 m_0).

## When To Use

- Program/erase of flash and EEPROM cells.
- Modeling gate leakage in MOSFETs with t_ox > 4 nm.
- Reverse engineering oxide barrier heights from log(I/E^2) vs 1/E plots.

## Risks & Pitfalls

- Generates oxide traps and interface states; primary degradation mechanism in floating-gate endurance.
- Direct tunneling dominates at modern thin oxides; F-N model overestimates barrier sensitivity.

## Related Concepts

- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/mis-capacitor]]
- [[concepts/dielectric-breakdown]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
