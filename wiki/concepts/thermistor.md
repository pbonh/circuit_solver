---
title: "Thermistor"
type: concept
tags: [semiconductor, device-physics, sensor, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/19-chapter-14-sensors.txt"]
confidence: low
---

## Definition

A thermistor is a semiconductor temperature sensor whose resistance varies strongly with temperature. Negative-temperature-coefficient (NTC) thermistors -- typically polycrystalline metal-oxide ceramics -- show resistance decreasing exponentially with T per R(T) = R_0 exp(B (1/T - 1/T_0)), where B is the material constant (Steinhart-Hart equation).

## How It Works

In NTC thermistors, carrier density rises with T as more carriers are thermally excited above the bandgap of the semiconducting oxide or as more dopants ionize. The temperature coefficient is typically several percent per K, far larger than the linear ~0.4%/K of metallic RTDs. Positive-temperature-coefficient (PTC) thermistors use ferroelectric ceramics whose resistance jumps near the Curie temperature.

## Key Parameters

- Resistance at 25 deg C (R_25, kohms or Mohms).
- Beta coefficient B (Kelvin).
- Operating range and dissipation constant.

## When To Use

- Temperature measurement in consumer electronics and HVAC.
- Inrush-current limiting (NTC) and self-regulating heaters (PTC).
- Battery-pack temperature monitoring.

## Risks & Pitfalls

- Nonlinear; usually digitized with a lookup table.
- Self-heating from probe current introduces offsets.

## Related Concepts

- [[concepts/semiconductor-sensor]]
- [[concepts/carrier-concentration]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
