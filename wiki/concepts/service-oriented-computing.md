---
title: Service-Oriented Computing (SOC)
type: claim
id: claim-service-oriented-computing
tags:
- simulation
- modeling
- soa
- distributed
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/18-14-service-based-software-systems.txt
confidence:
  base: 0.65
---

## Definition

Service-Oriented Computing (SOC) is a distributed-computing paradigm in which software is built as compositions of well-defined, self-contained services that provide functionality to subscribers via standardized message-based interactions. Service-Based Software Systems (SBS) are concrete realizations.

## How It Works

Service providers publish service descriptions (WSDL) to a service broker (registered via UDDI). Subscribers look up services via the broker, then invoke providers directly using SOAP-encoded messages. The architecture enforces loose coupling: providers and subscribers know each other only through broker-mediated descriptors.

## Key Parameters

- Service description language (WSDL)
- Discovery and registry standard (UDDI)
- Invocation protocol (SOAP/HTTP)
- QoS attributes (accuracy, timeliness, throughput)

## When To Use

- Loosely coupled distributed systems
- Cross-organization service federations
- Adaptive software that must reconfigure dynamically
- SoS within enterprise IT

## Risks & Pitfalls

- Broker as a bottleneck
- QoS negotiation complexity grows with composition depth
- Standards proliferation (WSDL/UDDI/SOAP vs. REST/JSON)

## Related Concepts

- [[concepts/service-oriented-architecture]]
- [[concepts/quality-of-service]]
- [[concepts/soad-framework]]
- [[concepts/publish-subscribe]]

## Sources

- [[summaries/modeling-simulation-systems-18-14-service-based-software-systems]]
- [[summaries/modeling-simulation-systems-19-15-cloud-system-simulation-modeling]]
