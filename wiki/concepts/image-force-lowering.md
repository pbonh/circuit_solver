---
title: "Image-Force Lowering"
type: concept
tags: [semiconductor, device-physics, schottky-barrier, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/07-chapter-3-metal-semiconductor-contacts.txt"]
confidence: medium
---

## Definition

Image-force (or Schottky) barrier lowering is the bias-dependent reduction of the effective barrier height of a metal-semiconductor or metal-vacuum junction caused by the image-charge attraction between an emerging electron and the metal surface.

## How It Works

An electron at distance x from a metal surface experiences an attractive force toward an image charge of opposite sign at -x. The induced potential lowers the apparent barrier maximum by D phi = sqrt(q E_m / (4 pi eps_s)), where E_m is the field at the surface. The result is a soft reverse-bias I-V that does not truly saturate, and a barrier that varies as the 1/4 power of the field.

## Key Parameters

- Field at the metal surface E_m (rises with reverse bias and doping).
- Dielectric permittivity (often the optical, not the static, permittivity for fast electrons).
- Image-charge plane location (often within an angstrom of the geometric metal surface).

## When To Use

- Modeling Schottky-diode reverse leakage.
- Extracting true barrier height from C-V vs I-V data: I-V gives effective phi_B reduced by image-force lowering.

## Risks & Pitfalls

- Image-force lowering is a small effect at modest reverse bias but grows with sqrt(V).
- Underestimates the lowering if the wrong permittivity is used.

## Related Concepts

- [[concepts/schottky-barrier]]
- [[concepts/thermionic-emission]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
