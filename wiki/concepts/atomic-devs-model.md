---
title: Atomic DEVS Model
type: claim
id: concepts/atomic-devs-model
tags:
- simulation
- modeling
- devs
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/04-2-devs-integrated-development-environments.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

An Atomic DEVS Model is the leaf-level specification of behavior in the DEVS formalism. It contains the sets (inputs, states, outputs) and functions (internal transition, external transition, output, and time-advance) that constructively specify a single dynamic system on a continuous time base.

## How It Works

Given a current state, the time-advance function determines how long the system remains there. When that time elapses the internal transition fires, producing an output via the output function and moving to a new state. If an external input arrives first, the external transition function reacts based on the elapsed time and the input message. Together these functions specify exactly one dynamic system per atomic model when the well-definition conditions hold.

## Key Parameters

- Input port set with message types
- Output port set with message types
- State set
- Internal transition function
- External transition function
- Output function
- Time-advance function

## When To Use

- Modeling leaf components of any DEVS hierarchy
- Capturing discrete-event behavior at the lowest level
- Generating Java/C++ implementations from FDDEVS specifications

## Risks & Pitfalls

- Forgetting the time-advance for hold states
- Confluent-event handling requires care (Parallel DEVS)
- External-transition logic that ignores elapsed time can yield unintended behavior

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/coupled-devs-model]]
- [[concepts/finite-deterministic-devs]]
- [[concepts/closure-under-coupling]]

## Sources

- [[summaries/modeling-simulation-systems-04-2-devs-integrated-development-environments]]
- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
- [[summaries/modeling-simulation-systems-09-7-managing-inheritance-in-pruning]]
- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
- [[summaries/modeling-simulation-systems-23-19-devs-support-for-markov-modeling-and-simulation]]
