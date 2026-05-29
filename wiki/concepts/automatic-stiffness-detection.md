---
title: Automatic Stiffness Detection
type: claim
id: claim-automatic-stiffness-detection
tags:
- ode
- numerical-integration
- stiff
- runtime-diagnostics
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.85
---

## Definition

Automatic stiffness detection is a runtime heuristic that lets an explicit-RK code warn the user (or switch to a stiff solver) when the integrated problem becomes stiff. The two main techniques are the Shampine–Hiebert second error estimator and a dominant-eigenvalue power-method estimate of the Jacobian's spectral radius.

## How It Works

The Shampine–Hiebert estimator augments an embedded RK pair with a third order auxiliary estimator at a slightly different node distribution; on stiff problems this third estimator agrees poorly with the first, signalling that the step-size restriction is stability-driven rather than accuracy-driven. The dominant-eigenvalue method (also called the "power-method stiffness factor") performs one or two power iterations on h J using the difference (f(y + δ) − f(y))/‖δ‖ as a Jacobian-vector product; the resulting ρ̂(hJ) compared against the stability boundary of the running method gives a direct numerical stiffness indicator. Hairer–Wanner's DOPRI5 ships both — when the estimate exceeds a threshold the code prints a "problem appears stiff" warning.

## Key Parameters

- Stiffness-factor threshold (e.g. ρ̂ h > 3.3 for DOPRI5).
- Number of power-iteration steps.
- Cost of one Jacobian-vector product (typically one extra f evaluation).

## When To Use

- Inside explicit-RK codes that want to alert users to misuse.
- Diagnostic mode in adaptive multi-method solvers.
- Hybrid solvers that switch between explicit and implicit method based on detected stiffness.

## Risks & Pitfalls

- False positives near regions with mild stiffness or transient stiffness.
- Power-iteration estimate has error O(h ‖f_{yy}‖); for highly nonlinear problems the estimate degrades.
- Detection is no substitute for choosing the right method — the warning still requires user action.

## Related Concepts

- [[concepts/stiff-circuit]]
- [[concepts/explicit-runge-kutta]]
- [[concepts/stability-region]]
- [[concepts/pi-step-size-control]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
