---
title: Quantum-Well Infrared Photodetector (QWIP)
type: claim
id: claim-quantum-well-infrared-photodetector
tags:
- semiconductor
- device-physics
- photonic
- heterojunction
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt
confidence:
  base: 0.65
---

## Definition

Per Sze & Ng (Sect. 13.8): the first functional QWIP "based on bound-to-bound intersubband transition in a GaAs/AlGaAs heterostructure, was realized by Levine et al. and Choi et al. in 1987. The same group also presented improved detector results on bound-to-continuum transition in 1988. Another type of transition, bound-to-miniband had been observed in 1991." Absorbed long-wave-infrared photons excite confined electrons from a bound subband to an extended or higher-bound state, from which they can be swept out by an applied bias as photocurrent.

## How It Works

A periodic GaAs/AlGaAs (or InGaAs/InP) multiple-quantum-well stack is designed so that the subband spacing matches the desired wavelength (typically 8-12 µm for thermal imaging). Per Sze Fig. 37 the GaAs quantum-well layers are "about 5 nm and are usually doped to n-type in the 10¹⁷ cm⁻³ range. The barrier layers are undoped and have a thickness in the range of 30-50 nm." Selection rules forbid normal-incidence absorption; Sze illustrates the two standard coupling solutions (Fig. 37a/b): "Light is incident normal to a polished facet making a 45° angle to the quantum well" or "A grating is used to refract light coming from the substrate."

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
