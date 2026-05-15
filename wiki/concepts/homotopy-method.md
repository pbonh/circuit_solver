---
title: "Homotopy Method (Continuation)"
type: concept
tags: [analog, dc, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Homotopy methods (a.k.a. continuation methods) are convergence aids used when plain [[concepts/newton-raphson-method]] fails to find the DC solution of a circuit. They modify the original problem by introducing a parameter λ, choose a parameterization in which λ = 0 yields a trivially solvable problem and λ = 1 recovers the original, and step λ from 0 to 1 — using the solution at each step as the initial guess for the next. The continuity of the solution trajectory means each step is "close enough" for NR to converge.

## How It Works

A simulator picks a continuation parameter and solves the easy endpoint. It then increments λ, runs NR with the previous solution as the initial guess, and repeats. The three canonical variants for SPICE-class simulators are:

- [[concepts/source-stepping]] — λ multiplies independent-source values; at λ = 0 all sources are zero and the solution is identically zero.
- [[concepts/gmin-stepping]] — λ controls a parallel conductance swept from 1 Ω to ~10¹² Ω across every nonlinear device; at high Gmin the circuit is linear-dominated and easy.
- [[concepts/pseudo-transient-analysis]] — adds a 1 F capacitor from every node to ground and runs a transient from time 0 to ∞; time itself is the homotopy parameter.

## Key Parameters

- Step-size adaptation in λ (must shrink near folds)
- Choice of continuation parameter — sources, Gmin, time, or a combination
- Arc-length reformulation, which can traverse simple folds but adds cost per fold

## When To Use

- After plain NR from a zero or nodeset initial guess fails on the DC analysis.
- As a fallback chain: simulators typically try source stepping, then Gmin stepping, then pseudo-transient — each is non-guaranteed but covers a different failure mode.

## Risks & Pitfalls

- **Folds** in the solution trajectory occur naturally on circuits with multiple equilibria; arc-length methods can step around them but at significant per-fold cost. A circuit with k stacked bistable cells can have on the order of 3ᵏ − 1 folds.
- **Bifurcations** arise from perfect symmetry in both the circuit and the starting point — easily defeated with a small random perturbation of the start.
- **Discontinuities** in device models break the trajectory's continuity entirely; fix the model.
- **Oscillation in pseudo-transient** — when time is mapped to λ ∈ [0, 1], an undamped oscillator pushes the trajectory through infinitely many cycles as λ → 1, another form of discontinuity. Pseudo-transient cannot solve such circuits.

## Related Concepts

- [[concepts/newton-raphson-method]]
- [[concepts/dc-analysis]]
- [[concepts/source-stepping]]
- [[concepts/gmin-stepping]]
- [[concepts/pseudo-transient-analysis]]
- [[concepts/nodeset]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
