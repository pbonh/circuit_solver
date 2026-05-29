---
title: DEVS↔UML Mapping
type: claim
id: claim-devs-uml-mapping
tags:
- simulation
- modeling
- devs
- uml
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/15-12-languages-for-constructing-devs-models.txt
confidence:
  base: 0.65
---

## Definition

Per Chapter 12 ("Languages for Constructing DEVS Models"): "Let's consider mapping DEVS to UML which amounts to providing another form of DEVS simulator. In particular, DEVS models can be mapped to the UML component and statechart diagrams (Zinoviev 2005). From the distributed simulation perspective, DEVS atomic models can be mapped to XML statecharts (Risco-Martin et al. 2009). In another work, atomic models are expressed as statecharts (Mooney and Sarjoughian 2009). Users can develop DEVS-UML statecharts that are executable as DEVS models. This accounts for time in UML models through implementing the DEVS Protocol as a system-wide protocol of events without relying on the timing in the statechart simulator. Equivalency between statecharts and DEVS was employed in modeling embedded systems (Schulz et al. 2000)."

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
