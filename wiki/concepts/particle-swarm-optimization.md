---
title: "Particle Swarm Optimization (PSO)"
type: concept
tags: [algorithm, optimization, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/11-8-placement-of-on-chip-distributed-voltage-regulators.txt"]
confidence: medium
---

## Definition

Particle Swarm Optimization (PSO) is a population-based stochastic global optimization algorithm (Eberhart and Kennedy, 1995) that explores a search space using a "swarm" of candidate solutions ("particles") whose positions update according to each particle's personal best and the swarm's global best.

## How It Works

Each particle has a position x_i and velocity v_i. Each iteration updates v_i ← ω v_i + c_1 r_1 (p_i − x_i) + c_2 r_2 (g − x_i) and x_i ← x_i + v_i, where p_i is the particle's best-known position, g is the swarm's best-known position, ω, c_1, c_2 are weights, and r_1, r_2 are random factors. Discrete PSO variants quantize positions for combinatorial problems such as voltage regulator placement.

## Key Parameters

- Swarm size.
- Inertia weight ω.
- Cognitive and social coefficients c_1, c_2.
- Stopping criterion (iteration count or fitness threshold).

## When To Use

- Black-box global optimization over high-dimensional continuous or discrete spaces.
- VLSI placement and partitioning optimizations.
- Neural architecture search and other hyperparameter tasks.

## Risks & Pitfalls

- Premature convergence to local optima.
- Parameter sensitivity; no guarantees on global optimality.

## Related Concepts

- [[concepts/voltage-regulator-placement]]
- [[concepts/graph-partitioning]]

## Sources

- [[summaries/graphs-in-vlsi-11-8-placement-of-on-chip-distributed-voltage-regulators]]
- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
