---
title: Resonant-Tunneling Diode (RTD)
type: claim
id: concepts/resonant-tunneling-diode
tags:
- semiconductor
- device-physics
- heterojunction
- tunneling
- rf
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/12-chapter-8-tunnel-devices.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A resonant-tunneling diode is a double-barrier heterostructure that encloses a quantum well between two thin tunneling barriers. Transmission is large only when the incoming carrier energy aligns with a quasi-bound state in the well; sweeping the bias produces a sharp peak-and-valley I-V with very high peak-to-valley ratio and ultra-fast intrinsic switching speed.

## How It Works

Two heterojunction barriers (e.g., AlAs / GaAs / AlAs) form a quantum well with energies E_1, E_2, .... Carriers tunnel into and out of the quasi-bound states; transmission is dramatically enhanced by resonance (analogous to a Fabry-Perot cavity). As applied bias misaligns the emitter Fermi sea with the resonant level, current drops, producing negative differential resistance.

## Key Parameters

- Barrier height (band offset) and width.
- Well width (sets resonance energy).
- Peak-to-valley current ratio.
- Cutoff/oscillation frequency (can exceed 1 THz).

## When To Use

- Sub-THz oscillators and detectors.
- Multi-state logic (peak/valley combinations).
- A/D converter folding/interpolation stages.

## Risks & Pitfalls

- Sensitive to barrier-thickness variation (exponential).
- Bistability and hysteresis in some bias regimes.

## Related Concepts

- [[concepts/quantum-mechanical-tunneling]]
- [[concepts/heterojunction]]
- [[concepts/quantum-well]]
- [[concepts/negative-differential-resistance]]
- [[concepts/tunnel-diode]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-12-chapter-8-tunnel-devices]]
