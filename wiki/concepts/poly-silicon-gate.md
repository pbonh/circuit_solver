---
title: Polysilicon Gate
type: claim
id: concepts/poly-silicon-gate
tags:
- semiconductor
- device-physics
- mosfet
- process
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

A polysilicon (poly-Si) gate is a MOSFET gate electrode made of polycrystalline silicon. Sze & Ng (Sect. 6.5.2 "Gate Stack") name the three properties that made it the workhorse gate material for decades: "compatibility with the silicon processing", the "ability to withstand high-temperature anneal that is required after self-aligned source/drain implantation", and the fact that "the work function can be varied by doping it into n-type and p-type. Such flexibility is crucial for a symmetric CMOS technology."

## How It Works

Polysilicon is deposited by LPCVD over the gate oxide and patterned by lithography. It is then doped (n+ for nMOS, p+ for pMOS) by ion implantation, often in the same step as source/drain doping (the source/drain implants are aligned to the gate edge, giving "self-aligned" structures). The poly-Si work function is set by doping: n+ poly for nMOS gives V_FB ~ -0.95 V; p+ poly for pMOS gives V_FB ~ +0.95 V.

## Key Parameters

- Doping (n+ or p+) and dopant species (As, B, BF2).
- Grain size (smaller = higher resistance; large = better device matching).
- Sheet resistance after silicidation.

## When To Use

- Bulk CMOS technologies up to roughly 65 nm.
- Memory and analog devices where metal gates are not cost-justified.

## Risks & Pitfalls

- Two limitations enumerated by Sze (Sect. 6.5.2): (1) "relatively high resistance" — does not penalise DC characteristics because the gate terminates on the gate insulator, but the penalty "shows up in high-frequency parameters such as noise and f_max"; (2) "finite depletion width at the oxide interface ... reduces the effective gate capacitance and becomes more severe with thinner oxides".
- To circumvent both, Sze names "silicides and metals" as obvious replacement gate materials, with "potential candidates ... TiN, TaN, W, Mo, and NiSi".
- Boron penetration through gate oxide for p+ poly in pMOS limits aggressive thinning of SiO₂ without nitridation.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/mis-capacitor]]
- [[concepts/threshold-voltage]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
