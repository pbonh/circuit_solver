---
title: Power Delivery Exploration
type: claim
id: concepts/power-delivery-exploration
tags:
- vlsi
- power-integrity
- optimization
- novel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Power delivery exploration is the early-stage VLSI design methodology that performs constrained global optimization of the power distribution network — jointly considering electrical metrics (IR drop, efficiency, frequency) and non-electrical metrics (cost, area, MTTF) — before committing to physical placement and routing.

## How It Works

A simplified power network model (cascaded RL-RLC stages: PCB, package, die) is constructed. Design variables (supply voltage, decoupling capacitor values, interconnect widths, number of voltage domains) are exposed to a global optimizer. The objective f(x) combines electrical and non-electrical terms; constraints c(x) ≤ 0 enforce IR drop, power, frequency, and reliability requirements. A fast Laplace-transform symbolic simulator evaluates each candidate quickly.

## Key Parameters

- Number and granularity of voltage domains.
- Decoupling capacitor values at each level.
- Supply voltage and metal dimensions.
- Cost weights for area at PCB, package, die.

## When To Use

- Pre-floorplan stage of high-performance SoC design.
- Trade-off studies (rail count, decap budget).
- Reducing late-stage redesign iterations.

## Risks & Pitfalls

- Early-stage models can be inaccurate; assumptions must be carefully chosen.
- Premature convergence to local minima — needs swarm diversity or restarts.
- Saving downstream iterations depends on accurate exploration time vs. simulate-and-correct time accounting.

## Related Concepts

- [[concepts/power-distribution-network]]
- [[concepts/laplace-transform-simulator]]
- [[concepts/decoupling-capacitor]]
- [[concepts/voltage-regulator-placement]]
- [[concepts/particle-swarm-optimization]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
- [[summaries/graphs-in-vlsi-15-12-conclusions]]
