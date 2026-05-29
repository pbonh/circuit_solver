---
title: Pseudo-Transient Analysis
type: claim
id: claim-pseudo-transient-analysis
tags:
- analog
- dc
- transient
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/simulation_whitepaper_v1/simulation_whitepaper1.txt
confidence:
  base: 0.85
---

## Definition

Pseudo-transient analysis is a [[concepts/homotopy-method]] convergence aid that recasts a stubborn DC problem as a transient one: 1 F linear capacitors are added from every node to ground, all capacitors start at zero, and a [[concepts/transient-analysis]] is run with time as the continuation parameter, sweeping from t = 0 (trivial solution) toward t = ∞ (the desired DC operating point).

## How It Works

The simulator appends node-to-ground capacitors with initial voltage zero. Independent sources retain their final values throughout. A standard transient integration scheme then drives the circuit toward its equilibrium; once the trajectory settles, the final state is reported as the DC solution.

## Key Parameters

- Magnitude of the added capacitance (1 F is the default — large enough to slow the trajectory smoothly)
- Transient stopping criterion (settling threshold or maximum simulated time)
- Underlying [[concepts/integration-method]] and its tolerances

## When To Use

When neither [[concepts/source-stepping]] nor [[concepts/gmin-stepping]] converges. Pseudo-transient avoids the fold and bifurcation pathologies of those methods because the trajectory is a real time-evolution of a damped system.

## Risks & Pitfalls

- The augmented circuit can **oscillate**. Once the time-domain trajectory enters a stable limit cycle, increasing simulated time never settles, so pseudo-transient never terminates. This is the only major discontinuity it cannot escape.
- Adds artificial dynamics, so it is unsuitable for finding unstable equilibria (the trajectory would never settle there).
- Slower than the other two homotopy methods per problem, but more reliable on multi-equilibrium circuits.

## Related Concepts

- [[concepts/homotopy-method]]
- [[concepts/source-stepping]]
- [[concepts/gmin-stepping]]
- [[concepts/transient-analysis]]
- [[concepts/dc-analysis]]

## Sources

- [[summaries/kundert-bctm98-simulation-tutorial]]
