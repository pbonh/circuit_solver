---
title: Threshold Voltage
type: claim
id: claim-threshold-voltage
tags:
- semiconductor
- device-physics
- mosfet
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/PhysicsOfSemiconductorDevices3rdEdition-S.M.SzeAndKwokK.Ng/_txt/08-chapter-4-metal-insulator-semiconductor-capacitors.txt
confidence:
  base: 0.85
---

## Definition

The threshold voltage Vt of a MOSFET (or MIS capacitor) is the gate voltage at which strong inversion is reached and a conducting minority-carrier channel forms at the semiconductor surface. For an n-channel device on a p-type substrate, Vt = V_FB + 2 phi_B + sqrt(2 eps_s q N_a (2 phi_B)) / C_ox, where V_FB is the flatband voltage, phi_B = (kT/q) ln(N_a/n_i) is the Fermi-level depth from intrinsic, N_a the substrate doping, and C_ox the oxide capacitance per unit area.

## How It Works

As V_G is increased from V_FB the surface bands bend downward. Once the band bending equals 2 phi_B, the minority-carrier density at the surface equals the bulk majority-carrier density, and any further increase in V_G is screened by the inversion charge (rather than expanding the depletion width). Below Vt, drain current decays exponentially (subthreshold regime). Body bias V_BS modulates Vt (body effect): Vt(V_BS) increases as |V_BS| increases.

## Key Parameters

- Substrate doping N_a (heavier doping raises Vt and the body effect).
- Oxide capacitance C_ox = eps_ox / t_ox (thinner oxide gives steeper coupling, lower Vt).
- Work-function difference phi_MS (sets V_FB).
- Oxide and interface charges Q_f, Q_m, Q_ot, Q_it (shift V_FB).
- Body bias V_BS (body-effect coefficient gamma).

## When To Use

- Setting MOSFET design point (low Vt for high speed, high Vt for low leakage).
- Compact-model parameter for SPICE BSIM, EKV.
- Tuning circuit-level performance (multi-Vt libraries in CMOS).

## Risks & Pitfalls

- Variability: random dopant fluctuation and line-edge roughness cause Vt scatter at scaled nodes.
- Short-channel effects (DIBL) reduce effective Vt at high V_DS.
- Quantum confinement of inversion charge shifts the centroid and modifies the relationship.

## Related Concepts

- [[concepts/mis-capacitor]]
- [[concepts/mosfet]]
- [[concepts/inversion-layer]]
- [[concepts/flatband-voltage]]

## Sources

- [[summaries/sze-physics-semiconductor-devices-08-chapter-4-metal-insulator-semiconductor-capacitors]]
- [[summaries/sze-physics-semiconductor-devices-09-chapter-6-mosfets]]
