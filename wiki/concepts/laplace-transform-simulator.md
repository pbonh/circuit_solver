---
title: Laplace Transform Simulator
type: claim
id: claim-laplace-transform-simulator
tags:
- vlsi
- simulation
- analysis
- novel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt
confidence:
  base: 0.65
---

## Definition

A Laplace-transform-based simulator represents a linear circuit symbolically in the s-domain, applies modified nodal analysis once to obtain symbolic transfer functions H(s) = N(s) / D(s) whose coefficients depend on design variables, and re-uses this symbolic form across many optimization iterations to drastically reduce per-evaluation runtime.

## How It Works

Each linear element is expressed in the s-domain: a capacitor with capacitance C becomes Z_C = R_esr + L_esl s + 1/(C s); resistors and inductors similarly. Variable parameters (e.g., capacitance C) remain symbolic. MNA produces the symbolic matrix system [[Y B]; [C D]] [V; I] = [J; F] which is solved once to obtain transfer-function coefficients as functions of the design variables. Time-domain simulation is then performed by converting H(s) to a state-space model and integrating. Speedup grows with iteration count: speedup = t_n / (t_setup/N + t_L) for N iterations.

## Key Parameters

- Number of variable design parameters (symbolic dimensions).
- Iteration count N during outer optimization.
- Setup cost t_setup vs. per-iteration cost t_L.

## When To Use

- Inner-loop circuit simulation during constrained global optimization of linear power networks.
- Repetitive simulation with same topology and changing parameter values.
- Pre-floorplan power-delivery exploration.

## Risks & Pitfalls

- Strict applicability to linear or piecewise-linear circuits.
- High t_setup if many variable parameters.
- Nonlinear elements (power gating, switching converters) require numerical simulators or piecewise-linearization.

## Related Concepts

- [[concepts/modified-nodal-analysis]]
- [[concepts/state-space-model]]
- [[concepts/power-delivery-exploration]]
- [[entities/spice]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
