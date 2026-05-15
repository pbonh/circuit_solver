---
title: "Two-Dimensional Electron Gas (2DEG)"
type: concept
tags: [semiconductor, device-physics, heterojunction, quantum, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/10-chapter-7-jfets-mesfets-and-modfets.txt"]
confidence: medium
---

## Definition

A two-dimensional electron gas is an electron population confined in one spatial direction to a layer of nanometer-scale thickness (typically by a heterojunction or MOSFET inversion layer) so that motion is quantized in that direction. The electrons occupy discrete subbands and form a strictly 2-D system with step-function density of states.

## How It Works

In a MODFET / HEMT, the conduction-band discontinuity between a wide-gap donor layer and an undoped narrow-gap channel forms a triangular potential well. Electrons from the donors spill into the well and occupy quantized subbands at the interface. Because the dopants reside in the barrier layer and the channel is undoped, ionized-impurity scattering is largely eliminated and mobility is high. The 2DEG sheet density is electrostatically controlled by a gate above.

## Key Parameters

- Sheet density n_s (10^11 - 10^13 cm^-2 typical).
- Subband energies (set by well depth and channel-effective mass).
- Mobility (8000+ cm^2/V-s in GaAs at 300 K; >10^6 at 4 K).

## When To Use

- HEMT and MODFET RF devices.
- Quantum Hall and mesoscopic-physics experiments.
- Low-T high-mobility electron systems for quantum-information research.

## Risks & Pitfalls

- 2DEG density depends sensitively on surface traps in the barrier layer (HEMT current collapse).
- Persistent photoconductivity and DX-center effects in AlGaAs.

## Related Concepts

- [[concepts/heterojunction]]
- [[concepts/modfet]]
- [[concepts/quantum-well]]
- [[concepts/inversion-layer]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-10-chapter-7-jfets-mesfets-and-modfets]]
