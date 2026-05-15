---
title: "Source Stepping"
type: concept
tags: [analog, dc, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/simulation_whitepaper_v1/simulation_whitepaper1.txt"]
confidence: high
---

## Definition

Source stepping is a [[concepts/homotopy-method]] continuation aid for [[concepts/dc-analysis]] in which every independent source's value is multiplied by a continuation parameter λ swept from 0 to 1. At λ = 0 all stimuli are zero and the trivial zero solution satisfies the system; at λ = 1 the circuit recovers its specified operating condition.

## How It Works

The simulator scales `V_source(λ) = λ · V_specified` and `I_source(λ) = λ · I_specified` for every independent source. It solves the system at λ = 0 (zero solution), increments λ, re-runs [[concepts/newton-raphson-method]] using the previous solution as the initial guess, and repeats until λ = 1.

## Key Parameters

- λ step size and adaptive step-size control on convergence failure
- Maximum NR iterations per λ step

## When To Use

A fallback for [[concepts/dc-analysis]] convergence when plain NR and a user [[concepts/nodeset]] are insufficient. Cheapest of the three classical aids per successful step.

## Risks & Pitfalls

- Source-stepping trajectories tend to be heavily folded in circuits with feedback or hysteresis, which causes step-size collapses or outright failure. [[concepts/gmin-stepping]] usually works better.
- Like all homotopy methods, vulnerable to discontinuities in device models.
- Symmetric circuits with symmetric initial points can bifurcate — perturb the start.

## Related Concepts

- [[concepts/homotopy-method]]
- [[concepts/gmin-stepping]]
- [[concepts/pseudo-transient-analysis]]
- [[concepts/dc-analysis]]
- [[concepts/newton-raphson-method]]

## Sources

- [[summaries/computer-methods-circuit-analysis-design-15-chapter-12-dc-solution-of-networks]]
- [[summaries/kundert-bctm98-simulation-tutorial]]
