---
title: "Forward Euler"
type: concept
tags: [analog, transient, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Forward Euler (FE) is the simplest explicit first-order [[concepts/integration-method]]. It approximates the derivative at the previous timepoint by a forward difference: v̇_{k-1} = (v_k - v_{k-1}) / h. The new value v_k is given algebraically by the past values, so no nonlinear solve is required per step.

## How It Works

Substituting FE into the circuit's differential equation produces a difference equation in which v_k appears only linearly (and only via the derivative term) — the right-hand-side depends on v_{k-1}. Combined with linear (constant) capacitors and grounded node-to-ground structure, this eliminates both the matrix solve and the [[concepts/newton-raphson-method]] iteration per timestep, which is why timing simulators favor FE.

## Key Parameters

- Step size h — bounded above by the fastest time constant present (stability requirement)
- Order = 1 — local truncation error scales as O(h²) per step

## When To Use

In [[concepts/timing-simulation]] of non-stiff MOS digital partitions, where the per-step cost reduction (no matrix, no Newton iteration) outweighs the limitations.

## Risks & Pitfalls

- **Unstable on stiff circuits.** When any time constant is much shorter than the desired timestep, FE diverges. Practical timesteps must stay below the fastest time constant.
- Not used in general-purpose circuit simulation for that reason.
- Local truncation error is O(h²); needs small h for accuracy on smooth trajectories.

## Related Concepts

- [[concepts/integration-method]]
- [[concepts/backward-euler]]
- [[concepts/trapezoidal-rule]]
- [[concepts/gear-bdf]]
- [[concepts/stiff-circuit]]
- [[concepts/timing-simulation]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
