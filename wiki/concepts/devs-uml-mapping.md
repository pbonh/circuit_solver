---
title: "DEVS↔UML Mapping"
type: concept
tags: [simulation, modeling, devs, uml, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/15-12-languages-for-constructing-devs-models.txt"]
confidence: low
---

## Definition

The DEVS↔UML Mapping is the bidirectional correspondence between DEVS atomic/coupled models and UML component diagrams, class diagrams, and statecharts. UML can serve both as a source framework for authoring DEVS models and as a target framework for visualizing or implementing them.

## How It Works

DEVS atomic models map naturally to UML statecharts (each phase ↔ state, time-advance ↔ time-elapsed transitions, external transitions ↔ event-triggered transitions). Coupled models map to UML component diagrams or composite-structure diagrams with port-to-port assemblies. Tools such as eUDEVS (Risco-Martin et al.) make UML statecharts executable as DEVS by enforcing the DEVS Simulation Protocol externally. Conversely, code generators produce UML class snippets from DEVS specifications.

## Key Parameters

- Statechart ↔ atomic model state machine
- Component diagram ↔ coupled model assembly
- Sequence diagram ↔ message-exchange trace
- Time-management discipline (DEVS protocol vs. UML statechart simulator)

## When To Use

- Bringing UML-skilled developers into DEVS-based projects
- Generating documentation diagrams from DEVS models
- Importing legacy UML statecharts into DEVS simulation environments

## Risks & Pitfalls

- UML statechart timing semantics differ from DEVS unless the DEVS protocol is enforced
- Round-tripping DEVS↔UML may lose information without standard profiles
- UML's expressiveness may exceed FDDEVS, requiring elaboration

## Related Concepts

- [[concepts/discrete-event-system-specification]]
- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]
- [[concepts/emf-devs]]

## Sources

- [[summaries/modeling-simulation-systems-15-12-languages-for-constructing-devs-models]]
