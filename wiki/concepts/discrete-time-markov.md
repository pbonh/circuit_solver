---
title: Discrete-Time Markov (DTM)
type: claim
id: claim-discrete-time-markov
tags:
- simulation
- modeling
- markov
- stochastic
- devs
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/23-19-devs-support-for-markov-modeling-and-simulation.txt
confidence:
  base: 0.65
---

## Definition

Chapter 19 ("DEVS Support for Markov Modeling and Simulation"): three Markov classes — Continuous-Time Markov (CTM), Discrete-Time Markov (DTM), and Markov Matrix (MM) — "have been implemented in MS4 Me using the Finite Probability DEVS (FP-DEVS) capabilities". DTM is described directly: "the discrete-time version is parameterized by a time step (or cycle length in the common Markov terminology). As with the basic FP-DEVS convention presented in the MS4 Me Users Guide, transitions occur with a time advance equal to the time step and with specified probabilities, in this case, determined by the given rates and the time step."

## How It Works

Per book: "For small enough time steps, the employed probabilities are given by the product of the corresponding CTM values and the time step. For larger time steps, a better approximation is given by employing probabilities equal to `1-exp(-h*p)` where h is the time step and p the corresponding CTM" rate. At each time step the model samples its next state according to that transition vector.

## Key Parameters

- Time step size
- Transition probability vector per state
- Random seed sequence

## When To Use

- Educational exposition of Markov processes
- Coupling with discrete-time DSP components
- Coarse pre-screening before running CTMs

## Risks & Pitfalls

- Time-step choice biases dynamics
- Less accurate than CTM for rare-event analysis
- Larger memory footprint for fine-grained time

## Related Concepts

- [[concepts/continuous-time-markov]]
- [[concepts/markov-matrix-model]]
- [[concepts/finite-probability-devs]]

## Sources

- [[summaries/modeling-simulation-systems-23-19-devs-support-for-markov-modeling-and-simulation]]
