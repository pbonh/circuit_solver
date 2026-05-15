---
title: "Step-Size Control (Adaptive Integration)"
type: concept
tags: [transient, numerical-integration, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Computer-Methods-for-Circuit-Analysis-and-Design/_txt/16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations.txt"]
confidence: medium
---

## Definition

Step-size control adapts the time step h during numerical integration based on an estimate of the local truncation error (LTE). When LTE is too large, h is decreased; when LTE is comfortably below tolerance, h is increased. The predictor-corrector difference is the classical LTE estimator for LMS methods.

## How It Works

After each corrector step:
1. Estimate LTE ≈ ||x_corrector - x_predictor||.
2. If LTE > tolerance: reject the step, halve h, re-solve.
3. If LTE << tolerance: accept the step, possibly double h for the next.
4. Otherwise: accept and keep h.

In BDF codes, changing h requires re-computing the BDF coefficients (which depend on the step-size ratio) and refactoring the Jacobian. Therefore step changes are not free.

## Key Parameters

- Local truncation error tolerance (relative or absolute).
- Step-size factor (typically 0.5 for reduction, 2 for increase).
- Maximum and minimum h limits.

## When To Use

- Any production transient solver.
- Stiff or non-stiff systems with rapidly varying time scales.
- Simulations spanning many decades in time.

## Risks & Pitfalls

- Aggressive step-size changes can destabilize the integrator.
- LTE estimators may be unreliable near sharp transitions.
- Step-size factor must be conservative to avoid oscillation between accept/reject.

## Related Concepts

- [[concepts/order-control]]
- [[concepts/predictor-corrector]]
- [[concepts/linear-multistep-methods]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-16-chapter-13-numerical-integration-of-differential-and-algebraic-differential-equations]]
