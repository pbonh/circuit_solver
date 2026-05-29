---
title: Quantum Well
type: claim
id: concepts/quantum-well
tags:
- semiconductor
- device-physics
- quantum
- heterojunction
- photonic
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A quantum well is a thin layer of narrow-bandgap semiconductor sandwiched between two wider-bandgap layers, creating a potential well that confines electrons or holes in one spatial direction. The result is a quasi-two-dimensional electron (or hole) gas with quantized subbands.

## How It Works

The Schrodinger equation inside the well of width L_z and depth phi_b gives standing-wave solutions psi_i ~ sin(i pi x / L_z) and quantized energies E_i = (i hbar pi)^2 / (2 m* L_z^2) for infinite walls; finite walls allow some wavefunction leakage. The density of states becomes a step function instead of the bulk square-root form, and the effective optical bandgap is enlarged. Multiple quantum wells with thin barriers form a superlattice with minibands.

## Key Parameters

- Well width L_z and barrier height phi_b.
- Conduction- and valence-band offsets AE_c, AE_v.
- Effective mass m* in the well (and barrier).
- Number of confined subbands.

## When To Use

- Quantum-well lasers and photodetectors with engineered emission/absorption wavelengths.
- HEMTs / MODFETs with a 2DEG in a triangular or rectangular well.
- Quantum-cascade lasers and intersubband detectors.

## Risks & Pitfalls

- Strain and interdiffusion alter subband energies.
- Multiple-quantum-well stack design must balance gain, optical confinement, and electrical transport.

## Related Concepts

- [[concepts/heterojunction]]
- [[concepts/energy-band-structure]]
- [[concepts/effective-mass]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
