---
title: Thermistor
type: claim
id: claim-thermistor
tags:
- semiconductor
- device-physics
- sensor
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/19-chapter-14-sensors.txt
confidence:
  base: 0.65
---

## Definition

Sze & Ng (Sect. 14.2.1): "The name thermistor comes from *thermally sensitive resistor*. ... Thermistors usually imply semiconducting materials, and they are of two distinct classes: metal oxides and single-crystal semiconductors." Forms listed: beads, disks, washers, rods, probes, thin films. Metal-oxide thermistors are sintered from fine powders of materials including Mn₂O₃, NiO, Co₂O₃, Cu₂O, Fe₂O₃, TiO₂, and U₂O₃; for very high temperatures: Al₂O₃, BeO, MgO, ZrO₂, Y₂O₃, Dy₂O₃. Single-crystal Ge and Si thermistors are doped to 10¹⁶-10¹⁷ cm⁻³, sometimes with compensating dopants of a few percent.

## How It Works

In NTC thermistors, carrier density rises with T as more carriers are thermally excited above the bandgap. The temperature coefficient is typically several percent per K, far larger than the linear ~0.4 %/K of metallic RTDs. Sze notes that "the range of temperature sensing depends, to the first order, on the energy gap of the materials, that is, larger Eg for higher temperature": Ge thermistors cover the cryogenic range 1-100 K; Si is restricted to below 250 K (above which a positive temperature coefficient sets in); metal-oxide thermistors cover 200-700 K. Positive-temperature-coefficient (PTC) thermistors use ferroelectric ceramics whose resistance jumps near the Curie temperature.

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
