---
title: "Gear's Backward Differentiation Formula (BDF)"
type: concept
tags: [analog, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Gear's Backward Differentiation Formulas (BDF) are a family of implicit multistep [[concepts/integration-method]]s designed for stiff ODE/DAE systems. The second-order variant (Gear2 / BDF2) is offered by SPICE-class simulators alongside Trapezoidal Rule and Backward Euler. Gear2: v̇_k = (3/2h)v_k - (2/h)v_{k-1} + (1/2h)v_{k-2}.

## How It Works

Each BDF formula expresses the derivative at the new timepoint as a linear combination of the new value plus a fixed number of past values, scaled by 1/h. Substituting into the circuit DAE produces a nonlinear implicit system solved by [[concepts/newton-raphson-method]]. Higher-order BDFs are stiffly stable only through order 6; beyond that they lose A-stability. Most circuit simulators stop at Gear2.

## Key Parameters

- Order (1 → BE, 2 → Gear2, up to 6 for general DAE solvers)
- Step size h
- Step-size and order adaptation strategy (variable-step, variable-order BDF — VSVO BDF)

## When To Use

- When [[concepts/trapezoidal-rule]] ringing on stiff circuits is unacceptable but BE is too damped.
- For long transients on dissipative-or-quasi-dissipative circuits where Gear2's mild damping suppresses spurious oscillation without grossly distorting genuine signals.

## Risks & Pitfalls

- **Artificial damping** is present (less than BE, more than TR) — visible on lossless LC tanks as a slow amplitude decay. See [[concepts/numerical-damping]].
- Multistep methods need a startup procedure (the first step uses a lower-order method, typically BE).
- After timestep rejections the step-size history must be reset carefully.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/forward-euler]]
- [[concepts/stiff-circuit]]
- [[concepts/numerical-damping]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
- [[summaries/computer-methods-circuit-analysis-design-19-chapter-16-time-domain-sensitivities-and-steady-state]]
- [[summaries/hairer-ode-ii-01-preface]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/hairer-ode-ii-06-chapter-vii-differential-algebraic-equations]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
