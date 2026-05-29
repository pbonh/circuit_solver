---
title: Radiative Recombination
type: claim
id: concepts/radiative-recombination
tags:
- semiconductor
- device-physics
- photonic
- recombination
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Radiative recombination is the band-to-band recombination of an electron and hole in which the energy is released as a photon. It is the inverse process of optical absorption and is the basis of LED and laser-diode operation.

## How It Works

In a direct-bandgap semiconductor, the conduction-band minimum and valence-band maximum share the same k, so momentum is conserved without a phonon and radiative recombination rates are large (R_ec ~ 1e-10 cm^3/s). In indirect-bandgap materials (Si, Ge), a phonon must participate and the rate is orders of magnitude smaller. The emission peak occurs near Eg with a thermal linewidth.

## Key Parameters

- Radiative recombination coefficient R_ec.
- Carrier densities n, p.
- Photon energy h*nu near Eg.
- Internal quantum efficiency.

## When To Use

- LED and laser-diode design (require direct-gap or engineered direct-like material).
- Photovoltaic detailed-balance limit (intrinsic radiative loss).

## Risks & Pitfalls

- Competes with SRH and Auger; high doping or injection enhances Auger and reduces eta_int.
- Self-absorption inside the device limits extraction.

## Related Concepts

- [[concepts/bandgap]]
- [[concepts/shockley-read-hall-recombination]]
- [[concepts/light-emitting-diode]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
