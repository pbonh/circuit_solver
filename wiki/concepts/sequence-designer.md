---
title: Sequence Designer
type: claim
id: claim-sequence-designer
tags:
- simulation
- modeling
- devs
- tooling
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/06-4-devs-natural-language-models-and-elaborations.txt
confidence:
  base: 0.65
---

## Definition

The Sequence Designer (SD) is an MS4 Me tool that transforms sequence diagrams (actors and message arrows) directly into FDDEVS atomic models and a coupling SES, dramatically accelerating early-stage DEVS model authoring.

## How It Works

The modeler draws a sequence diagram with actors as lifelines and labeled message arrows. The SD synthesizes an SES capturing the actor decomposition plus all the coupling statements, and a `*.dnl` file for each actor capturing the wait/send phases that realize the message sequence. An actor can be re-classed from atomic to coupled and elaborated with its own internal sequence diagram, supporting hierarchical model construction (as shown in the UAS testing example).

## Key Parameters

- Actors (lifelines) per diagram
- Message arrows with names and ordering
- Atomic-vs-coupled actor type flag
- Generated SES and dnl artifacts

## When To Use

- Rapid prototyping of multi-component scenarios
- Bootstrapping models from existing UML-style sequence diagrams
- Refining a coupled component into deeper hierarchy without losing the parent interface

## Risks & Pitfalls

- The auto-generated atomic models are skeletons — elaborations still needed for real behavior
- Sequence-diagram ordering may oversimplify concurrent scenarios
- Specializations and multi-aspects must be added after SD generation

## Related Concepts

- [[concepts/finite-deterministic-devs]]
- [[concepts/system-entity-structure]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
