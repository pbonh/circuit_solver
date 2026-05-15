---
title: "Effective Mass"
type: concept
tags: [semiconductor, device-physics, band-structure, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: high
---

## Definition

The effective mass m* of a charge carrier in a semiconductor is defined by the curvature of the energy band at the relevant extremum: 1/m*_ij = (1/hbar^2) d^2E/(dk_i dk_j). It allows carriers near band edges to be treated as quasi-free particles obeying classical Newtonian equations of motion with mass m*.

## How It Works

For an ellipsoidal energy surface (as for the Si conduction band), separate longitudinal m_l and transverse m_t effective masses describe motion along principal axes. Density-of-states effective mass mde = (M_c^2 m_l m_t^2)^(1/3) governs the carrier density at a given Fermi level. Conductivity effective mass enters mobility expressions.

## Key Parameters

- Longitudinal and transverse effective masses for ellipsoidal valleys.
- Heavy-hole and light-hole effective masses in the valence band.
- Density-of-states and conductivity effective masses.

## When To Use

- Computing Nc and Nv from m_de, m_dh.
- Calculating mobility from scattering theories.
- Estimating quantum-confinement subband energies E_i ~ (hbar pi i)^2/(2 m* L^2).

## Risks & Pitfalls

- Parabolic approximation fails at high carrier energies (nonparabolicity).
- Effective mass is anisotropic in general; using a scalar value introduces error.

## Related Concepts

- [[concepts/energy-band-structure]]
- [[concepts/carrier-mobility]]
- [[concepts/carrier-concentration]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-21-appendix-e-properties-of-important-semiconductors]]
