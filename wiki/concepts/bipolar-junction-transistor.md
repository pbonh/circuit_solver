---
title: "Bipolar Junction Transistor (BJT)"
type: concept
tags: [semiconductor, device-physics, bjt, analog, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/02-introduction.txt"]
confidence: medium
---

## Definition

A bipolar junction transistor is a three-terminal device formed by two closely coupled p-n junctions (npn or pnp) sharing a thin central base region. Base-emitter forward bias injects minority carriers that are collected by the reverse-biased base-collector junction, yielding current gain.

## How It Works

In an npn device, electrons injected from the n-emitter diffuse across the narrow p-base, where most reach the depleted base-collector junction and are swept into the collector. The collector current scales exponentially with V_BE; small base currents control large collector currents (current gain beta).

## Key Parameters

- Forward current gain beta_F (or h_FE).
- Cutoff frequency f_T, transition frequency.
- Early voltage V_A (output resistance).
- Saturation current Is and ideality factors.
- Breakdown voltages BV_CEO, BV_CBO.

## When To Use

- Analog amplifiers needing high transconductance, low input-referred noise.
- High-speed circuits (HBT in RF and mm-wave applications).
- Driving high currents or where exponential I-V is desired (bandgap references, log amplifiers).

## Risks & Pitfalls

- Thermal runaway if not properly biased or paralleled.
- Secondary breakdown at high V_CE, I_C combinations.
- Charge storage limits switching speed.

## Related Concepts

- [[concepts/p-n-junction]]
- [[concepts/carrier-lifetime]]
- [[concepts/semiconductor-device]]
- [[concepts/mosfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-02-introduction]]
- [[summaries/sze-physics-semiconductor-devices-15-chapter-11-thyristors-and-power-devices]]
- [[summaries/sze-physics-semiconductor-devices-19-chapter-14-sensors]]
