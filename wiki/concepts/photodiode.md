---
title: Photodiode
type: claim
id: concepts/photodiode
tags:
- semiconductor
- device-physics
- photonic
- p-n-junction
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/18-chapter-13-photodetectors-and-solar-cells.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

A photodiode is a reverse-biased p-n (or p-i-n, Schottky, or heterojunction) diode in which absorbed photons generate electron-hole pairs that are swept out of the depletion region as a photocurrent. The output photocurrent is proportional to the incident optical power, providing the elementary optical-to-electrical transducer used in detectors, receivers, image sensors, and instrumentation.

## How It Works

Photons with h*nu > Eg generate electron-hole pairs by interband absorption (alpha(lambda) sets absorption depth). Pairs generated within or within one diffusion length of the depletion region are collected as photocurrent I_ph = eta q P_opt / (h*nu). Quantum efficiency eta = (1 - R)(1 - exp(-alpha W)) for a depletion-region-limited diode. Reverse bias minimizes capacitance and transit time, enabling high bandwidth.

## Key Parameters

- Quantum efficiency eta and responsivity R (A/W).
- Dark current (limits sensitivity).
- Bandwidth (set by RC and transit time).
- Noise-equivalent power (NEP) and specific detectivity D*.

## When To Use

- Fiber-optic receivers, especially in p-i-n form for telecom wavelengths.
- Optical communications, sensing, instrumentation.
- Pixel of CMOS / CCD image sensors (pinned-photodiode).

## Risks & Pitfalls

- High reverse bias can drive into avalanche unintentionally.
- Surface generation increases dark current; passivation is important.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/p-i-n-diode]]
- [[concepts/avalanche-photodiode]]
- [[concepts/charge-coupled-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
