---
title: 'Modeling and Simulation of Systems — Chapter 10: Dynamic Structure: Agent
  Modeling and Publish/Subscribe'
type: source
id: source-modeling-simulation-systems-13-10-dynamic-structure-agent-modeling-and-publish-subscribe
kind: derived-summary
tags:
- simulation
- modeling
- devs
- dynamic-structure
- agent-based
- publish-subscribe
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/ModelingAndSimulationOfSystems/_txt/13-10-dynamic-structure-agent-modeling-and-publish-subscribe.txt
---

## Key Points

- Dynamic-structure DEVS allows models to add/remove components and couplings at run time via `addChildModel(model)`, `removeChildModel(model)`, and `addCoupling(source, srcport, destination, destport)`.
- Agent/Actor pattern: an Agent processor receives an Actor (smart object) as input, adds it to the parent coupled model, dynamically wires couplings for the conversation, interacts via input/output ports, then removes the Actor and re-emits it on its output port.
- Publish/Subscribe (P/S) is a data-centric communication paradigm: clients register as publishers or subscribers for named topics with a PublishSubscribeRouter; subscribers receive any updates a publisher places on a matching topic.
- In DEVS, P/S topics are realized as dynamically added output ports on the Router. On a SubscribeRequest, the Router calls `addCoupling` to connect the topic's output port to the subscriber's input port; the topic becomes a port-level subscription channel.
- The Router maintains PublisherTopic, TopicSubscriber, and TopicValue relations and supports PublishRequest, SubscribeRequest, TopicUpdate, PublishRemoveRequest, SubscribeRemoveRequest.
- Data-centric vs. connection-centric: in P/S clients only need to know topic names; in connection-centric, publishers need addresses of consumers. P/S adds flexibility at the cost of central-router scaling.
- DDS (Data Distribution Service, OMG standard) is the real-time middleware embodying P/S: Domain, Topic, Publisher/Subscriber, DataWriter/DataReader, Participant, Application classes; supports QoS, automatic discovery, marshalling.
- DEVS/DDS architecture maps the DEVS Simulation Protocol onto DDS topics: GetTN, MyTN, GetOutput, MyOutput, StoreInputForModel (per-model), DoDelta.
- DEVS messages are exchanged as XML (port-value pairs) for platform-neutral interoperability between Java, C++, and other DEVS engines on DDS middleware.
- Topic-design trade-off: per-model topics (StoreInputForModel) violate the data-centric spirit and reduce flexibility/efficiency. Multi-aspect `all` and `each` coupling forms map to general vs. per-recipient topics; clever message design (e.g., including the player name in Comply) lets `each` couplings collapse to a single topic.
- Actor-tracking application: a coordination center subscribes to actor-name topics; care-center agents publish entry/exit events with timestamps, so the coordination center receives location updates without polling.

## Relevant Concepts

- [[concepts/dynamic-structure-devs]] — runtime add/remove of components and couplings.
- [[concepts/devs-agent-modeling]] — Agent/Actor interaction pattern.
- [[concepts/publish-subscribe]] — topic-based data-centric messaging.
- [[concepts/data-distribution-service]] — OMG DDS middleware.
- [[concepts/devs-dds-architecture]] — mapping DEVS protocol onto DDS topics.
- [[concepts/devs-simulation-protocol]] — protocol being implemented over DDS.
- [[concepts/data-distribution-middleware]] — general middleware concept.
- [[concepts/ses-uniform-coupling]] — all/each forms inform topic specificity.

## Source Metadata

- Source type: book chapter
- Book title: Guide to Modeling and Simulation of Systems of Systems
- Chapter: 10 — Dynamic Structure: Agent Modeling and Publish/Subscribe
- File path: `raw/ModelingAndSimulationOfSystems/_txt/13-10-dynamic-structure-agent-modeling-and-publish-subscribe.txt`
- Authors: Bernard P. Zeigler, Hessam S. Sarjoughian (Springer, 2nd ed. 2017)
