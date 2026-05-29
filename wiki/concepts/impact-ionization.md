---
title: Impact Ionization
type: claim
id: concepts/impact-ionization
tags:
- semiconductor
- device-physics
- transport
- breakdown
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Impact ionization is the generation of an electron-hole pair when a high-energy carrier collides with a lattice atom, transferring enough energy to excite a valence electron into the conduction band. It is characterized by an ionization rate alpha_n (for electrons) and alpha_p (for holes), each measured in cm^-1: the number of pairs generated per unit distance traveled.

## How It Works

A carrier accelerated by an electric field E gains energy at a rate qE v. Once its kinetic energy exceeds the ionization threshold (~1.5 Eg, broadly), it can ionize a valence electron and lose much of its energy. The ionization rate is a strong nonlinear function of field, well fit by alpha = A exp(-b/E) or alpha = A exp(-(b/E)^2). Avalanche breakdown occurs when integrated ionization across the high-field region drives the multiplication factor to infinity.

## Key Parameters

- Ionization threshold energy E_I (3.6 eV electrons in Si, 5.0 eV holes).
- Threshold field parameters for thermal, optical, ionization scattering.
- Bandgap Eg (larger Eg => smaller alpha at a given field => higher breakdown voltage).
- Temperature (alpha decreases with rising T at a given field).

## When To Use

- Designing avalanche photodiodes and IMPATT/BARITT oscillators.
- Calculating breakdown voltage of diodes, MOSFETs (drain breakdown), BJTs.
- Setting voltage-derating rules for reliable operation.

## Risks & Pitfalls

- Hot-carrier injection into gate oxide (MOSFET reliability).
- Latch-up in CMOS through impact-ionization-induced substrate current.
- Anisotropy: in some materials (GaAs, GaP) alpha depends on crystal orientation.

## Related Concepts

- [[concepts/bandgap]]
- [[concepts/carrier-mobility]]
- [[concepts/p-n-junction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-13-chapter-9-impatt-diodes]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
