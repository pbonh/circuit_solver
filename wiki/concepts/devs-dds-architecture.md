---
title: DEVS/DDS Architecture
type: claim
id: claim-devs-dds-architecture
tags:
- simulation
- modeling
- devs
- distributed
- middleware
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/13-10-dynamic-structure-agent-modeling-and-publish-subscribe.txt
confidence:
  base: 0.65
---

## Definition

The DEVS/DDS Architecture maps the DEVS Simulation Protocol onto DDS middleware so that coordinator and simulators communicate exclusively through DDS topics. A DEVS participant contains a model and a simulator (or coordinator) with Data Writers and Data Readers mediating publications and subscriptions.

## How It Works

Standard DEVS protocol cycle over DDS:
1. Coordinator publishes to topic `GetTN` → simulators subscribe.
2. Each simulator publishes to `MyTN`.
3. Coordinator publishes `GetOutput` (carrying global time).
4. Imminent simulators publish to `MyOutput` with (name, output).
5. Coordinator collects all `MyOutput` data, applies coupling.
6. Coordinator publishes per-model input on `StoreInputForModel`.
7. Coordinator publishes `DoDelta` → simulators apply transitions.

DEVS messages are encoded in XML as MessageBag/Message/Port/Data trees, enabling Java/C++/etc. interoperability.

## Key Parameters

- Topic set (GetTN, MyTN, GetOutput, MyOutput, StoreInputForModel, DoDelta)
- XML schema for DEVS messages
- DDS QoS profile per topic
- Termination convention (e.g., minimum tN = infinity)

## When To Use

- Real-time DEVS distributed simulation
- Integration with non-DEVS DDS participants
- Service-oriented and cloud-native simulation deployments

## Risks & Pitfalls

- Per-model topics violate data-centric ideal — design coupling to minimize specificity
- XML overhead can dominate small simulations
- Synchronization requires blocking subscription waits

## Related Concepts

- [[concepts/data-distribution-service]]
- [[concepts/publish-subscribe]]
- [[concepts/devs-simulation-protocol]]

## Sources

- [[summaries/modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe]]
