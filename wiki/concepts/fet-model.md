---
title: FET Model (JFET, MESFET, MOSFET)
type: claim
id: concepts/fet-model
tags:
- device-model
- analog
- mosfet
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/14-chapter-11-modeling.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Field-effect transistors (FETs) are 3- or 4-terminal devices with I_G = 0 (no DC gate current) and I_D as a nonlinear function of V_GS and V_DS. Vlach & Singhal describe two families:
- JFET / MESFET (junction or metal-semiconductor barrier).
- MOSFET / IGFET (metal-oxide or insulated-gate).

## How It Works

JFET/MESFET (n-channel):
- Linear region (V_DS < V_GS - V_0 + V_a): I_D/I_0 = beta(V_DS/V_0) - beta[(V_GS - V_DS + V_a)/V_0]^{3/2} + beta[(V_GS + V_a)/V_0]^{3/2}.
- Saturation region (V_DS > V_GS - V_0 + V_a): I_D/I_0 = (1/3) + ... (Eq. 11.2.4).
- Capacitance C_GS bias-dependent; C_GD typically 50x smaller than C_GS.

MOSFET (4-terminal with bulk):
- Diodes I_BD' = I_S [exp(V_BD'/V_T) - 1] and I_BS' = I_S [exp(V_BS'/V_T) - 1] model the substrate junctions.
- Linear region: I_D = beta [(V_GS - V_t) V_DS - V_DS^2/2].
- Saturation: I_D = (beta/2) (V_GS - V_t)^2.

## Key Parameters

- V_t (threshold voltage).
- beta (transconductance parameter).
- V_a (built-in potential).
- V_0 (pinch-off voltage).
- Junction capacitances C_BS, C_BD, etc.

## When To Use

- IC simulation of CMOS, NMOS, PMOS, GaAs MESFET circuits.
- RF and microwave circuit simulation.
- Hand-analysis of FET amplifier circuits.

## Risks & Pitfalls

- Level-1 models are quite inaccurate for short-channel devices; modern circuits use BSIM/PSP.
- Discontinuity in derivative at V_DS = V_GS - V_t (linear/saturation boundary) causes Newton convergence issues; smoothed models are preferred.
- Bulk-junction modeling matters for analog precision but is often ignored in digital simulation.

## Related Concepts

- [[concepts/device-modeling]]
- [[concepts/hybrid-pi-model]]
- [[concepts/diode-model]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
