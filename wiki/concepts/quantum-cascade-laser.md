---
title: "Quantum-Cascade Laser"
type: concept
tags: [semiconductor, device-physics, photonic, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt"]
confidence: medium
---

## Definition

Sze & Ng (Sect. 12.6.3): "In a quantum cascade laser, the electron transition to emit a photon is between quantized subband energy levels, created by a quantum well or superlattice, within the same conduction band (Fig. 47). The major difference is intersubband transition as opposed to interband transition in a regular laser. Since the transition between subbands is much smaller than the energy gap, the quantum cascade laser is capable of lasing in long wavelengths, without facing the material difficulties of very narrow energy gap ... Wavelengths beyond 70 µm have been achieved. Besides, the wavelength is tunable by the quantum-well thickness without being fixed by the energy gap."

## How It Works

Per Sze: "The active region is composed of multiple quantum wells or a superlattice. The most-common design is between two to three quantum wells. In the active region, electrons are injected through resonant tunneling, to the sublevel E₃. ... The radiative transition between E₃ and E₂ is responsible for lasing. Electrons in E₂ relax to E₁ and then tunnel to the miniband of the succeeding injector through resonant tunneling..." Tens of stages give multiple photons per electron and high quantum efficiency.

## Key Parameters

- Subband-spacing engineering (set by layer design).
- Number of cascade stages.
- Operating wavelength (mid-IR to THz).
- Threshold current density (often kA/cm^2).

## When To Use

- Mid-IR / far-IR / terahertz coherent sources.
- Gas-sensing spectroscopy (CO2, CH4, NO, NH3 absorption lines).
- Standoff chemical detection.

## Risks & Pitfalls

- High threshold currents and operating voltages (often tens of V) demand careful thermal management.
- Continuous-wave operation often requires cryogenic cooling, especially for THz QCLs.

## Related Concepts

- [[concepts/semiconductor-laser]]
- [[concepts/quantum-well]]
- [[concepts/heterojunction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-17-chapter-12-leds-and-lasers]]
