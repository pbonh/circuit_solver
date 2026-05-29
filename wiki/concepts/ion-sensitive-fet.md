---
title: Ion-Sensitive FET (ISFET)
type: claim
id: concepts/ion-sensitive-fet
tags:
- semiconductor
- device-physics
- mosfet
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

## Definition

Sze & Ng (Sect. 14.5.2): "The ion-sensitive field-effect transistor (ISFET) is one of the most common of the chemically sensitive field-effect transistors. The ISFET was proposed and demonstrated by Bergveld in 1970 [reference 11-12]. Since the inclusion of a reference electrode in contact with the electrolyte was reported in 1974, such an electrode has been considered to be an integral part of an ISFET." The electrolyte plus reference electrode "becomes the gate of a MOSFET (Fig. 23), replacing the conventional poly-Si gate." Reference electrode is typically Ag-AgCl. Examples of ion-selective dielectric layers listed in the book: Si₃N₄, Al₂O₃, TiO₂, Ta₂O₅. Typical channel dimensions: tens to hundreds of microns.

## How It Works

The ion concentration in the solution sets the surface potential of the membrane via Nernstian equilibrium (D V_surface ~ 59 mV per decade of activity at 25 deg C for an ideal pH-sensitive membrane). This shifts the effective gate voltage and therefore the drain current. Operating at constant drain current and measuring the source-follower output gives a direct readout of the ion activity.

## Key Parameters

- Sensitivity (mV/pH or mV/decade of ion activity).
- Selectivity (cross-sensitivity to interfering ions).
- Drift rate (typically 1-5 mV/h).
- Hysteresis and lifetime.

## When To Use

- pH probes, glucose biosensors, ion-selective measurements in biomedical and environmental applications.
- Ion-imaging arrays for DNA sequencing (Ion Torrent).

## Risks & Pitfalls

- Reference-electrode drift is a major source of error.
- Membrane fouling in biological media degrades sensitivity over time.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/mis-capacitor]]
- [[concepts/semiconductor-sensor]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
