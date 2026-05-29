---
title: Integration Method
type: claim
id: concepts/integration-method
tags:
- analog
- transient
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

An integration method is the finite-difference formula used inside [[concepts/transient-analysis]] to replace the continuous time-derivative d/dt with a discrete operator that produces a difference equation solvable one timestep at a time. SPICE-class simulators offer a small menu — Forward Euler, Backward Euler, Trapezoidal Rule, Gear's second-order BDF — chosen for accuracy, stability, and stiffness properties.

## How It Works

Each method writes the derivative at the new timepoint as a linear combination of the value at the new and previous timepoints, scaled by 1/h:

- **Forward Euler (FE)**: v̇_{k-1} = (v_k - v_{k-1}) / h — explicit, the new value v_k is given algebraically by past values.
- **Backward Euler (BE)**: v̇_k = (v_k - v_{k-1}) / h — implicit, v_k appears in the rhs.
- **Trapezoidal Rule (TR)**: v̇_k = (2/h)(v_k - v_{k-1}) - v̇_{k-1} — implicit, the average of FE and BE.
- **Gear's BDF2 (G2)**: v̇_k = (3/2h)v_k - (2/h)v_{k-1} + (1/2h)v_{k-2} — implicit, second-order.

The discrete derivative is substituted into the [[concepts/modified-nodal-analysis]] equations to produce a nonlinear difference equation that [[concepts/newton-raphson-method]] solves at each new timepoint.

## Key Parameters

- Order: first (FE, BE) vs. second (TR, G2)
- Explicit vs. implicit
- Stiff stability: stiffly stable (BE, G2), marginally stable (TR), unstable on stiff problems (FE)
- Numerical damping: high (BE) → moderate (G2) → none (TR) → none/unstable (FE)

## When To Use

- **TR** — default for analog circuit simulation: second-order accurate, no artificial damping on LC tanks; accept the marginal-stability ringing on stiff problems and tighten tolerances if it bites.
- **Gear2** — when artificial damping is tolerable and ringing isn't (e.g., long transients on stiff but non-resonant circuits).
- **BE** — robust workhorse for cases where damping is wanted to suppress ringing.
- **FE** — only in timing simulators for non-stiff MOS digital partitions.

## Risks & Pitfalls

- FE on stiff circuits is unstable; the timestep would have to be smaller than the fastest time constant, defeating the point.
- TR ringing on stiff circuits is characteristic point-to-point oscillation.
- BE and Gear2 introduce artificial numerical damping; an LC tank simulated with BE will appear to decay even though the simulated circuit has no loss.
- Mixing methods (e.g., starting with BE then switching to TR) can cause one-step transients at the switch.

## Related Concepts

- [[concepts/forward-euler]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/transient-analysis]]
- [[concepts/stiff-circuit]]
- [[concepts/local-truncation-error]]
- [[concepts/numerical-damping]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
