---
title: "Adaptive Service-Based Software System (ASBS)"
type: concept
tags: [simulation, modeling, soa, adaptive, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/18-14-service-based-software-systems.txt"]
confidence: medium
---

## Definition

An Adaptive Service-Based Software System (ASBS) is a service-based system extended with Monitoring and Adaptation subsystems that select among alternative services at run time so that overall QoS requirements continue to be met in the presence of uncontrollable but predictable changes (load, availability, errors).

## How It Works

The Monitoring subsystem instruments the running services with transducers measuring accuracy, timeliness, and throughput metrics. The Adaptation subsystem evaluates whether current performance meets the QoS target and, if not, selects an alternative provider, retries via the broker, or reconfigures the composition. The DEVS-Suite SOAD framework simulates both the services and the Monitoring/Adaptation subsystems.

## Key Parameters

- Monitoring metric set
- Adaptation policy (rule-based, learned)
- QoS targets per attribute
- Retry and fallback rules

## When To Use

- Cloud systems with variable provider availability
- Mission-critical SBS where QoS violation is unacceptable
- Studying composition strategies under simulated change profiles

## Risks & Pitfalls

- Adaptation thrashing if hysteresis is missing
- Monitoring overhead competes for resources
- Predictability assumption may fail under novel disruptions

## Related Concepts

- [[concepts/service-oriented-computing]]
- [[concepts/quality-of-service]]
- [[concepts/soad-framework]]

## Sources

- [[summaries/modeling-simulation-systems-18-14-service-based-software-systems]]
