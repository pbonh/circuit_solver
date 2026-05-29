---
title: Charge-Coupled Device (CCD)
type: claim
id: concepts/charge-coupled-device
tags:
- semiconductor
- device-physics
- photonic
- mosfet
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

A charge-coupled device is an array of MOS capacitors whose stored minority-carrier charge can be transferred between adjacent capacitors by clocked gate voltages. CCDs serve as imaging sensors (each capacitor is a pixel that integrates photo-generated charge) and as analog shift registers (delay lines).

## How It Works

Photo-generated electrons in a p-type substrate are collected under positively biased gate electrodes that form deep-depletion potential wells. After integration, multi-phase (2-, 3-, or 4-phase) clocking sequentially moves the charge packet through the array to an output amplifier. Charge-transfer efficiency (CTE) > 0.99999 per transfer is achieved with buried-channel structures that keep carriers away from the interface.

## Key Parameters

- Quantum efficiency, dark current.
- Charge-transfer efficiency.
- Full-well capacity and dynamic range.
- Read noise (set by the output amplifier).

## When To Use

- High-end imaging: scientific, astronomical, medical.
- Spectrometers and analog delay lines.

## Risks & Pitfalls

- Blooming when a pixel saturates (anti-blooming structures help).
- Cosmic-ray hits and radiation damage degrade CTE in space.
- Largely displaced in consumer imaging by CMOS image sensors.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/mosfet]]
- [[concepts/photodiode]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-18-chapter-13-photodetectors-and-solar-cells]]
