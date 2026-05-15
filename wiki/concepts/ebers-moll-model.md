---
title: "Ebers-Moll Model"
type: concept
tags: [device-model, bjt, analog, dc, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/02-motivation.txt"]
confidence: medium
---

## Definition

The Ebers-Moll model is a classical large-signal model of the bipolar junction transistor (BJT), comprising two coupled exponential diodes (base-emitter and base-collector) plus current sources representing forward and reverse common-base gains alpha_F and alpha_R. Vlach and Singhal use the Ebers-Moll model in the TTL-gate example of the Motivation chapter to represent each transistor.

## How It Works

The injection version of Ebers-Moll defines:
    I_E = I_ES * (exp(V_BE/V_T) - 1) - alpha_R * I_CS * (exp(V_BC/V_T) - 1)
    I_C = alpha_F * I_ES * (exp(V_BE/V_T) - 1) - I_CS * (exp(V_BC/V_T) - 1)
with alpha_F = beta_F / (1 + beta_F) and alpha_R = beta_R / (1 + beta_R). The textbook figure also adds the junction capacitances C_je, C_jc, base-collector C_bc, substrate C_sub, etc., for transient analysis.

## Key Parameters

- I_ES, I_CS: emitter and collector saturation currents.
- alpha_F (forward), alpha_R (reverse), and equivalently beta_F, beta_R.
- V_T: thermal voltage kT/q.
- Junction capacitances (for transient).

## When To Use

- Hand calculations and simple simulations of BJT circuits — TTL, ECL, emitter-coupled pairs.
- Educational presentations of BJT operation in all four regions (active, saturation, cutoff, reverse).

## Risks & Pitfalls

- Less accurate than Gummel-Poon for high-injection, Early effect, and base-width modulation.
- The exponentials demand careful overflow handling and damping in DC Newton iterations.
- Junction capacitance variation with bias is not captured by the basic Ebers-Moll equations alone.

## Related Concepts

- [[concepts/bjt-model]]
- [[concepts/device-modeling]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-02-motivation]]
- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
