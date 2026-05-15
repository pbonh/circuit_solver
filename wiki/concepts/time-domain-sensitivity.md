---
title: "Time-Domain Sensitivity"
type: concept
tags: [sensitivity, transient, analog, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/13-chapter-10-numerical-laplace-transform-inversion.txt"]
confidence: medium
---

## Definition

Time-domain sensitivity computes d v(t) / d h, the change in time-domain response at time t due to an infinitesimal change in element h. Direct time-stepping methods require backward-in-time integration of an adjoint system, demanding storage of the entire forward trajectory.

## How It Works

Using numerical Laplace transform inversion (NILT):
1. Compute the Laplace-domain sensitivity dV(s)/dh by the adjoint method of Chapter 6 (one extra adjoint solve per output).
2. Apply NILT to dV(s)/dh to get d v(t) / d h directly at any time t of interest.

Because NILT evaluates at discrete complex frequencies, no time-history storage is needed. This makes NILT especially attractive for time-domain optimization where gradients with respect to many parameters are required at a single time point.

## Key Parameters

- Output of interest (linear combination of state variables).
- Parameters with respect to which sensitivities are computed.
- Frequency-domain sensitivity formula (from Chapter 6).
- NILT order (M, N).

## When To Use

- Time-domain optimization of linear circuits.
- Pulse-shape design where gradient information is needed at specific time points.
- Verification of time-stepping adjoint computations.

## Risks & Pitfalls

- Only valid for linear (or piecewise-linear) networks.
- Inversion errors propagate to sensitivity errors.
- Requires Laplace-domain formulation; nonlinear networks must be linearized first.

## Related Concepts

- [[concepts/numerical-laplace-transform-inversion]]
- [[concepts/adjoint-method]]
- [[concepts/transpose-system-method]]
- [[concepts/sensitivity-analysis]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-13-chapter-10-numerical-laplace-transform-inversion]]
- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
