---
title: Finite Probability DEVS (FP-DEVS)
type: claim
id: concepts/finite-probability-devs
tags:
- simulation
- modeling
- devs
- markov
- stochastic
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/23-19-devs-support-for-markov-modeling-and-simulation.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

> Chapter 19 attributes the FP-DEVS construct to the MS4 Me toolset and grounds the three Markov modeling classes (CTM, DTM, MM) on it: "Markov models, having both discrete and continuous time bases, have been implemented in MS4 Me using the Finite Probability DEVS (FP-DEVS) capabilities (described below)." The text refers to "the basic FP-DEVS convention presented in the MS4 Me Users Guide" — transitions occur with a time-advance and specified probabilities, parameters that are interpreted differently for CTM, DTM, and MM models.

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
