---
title: BJT Model (Ebers-Moll, Hybrid-pi)
type: claim
id: concepts/bjt-model
tags:
- device-model
- bjt
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/14-chapter-11-modeling.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Bipolar junction transistors (BJTs) are 3-terminal current-controlled devices (with substrate as 4th terminal in some models). Vlach & Singhal describe two main BJT models:
- Ebers-Moll (large-signal, nonlinear): two coupled exponential-diode currents with forward and reverse alphas.
- Hybrid-pi (small-signal, linear): R_bb', R_b'e, R_b'c, R_ce, C_b'e, C_b'c, C_ce, and g_m V_b'e current source.

## How It Works

Ebers-Moll (already covered in concepts/ebers-moll-model):
- Coupled diode currents with alpha_F, alpha_R.
- Used for DC operating-point and transient analyses.
- Needs MNA because of voltage-controlled current sources.

Hybrid-pi (small-signal):
- Linearized around DC operating point.
- Stamps directly into nodal admittance.
- Typical parameter ranges (Section 11.3):
  - R_bb' = 25-200 Ohm.
  - R_b'e = 150-1000 Ohm.
  - R_pi or R_be = 10^6-10^7 Ohm.
  - R_ce = 2e4-1e5 Ohm.
  - C_b'e = 10-200 pF.
  - C_b'c = 0.2-6 pF.
  - g_m = 0.02-0.2 mmho.

Modern models (Gummel-Poon, VBIC) add base-width modulation, high-injection effects, and more capacitances.

## Key Parameters

- beta_F (forward common-emitter gain).
- beta_R (reverse common-emitter gain).
- I_S (saturation current).
- V_T (thermal voltage).
- Junction capacitances.

## When To Use

- BJT circuit simulation: ECL, TTL, analog audio amplifiers.
- Transistor-level design of analog signal-processing blocks.
- Educational illustration of nonlinear device behavior.

## Risks & Pitfalls

- Ebers-Moll misses base-width modulation; Gummel-Poon is preferred for accuracy.
- Exp() overflow in DC iterations must be handled.
- Hybrid-pi parameters depend strongly on bias point.

## Related Concepts

- [[concepts/ebers-moll-model]]
- [[concepts/hybrid-pi-model]]
- [[concepts/device-modeling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
