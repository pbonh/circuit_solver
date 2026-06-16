---
title: Numerical Integration Methods
type: concept
slug: integration-methods
created: 2026-06-16
updated: 2026-06-16
summary: Finite-difference approximations used in transient circuit simulation to replace time-derivatives with algebraic expressions solvable by Newton-Raphson at each timestep.
tags: [numerical-methods, transient-analysis, circuit-simulation, ode]
sources: [simulation-analog-mixed-signal-circuits]
status: active
---

# Numerical Integration Methods

Transient simulation converts a circuit's differential equations into a difference equation by replacing the time-derivative d/dt with a finite-difference approximation. The approximation turns the ODE into an algebraic system solvable by [[newton-raphson]] at each discrete timepoint. The choice of method determines stability, accuracy, and numerical damping characteristics.

## Common Methods in Circuit Simulation

| Method | Type | Order | Notes |
|---|---|---|---|
| Forward Euler (FE) | Explicit | 1 | Unstable on stiff circuits; only in timing simulation |
| Backward Euler (BE) | Implicit | 1 | Stiffly stable; significant numerical damping |
| Trapezoidal Rule (TR) | Implicit | 2 | Marginally stable; rings on stiff circuits |
| Gear's BDF2 (G2) | Implicit | 2 | Stiffly stable; less damping than BE |

**Stiff circuit**: one where some time constants are much shorter than the desired timestep. FE (explicit) is unstable on stiff circuits and forces the timestep down to the fastest time constant.

## Tradeoffs

- **TR** is exact for parabolic trajectories and adds no artificial damping, but its marginal stability produces characteristic point-to-point ringing on stiff circuits (visible as oscillating numerical artifacts)
- **BE** is overly stable: it adds artificial numerical damping — an LC tank will show asymptotic amplitude decay even with lossless components
- **G2** is the preferred middle ground: stiffly stable with less artificial damping than BE
- **FE** is only used in timing simulators where non-stiff circuits and explicit integration are assumed

## Local Truncation Error (LTE) Control

Simulators choose the timestep to control LTE:
- LTE estimated as the difference between the solution at the current step and the value predicted by extrapolating from prior points using the same polynomial order
- If LTE > threshold: reject timepoint, shrink timestep
- If LTE < threshold: accept, possibly grow timestep

**SPICE**: controls LTE in charge; can give large voltage errors on stiff circuits (small charge error ≠ small voltage error on small capacitors).
**Spectre**: controls LTE in voltage — more meaningful to users and more accurate on stiff circuits.

## Error Accumulation

LTE errors fade in dissipative circuits (short time constants). In non-dissipative circuits (long time constants — oscillators, integrators, charge-storage circuits), errors accumulate. Oscillator simulations exhibit phase drift at a rate related to the integration method's frequency underestimation.

## Why it matters

- Method choice directly affects simulation accuracy, stability, and runtime
- TR ringing is commonly misdiagnosed as circuit behavior
- LTE domain (charge vs. voltage) is a subtle but important design choice distinguishing SPICE from Spectre

## Related concepts and entities

- [[newton-raphson]] - solves the algebraic system created at each timestep
- [[spice-simulation]] - uses these methods in transient analysis
- [[circuit-simulation]] - parent topic
