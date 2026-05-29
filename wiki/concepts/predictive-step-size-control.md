---
title: Predictive Step-Size Control
type: claim
id: claim-predictive-step-size-control
tags:
- ode
- numerical-integration
- adaptive-control
- stiff
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

Predictive step-size control (Gustafsson 1994) is the stiff-solver-oriented variant of [[concepts/pi-step-size-control]]. Instead of treating the step-size choice as feedback on the *current* error, it builds a small forecasting model on the logarithm of the error and projects the next-step optimal h from log C_{n+1} − log C_n behaving as a near-constant.

## How It Works

The estimator regresses log err_n on integration time and uses the regression slope to predict err at the trial next step; the step is sized so the prediction equals the tolerance. Hairer–Wanner's RADAU5 implements this as the default. Compared with PI control, the predictive variant handles two stiff-solver-specific phenomena: (i) abrupt transitions between boundary layers and the smooth régime, and (ii) the cost of Jacobian / LU updates, which a predictive policy can avoid by holding h constant longer when error trends predict a violation is far ahead.

## Key Parameters

- Window length for the error trend.
- Tolerance Tol on global error.
- Safety factors on step-size growth / shrinkage rates.
- Combined with Newton-convergence health check (refactor trigger).

## When To Use

- Stiff IRK / Rosenbrock codes (RADAU5, RODAS) where Jacobian / LU cost dominates.
- DAE codes with boundary-layer transients (singular-perturbation problems).
- Long-time integrations where holding h constant amortises factorisation cost.

## Risks & Pitfalls

- Predictive policy can lag a real error increase if the trend window is too long.
- Combining predictive control with PI feedback on the same exponent doubles up; pick one or the other.
- Requires log-scale arithmetic and trend storage; slightly more bookkeeping than I- or PI-control.

## Related Concepts

- [[concepts/pi-step-size-control]]
- [[concepts/implicit-runge-kutta]]
- [[concepts/rosenbrock-method]]
- [[concepts/simplified-newton-iteration]]
- [[entities/radau5]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
