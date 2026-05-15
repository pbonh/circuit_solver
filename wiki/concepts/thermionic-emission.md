---
title: "Thermionic Emission"
type: concept
tags: [semiconductor, device-physics, transport, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: medium
---

## Definition

Thermionic emission is the current of thermally excited majority carriers that cross a potential barrier whenever the barrier is thinner than the mean free path. It is the dominant transport mechanism in moderately doped Schottky diodes and in some heterojunction devices.

## How It Works

The Fermi-Dirac tail leaves a small but finite population of carriers with energies above the barrier height q phi_B. Integrating their density and velocity gives the Richardson-Dushman current J = A* T^2 exp(-q phi_B / kT) [exp(qV/kT) - 1], where A* = 4 pi q m* k^2 / h^3 is the effective Richardson constant. The shape of the barrier does not matter, only its peak height.

## Key Parameters

- Barrier height phi_B.
- Effective Richardson constant A*.
- Temperature T.
- Image-force lowering (and quantum-mechanical reflection corrections).

## When To Use

- Modeling Schottky-barrier I-V characteristics.
- Computing emitter injection in heterojunction BJTs.
- Predicting hot-cathode and vacuum-tube currents (the original Richardson context).

## Risks & Pitfalls

- Image-force barrier lowering reduces effective phi_B at high field.
- For very thin barriers, tunneling dominates and the simple thermionic formula underestimates current (thermionic-field emission).
- Effective mass and band structure must be used (rather than free-electron values).

## Related Concepts

- [[concepts/schottky-barrier]]
- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/effective-mass]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
- [[summaries/sze-physics-semiconductor-devices-07-chapter-3-metal-semiconductor-contacts]]
