---
title: "Fermi-Level Pinning"
type: concept
tags: [semiconductor, device-physics, surface-physics, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt"]
confidence: medium
---

## Definition

Fermi-level pinning is the tendency for the Fermi level at a semiconductor surface or interface to be locked near a specific energy in the bandgap (often a charge-neutrality level) by a high density of interface states, making the barrier height of a metal-semiconductor contact relatively insensitive to the choice of metal.

## How It Works

Surface or interface states near mid-gap can accept or donate charge to keep the Fermi level near the neutrality energy regardless of the bulk doping or the metal work function. In III-V materials (especially GaAs), this pinning typically locks phi_Bn near 0.7-0.9 eV; in elemental Si, pinning is weaker and metal work function plays a larger role. Pinning can be partially relieved by clean / unpinning surface treatments (e.g., sulfur passivation of GaAs) or by ultrathin oxide interlayers (MIS Schottky).

## Key Parameters

- Surface-state density and energy distribution.
- Charge-neutrality level (material-specific).
- Metal work function (matters less when pinning is strong).

## When To Use

- Designing Schottky barriers on III-V materials.
- Engineering source/drain Schottky-barrier MOSFETs.
- Understanding why barrier heights differ between in-situ-clean and ex-situ contacts.

## Risks & Pitfalls

- Pinning makes barrier-height tailoring difficult; processes that remove or passivate states tend to be fragile.
- Variability in surface-state density yields run-to-run variation in phi_B.

## Related Concepts

- [[concepts/schottky-barrier]]
- [[concepts/mis-capacitor]]
- [[concepts/ohmic-contact]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
