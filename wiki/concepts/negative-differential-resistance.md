---
title: Negative Differential Resistance
type: claim
id: concepts/negative-differential-resistance
tags:
- semiconductor
- device-physics
- rf
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/12-chapter-8-tunnel-devices.txt
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/14-chapter-10-transferred-electron-and-real-space-transfer-devices.txt
confidence:
  base: 0.85
  source_count: 2
  contradicted: false
  effective: 0.884
  inputs_hash: 48379ae317c01c10
---

## Definition

Negative differential resistance (NDR) is a region of a device's I-V curve in which dI/dV is negative -- that is, increasing voltage decreases current. Devices that exhibit NDR over a useful frequency range can generate microwave power, sustain oscillations, behave as bistable memories, or perform multi-valued logic.

## How It Works

NDR arises from any mechanism by which available conduction states diminish as bias rises: band-to-band tunneling overlap shrinking (tunnel diodes), transfer of electrons from a high- to a low-mobility valley (transferred-electron / Gunn devices), transmission peaking through a resonant level (RTD), or transit-time / impact-ionization phasing (IMPATT/BARITT/TUNNETT diodes).

## Key Parameters

- Magnitude and width of the negative-resistance region.
- Frequency response (intrinsic time constants of the underlying mechanism).
- Peak-to-valley current or voltage ratio.

## When To Use

- Microwave oscillators (Gunn, IMPATT, tunnel, RTD).
- Bistable memories and Schmitt-trigger-like elements.
- Functional logic with reduced transistor count.

## Risks & Pitfalls

- Bias circuit must be stable: load-line must intersect the NDR region carefully or oscillation will occur.
- Many NDR devices are two-terminal, so circuit isolation is limited.

## Related Concepts

- [[concepts/tunnel-diode]]
- [[concepts/resonant-tunneling-diode]]
- [[concepts/impatt-diode]]
- [[concepts/transferred-electron-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-12-chapter-8-tunnel-devices]]
- [[summaries/sze-physics-semiconductor-devices-13-chapter-9-impatt-diodes]]
- [[summaries/sze-physics-semiconductor-devices-14-chapter-10-transferred-electron-and-real-space-transfer-devices]]
