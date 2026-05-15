---
title: "Slew Rate"
type: concept
tags: [device-model, analog, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/14-chapter-11-modeling.txt"]
confidence: medium
---

## Definition

The slew rate S_R of an operational amplifier is the maximum time rate of change of the output voltage it can produce: |dV_o/dt| ≤ S_R. It arises because the OPAMP's internal transistors cannot supply unlimited current to charge load capacitances.

## How It Works

In the OPAMP macromodel, slew-rate limiting is captured by a nonlinear VCT that saturates at +/- I_m, in series with a capacitor C_l. The output capacitor voltage rate-of-change is then bounded:
|dV_o/dt| = |I/C_l| ≤ I_m/C_l = S_R.

For unity-gain configurations, slew rate typically dominates large-signal step response. For small signals, the linear gain-bandwidth product is the relevant limit.

## Key Parameters

- S_R (slew rate, typically V/microsecond).
- Sign convention: + and - rates can differ in practical OPAMPs.
- Load capacitance.
- Input slew (some macromodels also model input slew limits).

## When To Use

- High-speed signal-chain design.
- Large-signal step-response analysis.
- Communications and pulse processing where edge rates matter.

## Risks & Pitfalls

- A circuit limited by slew rate may distort large-signal sinusoids even when the small-signal bandwidth is adequate.
- Slew rate may be asymmetric (different for rising vs. falling edges).

## Related Concepts

- [[concepts/operational-amplifier-macromodel]]
- [[concepts/operational-amplifier]]
- [[concepts/macromodeling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
