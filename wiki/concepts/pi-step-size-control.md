---
title: PI Step-Size Control
type: claim
id: concepts/pi-step-size-control
tags:
- ode
- numerical-integration
- adaptive-control
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/solving_ordinary_differential_equations_ii/_txt/
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

PI step-size control (Gustafsson, Lundh, Söderlind 1988) is a proportional-integral feedback controller for adaptive ODE integration. Replacing the standard I-controller h_{n+1} = h_n · (Tol/err_n)^{1/(p+1)} with the PI form h_{n+1} = h_n · (Tol/err_n)^α · (err_{n−1}/err_n)^β damps the ragged step-size oscillations that the simple I-controller produces near a method's stability boundary.

## How It Works

The exponent α plays the role of the integral gain (it pulls h toward the accuracy-optimal value); β is the proportional gain on the rate of error change (it damps overshoot when err_n is swinging). Typical values are α = 0.7/(p+1), β = 0.4/(p+1) for embedded RK pairs of order p. The controller is robust whether the step is accuracy-limited or stability-limited: in the accuracy regime it behaves like an I-controller; near the stability boundary it spots the err-swing pattern and slows the response. This was a substantial practical improvement over the 1970s-era I-controllers, especially for explicit RK codes on mildly stiff problems.

## Key Parameters

- α (integral gain), typically 0.7/(p+1) to 1/(p+1).
- β (proportional gain), typically 0.4/(p+1) to 0.5/(p+1).
- Method order p.
- Safety factor (≈ 0.9) on the step prediction.

## When To Use

- Embedded RK codes integrating mildly stiff or accuracy-critical problems.
- Solvers that exhibit ragged step rejection patterns under simple I-control.
- DAE / SPP codes where the boundary-layer transition makes naive control oscillate.

## Risks & Pitfalls

- Tuning α, β too aggressively can de-stabilise the controller itself.
- The predictive variant ([[concepts/predictive-step-size-control]], Gustafsson 1994) is better for stiff codes; PI alone is the explicit-RK default.
- For deeply stiff regimes, step-size selection should depend on Newton-convergence health, not just error.

## Related Concepts

- [[concepts/predictive-step-size-control]]
- [[concepts/explicit-runge-kutta]]
- [[concepts/automatic-stiffness-detection]]
- [[concepts/runge-kutta-method]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
