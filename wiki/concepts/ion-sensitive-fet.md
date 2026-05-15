---
title: "Ion-Sensitive FET (ISFET)"
type: concept
tags: [semiconductor, device-physics, mosfet, sensor, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/19-chapter-14-sensors.txt"]
confidence: low
---

## Definition

An ion-sensitive field-effect transistor is a MOSFET in which the metal gate electrode is replaced by an electrolyte solution contacted by a reference electrode, with an ion-selective membrane (e.g., Si3N4, Ta2O5, glass, or polymer) on top of the gate insulator. Charge accumulation at the membrane-electrolyte interface shifts the threshold voltage, providing a continuous electrochemical-to-electrical transduction.

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
