---
title: Sensitivity Network (Time-Domain)
type: claim
id: claim-sensitivity-network
tags:
- sensitivity
- transient
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/19-chapter-16-time-domain-sensitivities-and-steady-state.txt
confidence:
  base: 0.65
---

## Definition

The sensitivity network is the linear time-varying system obtained by differentiating the nonlinear algebraic-differential system equations with respect to a parameter h. Its solution y(t) = dq(t)/dh, z(t) = dx(t)/dh gives the time-domain sensitivities of charges/fluxes and node voltages to parameter h.

## How It Works

Starting from the original DAE q' - Ex = 0, f(q, x, w, h, t) = 0 with initial condition q_0 = q(0):
1. Differentiate with respect to h to obtain y' - Ez = 0, (df/dq) y + (df/dx) z + df/dh = 0.
2. At each BDF step of the original system, the Jacobian (df/dq, df/dx) is already factored.
3. Solve the linear sensitivity system via one extra forward/back substitution per parameter.

Initial condition y_0 = dq_0/dh; if initial conditions depend on parameters, this must be computed by differentiating the DC operating-point equations.

## Key Parameters

- Number of parameters (each adds one forward/back substitution per step).
- Memory: storing y(t) for many parameters can be significant.
- BDF order and step size (shared with main integration).

## When To Use

- Time-domain optimization of nonlinear circuits.
- Steady-state computation via shooting methods.
- Sensitivity studies of transient waveforms.

## Risks & Pitfalls

- Memory grows linearly with number of parameters.
- Sensitivity of sharp transitions (switching edges) can be numerically delicate.
- Adjoint-method alternative is more efficient when many parameters but one output objective.

## Related Concepts

- [[concepts/time-domain-sensitivity]]
- [[concepts/charge-flux-formulation]]
- [[concepts/gear-bdf]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
