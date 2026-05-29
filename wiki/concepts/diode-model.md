---
title: Diode Model
type: claim
id: claim-diode-model
tags:
- device-model
- analog
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/14-chapter-11-modeling.txt
confidence:
  base: 0.85
---

## Definition

The semiconductor diode is the simplest nonlinear two-terminal device. Its terminal equation is the Shockley equation: I = I_s [exp(qV/kT) - 1], where I_s is the saturation current (typically 10^-6 to 10^-9 A) and V_T = kT/q ≈ 25 mV at room temperature.

## How It Works

Operating point: I_0 = I_s [exp(V_0/V_T) - 1]. Dynamic conductance g(V_0) = (I_s/V_T) exp(V_0/V_T). For small signals at the operating point, the diode is a conductance g + parallel capacitance.

High-frequency model adds:
- Bulk resistance R_b in series.
- Depletion capacitance C_j(V) = C_j0 / (1 - V/phi)^gamma — tangent-line continuation needed for V close to phi.
- Diffusion capacitance C_D = tau (dI/dV).

In Newton-Raphson DC analysis, the linearized stamp at iterate V^(k) contributes a conductance g(V^(k)) plus an equivalent source I_eq = I(V^(k)) - g(V^(k)) V^(k).

## Key Parameters

- I_s (saturation current).
- V_T (thermal voltage, temperature-dependent).
- R_b (bulk resistance).
- C_j0, phi, gamma (depletion capacitance parameters).
- tau (transit time for diffusion capacitance).

## When To Use

- All semiconductor circuit simulation involving diodes.
- Rectifier, clamping, mixer, and varactor circuits.
- Junction modeling within transistor models.

## Risks & Pitfalls

- Exp() overflow at high forward bias — limit or use diode-specific limiting in Newton iterations.
- C_j(V) → infinity at V = phi; tangent-line replacement is needed.
- Reverse breakdown not captured by basic Shockley equation.

## Related Concepts

- [[concepts/device-modeling]]
- [[concepts/newton-raphson-method]]
- [[concepts/modified-nodal-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
