---
title: 'Physics of Semiconductor Devices (Sze & Ng, 3rd ed.) — Chapter 14: Sensors'
type: source
id: source-sze-physics-semiconductor-devices-19-chapter-14-sensors
kind: derived-summary
tags:
- semiconductor
- device-physics
- sensor
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/19-chapter-14-sensors.txt
---

## Key Points

- A sensor is a device that detects or measures an external signal. Semiconductor implementations exist for all four signal classes covered in this chapter -- thermal, mechanical, magnetic, and chemical -- complementing the electrical (Chapters 2-11) and optical (Chapters 12-13) sensors covered earlier.
- Thermal sensors: thermistor (resistance varies with T per a Steinhart-Hart-like exponential), diode thermal sensor (V_BE at constant I varies linearly with T at about -2 mV/K), transistor thermal sensor (more accurate via DV_BE proportional to T -- the basis of bandgap references and on-chip temperature sensors).
- Mechanical sensors: piezoresistive strain gauge in silicon (gauge factor 50-100 vs 2 for metal-foil gauges); MEMS pressure sensors, accelerometers, and gyroscopes use silicon micromachined diaphragms / proof masses with piezoresistive or capacitive readout. Interdigital transducer (IDT) on piezoelectric substrates implements surface-acoustic-wave (SAW) sensors and filters.
- Magnetic sensors: Hall plate (Hall voltage proportional to B and bias current); magnetoresistor (resistance increases with B^2 in a transverse geometry); magnetodiode (asymmetric diode whose I-V depends on B via Lorentz deflection of carriers); magnetotransistor (BJT with B-dependent emitter-collector path); magnetic-field-sensitive FET (MAGFET) with split drain; carrier-domain magnetic-field sensor.
- Chemical sensors: metal-oxide gas sensors (SnO2, ZnO) whose grain-boundary resistance changes with adsorbed gas; ion-sensitive FET (ISFET) where the metal gate is replaced by an electrolyte and an ion-selective membrane, making V_T sensitive to ion activity (e.g., pH); catalytic-metal sensors (Pd gate FET for hydrogen detection); biosensors (immobilized enzymes or antibodies on the gate convert biological recognition into a measurable Vt shift).
- Sensitivity, noise, drift, hysteresis, linearity, and cross-sensitivity are the universal sensor figures of merit. Integration with on-chip MOSFET amplifiers is the central advantage of semiconductor sensors over discrete competitors.

## Relevant Concepts

- [[concepts/semiconductor-sensor]]
- [[concepts/hall-effect]]
- [[concepts/piezoresistivity]]
- [[concepts/ion-sensitive-fet]]
- [[concepts/thermistor]]
- [[concepts/bandgap]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/mosfet]]

## Source Metadata

- Source type: book chapter
- Book title: Physics of Semiconductor Devices, 3rd Edition
- Chapter: Chapter 14 — Sensors
- File path: `raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/19-chapter-14-sensors.txt`
- Authors: S. M. Sze and Kwok K. Ng (John Wiley & Sons, 2006)
