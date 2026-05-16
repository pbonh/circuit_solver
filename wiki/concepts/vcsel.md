---
title: "VCSEL (Vertical-Cavity Surface-Emitting Laser)"
type: concept
tags: [semiconductor, device-physics, photonic, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt"]
confidence: medium
---

> Sze & Ng (Sect. 12.6.2): "In a surface-emitting laser, the light output is orthogonal to the active layer (heterointerfaces) and the semiconductor surface ... the optical cavity is now defined by planes parallel to the heterointerfaces ... formed by two distributed-Bragg reflectors (DBRs) surrounding the active layer. These DBRs have high reflectivity larger than 90%. The high reflectivity is required since the optical gain per pass is small due to the small optical cavity compared to an edge-emitting laser." Active layer is "usually formed by multiple quantum wells." Advantages enumerated: low threshold current, single-mode lasing (wide mode separation per Eq. 43), 2-D laser array realisation, ease of fiber coupling, IC-process compatibility, high-volume / low-cost production, on-wafer testing.

## Definition

A vertical-cavity surface-emitting laser is a semiconductor laser whose optical cavity is oriented perpendicular to the wafer surface, formed by two high-reflectivity distributed Bragg reflectors (DBRs) above and below a thin quantum-well active region. Light exits through the top surface, enabling on-wafer testing and 2-D array fabrication.

## How It Works

A short (~1 wavelength) cavity sandwiched between two DBRs (each ~20 pairs of AlAs/GaAs giving R > 99.9%) confines the lasing mode vertically. Carriers are injected through the DBRs (often current-confined by oxide apertures or proton implantation). The short cavity gives single longitudinal mode operation with small free-spectral range that pushes adjacent modes out of the gain bandwidth.

## Key Parameters

- DBR reflectivity (must be > 99% to overcome short active-region gain).
- Aperture diameter (sets single-mode operation and threshold).
- Operating wavelength (typically 850 nm GaAs/AlGaAs; 980 nm, 1300 nm with InGaAs(N)).

## When To Use

- Short-range data communication (multimode fiber, AOC cables, datacenter).
- Optical sensors (mice, gesture recognition).
- 2-D arrays for LIDAR illumination.

## Risks & Pitfalls

- Single-mode operation requires small apertures, limiting power.
- Wavelength tuning with current/temperature is small but useful for sensing.

## Related Concepts

- [[concepts/semiconductor-laser]]
- [[concepts/quantum-well]]
- [[concepts/heterojunction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
