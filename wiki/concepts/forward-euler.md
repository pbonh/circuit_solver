---
title: "Forward Euler"
type: concept
tags: [analog, transient, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources:
  - "raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"
  - "raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt"
confidence: high
---

## Definition

Forward Euler (FE) — also called the Forward Euler Method or explicit Euler — is the simplest explicit one-step formula for integrating x' = f(x, t):

  x_{n+1} = x_n + h · x'_n = x_n + h · f(x_n, t_n)

It is first-order accurate (p = 1) with leading truncation-error coefficient c₂ = −1/2. The new value is given algebraically by the past values — no nonlinear solve, no matrix solve is required per step. Equivalently, FE approximates the derivative at the previous timepoint by a forward difference v̇_{k−1} = (v_k − v_{k−1}) / h.

## How It Works

At each step evaluate f(x_n, t_n) once and advance the solution along the tangent line at the current state. Cost per step is a single f-evaluation. Combined with linear (constant) capacitors and grounded node-to-ground structure, this eliminates both the matrix solve and the [[concepts/newton-raphson-method]] iteration per timestep, which is why [[concepts/timing-simulation|timing simulators]] favour FE for non-stiff MOS digital partitions.

**Stability.** The region of absolute stability in q = h λ is the unit disk centered at −1 — i.e. step size must be small enough to bring every eigenvalue of the linearised system into this disk (|1 + h λ| ≤ 1). For [[concepts/stiff-systems|stiff systems]] that bound is set by the *fastest* eigenvalue and is far smaller than the accuracy-driven step, which makes FE effectively useless for stiff problems.

## Key Parameters

- Step size h.
- Stability bound: max |1 + h λ_i| ≤ 1 for every eigenvalue λ_i of the linearised system. In the stiff regime h is forced down to the fastest time-constant scale.
- Order p = 1; local truncation error scales as O(h²) per step.

## When To Use

- [[concepts/timing-simulation]] of non-stiff MOS digital partitions, where the per-step cost reduction (no matrix solve, no Newton iteration) outweighs the limitations.
- Predictor stage of a [[concepts/predictor-corrector]] / [[concepts/predictor-corrector-method]] pair.
- Pedagogical introduction to numerical integration of ODEs.
- Non-stiff systems where small h is acceptable for other reasons.

## Risks & Pitfalls

- **Unstable on stiff circuits.** When any time constant is much shorter than the desired timestep, FE diverges. Not used in general-purpose [[concepts/transient-analysis|SPICE-style transient]] simulation for that reason.
- Local error has consistent sign (c₂ = −1/2) — may bias the integration on long runs.
- O(h²) per-step accuracy requires small h on smooth trajectories even when stability allows larger steps.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/predictor-corrector]]
- [[concepts/a-stability]]
- [[concepts/stiff-circuit]]
- [[concepts/stiff-systems]]
- [[concepts/timing-simulation]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
