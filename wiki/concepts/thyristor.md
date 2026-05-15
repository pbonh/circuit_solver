---
title: "Thyristor"
type: concept
tags: [semiconductor, device-physics, power-device, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt"]
confidence: low
---

## Definition

A thyristor is a four-layer p-n-p-n semiconductor device that exhibits a bistable on/off characteristic. It can be triggered into a low-impedance conducting state by a brief gate current pulse and stays on until the main current drops below a holding value. Variants include the silicon-controlled rectifier (SCR), gate turn-off thyristor (GTO), MOS-controlled thyristor (MCT), and triac.

## How It Works

The p-n-p-n structure is equivalent to two cross-coupled BJTs (pnp + npn). When the loop gain alpha_pnp + alpha_npn approaches 1, regenerative feedback latches the device on. Re-blocking requires the main current to fall below the holding current (for SCR) or active forced commutation (for GTO/MCT).

## Key Parameters

- Forward breakover voltage and reverse blocking voltage.
- Holding and latching currents.
- On-state voltage drop and surge current rating.
- di/dt and dv/dt limits.

## When To Use

- High-voltage, high-current line-frequency switching: phase control, AC motor drives, induction heating, HVDC valves.

## Risks & Pitfalls

- Loss of forward blocking from rapid dv/dt or surge-induced latch-up.
- Slow turn-off compared to MOSFETs/IGBTs; commutation circuitry adds complexity.
- Susceptible to parasitic latch-up in CMOS circuits where unintentional p-n-p-n structures exist.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/bipolar-junction-transistor]]
- [[concepts/igbt]]
- [[concepts/semiconductor-device]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-11-part-iv-negative-resistance-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
