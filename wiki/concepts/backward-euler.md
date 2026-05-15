---
title: "Backward Euler"
type: concept
tags: [analog, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Backward Euler (BE) is the first-order implicit [[concepts/integration-method]] used as a robust workhorse in [[concepts/transient-analysis]]. It approximates the derivative at the new timepoint by a backward difference: v̇_k = (v_k - v_{k-1}) / h. Because v̇_k depends on v_k itself, the resulting difference equation is implicit and requires a Newton solve per step.

## How It Works

Substituting BE into the circuit's differential equation yields a nonlinear algebraic system at each new timepoint that is solved with [[concepts/newton-raphson-method]] using the previous timepoint's solution as the initial guess.

## Key Parameters

- Step size h — limited primarily by accuracy, not stability
- Order = 1 — LTE scales as O(h²) per step

## When To Use

- As a robust default when [[concepts/trapezoidal-rule]] ringing on a stiff circuit is intolerable.
- For startup of multistep methods (the first step of Gear2 reduces to BE).
- When artificial damping is desirable to suppress unphysical oscillation.

## Risks & Pitfalls

- **Overly stable** — introduces artificial numerical damping. An LC tank simulated with BE will decay even though no physical loss is present. See [[concepts/numerical-damping]].
- First-order accuracy is coarser than TR or Gear2; tighter timesteps are needed for equivalent accuracy on smooth waveforms.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/forward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/numerical-damping]]
- [[concepts/stiff-circuit]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
