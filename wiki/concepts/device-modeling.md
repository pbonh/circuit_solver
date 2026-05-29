---
title: Device Modeling
type: claim
id: concepts/device-modeling
tags:
- device-model
- analog
- well-established
- foundational
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

Device modeling represents the electrical behavior of semiconductor devices (diodes, transistors) by sets of equations, equivalent circuits, or interpolating tables. CAD specialists usually receive these models from device-physics specialists and are responsible for implementation in the simulator.

## How It Works

Device models are typically formulated as I = f(V) (resistive) or I, V, dQ/dt relations (with capacitive elements). They are stamped into MNA matrices via linearization around the operating point:
- DC analysis: nonlinear f(V) is iterated by Newton-Raphson; the Jacobian provides linearized stamps.
- AC analysis: small-signal model uses the Jacobian directly.
- Transient analysis: capacitive Q(V) is differentiated by the integration formula (companion model).

Modern models (BSIM, EKV, PSP) have hundreds of parameters; simpler models (Ebers-Moll, level-1 MOS) are used for hand calculations and pedagogy.

## Key Parameters

- Device type (diode, BJT, FET, MOSFET, GaAs MESFET).
- Model level (level-1 to level-3 SPICE MOS, BSIM3/4, etc.).
- Operating regime (linear, saturation, subthreshold).
- Temperature.

## When To Use

- Any simulation involving semiconductor devices.
- Compact-model development for new processes.
- Model-parameter extraction from measurements.

## Risks & Pitfalls

- Models are only valid within their parameter-extraction ranges.
- Derivative discontinuities cause Newton-Raphson convergence problems.
- Numerical overflow in exponentials must be carefully handled.

## Related Concepts

- [[concepts/diode-model]]
- [[concepts/fet-model]]
- [[concepts/bjt-model]]
- [[concepts/spline-approximation]]
- [[concepts/macromodeling]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-14-chapter-11-modeling]]
