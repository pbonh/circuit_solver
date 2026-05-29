---
title: Hybrid-π Model (Small-Signal BJT/FET)
type: claim
id: claim-hybrid-pi-model
tags:
- device-model
- analog
- ac
- well-established
- mosfet
- bjt
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/05-chapter-2-network-equations-and-their-solution.txt
confidence:
  base: 0.85
---

## Definition

The hybrid-π model is the standard small-signal equivalent circuit of bipolar (BJT) and field-effect (FET) transistors, used for AC analysis. The FET model in Vlach & Singhal Fig. 2.2.3 uses C_GD, C_GS, C_DS, R_DS and a transconductance g_m; the BJT model in Fig. 2.2.4 uses R_bb', C_b'e, R_b'e, C_b'c, R_b'c, C_ce, R_ce and a g_m V_b'e current source.

## How It Works

The model is linear around a chosen DC operating point. Its admittance parameters are stamped directly into the nodal admittance matrix Y. For a FET stamped between nodes G (gate), D (drain), S (source):
- The C_GD, C_GS, C_DS and R_DS admittances contribute symmetric two-terminal patterns.
- The g_m V_GS current source from drain to source contributes an asymmetric VCT pattern.

The resulting Y matrix is structurally symmetric but numerically asymmetric due to the controlled source.

## Key Parameters

- g_m (transconductance) — determines small-signal gain.
- C_gd, C_gs, C_ds (capacitances) — high-frequency response.
- R_pi or R_b'e (BJT) / R_ds (FET) — output resistance.
- Operating point (must be determined first by DC analysis).

## When To Use

- AC and small-signal analysis of amplifier circuits.
- Linearized analysis of circuits near a chosen DC bias.
- Frequency-response computation for transistor circuits.

## Risks & Pitfalls

- Only valid for small signals around the chosen operating point.
- Does not capture large-signal switching, distortion, or saturation.
- Requires accurate DC operating point; sensitive to bias errors.

## Related Concepts

- [[concepts/ebers-moll-model]]
- [[concepts/nodal-analysis]]
- [[concepts/dependent-source]]
- [[concepts/device-modeling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-05-chapter-2-network-equations-and-their-solution]]
- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
