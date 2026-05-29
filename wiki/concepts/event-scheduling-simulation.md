---
title: Event-Scheduling Simulation
type: claim
id: claim-event-scheduling-simulation
tags:
- simulation
- modeling
- discrete-event
- well-established
- foundational
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt
confidence:
  base: 0.65
---

## Definition

Event-Scheduling Simulation is a classical discrete-event simulation paradigm in which the simulator maintains a global event list ordered by time and repeatedly executes the imminent event's code, which can post or cancel further events and produce outputs.

## How It Works

Core operations: `GetTimeOfImminentEvent()` returns the smallest scheduled time `tN`; `GetNRemoveImminentEvent(t)` advances to time `t == tN`, executes that event's code (which may post new events and produce output); `AddEvent(m, t)` treats `m` as external input at current time. A DEVS Simulator wrapper can translate the DEVS protocol's GetTN/GetOutput/StoreInput operations to these calls, federating event-scheduling simulators with DEVS coupled models.

## Key Parameters

- Time-ordered event list
- Event code (output generation, event posting, cancellation)
- External event injection time

## When To Use

- Wrapping legacy discrete-event simulators in DEVS-based federations
- Educational exposition of discrete-event semantics
- Standalone simulations without DEVS hierarchy

## Risks & Pitfalls

- Event-scheduling code can mix model and execution concerns
- Cancellation logic is error-prone
- Integration with DEVS protocol requires message-format translation

## Related Concepts

- [[concepts/devs-simulation-protocol]]
- [[concepts/discrete-event-system-specification]]
- [[concepts/simulation-interoperability]]

## Sources

- [[summaries/modeling-simulation-systems-12-9-devs-simulation-protocol]]
