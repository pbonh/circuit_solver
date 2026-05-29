---
title: DEVS Ports and Messages
type: claim
id: claim-devs-port-and-message
tags:
- simulation
- modeling
- devs
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/06-4-devs-natural-language-models-and-elaborations.txt
confidence:
  base: 0.85
---

## Definition

In DEVS implementations, a Port is a typed channel through which messages enter or leave an atomic model; a Message is a pairing of a Port with a Serializable value belonging to the port's associated class; a MessageBag is a multiset of messages used to represent the inputs and outputs at any one DEVS event instant.

## How It Works

Ports are added via `addInputPort("name", Class.class)` and `addOutputPort("name", Class.class)`. The output function builds a `MessageBag` by calling `output.add(port, value)`. The external transition function receives a `MessageBag input` and inspects it with `input.hasMessages(port)` and `port.getMessages(input)`. FDDEVS's default Java translation looks at only the first content element of each input bag; richer multi-input/multi-output behavior is achieved via dnl elaboration.

## Key Parameters

- Port name and associated value class
- Serializable value type
- MessageBag collection with possible duplicates
- Bag iteration patterns for multi-input handling

## When To Use

- All DEVS atomic-model implementations
- Modeling cyber-physical interfaces where multiple inputs may arrive simultaneously
- High-priority/low-priority routing via multiple ports

## Risks & Pitfalls

- Forgetting that bags can carry multiple values per port
- Ignoring elapsed time in external transitions
- Type mismatches between port declarations and value casts in elaboration code

## Related Concepts

- [[concepts/atomic-devs-model]]
- [[concepts/dnl-elaboration]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-06-4-devs-natural-language-models-and-elaborations]]
