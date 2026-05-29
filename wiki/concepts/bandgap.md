---
title: Bandgap
type: claim
id: claim-bandgap
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

The bandgap Eg is the energy difference between the lowest conduction-band edge E_c and the highest valence-band edge E_v of a semiconductor. It is the most-important single parameter describing a semiconductor and governs intrinsic carrier density, optical absorption threshold, breakdown field, and many device properties.

## How It Works

Eg arises from the periodic crystal potential breaking the free-electron continuum. It varies with temperature approximately as Eg(T) = Eg(0) - alpha T^2 / (T + beta). It also varies with hydrostatic pressure, with alloy composition, and with quantum confinement (effective gap grows in quantum wells).

## Key Parameters

- Eg value (1.12 eV Si, 1.42 eV GaAs, 6.0 eV diamond, etc.) at 300 K.
- Temperature coefficient dEg/dT.
- Direct vs. indirect (controls radiative-recombination efficiency).
- Pressure coefficient dEg/dP.

## When To Use

- Selecting a semiconductor for an optical wavelength: hv ~ Eg defines the absorption/emission edge.
- Selecting materials for high-temperature or high-voltage power devices (wide-bandgap SiC, GaN).
- Computing intrinsic carrier density via ni = sqrt(Nc Nv) exp(-Eg/2kT).

## Risks & Pitfalls

- Bandgap narrowing at heavy doping invalidates simple ni formulas.
- For alloys, Eg follows a bowing law that must be measured rather than linearly interpolated.

## Related Concepts

- [[concepts/energy-band-structure]]
- [[concepts/carrier-concentration]]
- [[concepts/impact-ionization]]
- [[concepts/effective-mass]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
- [[summaries/sze-physics-semiconductor-devices-21-appendix-e-properties-of-important-semiconductors]]
