---
title: "Charge-Flux Formulation (Nonlinear Reactive Elements)"
type: concept
tags: [transient, analog, well-established, foundational]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt"]
confidence: high
---

## Definition

The charge-flux formulation introduces charges q and fluxes phi as additional state variables for nonlinear capacitors and inductors. Differential equations become linear (dq/dt = i, dphi/dt = v), while the nonlinearity is contained in the algebraic constitutive equations q = f_q(v) and phi = f_phi(i).

## How It Works

A nonlinear capacitor has i = dq/dt = (dq/dv)(dv/dt) = C(v) dv/dt — this puts a nonlinearity inside the derivative term and complicates LMS integration. By introducing q as a state variable:
- Algebraic constraint: q = f_q(v) (nonlinear).
- Differential equation: dq/dt = i (linear).

Similarly for inductors: phi = f_phi(i) algebraic; dphi/dt = v linear.

Modern SPICE-class simulators use this approach because:
1. LMS formulas operate on linear differential parts directly.
2. Nonlinearities are confined to Newton-Raphson within each step.
3. The Jacobian structure remains regular and sparse.

## Key Parameters

- Charge/flux state-variable count (equals number of nonlinear reactive elements).
- Constitutive functions f_q, f_phi.
- Newton-Raphson convergence tolerance.

## When To Use

- Any simulator handling nonlinear capacitors (MOS junction caps, varactors).
- Nonlinear inductor models (saturation, hysteresis with care).

## Risks & Pitfalls

- Increases the state-vector dimension; trades algebraic complexity for state-space size.
- Initial conditions on charges/fluxes must be consistent.

## Related Concepts

- [[concepts/algebraic-differential-equations]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/diode-model]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
