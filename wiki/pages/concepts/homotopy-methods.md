---
title: Homotopy Methods
type: concept
slug: homotopy-methods
created: 2026-06-16
updated: 2026-06-16
summary: Convergence recovery techniques that parameterize circuit equations from an easy-to-solve form to the desired form, using each step's solution as the next step's initial guess.
tags: [numerical-methods, convergence, circuit-simulation, continuation]
sources: [simulation-analog-mixed-signal-circuits]
status: active
---

# Homotopy Methods

Also called **continuation methods**. When [[newton-raphson]] fails to converge in [[spice-simulation]], the simulator parameterizes the circuit equations with λ ∈ [0, 1] such that λ=0 yields an easy-to-solve problem and λ=1 yields the original problem. Solving a sequence of problems along this path — using each solution as the next initial guess — exploits both the continuity of the solution trajectory and NR's local convergence properties.

## Variants

### Source Stepping
- λ multiplies all independent source values; sweep from 0 (all sources = 0, trivial solution) to 1 (desired values)
- **Weakness**: trajectories frequently hit folds (discontinuities from multiple solutions), causing failure

### Gmin Stepping
- λ controls Gmin conductors across every nonlinear device; sweep from 1 Ω to 10^12 Ω
- Less susceptible to folds than source stepping; generally reliable
- **Preferred** homotopy method in practice

### Pseudo-Transient Analysis
- Adds 1 F capacitors from every node to ground; runs transient from t=0 to ∞ (λ = time)
- Not subject to folds or bifurcations
- **Weakness**: circuit may oscillate, preventing convergence; oscillation becomes infinitely fast as λ→1

## Why Homotopy Methods Fail

Four discontinuity types on the homotopy trajectory:
1. **Simple discontinuities**: discontinuous model equations
2. **Folds**: natural consequence of multiple equilibrium points; arc-length reformulation can handle them but is expensive (a circuit with N copies of a latch with 3 equilibria has 3^N - 1 folds)
3. **Bifurcations**: from symmetric circuits with symmetric starting points; easily avoided with a random initial guess
4. **Oscillations** (pseudo-transient only): oscillating circuit response; maps to infinite-frequency discontinuity as λ→1

## Why it matters

- Most practical SPICE convergence problems are solved by Gmin stepping
- Understanding fold structure explains why source stepping is unreliable on digital circuits with many latches
- Knowing when pseudo-transient will oscillate prevents wasted simulation time

## Related concepts and entities

- [[newton-raphson]] - the underlying solver homotopy assists
- [[spice-simulation]] - context in which homotopy methods are invoked
- [[circuit-simulation]] - parent topic
