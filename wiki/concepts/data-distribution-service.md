---
title: "Data Distribution Service (DDS)"
type: concept
tags: [simulation, modeling, middleware, distributed, omg, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/ModelingAndSimulationOfSystems/_txt/13-10-dynamic-structure-agent-modeling-and-publish-subscribe.txt"]
confidence: medium
---

## Definition

Data Distribution Service (DDS) is an Object Management Group (OMG) data-centric standard for real-time communication middleware based on the publish/subscribe paradigm. DDS automatically handles addressing, marshalling, delivery, flow control, and retries; supports QoS specification; and provides automatic node discovery for plug-and-play anonymous communication.

## How It Works

DDS organizes participants into Domains containing Topics. A Publisher writes to topics via Data Writers (marshalling); a Subscriber reads via Data Readers (unmarshalling). Participants discover each other automatically. The DEVS/DDS architecture maps the DEVS Simulation Protocol onto DDS topics: GetTN, MyTN, GetOutput, MyOutput, per-model StoreInputForModel, DoDelta.

## Key Parameters

- Domain (federation container)
- Topic (named channel with data type)
- Publisher / Data Writer
- Subscriber / Data Reader
- Participant / Application
- QoS profile

## When To Use

- Real-time DEVS distributed simulation
- Cyber-physical systems with strict timing
- Net-centric SoS integration
- Replacing per-host RPC with topic-based dissemination

## Risks & Pitfalls

- QoS misconfiguration silently changes behavior
- XML marshalling overhead for large DEVS messages
- Domain partitioning required to avoid cross-talk

## Related Concepts

- [[concepts/publish-subscribe]]
- [[concepts/devs-dds-architecture]]
- [[concepts/data-distribution-middleware]]

## Sources

- [[summaries/modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe]]
- [[summaries/modeling-simulation-systems-14-11-interest-based-information-exchange-mappings-and-models]]
