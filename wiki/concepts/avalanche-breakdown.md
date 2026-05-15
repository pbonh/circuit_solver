---
title: "Avalanche Breakdown"
type: concept
tags: [semiconductor, device-physics, p-n-junction, breakdown, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt"]
confidence: high
---

## Definition

Avalanche breakdown is the rapid increase of reverse current at a critical reverse bias V_BR, caused by impact-ionization-driven multiplication of carriers in the high-field depletion region. It is the dominant breakdown mechanism in lightly doped junctions where the depletion region is wide enough for carriers to accelerate to ionizing energies.

## How It Works

At sufficiently high reverse bias, the peak field reaches the critical field for impact ionization. A carrier traversing the depletion region creates electron-hole pairs at a rate alpha(E); each new carrier can in turn ionize others. The multiplication factor M -> infinity as the ionization integral approaches 1, defining V_BR. V_BR rises with the bandgap (wide-gap materials breakdown at higher V) and falls with doping (heavier doping -> narrower W -> higher field for given V).

## Key Parameters

- Doping N (sets W and field).
- Bandgap Eg (sets ionization threshold and critical field).
- Junction profile (abrupt, linearly graded).
- Temperature (alpha decreases with T at fixed E, so V_BR slightly increases).

## When To Use

- Avalanche photodiodes (controlled avalanche gain for sensitivity).
- IMPATT and TRAPATT microwave oscillators.
- Setting maximum operating voltage of any p-n junction device.

## Risks & Pitfalls

- Local hot spots can cause secondary breakdown and destructive failure.
- Non-uniform breakdown (edges, curvature) typically occurs well below planar V_BR; junction termination (guard rings, field plates) is essential.

## Related Concepts

- [[concepts/impact-ionization]]
- [[concepts/p-n-junction]]
- [[concepts/zener-breakdown]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
- [[summaries/sze-physics-semiconductor-devices-13-chapter-9-impatt-diodes]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
