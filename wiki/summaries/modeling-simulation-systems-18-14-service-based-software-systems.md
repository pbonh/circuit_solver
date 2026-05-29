---
title: 'Modeling and Simulation of Systems — Chapter 14: Service-Based Software Systems'
type: source
id: summaries/modeling-simulation-systems-18-14-service-based-software-systems
kind: publication
tags:
- simulation
- modeling
- devs
- soa
- qos
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/18-14-service-based-software-systems.txt
---

## Key Points

- Service-Oriented Computing (SOC) treats software-as-a-service: self-contained modules providing functionality to subscribers. Service-Based Software Systems (SBS) are SoS within information technology.
- SOA virtual-build-and-test must satisfy two goals: complying with SOA standards (WSDL, UDDI, SOAP) and meeting multiple, competing Quality-of-Service (QoS) requirements.
- Basic QoS attributes: Accuracy (loss rate, error rate), Timeliness (response time, service delay, jitter), Throughput (data rate, bandwidth).
- Adaptive Service-Based Software System (ASBS): a Monitoring + Adaptation subsystem chooses services under uncontrollable but predictable changes to maintain QoS.
- SOAD (SOA-compliant DEVS) framework maps SOA elements onto DEVS: services (provider, client, broker) → atomic models; messaging framework → ports/couplings; service registry → executive model; composition → coupled model. Built on DEVS-Suite (DEVSJAVA extension).
- Three message types in SOAD: ServiceInfo (publication, like WSDL with name/endpoints/binding), ServiceLookup (subscriber's lookup request), ServiceCall (SOA invocation with data payload).
- ServiceProvider has two time logics: Processing Time (queue wait before service starts) and Service Duration (actual service time). Multi-client support via a RequestList iterated each loop.
- ServiceClient retries lookups against the broker until found, with configurable retry intervals and maximum attempts; once an endpoint is found, it sends the call and waits for response within a response-time bound.
- Composite Services contain at least two service providers (primitive or composite) hierarchically; BPEL specifies the invocation workflow.
- Travel Agent Service example: subscriber (Travel Agent) + two publishers (USZip, Ski Resort) + broker + hardware Router Link, with transducers measuring turnaround time, data received, throughput, transmission time, and utilization for a 71.5-second run.
- The Voice Communication Service is used to validate the SOAD design with a real composite service implementation.
- Differences vs. dynamic-structure DEVS: SOA broker is conceptually distinct from a DEVS executive; SOA enforces broker mediation for all publisher/subscriber discovery, while DEVS only requires it for structural changes.

## Relevant Concepts

- [[concepts/service-oriented-computing]] — software-as-a-service paradigm.
- [[concepts/quality-of-service]] — accuracy, timeliness, throughput attributes.
- [[concepts/soad-framework]] — DEVS mapping of SOA model elements.
- [[concepts/adaptive-service-based-software-system]] — Monitoring/Adaptation pattern.
- [[concepts/devs-agent-modeling]] — predecessor for dynamic-structure SOA modeling.
- [[concepts/discrete-event-system-specification]] — formal base.
- [[concepts/publish-subscribe]] — message-pattern foundation.
- [[concepts/coupled-devs-model]] — composite service representation.
- [[concepts/service-oriented-architecture]] — SOA standards (WSDL, UDDI, SOAP).
- [[entities/devs-suite]] — simulator used for SOAD examples.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 14 — Service-Based Software Systems
- File path: `raw/ModelingAndSimulationOfSystems/_txt/18-14-service-based-software-systems.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
