---
title: "DEVS Coordinator"
type: concept
tags: [simulation, modeling, devs, distributed, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt"]
confidence: high
---

## Definition

A DEVS Coordinator is the runtime object that orchestrates one or more DEVS Simulators executing the components of a coupled model. It holds the coupling specification, manages global time, and dispatches the DEVS Simulation Protocol operations.

## How It Works

The coordinator queries every simulator for its next-event time, computes the minimum (becomes new global time), requests outputs from imminent simulators, applies the coupling map to produce per-target input bags, delivers them, and triggers transitions. In the standard protocol the coordinator routes all messages; peer-exchange variants distribute coupling segments to simulators and reduce the coordinator's role; real-time variants leave time advancement to simulators.

## Key Parameters

- Coupling specification of the coupled model
- Global time
- Set of attached simulators
- Implementation variant

## When To Use

- All DEVS coupled-model simulations
- Distributed/parallel execution across multiple cores or hosts
- Hierarchical compositions where a coordinator may itself be presented as a simulator one level up

## Risks & Pitfalls

- Bottleneck in standard variant for large simulator counts
- Network latency in distributed variants violates strict time semantics
- Recursive composition needs careful interface alignment

## Related Concepts

- [[concepts/devs-simulation-protocol]]
- [[concepts/abstract-devs-simulator]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
