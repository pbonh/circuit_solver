---
title: "SOAD Framework (SOA-Compliant DEVS)"
type: concept
tags: [simulation, modeling, devs, soa, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/18-14-service-based-software-systems.txt"]
confidence: medium
---

## Definition

The SOAD (SOA-Compliant DEVS) framework maps Service-Oriented Architecture elements onto DEVS modeling constructs to support virtual build and test of service-based software systems. Services map to DEVS atomic/coupled models; messaging framework maps to ports and couplings; service registry maps to a DEVS executive model.

## How It Works

Three primitive services (Service Provider, Service Client, Service Broker) are DEVS atomic models. Composite services are DEVS coupled models containing at least two primitive providers. Three message types — ServiceInfo (publication), ServiceLookup (subscriber lookup), ServiceCall (invocation with payload) — flow through DEVS ports. The framework is implemented in DEVS-Suite (a DEVSJAVA extension).

## Key Parameters

- Atomic-model mapping for provider/client/broker
- Coupled-model mapping for composite services
- Message-type schemas (ServiceInfo, ServiceLookup, ServiceCall)
- Processing-time and service-duration parameters per provider

## When To Use

- Evaluating QoS of proposed SBS architectures before deployment
- Studying adaptive composition policies
- Validating monitoring/adaptation subsystems with simulated services
- Hierarchical service composition workflows (BPEL-style)

## Risks & Pitfalls

- Broker queue management at high request rates
- Multi-client request lists need bounded sizes
- Time-logic distinction between processing and service duration must be explicit

## Related Concepts

- [[concepts/service-oriented-computing]]
- [[concepts/quality-of-service]]
- [[concepts/discrete-event-system-specification]]
- [[concepts/atomic-devs-model]]
- [[concepts/coupled-devs-model]]

## Sources

- [[summaries/modeling-simulation-systems-18-14-service-based-software-systems]]
- [[summaries/modeling-simulation-systems-19-15-cloud-system-simulation-modeling]]
