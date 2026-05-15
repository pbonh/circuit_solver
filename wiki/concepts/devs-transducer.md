---
title: "DEVS Transducer"
type: concept
tags: [simulation, modeling, devs, instrumentation, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/06-4-devs-natural-language-models-and-elaborations.txt"]
confidence: medium
---

## Definition

A DEVS Transducer is an atomic model that observes events flowing through a coupled simulation to compute performance metrics such as turnaround (completion) time and throughput. It serves as the standard instrumentation pattern in DEVS-based experiments.

## How It Works

The Transducer holds in an `observe` phase for the observation window, accepts `Ariv` (arrival) and `Solved` (completion) messages tagged with WorkToDo identifiers, and tracks `jobsArrived`, `jobsSolved`, `totalTa`, and `clock`. On completion of the observation window it computes average turnaround time and throughput, then transitions to `done` and outputs `Stop` to terminate generation.

## Key Parameters

- Observation time horizon
- Arrival and completion ports
- Maps from job IDs to arrival timestamps
- Cumulative turnaround time and job count

## When To Use

- Measuring throughput and latency in workflow simulations
- Driving experimental design across parameter sweeps
- Observing equivalence between models without disturbing their semantics

## Risks & Pitfalls

- Choosing too short an observation window biases metrics
- Failing to correlate arrivals with completions if IDs are not preserved
- Adding to simulation overhead if transducer logic is expensive

## Related Concepts

- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]
- [[concepts/discrete-event-system-specification]]

## Sources

- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
