---
title: IMPATT Diode
type: claim
id: concepts/impatt-diode
tags:
- semiconductor
- device-physics
- rf
- mm-wave
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/13-chapter-9-impatt-diodes.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

An IMPATT (impact-ionization avalanche transit-time) diode is a two-terminal microwave-power device that exploits avalanche-driven carrier injection followed by transit-time drift through a depleted region to produce a 180-deg phase lag of current behind voltage at microwave frequencies, yielding a useful AC negative resistance.

## How It Works

A reverse-biased p-n or Schottky junction with a heavily doped side carries an avalanche multiplication region; carriers injected by impact ionization drift across a low-field intrinsic region at the saturation velocity. The avalanche process contributes ~90 deg phase lag, the drift another ~90 deg, giving total ~180 deg between AC voltage and AC current and thus negative resistance at f ~ v_s / (2 L). Read's small-signal theory and Scharfetter-Gummel large-signal simulation describe the operation.

## Key Parameters

- Breakdown voltage and avalanche-region location.
- Drift-region length L (sets center frequency).
- Saturation velocity v_s.
- Thermal resistance from junction to heat sink.

## When To Use

- Millimeter-wave CW power sources (30-300 GHz): radar, communication, automotive.
- High-power pulsed sources at lower frequencies.

## Risks & Pitfalls

- Very noisy due to avalanche statistics; not suitable as a low-noise amplifier source.
- Thermal handling dominates packaging design.

## Related Concepts

- [[concepts/impact-ionization]]
- [[concepts/avalanche-breakdown]]
- [[concepts/negative-differential-resistance]]
- [[concepts/p-n-junction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-13-chapter-9-impatt-diodes]]
