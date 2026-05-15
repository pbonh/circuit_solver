---
title: "Modeling and Simulation of Systems — Chapter 15: Cloud System Simulation Modeling"
type: summary
tags: [simulation, modeling, devs, soa, cloud, co-design, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/19-15-cloud-system-simulation-modeling.txt"]
confidence: high
---

## Key Points

- A "cloud system" is defined as a synthesis of software services and networked hardware, where service performance depends on hardware resources (CPU, memory, bandwidth, network load). This contrasts with chapter 14's software-only SOC view.
- Software/hardware co-design provides three degrees of freedom: separate software specification, separate hardware specification, integrated system-of-systems composition. Inspired by embedded-systems co-design (Wolf 1994; Edwards 1997).
- SOC-DEVS extends SOAD by adding hardware modeling: software services (atomic models) interact with networked hardware components via "jobs" (CPU cycle and memory load) and "messages" (service-to-service communication).
- The software service maintains a list of supported operations; each invocation creates a Service Context with executionTime, operationID, contextID, and queueIOCardEvent calls that submit jobs to hardware.
- Hardware modeling captures CPU cycle constraints, memory load, communication bandwidth, processing delay at routers/switches — these collectively determine time-dependent service execution.
- The voice communication system is the chapter's exemplar: services execute on assemblies ranging from mobile devices to grid computers, illustrating cloud-system breadth.
- The mapping (assignment) specification declares which software services execute on which hardware components, supporting both single-resource and networked-multi-resource cases.
- Two key mapping cases:
  1. Multiple services on a single hardware resource (compete for CPU cycles and memory).
  2. Services distributed across networked resources (additionally constrained by communication bandwidth).
- SOC-DEVS simulation models combine flexibly with actual services (e.g., security services) and can be integrated with real cloud systems for scalability and operational-efficiency evaluation under QoS attributes.
- Chapter exercises use SES and pruning to describe architectures, foreshadowing the CoSMoS environment (next chapter) that adds database support for SES-style model families.

## Relevant Concepts

- [[concepts/cloud-system]] — software+hardware synthesis.
- [[concepts/soc-devs]] — DEVS framework for cloud-system co-design.
- [[concepts/software-hardware-co-design]] — three degrees of freedom for separate specifications.
- [[concepts/service-context]] — per-invocation execution-state object.
- [[concepts/quality-of-service]] — timing and accuracy attributes evaluated under hardware constraints.
- [[concepts/service-oriented-computing]] — software-only predecessor.
- [[concepts/soad-framework]] — predecessor framework extended here.
- [[concepts/discrete-event-system-specification]] — formal base.
- [[entities/devs-suite]] — simulator hosting SOC-DEVS package.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 15 — Cloud System Simulation Modeling
- File path: `raw/ModelingAndSimulationOfSystems/_txt/19-15-cloud-system-simulation-modeling.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
