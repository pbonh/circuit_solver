---
title: "Modeling and Simulation of Systems — Chapter 4: DEVS Natural Language Models and Elaborations"
type: summary
tags: [simulation, modeling, devs, fddevs, java, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/06-4-devs-natural-language-models-and-elaborations.txt"]
confidence: high
---

## Key Points

- FDDEVS files (`*.dnl`) are MS4 Me's natural-language specifications of atomic DEVS models; they automatically generate `AtomicModelImpl`-derived Java classes alongside.
- A worked example builds a GeneratorOfJobs that emits a Job every 10 time units in state `generate` and passivates on receiving Stop; the Java class contains `holdIn`, `phaseIs`, `passivate`, `getTimeAdvance`, and `getOutput` methods.
- Mapping concepts: a Port is a typed instance (`Port<WorkToDo>`); a Message pairs a port with a Serializable value; a MessageBag collects messages with possible duplicates; FDDEVS by default reads only the first content element of an arriving bag.
- Elaboration mechanism: tagged blocks (`<% ... %>!`) in dnl files inject Java code at points such as `Initialize variables`, `internal event for STATE`, `external event for STATE with PORT`, `output event for STATE`, and `add Library`.
- Auxiliary class definitions can be declared inside dnl: `A WorkToDo has id, processingTime, startTime!` and range statements give types and defaults.
- Instance-variable declarations: `use NAME with type TYPE and default "EXPR"!`.
- Port-type declarations: `accepts input on PORT with type CLASS!`, `generates output on PORT with type CLASS!`.
- Non-deterministic transitions: tagged blocks for internal events can call `passivateIn("phase")` to override the FDDEVS default next state, supporting state-dependent branching like job-count limits.
- Multi-input handling: elaborate the external event block to iterate `messageList` and handle simultaneous bag messages — important for ODE-style simulations where all components fire at the same clock tick.
- Multi-output handling: the output bag can carry multiple `output.add(port, value)` calls across the same or multiple ports.
- Transducer pattern: a dedicated DEVS model observes arrivals and completions to measure turnaround time and throughput (jobsArrived, jobsSolved, totalTa, clock).
- Sequence Designer accelerates model authoring: it generates atomic and coupled models, can re-target an atomic component as a coupled one, and the resulting SES can be enhanced with specializations and additional aspects.
- The UAS testing case study uses the Sequence Designer twice — first for the top model, second to elaborate the SensorPackage into a coupled model with sensors and a data handler.

## Relevant Concepts

- [[concepts/finite-deterministic-devs]] — the natural-language modeling layer.
- [[concepts/atomic-devs-model]] — Java target generated from dnl files.
- [[concepts/coupled-devs-model]] — generated from SES coupling specifications.
- [[concepts/dnl-elaboration]] — tagged-block mechanism for injecting Java into FDDEVS.
- [[concepts/devs-port-and-message]] — typed ports, messages, and message bags.
- [[concepts/devs-transducer]] — instrumented observer measuring throughput and turnaround.
- [[concepts/sequence-designer]] — diagram-driven model generation in MS4 Me.
- [[concepts/object-oriented-simulation]] — Java/OO target for DEVS implementations.
- [[entities/ms4-me]] — environment hosting the generation and elaboration toolchain.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 4 — DEVS Natural Language Models and Elaborations
- File path: `raw/ModelingAndSimulationOfSystems/_txt/06-4-devs-natural-language-models-and-elaborations.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
