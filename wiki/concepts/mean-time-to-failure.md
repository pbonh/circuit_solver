---
title: Mean Time to Failure (MTTF)
type: claim
id: claim-mean-time-to-failure
tags:
- vlsi
- reliability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GraphsInVLSI/_txt/12-9-exploratory-methodology-for-power-delivery.txt
confidence:
  base: 0.65
---

## Definition

Per GraphsInVLSI Chapter 9: "if the mean time to failure (MTTF) is of concern, optimizing MTTF would place an upper limit on the current density and temperature, as shown in [538]". The book's Eq. 9.3 is Black's equation:

> MTTF = (K / j^n) · exp(E_a / kT)

"where K and n are material and process constants, Ea is the activation energy, k is the Boltzmann constant, T is the temperature, and j is the current density."

## How It Works

GraphsInVLSI Eq. 9.5 adapts Black's equation into a form parametrised by interconnect geometry (after ref. [539]):

> MTTF = K_1 W^n H^n / I_rms^n · exp(K_2 W^2 H^2 / I_rms^2)

This couples physical design variables (interconnect width W and thickness H) to the reliability objective directly through I_rms. Wider/thicker conductors increase MTTF but consume area, making MTTF a natural objective in the chapter's exploratory power-delivery optimization framework.

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
