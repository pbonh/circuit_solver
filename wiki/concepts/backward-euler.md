---
title: Backward Euler
type: claim
id: claim-backward-euler
tags:
- analog
- transient
- numerical-integration
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
- raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt
confidence:
  base: 0.85
---

## Definition

Backward Euler (BE) — also called the Backward Euler Method or implicit Euler — is the simplest implicit one-step formula for integrating x' = f(x, t):

  x_{n+1} = x_n + h · x'_{n+1} = x_n + h · f(x_{n+1}, t_{n+1})

It is first-order accurate (p = 1) with leading truncation-error coefficient c₂ = +1/2, and is the first-order workhorse [[concepts/integration-method]] of [[concepts/transient-analysis]]. Equivalently, it approximates the derivative at the new timepoint by a backward difference v̇_k = (v_k − v_{k-1}) / h. Because v̇_k depends on v_k itself, the resulting difference equation is implicit and requires a solve per step.

## How It Works

Substituting BE into the circuit's ODE/DAE yields a nonlinear algebraic system at each new timepoint, solved with [[concepts/newton-raphson-method]] using the previous timepoint's solution as the initial guess. For a linear system x' = A x + w the step reduces to the matrix equation

  (I − h A) x_{n+1} = x_n + h w_{n+1}

so each step is one linear solve.

**Stability.** The region of absolute stability in q = h λ is the exterior of the unit disk centered at +1 — i.e. BE is [[concepts/a-stability|A-stable]] (stable for all Re λ < 0 with no upper bound on h). On a linear stable test problem the method is unconditionally stable.

## Key Parameters

- Step size h — bounded by accuracy requirements, not by stability.
- Order p = 1; local truncation error scales as O(h²) per step.
- Newton / fixed-point convergence tolerance.

## When To Use

- Default A-stable integrator on [[concepts/stiff-circuit|stiff circuits]] and [[concepts/stiff-systems|stiff systems]] when high order is not required.
- As a robust fallback when [[concepts/trapezoidal-rule]] ringing on stiff problems is intolerable.
- Startup of multistep methods — the first step of Gear2 / [[concepts/gear-bdf|BDF]] reduces to BE.
- DC-operating-point continuation via [[concepts/pseudo-transient-analysis|pseudo-transient]] methods.
- When artificial damping is desirable to suppress unphysical oscillation.

## Risks & Pitfalls

- **Overly stable / numerical damping.** BE introduces artificial dissipation: an LC tank simulated with BE will decay even though no physical loss is present (see [[concepts/numerical-damping]]). May give the wrong steady-state for lightly damped oscillators.
- **First-order accuracy** is coarser than TR or Gear2; tighter timesteps are needed for equivalent accuracy on smooth waveforms.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/forward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/a-stability]]
- [[concepts/stiff-circuit]]
- [[concepts/stiff-systems]]
- [[concepts/newton-raphson-method]]
- [[concepts/numerical-damping]]
- [[concepts/linearly-implicit-euler]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
- [[summaries/hairer-ode-ii-04-chapter-v-multistep-methods-for-stiff-problems]]
- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
