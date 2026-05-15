---
title: "Forward Euler Method"
type: concept
tags: [transient, numerical-integration, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/12-chapter-9-introduction-to-numerical-integration-of-differential-equations.txt"]
confidence: high
---

## Definition

The forward Euler method is the simplest explicit one-step formula for integrating x' = f(x, t): x_{n+1} = x_n + h x'_n. It is first-order (p=1) with truncation error coefficient c_2 = -1/2.

## How It Works

At each step, evaluate f(x_n, t_n) and advance the solution along the tangent line. Cost per step is one f-evaluation — no equation to solve.

Stability: the region of absolute stability in q = h lambda is the unit disk centered at -1 (Re lambda < 0 required for stability). Step size must be small enough to bring all eigenvalues of the system into this region — restrictive for stiff systems.

## Key Parameters

- Step size h.
- Stability bound: max |h Re lambda_i| ≤ small constant for all eigenvalues lambda_i.

## When To Use

- Pedagogical introduction to numerical integration.
- Predictor stage of predictor-corrector methods.
- Non-stiff systems where small h is acceptable.

## Risks & Pitfalls

- Unconditionally unstable for stiff systems unless h is reduced to bring fast poles into the small stability region.
- Local error has consistent sign — may bias the integration.
- Not suitable for production circuit simulation.

## Related Concepts

- [[concepts/backward-euler-method]]
- [[concepts/trapezoidal-rule]]
- [[concepts/predictor-corrector]]
- [[concepts/a-stability]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-12-chapter-9-introduction-to-numerical-integration-of-differential-equations]]
