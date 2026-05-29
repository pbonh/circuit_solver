---
title: Piezoresistivity
type: claim
id: concepts/piezoresistivity
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
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

> Sze & Ng Sect. 14.3 illustrates the use of piezoresistance (Fig. 6a, "Si pressure sensor") and Fig. 6b ("acceleration sensor") with diffused Si gauges where "the diaphragm is formed by chemical etching of the silicon substrate" and "the built-in diffused gauge monitors the piezoresistance under differential pressure", citing C.S. Smith, "Piezoresistance Effect in Germanium and Silicon" (Phys. Rev. 94, 42, 1954) as the founding reference.

## Definition

Piezoresistivity is the change in electrical resistivity of a semiconductor under mechanical stress, caused by stress-induced changes in the band structure (specifically the splitting and warping of conduction-band valleys). The gauge factor (relative resistance change per unit strain) is much larger in semiconductors -- 50 to 100 in single-crystal Si -- than in metals (~2).

## How It Works

Stress shifts the relative energies of the multiple conduction-band valleys, redistributing electrons among them. Because valley-specific masses differ, the average conductivity changes. The effect is anisotropic and depends on the crystal direction and stress orientation. Polysilicon strain gauges have lower but still useful gauge factors (~30).

## Key Parameters

- Gauge factor in the relevant orientation.
- Doping (lighter doping enhances gauge factor up to a saturation point).
- Temperature coefficient (large; requires compensation).
- Stress concentration in the diaphragm/cantilever geometry.

## When To Use

- MEMS pressure sensors (automotive, medical, industrial).
- Accelerometers and gyroscopes.
- Tactile sensors.

## Risks & Pitfalls

- Thermal drift requires bridge-circuit compensation.
- Wafer-bonding stress can shift the operating point.

## Related Concepts

- [[concepts/semiconductor-sensor]]
- [[concepts/carrier-mobility]]
- [[concepts/energy-band-structure]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
