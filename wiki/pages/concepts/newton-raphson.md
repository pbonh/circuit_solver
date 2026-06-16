---
title: Newton-Raphson Algorithm
type: concept
slug: newton-raphson
created: 2026-06-16
updated: 2026-06-16
summary: Iterative method for solving nonlinear algebraic equations by linearizing the system at each step; the core solver in SPICE-family circuit simulators.
tags: [numerical-methods, convergence, nonlinear-solver, circuit-simulation]
sources: [simulation-analog-mixed-signal-circuits]
status: active
---

# Newton-Raphson Algorithm

An iterative root-finding procedure that converts solving a nonlinear equation f(x) = 0 into solving a sequence of linear equations. Starting from an initial guess, it evaluates f and f' at the current point, constructs a linear approximation, and uses that linear model's zero-crossing as the next guess. Repeats until convergence criteria are met.

## Convergence Guarantees and Conditions

Convergence is **guaranteed** only when all three conditions hold:
1. **Smooth models**: circuit equations (device models) must be sufficiently differentiable — discontinuities in models break NR
2. **Isolated solution**: the system must have an isolated solution (no continuum of solutions from floating nodes or inductor loops)
3. **Close initial guess**: starting point must be near the solution

In practice, condition 3 is hard to satisfy for arbitrary circuits. This motivates [[homotopy-methods]].

## Convergence Criteria in Circuit Simulators

Two checks must both pass before declaring convergence:

| Check | SPICE | Spectre |
|---|---|---|
| Update criterion | |Δv| < ε | |Δv| < ε |
| Residue criterion | ΔI check: |Δi| < δ (false convergence risk) | KCL check: |Σi| < δ (reliable) |

The ΔI check can falsely converge when NR stalls (e.g., a bad derivative in the model); the KCL check assures Kirchhoff's current law is actually satisfied.

## Why it matters

- Every SPICE analysis (DC, AC linearization, each transient timepoint) uses NR
- Convergence failures are a persistent pain point for large analog circuits
- The choice of residue criterion (ΔI vs KCL) has large practical implications for simulation reliability

## Related concepts and entities

- [[spice-simulation]] - uses NR at every analysis step
- [[homotopy-methods]] - alternative when NR fails to converge
- [[integration-methods]] - creates the algebraic equations NR solves at each transient timepoint
- [[ken-kundert]] - introduced KCL check in Spectre, eliminating false convergence
- [[circuit-simulation]] - parent topic
