---
title: "CMOS Logic"
type: concept
tags: [semiconductor, device-physics, digital, mosfet, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: low
---

## Definition

Complementary metal-oxide-semiconductor (CMOS) logic uses paired n-channel and p-channel MOSFETs to build digital gates in which one network pulls the output high and the other pulls it low. Because exactly one of the two networks conducts statically, the static power dissipation is essentially zero, ideal only being limited by leakage.

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
- Latch-up of the parasitic p-n-p-n structure in CMOS wells (guard rings, deep wells, layout rules).
- Leakage scales exponentially with temperature and inversely with Vt.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/threshold-voltage]]
- [[concepts/subthreshold-conduction]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
