---
title: "Varactor Diode"
type: concept
tags: [semiconductor, device-physics, p-n-junction, rf, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/06-chapter-2-p-n-junctions.txt"]
confidence: medium
---

## Definition

A varactor (or varicap) diode is a reverse-biased p-n junction used as a voltage-controlled capacitance. Its depletion-layer capacitance C_D = eps_s / W decreases as reverse bias widens W, providing electronically tunable capacitance for oscillators, tuners, and parametric amplifiers.

## How It Works

By engineering the doping profile (abrupt for C ~ V^-1/2, linearly graded for V^-1/3, hyperabrupt for V^-2 or steeper), the C(V) law can be tailored over a wide range. The Q factor at high frequency is limited by the series resistance of the undepleted region; high-Q varactors use GaAs or InP to reduce series R.

## Key Parameters

- Capacitance ratio C_max / C_min over the operating bias range.
- Series resistance R_s and cutoff frequency f_c = 1/(2 pi R_s C_min).
- C(V) law (set by doping profile).

## When To Use

- Voltage-controlled oscillators (VCOs) in PLLs, RF tuners.
- Parametric amplifiers and frequency multipliers.

## Risks & Pitfalls

- Operating at forward bias accidentally destroys the device or detunes the circuit.
- Temperature drift in C(V) impacts oscillator phase noise.

## Related Concepts

- [[concepts/junction-capacitance]]
- [[concepts/p-n-junction]]
- [[concepts/depletion-region]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-06-chapter-2-p-n-junctions]]
