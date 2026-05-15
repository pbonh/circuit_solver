---
title: "Polysilicon Gate"
type: concept
tags: [semiconductor, device-physics, mosfet, process, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: low
---

## Definition

A polysilicon (poly-Si) gate is a MOSFET gate electrode made of heavily doped polycrystalline silicon. Introduced in the 1970s, it replaced aluminum gates and enabled the self-aligned source/drain process that defined modern CMOS until metal-gate / high-k stacks displaced it at the 45/32 nm nodes.

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

- Polysilicon-gate depletion adds ~3-5 A to effective oxide thickness.
- Boron penetration through gate oxide for p+ poly in pMOS limits aggressive thinning of SiO2 without nitridation.

## Related Concepts

- [[concepts/mosfet]]
- [[concepts/mis-capacitor]]
- [[concepts/threshold-voltage]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
