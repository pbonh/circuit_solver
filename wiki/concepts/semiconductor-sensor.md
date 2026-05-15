---
title: "Semiconductor Sensor"
type: concept
tags: [semiconductor, device-physics, sensor, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/19-chapter-14-sensors.txt"]
confidence: medium
---

## Definition

A semiconductor sensor is a device that uses a semiconductor as the transduction element to convert a non-electrical input -- thermal, mechanical, magnetic, optical, or chemical -- into an electrical output signal (current, voltage, charge, capacitance, or frequency). The semiconductor process enables monolithic integration with signal-conditioning electronics, providing small size, high sensitivity, and low cost.

## How It Works

Different physical mechanisms are used for different signal domains: T modifies V_BE, n_i, and resistivity; strain modifies the band structure (piezoresistance); B deflects carriers by the Lorentz force (Hall effect, magnetoresistance); chemical environments adsorb on a sensitive layer that modifies a transistor's gate. Signal conditioning -- amplification, offset trimming, temperature compensation -- is integrated on the same chip.

## Key Parameters

- Sensitivity (output / input).
- Noise floor and resolution.
- Linearity, hysteresis, drift.
- Cross-sensitivity to unwanted parameters (e.g., temperature drift of a pressure sensor).

## When To Use

- Automotive, consumer-electronics, industrial, medical sensing.
- Any application where small, monolithically-integrated, low-cost sensing is desired.

## Risks & Pitfalls

- Packaging often dominates cost and reliability.
- Temperature compensation is universally required.
- Long-term drift and aging.

## Related Concepts

- [[concepts/hall-effect]]
- [[concepts/piezoresistivity]]
- [[concepts/ion-sensitive-fet]]
- [[concepts/mosfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-16-part-v-photonic-devices-and-sensors]]
- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
