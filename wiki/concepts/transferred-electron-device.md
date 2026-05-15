---
title: "Transferred-Electron Device (Gunn Diode)"
type: concept
tags: [semiconductor, device-physics, rf, mm-wave, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/14-chapter-10-transferred-electron-and-real-space-transfer-devices.txt"]
confidence: medium
---

## Definition

A transferred-electron device (TED), also called a Gunn diode, is a bulk two-terminal microwave oscillator that uses the intervalley scattering of conduction electrons between a high-mobility low-energy (Gamma) valley and a low-mobility higher-energy (L or X) satellite valley to produce a region of negative differential mobility and self-sustained current oscillations.

## How It Works

Above a threshold field (~3.3 kV/cm in GaAs), electrons gain enough energy to scatter into the satellite valley, where their effective mass is large and their drift velocity drops. The population shift makes the average v_d(E) curve decrease with E -- negative differential mobility (NDM). A high-field domain nucleates at the cathode, propagates to the anode at saturation velocity, and then a new domain forms, producing oscillation at f = v_s / L. Operating modes include transit-time, delayed-domain, quenched-domain, and LSA (limited-space-charge accumulation).

## Key Parameters

- Intervalley separation (~0.3 eV in GaAs).
- Threshold field for NDM.
- Sample length L (sets transit-time frequency).
- Saturation velocity v_s.

## When To Use

- CW microwave oscillators at X, Ku, Ka, and W bands (typically a few mW to several W).
- Local oscillators in automotive radar, microwave receivers, and Doppler sensors.

## Risks & Pitfalls

- Domain-formation depends sensitively on doping uniformity (n L product must exceed ~10^12 cm^-2 to allow domain formation).
- High operating field puts thermal stress on the device.

## Related Concepts

- [[concepts/negative-differential-resistance]]
- [[concepts/carrier-mobility]]
- [[concepts/energy-band-structure]]
- [[entities/gallium-arsenide]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-14-chapter-10-transferred-electron-and-real-space-transfer-devices]]
