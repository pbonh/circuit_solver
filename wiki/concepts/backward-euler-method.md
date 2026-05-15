---
title: "Backward Euler Method"
type: concept
tags: [transient, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt"]
confidence: high
---

## Definition

The backward Euler method is the simplest implicit one-step formula for integrating x' = f(x, t): x_{n+1} = x_n + h x'_{n+1}. It is first-order (p=1) with truncation error coefficient c_2 = +1/2.

## How It Works

Each step requires solving an equation for x_{n+1}. For nonlinear f, Newton-Raphson iteration (or fixed-point with a predictor) is used. For linear systems x' = A x + w: (I - h A) x_{n+1} = x_n + h w_{n+1}.

Stability: the region of absolute stability is the exterior of the unit disk centered at +1 in q = h lambda — A-stable (stable for all Re lambda < 0 and all h). Even unconditionally stable for stable systems.

## Key Parameters

- Step size h (no stability restriction from h alone).
- Newton/fixed-point convergence tolerance.

## When To Use

- Stiff systems (widely separated time constants).
- DC operating-point continuation via "pseudo-transient" methods.
- As the default A-stable integrator when high order is not required.

## Risks & Pitfalls

- Numerical damping: backward Euler artificially attenuates solution amplitudes — may give wrong steady-state for lightly damped oscillations.
- Only first-order accurate; small step sizes still needed for accuracy if not for stability.

## Related Concepts

- [[concepts/forward-euler-method]]
- [[concepts/trapezoidal-rule]]
- [[concepts/a-stability]]
- [[concepts/stiff-systems]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
