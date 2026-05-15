---
title: "RabbitMQ"
type: entity
tags: [messaging, broker, open-source]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: high
---

## Overview

RabbitMQ is a widely deployed open-source message broker first released around 2009. Written in Erlang, it primarily implements the Advanced Message Queuing Protocol (AMQP) standard and supports STOMP and MQTT via plugins. It is used across finance, telecommunications, and IoT systems.

## Characteristics

- Exchanges (direct, topic, fanout) route messages to queues via routing keys and bindings.
- Connections are heavyweight; multiple channels multiplex over a single connection but channels are not thread-safe.
- Each queue is owned by a single broker thread — multicore throughput requires multiple queues.
- High availability via mirrored queues (legacy) or quorum queues (RAFT-based, future direction).
- Default memory throttle kicks in at ~40% broker memory.

## Common Strategies

- Channel pools (e.g., Apache Commons Pool) for multi-threaded producers.
- Publisher confirms + persistent queues + manual consumer acks for end-to-end safety.
- Quorum queues for high-availability deployments.

## Related Entities

- [[entities/apache-kafka]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
