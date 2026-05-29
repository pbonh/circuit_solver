---
title: Parallel DEVS
type: claim
id: claim-parallel-devs
tags:
- simulation
- modeling
- devs
- foundational
- well-established
- parallel
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/03-1-modeling-and-simulation-of-systems-of-systems.txt
confidence:
  base: 0.65
---

## Definition

Parallel DEVS is a variant of the DEVS formalism that explicitly handles simultaneous events in coupled-model components. It generalizes the classic DEVS confluent-transition function and supports bag-valued ports so that multiple inputs/outputs at the same instant are accommodated without arbitrary serialization.

## How It Works

When multiple atomic components scheduled at the same time fire concurrently, Parallel DEVS uses a confluent function on each receiver to decide how concurrent external inputs combine with the component's own internal transition. This makes it well suited to multiprocessor and distributed simulation engines, including DEVS-Suite.

## Key Parameters

- Confluent transition function per atomic model
- Bag-valued input/output ports
- Concurrent-event handling at coupled-model level

## When To Use

- Distributed and parallel simulation of large SoS
- Systems with frequent simultaneous events (digital hardware, sensor networks)
- CoSMoS/DEVS-Suite hierarchical parallel DEVS workflows

## Risks & Pitfalls

- Confluent function authoring is easy to overlook
- Subtle differences vs. classic DEVS semantics in event-ordering edge cases
- Performance hinges on engine quality

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/finite-deterministic-devs]]

## Sources

- [[summaries/modeling-simulation-systems-03-1-modeling-and-simulation-of-systems-of-systems]]
- [[summaries/modeling-simulation-systems-20-16-model-development-and-execution-process-with-repositories-validation-and-verification]]
