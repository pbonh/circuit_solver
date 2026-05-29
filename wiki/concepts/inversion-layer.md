---
title: Inversion Layer
type: claim
id: concepts/inversion-layer
tags:
- semiconductor
- device-physics
- mosfet
- surface-physics
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

An inversion layer is a thin (~1-3 nm) sheet of minority carriers at the semiconductor surface of an MIS capacitor that forms when the gate voltage exceeds the threshold voltage Vt. In an n-channel MOSFET on a p-type body, the inversion layer is an electron sheet; in a p-channel MOSFET on an n-type body, it is a hole sheet. The inversion layer is the conducting channel of the MOSFET.

## How It Works

Beyond Vt the surface band bending stops growing and additional gate charge is balanced almost entirely by inversion-layer charge Q_n = -C_ox (V_G - Vt). The carriers are quantum-mechanically confined in a triangular potential well, leading to discrete subbands and a finite layer thickness that contributes ~3-5 A to the effective oxide thickness (poly-depletion plus inversion-layer thickness penalty).

## Key Parameters

- Sheet charge Q_n.
- Surface mobility mu_s (lower than bulk due to surface scattering).
- Quantization energies of the surface well.

## When To Use

- Determining drain current I_D = (W/L) mu_s C_ox (V_GS - Vt)^2 / 2 (saturation) and similar expressions.
- Modeling channel-length modulation, velocity saturation.

## Risks & Pitfalls

- Surface roughness scattering reduces mobility at high fields (universal-mobility-curve).
- Inversion-layer thickness must be accounted for in C-V interpretation at scaled oxides.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/threshold-voltage]]
- [[concepts/mosfet]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
