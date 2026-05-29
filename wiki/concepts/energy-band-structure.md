---
title: Energy Band Structure
type: claim
id: claim-energy-band-structure
tags:
- semiconductor
- device-physics
- band-structure
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt
confidence:
  base: 0.85
---

## Definition

The energy band structure of a crystalline solid is the relationship E(k) between electron energy and crystal momentum k. It is obtained from one-electron Schrodinger solutions in the periodic lattice potential (Bloch functions). In semiconductors, a forbidden range (the bandgap) separates the filled valence bands from the empty conduction bands.

## How It Works

By the Bloch theorem, E(k) is periodic in the reciprocal lattice and can be reduced to the first Brillouin zone (Wigner-Seitz cell). Numerical methods include orthogonalized plane waves, pseudopotentials, and k.p theory. Near band extrema, E(k) is approximately parabolic with curvature characterized by an effective mass tensor. Si has an indirect minimum along [100]; GaAs has a direct minimum at Gamma.

## Key Parameters

- Bandgap Eg (1.12 eV for Si, 1.42 eV for GaAs at 300 K).
- Direct vs. indirect: alignment of conduction-band minimum and valence-band maximum in k.
- Heavy- and light-hole bands; spin-orbit splitting.
- Number of equivalent valleys (Mc) and intervalley separations.
- Effective masses (along and transverse to symmetry axes).

## When To Use

- To derive carrier statistics (density of states, effective masses).
- To predict optical-absorption thresholds and selection rules.
- To explain device-relevant phenomena like transferred-electron negative resistance, tunneling, and quantum confinement.

## Risks & Pitfalls

- Parabolic-band approximation breaks down at high energies and in narrow-gap materials.
- Strain, alloying, and quantum confinement substantially alter band structure.

## Related Concepts

- [[concepts/bandgap]]
- [[concepts/effective-mass]]
- [[concepts/carrier-concentration]]
- [[concepts/heterojunction]]
- [[concepts/quantum-well]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-03-part-i-semiconductor-physics]]
- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-14-chapter-10-transferred-electron-and-real-space-transfer-devices]]
