---
title: Population Inversion
type: claim
id: concepts/population-inversion
tags:
- semiconductor
- device-physics
- photonic
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

Population inversion is the nonequilibrium condition in which more states at the upper level of an electronic transition are occupied than at the lower level. It is the necessary condition for net stimulated emission and laser action. In a semiconductor, the Bernard-Duraffourg condition requires the separation of the electron and hole quasi-Fermi levels to exceed the photon energy: E_Fn - E_Fp > h*nu.

## How It Works

Strong forward injection (electrical pumping) drives the quasi-Fermi levels deep into the conduction and valence bands, creating an inverted carrier distribution near the band edges over a range of states. Above a threshold current density the gain at some photon energy exceeds the loss in the cavity, and lasing starts.

## Key Parameters

- Quasi-Fermi-level separation E_Fn - E_Fp.
- Injected carrier density.
- Material gain at the lasing wavelength.

## When To Use

- Designing semiconductor lasers and optical amplifiers (SOAs).
- Predicting transparency current density and threshold.

## Risks & Pitfalls

- Heating raises the threshold and can quench inversion.
- Auger and intervalence-band absorption reduce effective gain at long wavelengths (1.55 um).

## Related Concepts

- [[concepts/semiconductor-laser]]
- [[concepts/radiative-recombination]]
- [[concepts/quantum-well]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
