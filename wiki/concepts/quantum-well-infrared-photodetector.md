---
title: "Quantum-Well Infrared Photodetector (QWIP)"
type: concept
tags: [semiconductor, device-physics, photonic, heterojunction, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt"]
confidence: low
---

## Definition

A QWIP is a semiconductor infrared photodetector based on intersubband absorption inside multiple quantum wells. Absorbed long-wave-infrared photons excite confined electrons from a bound subband to an extended or higher-bound state, from which they can be swept out by an applied bias as photocurrent.

## How It Works

A periodic GaAs/AlGaAs (or InGaAs/InP) multiple-quantum-well stack is designed so that the subband spacing matches the desired wavelength (typically 8-12 um for thermal imaging). Selection rules forbid normal-incidence absorption; QWIP arrays use diffraction gratings or random scatterers on top to provide a vertical component of the optical electric field.

## Key Parameters

- Subband-spacing energy (sets cutoff wavelength).
- Well width and barrier height.
- Operating temperature (typically 70-80 K for thermal imaging).
- Spectral bandwidth, dark current, photoconductive gain.

## When To Use

- Long-wave-IR imaging arrays (thermal cameras).
- Spectroscopic imaging in mid-IR.

## Risks & Pitfalls

- High dark current at room temperature limits applications without cooling.
- HgCdTe detectors offer higher D* but harder to manufacture in large uniform arrays.

## Related Concepts

- [[concepts/quantum-well]]
- [[concepts/heterojunction]]
- [[concepts/photoconductor]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
