---
title: "Steady-State Analysis"
type: concept
tags: [transient, harmonic-balance, ac, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/19-chapter-16-time-domain-sensitivities-and-steady-state.txt"]
confidence: high
---

## Definition

Steady-state analysis finds the periodic solution of a periodically excited nonlinear network: x(t + T) = x(t) for all t, where T is the excitation period. The steady state is what is observed in the laboratory after all initial transients have died out; computing it directly avoids wasteful integration through the transient phase.

## How It Works

Three families of methods:
1. Shooting methods: search for initial condition q_0 such that integrating one period returns to q_0. Newton-Raphson on this fixed-point condition uses the sensitivity matrix dq(T)/dq_0.
2. Frequency-domain methods (harmonic balance): represent x(t) as a Fourier series with unknown coefficients; solve nonlinear equations on the coefficients.
3. Extrapolation methods: integrate over several periods and use convergence-acceleration techniques (e.g., Shanks, Aitken) on the sequence q(0), q(T), q(2T), ... — the Vlach & Singhal Section 16.5 contribution.

## Key Parameters

- Period T (assumed known from excitation).
- Initial estimate q_0^{(0)}.
- Convergence tolerance.
- Choice of method (shooting vs. extrapolation vs. harmonic balance).

## When To Use

- Oscillator and mixer design (RF circuits).
- Class-C, switching power-supply design.
- Periodic-noise analysis.
- Anywhere the engineering interest is the steady state, not the transient.

## Risks & Pitfalls

- Multiple periodic solutions can exist (multiple basins of attraction).
- Newton-Raphson convergence depends on the spectral radius of dq(T)/dq_0.
- Harmonic balance requires careful truncation of the Fourier series.

## Related Concepts

- [[concepts/sensitivity-network]]
- [[concepts/shooting-method]]
- [[concepts/extrapolation-steady-state]]
- [[concepts/charge-flux-formulation]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
