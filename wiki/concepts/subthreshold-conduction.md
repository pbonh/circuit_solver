---
title: "Subthreshold Conduction"
type: concept
tags: [semiconductor, device-physics, mosfet, leakage, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/09-chapter-6-mosfets.txt"]
confidence: medium
---

## Definition

Subthreshold conduction is the drain current of a MOSFET in the weak-inversion regime where V_GS < Vt. The current depends exponentially on V_GS with a subthreshold slope S = (kT/q) ln(10)(1 + C_D/C_ox), in mV/decade, which approaches the kT-limited ideal of ~60 mV/dec at 300 K.

## How It Works

Below Vt the surface is depleted; the small minority-carrier density still provides a diffusion current whose source-end carrier concentration is set by the Boltzmann factor exp(q psi_s / kT). The capacitive divider between C_ox and the depletion capacitance C_D determines how much of V_GS reaches psi_s; the body-effect coefficient m = 1 + C_D/C_ox is the same one that appears in S.

## Key Parameters

- Subthreshold slope S (mV/dec).
- Off current at V_GS = 0 (sets standby leakage in CMOS).
- DIBL coefficient eta = -dVt/dV_DS (raises off-current at high V_DS).
- Body factor m = 1 + C_D/C_ox.

## When To Use

- Low-power circuit design (subthreshold logic, sensor analog front-ends).
- Setting Vt versus performance trade-off for digital ICs.
- Bandgap-like reference circuits exploiting the exponential I-V.

## Risks & Pitfalls

- Off-current grows exponentially with temperature; thermal-runaway considerations.
- Random-dopant fluctuation broadens the subthreshold characteristics from device to device.

## Related Concepts

- [[concepts/threshold-voltage]]
- [[concepts/short-channel-effects]]
- [[concepts/mosfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
