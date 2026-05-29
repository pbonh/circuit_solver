---
title: Experimental Frame
type: claim
id: concepts/experimental-frame
tags:
- simulation
- modeling
- devs
- experimentation
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/10-8-automated-and-rule-based-pruning-and-experimental-execution.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

An Experimental Frame (EF) is a structured trio of DEVS components that surrounds a model under test to define the conditions of an experiment: a Generator (input trajectories), an Acceptor (termination/condition checking), and a Transducer (data collection).

## How It Works

The EF is coupled to the model under test. The Simulator drives the EF and the model on a common time base. Each iteration the Acceptor is queried for termination; on termination, the Transducer is queried for collected metrics and the results are persisted. Repetition counts and random seeds parameterize the outer experimental loop.

## Key Parameters

- Generator input pattern
- Acceptor termination predicate
- Transducer data-collection schema
- NumReps repetition count
- Random seed sequence

## When To Use

- Performance evaluation of a simulated SoS
- Comparative experiments across PES variants
- Sensitivity analysis under parameter sweeps
- Distributed/parallel simulation experiments

## Risks & Pitfalls

- Acceptor logic must terminate the experiment reliably
- Transducer must capture all metrics of interest before termination
- Generator coverage gaps can bias conclusions

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/devs-transducer]]
- [[concepts/simulation-executive]]
- [[concepts/automated-pruning]]

## Sources

- [[summaries/modeling-simulation-systems-10-8-automated-and-rule-based-pruning-and-experimental-execution]]
- [[summaries/modeling-simulation-systems-22-18-activity-based-implementations-of-systems-of-systems]]
