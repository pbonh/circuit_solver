---
title: "Hall Effect"
type: concept
tags: [semiconductor, device-physics, transport, measurement, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: high
---

## Definition

The Hall effect is the appearance of a transverse voltage V_H across a current-carrying conductor placed in a magnetic field perpendicular to the current. In semiconductors it is the standard technique for measuring carrier type, carrier concentration, and mobility independently of one another.

## How It Works

The Lorentz force F = q v x B deflects moving carriers to one side of the sample, where they accumulate and set up a transverse Hall field that balances the deflection at steady state. The Hall coefficient R_H = V_H W / (I B) is r_H/(qp) for a p-type sample (positive) and -r_H/(qn) for n-type (negative). The Hall mobility mu_H = |R_H| sigma differs from drift mobility by the Hall factor r_H (~1.18 for phonon scattering, ~1.93 for ionized impurity scattering).

## Key Parameters

- Hall coefficient R_H (sign and magnitude).
- Hall factor r_H (1-2 typically).
- Sample geometry; van der Pauw method handles arbitrary shapes.

## When To Use

- Material characterization of carrier density (down to ~1e12 cm^-3) and mobility.
- Magnetic-field sensors (Hall-effect IC sensors, switches, position sensors).
- Studying fundamental phenomena (e.g., integer and fractional quantum Hall effect).

## Risks & Pitfalls

- Mixed conduction (electrons + holes) complicates interpretation.
- Contact misalignment introduces offset voltages requiring careful geometry or compensation.
- High magnetic field can produce magnetoresistance contributions.

## Related Concepts

- [[concepts/carrier-mobility]]
- [[concepts/carrier-concentration]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
