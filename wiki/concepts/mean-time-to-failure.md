---
title: "Mean Time to Failure (MTTF)"
type: concept
tags: [vlsi, reliability, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt"]
confidence: low
---

## Definition

Mean Time To Failure (MTTF) is the expected operating time of a device or component until its first failure. For VLSI interconnects, MTTF is commonly modeled using Black's equation MTTF = (K / J^n) · exp(E_a / kT), where J is current density, T is temperature, E_a is activation energy, and n ≈ 2 for many conductor materials.

## How It Works

In the power-delivery exploration framework, MTTF is rewritten in terms of interconnect width W, thickness H, and RMS current as MTTF = K_1 W^n H^n / I_rms^n · exp(K_2 W^2 H^2 / I_rms^2). This couples physical design variables (interconnect dimensions) to reliability objectives. Wider/thicker conductors increase MTTF but consume area.

## Key Parameters

- Current density and RMS current.
- Operating temperature.
- Conductor material constants.
- Wire dimensions.

## When To Use

- Reliability-driven optimization of power and clock distribution networks.
- Constraint specification in early-stage design exploration.

## Risks & Pitfalls

- Empirical model with significant fab-process variation.
- Worst-case temperature and current must be estimated accurately for meaningful results.

## Related Concepts

- [[concepts/electromigration]]
- [[concepts/power-delivery-exploration]]
- [[concepts/power-distribution-network]]

## Sources

- [[summaries/graphs-in-vlsi-12-9-exploratory-methodology-for-power-delivery]]
