---
title: Random Walk
type: claim
id: claim-random-walk
tags:
- algorithm
- stochastic
- graph
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/08-5-circuit-analysis.txt
confidence:
  base: 0.65
---

## Definition

A random walk is a stochastic process consisting of a sequence of steps drawn from a probability distribution over a state space. On a weighted graph, the walker at node v transitions to neighbor u with probability proportional to the weight of edge (v, u).

## How It Works

Random walks on graphs are intimately connected to electrical networks: commute time between nodes a and b (expected number of steps for a walker starting at a to visit b and return) equals the effective resistance R_ab times the total edge weight. This equivalence (Nash-Williams, Doyle and Snell) supports random-walk-based circuit simulators that estimate node voltages by sampling. Error scales as ε ∝ 1/√M where M is the number of walks.

## Key Parameters

- Number of walks M.
- Maximum walk length.
- Transition-probability matrix (proportional to edge weights).
- Importance-sampling strategy.

## When To Use

- Estimating effective resistance / voltage at a few nodes in a huge grid.
- Sensitivity analysis of power networks.
- Decoupling capacitor placement, electromigration analysis.
- Sparse preconditioning.

## Risks & Pitfalls

- Slow 1/√M convergence requires many samples for tight tolerance.
- Long walks negate any speedup; truncation degrades accuracy.

## Related Concepts

- [[concepts/effective-resistance]]
- [[concepts/lattice-graph]]
- [[concepts/modified-nodal-analysis]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-08-5-circuit-analysis]]
