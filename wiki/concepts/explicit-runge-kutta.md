---
title: Explicit Runge–Kutta
type: claim
id: concepts/explicit-runge-kutta
tags:
- ode
- numerical-integration
- runge-kutta
- nonstiff
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

A Runge–Kutta method is explicit if its coefficient matrix A is strictly lower triangular (a_{ij} = 0 for j ≥ i). Each stage Y_i is then computable from previously evaluated stages without solving a system. Classical examples include the four-stage classical RK4, the embedded Dormand–Prince DOPRI5(4) and DOPRI8(5,3) pairs, and Cash–Karp methods.

## How It Works

The [[concepts/stability-function]] of an explicit s-stage method is a polynomial R(z) = 1 + z + … + z^p/p! + O(z^{s+1}) of degree at most s. The [[concepts/stability-region]] is therefore *bounded*, so explicit RK methods are not A-stable and require h |λ| inside a finite region for stable integration. Embedded pairs (b, b̂) of orders p and p̂ enable cheap local error estimation: err_n = ‖y_n − ŷ_n‖. Adaptive step control then drives the error to a user tolerance, often paired with [[concepts/pi-step-size-control]] to damp oscillations near the stability boundary on mildly stiff problems. [[concepts/chebyshev-method]] families (Lebedev's DUMKA, van der Houwen–Sommeijer's RKC, Abdulle–Medovikov's ROCK4) construct R(z) from shifted Chebyshev polynomials to extend the stability region along the negative real axis, making explicit integration tractable on mildly stiff parabolic problems.

## Key Parameters

- Number of stages s.
- Order p of the principal method, p̂ of the embedded estimator.
- Stability region (longest real-axis / imaginary-axis interval).
- FSAL (first-same-as-last) flag for tableau efficiency.

## When To Use

- Nonstiff problems where step size is set by accuracy, not stability.
- Smooth right-hand sides where many derivative evaluations per step are inexpensive.
- Wave / hyperbolic PDE method-of-lines with eigenvalues on the imaginary axis.
- Mildly stiff parabolic problems via stabilised explicit ([[concepts/chebyshev-method]]).

## Risks & Pitfalls

- Useless on stiff problems — step-size restrictions become catastrophic.
- The [[concepts/automatic-stiffness-detection]] machinery is a runtime guard against this failure mode.
- Embedded estimators can underestimate the true error on stiff or boundary-layer problems.

## Related Concepts

- [[concepts/runge-kutta-method]]
- [[concepts/stability-region]]
- [[concepts/chebyshev-method]]
- [[concepts/pi-step-size-control]]
- [[concepts/automatic-stiffness-detection]]
- [[concepts/implicit-runge-kutta]]

## Sources

- [[summaries/hairer-ode-ii-03-chapter-iv-stiff-problems-one-step-methods]]
