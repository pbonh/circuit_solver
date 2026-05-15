---
title: "Simulation Executive"
type: concept
tags: [simulation, modeling, devs, distributed, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/10-8-automated-and-rule-based-pruning-and-experimental-execution.txt"]
confidence: medium
---

## Definition

A Simulation Executive is the control logic that orchestrates execution of a family of simulation models, including starting conditions, random seeds, alternative selection from SES, and aggregation of results across runs.

## How It Works

The book describes three levels:
1. Hand-coded main routine controlling a fixed Model-and-Frame pair.
2. SES-driven control that loads (ses, pes) files, invokes `InternalUseSeS` to prune and transform, then runs the resulting model.
3. Outer iteration loop that re-samples PESs from the residual subspace, runs each on a separate core or remote node, and runs statistical significance tests for termination.

## Key Parameters

- Number of repetitions
- Distributed processor/core pool
- Random seed sequence
- Termination significance criteria

## When To Use

- High-throughput design-space exploration
- Distributed simulation across compute clusters
- Statistical experiments with adaptive termination

## Risks & Pitfalls

- Distributed coordination of pruning and result aggregation
- Long simulations may need checkpointing
- Repro requires recording the random seed used per run

## Related Concepts

- [[concepts/experimental-frame]]
- [[concepts/automated-pruning]]
- [[concepts/discrete-event-system-specification]]

## Sources

- [[summaries/modeling-simulation-systems-10-8-automated-and-rule-based-pruning-and-experimental-execution]]
