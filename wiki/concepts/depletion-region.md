---
title: Depletion Region
type: claim
id: concepts/depletion-region
tags:
- semiconductor
- device-physics
- p-n-junction
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The depletion region (or space-charge region) of a p-n junction or Schottky contact is the zone near the junction in which mobile carriers are largely depleted, leaving behind the fixed charge of ionized dopants. In the depletion approximation, free-carrier densities are set to zero throughout this region.

## How It Works

At equilibrium, diffusion of majority carriers across the junction leaves behind ionized donors (positive) on the n-side and ionized acceptors (negative) on the p-side. These fixed charges set up an electric field that opposes further diffusion. The Poisson equation yields a triangular field profile (peak at the metallurgical junction) and a parabolic potential profile spanning the built-in potential V_bi. The depletion width W = sqrt(2 eps_s (V_bi - V) / (q N_eff)) for a one-sided junction. The two-tail correction (-2kT/q) accounts for majority-carrier transition regions.

## Key Parameters

- Net doping N_A, N_D on the two sides.
- Built-in potential V_bi and applied bias V.
- Dielectric permittivity eps_s.
- Maximum field E_m at the metallurgical junction.

## When To Use

- Computing junction capacitance C_D = eps_s / W.
- Predicting breakdown voltage (max field reaches critical field).
- Designing collector regions of BJTs or drift regions of power devices.

## Risks & Pitfalls

- Depletion approximation overestimates capacitance near zero bias.
- For nonuniform doping (e.g., diffused profiles), full numerical solution is required.
- High injection collapses the depletion region.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/built-in-potential]]
- [[concepts/junction-capacitance]]
- [[concepts/poisson-equation]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
