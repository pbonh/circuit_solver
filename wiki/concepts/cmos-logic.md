---
title: CMOS Logic
type: claim
id: concepts/cmos-logic
tags:
- semiconductor
- device-physics
- digital
- mosfet
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Complementary metal-oxide-semiconductor (CMOS) logic uses paired n-channel and p-channel MOSFETs to build digital gates. Sze & Ng (Sect. 6.6.1, Fig. 41a) describe the canonical inverter as "by far the most common ... where both n-channel and p-channel transistors are used. This logic consumes very low dc power because when the input is either high or low, one of the transistors in series is off so that there is very little steady-state current (subthreshold current) passing through them."

## How It Works

In a basic CMOS inverter, a pMOS connects V_DD to the output and an nMOS connects the output to ground; the two gates are tied together as the input. When the input is low, pMOS is on and nMOS is off; the output is V_DD. When input is high, the reverse holds. NAND and NOR gates are built by parallel/series combinations of pMOS and nMOS networks.

## Key Parameters

- Supply voltage V_DD.
- nMOS / pMOS Vt for noise margin.
- Capacitive load and drive strength (W/L of each device).
- Subthreshold leakage and gate-leakage currents.

## When To Use

- Essentially all modern digital ICs.
- Mixed-signal blocks (level shifters, transmission gates).

## Risks & Pitfalls

- Short-circuit current flows during input transitions; managed by sharp transitions.
- Latch-up of the parasitic p-n-p-n structure in CMOS wells (Sze Sect. 6.5.4: junction isolation in planar CMOS does *not* eliminate latch-up; only [[concepts/silicon-on-insulator]] technology does). Guard rings, deep wells, and layout rules are the bulk-CMOS countermeasures.
- Leakage scales exponentially with temperature and inversely with Vt.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/threshold-voltage]]
- [[concepts/subthreshold-conduction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
