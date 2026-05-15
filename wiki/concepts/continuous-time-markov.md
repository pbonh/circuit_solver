---
title: "Continuous Time Markov (CTM)"
type: concept
tags: [simulation, modeling, markov, stochastic, devs, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/23-19-devs-support-for-markov-modeling-and-simulation.txt"]
confidence: medium
---

## Definition

A Continuous Time Markov (CTM) model is a stochastic DEVS atomic model in which each outgoing arc from a state carries a rate parameter; the simulator samples an exponential distribution per arc, selects the minimum sample as the next-event time, and transitions to the corresponding target state.

## How It Works

For each phase with outgoing transitions of rates p1, p2, ..., the simulator computes σi = (1/pi) × (−ln ri) with ri uniform [0,1]. The next-event time is min(σi); the selected target is the arc that achieved the minimum. Self-loops are excluded from selection. The total exit rate of a state is Σpi, and the average residence time is 1/Σpi. The DEVS-Markov coupling lets CTMs be integrated into larger DEVS models like any atomic component.

## Key Parameters

- Per-arc rate parameter
- Random seed sequence
- State-to-state transition graph
- Average residence times per state

## When To Use

- Stochastic Stock Market models
- Cost-effectiveness analysis in healthcare
- Compartmental epidemiological models
- Queueing performance studies

## Risks & Pitfalls

- Rate normalization required when rates exceed unit-sum convention
- Reproducibility depends on seed management
- Long-run averages require sufficient sample horizons

## Related Concepts

- [[concepts/discrete-time-markov]]
- [[concepts/markov-matrix-model]]
- [[concepts/finite-probability-devs]]
- [[concepts/exponential-distribution-sampling]]

## Sources

- [[summaries/modeling-simulation-systems-23-19-devs-support-for-markov-modeling-and-simulation]]
