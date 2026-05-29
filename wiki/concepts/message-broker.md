---
title: Message Broker
type: claim
id: claim-message-broker
tags:
- distributed-systems
- streaming
- well-established
- messaging
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.65
---

## Definition

A message broker (a.k.a. message queue or message-oriented middleware) is an intermediary process that accepts messages from producers and delivers them to one or more consumers. It implements asynchronous, decoupled communication sitting between direct RPC and a database: low-latency message delivery like RPC, but durable temporary storage like a database. Examples include Apache Kafka, RabbitMQ, ActiveMQ, HornetQ, NATS, and historical commercial systems like TIBCO and IBM WebSphere MQ.

## How It Works

- Producers publish messages to a named queue or topic; the broker stores them temporarily and delivers them to one or more consumers/subscribers.
- A message is an opaque byte sequence with metadata; the broker does not interpret payloads.
- Delivery semantics vary (at-most-once, at-least-once, exactly-once) and depend heavily on configuration.
- Topics support fan-out to many consumers; queues support work-distribution to one consumer.
- A consumer can republish to another topic, building pipelines (see DDIA Chapter 11).

Advantages over direct RPC:
- Buffers spikes when recipients are slow or unavailable.
- Automatic redelivery on consumer crash.
- Decoupled discovery — the sender doesn't need the recipient's IP/port.
- Multicast: one message to many recipients.
- One-way communication is the default; request/reply requires an explicit reply queue.

## Key Parameters

- Durability and persistence model (memory, disk, replicated).
- Delivery semantics and acknowledgment mode.
- Partitioning/topic strategy and consumer group concurrency.
- Retention policy (length-based, time-based, compacted).

## When To Use

For event-driven architectures, write-ahead pipelines, change-data-capture streams, work queues, decoupling microservices, fan-out to multiple consumers. Broker-based dataflow is the foundation of stream processing covered in DDIA Part III.

## Risks & Pitfalls

- "Exactly-once" delivery semantics are subtle and broker-specific; idempotent consumers are safer.
- Backpressure and consumer lag must be monitored or the broker's retention fills up.
- Schema evolution is the producer's responsibility — preserve unknown fields when republishing.
- Distributed actor frameworks (Akka, Orleans, Erlang) integrate broker + actor model but inherit the same compatibility concerns during rolling upgrades.

## Related Concepts

- [[concepts/remote-procedure-call]]
- [[concepts/data-encoding]]
- [[concepts/schema-evolution]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
