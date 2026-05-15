---
title: "Finite Probability DEVS (FP-DEVS)"
type: concept
tags: [simulation, modeling, devs, markov, stochastic, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/23-19-devs-support-for-markov-modeling-and-simulation.txt"]
confidence: low
---

## Definition

Finite Probability DEVS (FP-DEVS) is the MS4 Me capability that extends FDDEVS with per-transition probability labels, enabling the same state-diagram authoring interface to generate Continuous Time Markov (CTM), Discrete-Time Markov (DTM), or Markov Matrix (MM) models depending on the chosen interpretation.

## How It Works

The state diagram uses dashed transition arrows labeled with rate or probability values. The simulator interpretation is selected per model. FP-DEVS handles normalization (e.g., computing the self-transition probability as 1 − Σ outgoing probabilities), random sampling for CTM/DTM, and deterministic matrix iteration for MM. The resulting model is a full DEVS atomic model that can be coupled with other DEVS components.

## Key Parameters

- Per-arc probability or rate label
- Interpretation flag (CTM/DTM/MM)
- Random seed (CTM/DTM)
- Convergence tolerance (MM)

## When To Use

- Stochastic agent-based DEVS modeling
- Queueing-network performance analysis
- Healthcare and epidemiological cost-effectiveness studies

## Risks & Pitfalls

- Mixing interpretations within one model
- Probability vs. rate confusion across CTM and MM views
- Tooling lock-in to MS4 Me's graphical menu

## Related Concepts

- [[concepts/continuous-time-markov]]
- [[concepts/markov-matrix-model]]
- [[concepts/finite-deterministic-devs]]
- [[concepts/discrete-event-system-specification]]

## Sources

- [[summaries/modeling-simulation-systems-23-19-devs-support-for-markov-modeling-and-simulation]]
