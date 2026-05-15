---
title: "Transient Analysis"
type: concept
tags: [analog, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Transient analysis is a time-domain simulation that approximates the solution of the nonlinear differential-algebraic equation system describing a circuit. Time is discretized; the solution trajectory is approximated by a piecewise low-order polynomial; d/dt is replaced by an [[concepts/integration-method]] finite-difference formula; the resulting nonlinear difference equation is solved at each timepoint via [[concepts/newton-raphson-method]].

## How It Works

Starting from an initial condition (the DC operating point by default, or user-supplied UIC values), the simulator picks a timestep h, applies its chosen integration method (Forward Euler, Backward Euler, Trapezoidal Rule, or Gear's BDF) to convert d/dt into a discrete operator, builds the resulting nonlinear algebraic system at t_k = t_{k-1} + h_k, and solves it with NR. After each successful solve, [[concepts/local-truncation-error]] is estimated by comparing the computed point to a low-order polynomial extrapolation from previous points; if LTE is above tolerance the point is rejected and h shrunk; if well below, h is grown.

## Key Parameters

- Timestep `h` (adaptive, bounded above by `tmax` / `tstep` and below by `tmin`)
- Integration method choice — typically TR (default), Gear2, or BE
- LTE controls — `reltol`, `abstol`, `chargetol`, `voltage_lte` vs `charge_lte` (SPICE uses charge; Spectre uses voltage)
- NR convergence tolerances per timestep
- Initial-condition mode: default (DC operating point), UIC, or non-UIC forced ICs

## When To Use

- Any large-signal time-domain question: switching, slew, settling, ringing, startup, large-amplitude distortion.
- As a fallback DC method via UIC (let the circuit settle).
- As the engine inside steady-state and RF analyses (shooting Newton, periodic transient).

## Risks & Pitfalls

- **Stiff circuits**: explicit methods are unstable; use implicit methods (BE, TR, G2) — see [[concepts/stiff-circuit]].
- **Trapezoidal ringing**: TR is only marginally stable; tightening reltol shrinks h and damps the artifact.
- **Numerical damping**: BE and Gear2 are overly stable and add artificial loss — visible as decaying oscillation on an LC tank that should ring forever.
- **Charge vs. voltage LTE control**: SPICE controls LTE in charge, which can let voltage waveforms drift on stiff circuits; Spectre controls LTE in voltage.
- **Long time constants** (integrators, oscillators, switched-capacitor circuits, charge-storage): truncation errors accumulate; tighter tolerances are needed.
- **UIC with parallel LC tanks**: charge/flux conservation on conflicting ICs gives results that often surprise.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/local-truncation-error]]
- [[concepts/stiff-circuit]]
- [[concepts/numerical-damping]]
- [[concepts/newton-raphson-method]]
- [[concepts/charge-conservation]]
- [[concepts/dc-analysis]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
