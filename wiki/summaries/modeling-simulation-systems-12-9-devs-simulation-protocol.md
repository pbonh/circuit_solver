---
title: "Modeling and Simulation of Systems — Chapter 9: DEVS Simulation Protocol"
type: summary
tags: [simulation, modeling, devs, distributed, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt"]
confidence: high
---

## Key Points

- DEVS enforces strict separation between models and simulators — a model written in any DEVS environment (MS4 Me, ADEVS, etc.) should produce identical results across any conforming Abstract DEVS Simulator implementation.
- The DEVS Simulation Protocol layers two interfaces: AbstractSimulator (which the model presents to its simulator with TimeAdvanceFn, OutputFn, ExternalTransitionFn, InternalTransitionFn, ConfluentTransitionFn) and DevsProtocol (which the simulator presents to a coordinator with OperationGetTN, OperationGetOutput, OperationStoreInput, OperationDoDelta).
- The four-step protocol cycle: (1) GetTN — coordinator queries each simulator's time of next event; (2) GetOutput(t) — coordinator requests output of imminent simulators; (3) StoreInput(m) — coordinator distributes composed input messages via couplings; (4) DoDelta — coordinator triggers internal/external/confluent transitions.
- Three implementation variants:
  - **Standard DEVS Protocol** — coordinator centrally routes all messages between simulators using all-to-one and one-to-all couplings.
  - **Peer Message Exchanging** — coupling information is partitioned across simulators which exchange messages directly (`all SimulatorPeer sends outMyOutput to all SimulatorPeer as inStoreInput!`), reducing coordinator load.
  - **Real-Time Peer Exchange** — simulators schedule their own time of next event in real time and exchange messages peer-to-peer; coordinator only starts and stops the run.
- The protocol can interoperate with non-DEVS simulators (e.g., Event-Scheduling simulators) by wrapping them in a DEVS Simulator that translates GetTN→GetTimeOfImminentEvent, GetOutput→GetNRemoveImminentEvent, StoreInput→AddEvent.
- Two facets of simulation interoperability: data exchange compatibility (syntactic, semantic, pragmatic agreements on messages) and time-management compatibility (single global time enforced by the DEVS protocol).
- The protocol is foundational for DEVS implementations on data-distribution and service-oriented-computing middleware.
- Annexes provide Simulator.dnl and Coordinator.dnl extracts showing FDDEVS state machines elaborated with Java tagged blocks: Simulator maintains tL, tN, t, myInput, myModel; Coordinator maintains minimum tN computation across simulators, applies coupling to generate per-simulator input bags.

## Relevant Concepts

- [[concepts/abstract-devs-simulator]] — model-side protocol interface.
- [[concepts/devs-simulation-protocol]] — coordinator/simulator interface.
- [[concepts/devs-coordinator]] — coupled-model orchestrator.
- [[concepts/peer-message-exchange]] — distributed implementation variant.
- [[concepts/real-time-devs-simulation]] — real-time variant where simulators self-schedule.
- [[concepts/event-scheduling-simulation]] — alternative paradigm wrapped via the DEVS protocol.
- [[concepts/simulation-interoperability]] — data and time-management compatibility requirements.
- [[concepts/coupled-devs-model]] — orchestrated artifact.
- [[concepts/atomic-devs-model]] — leaf simulators.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 9 — DEVS Simulation Protocol
- File path: `raw/ModelingAndSimulationOfSystems/_txt/12-9-devs-simulation-protocol.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
