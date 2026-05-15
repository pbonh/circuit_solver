---
title: "Quantum-Cascade Laser"
type: concept
tags: [semiconductor, device-physics, photonic, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/17-chapter-12-leds-and-lasers.txt"]
confidence: low
---

## Definition

A quantum-cascade laser (QCL) is a semiconductor laser based on intersubband transitions (between confined states inside the same band) of a designer multi-quantum-well structure, rather than interband transitions across the bandgap. Each injected electron makes the same transition many times as it cascades through a periodic structure, producing one photon per stage.

## How It Works

Each stage consists of an injector region and an active region with carefully designed subband energies. An electron tunneling into the upper subband of the active region radiates a photon of energy h*nu = E_upper - E_lower (independent of the host bandgap) and then tunnels into the injector of the next stage. Tens of stages give multiple photons per electron and high quantum efficiency.

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
