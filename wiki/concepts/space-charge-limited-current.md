---
title: "Space-Charge-Limited Current"
type: concept
tags: [semiconductor, device-physics, transport, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/04-chapter-1-physics-and-properties-of-semiconductors-a-review.txt"]
confidence: medium
---

## Definition

Space-charge-limited current is the drift current carried by injected carriers whose own charge dominates the local electric-field profile, so that the I-V relation departs from Ohm's law. It is observed in lightly doped or thin-film devices, vacuum diodes, and organic semiconductors.

## How It Works

Injected carriers form a space charge governed by Poisson's equation, and the local field self-consistently drives the current. In the low-field mobility regime (constant mu), the Mott-Gurney law gives J = (9/8) eps_s mu V^2 / L^3. In the velocity-saturation regime, J = 2 eps_s v_s V / L^2. In the ballistic (collisionless) regime the Child-Langmuir law gives J ~ V^(3/2) / L^2.

## Key Parameters

- Sample length L.
- Mobility mu or saturation velocity v_s.
- Dielectric permittivity eps_s.
- Applied voltage V.

## When To Use

- Diagnosing transport in organic LEDs, amorphous thin films, and lightly doped vacuum diodes.
- Modeling vertical thin-film transistors and high-voltage drift regions.

## Risks & Pitfalls

- Trap-filled-limit regions complicate the simple power-law behavior.
- Contact injection limits must be checked before attributing observed J(V) to space charge.

## Related Concepts

- [[concepts/poisson-equation]]
- [[concepts/drift-diffusion-equation]]
- [[concepts/carrier-mobility]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-04-chapter-1-physics-and-properties-of-semiconductors-a-review]]
