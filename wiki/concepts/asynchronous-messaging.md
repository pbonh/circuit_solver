---
title: Asynchronous Messaging
type: claim
id: concepts/asynchronous-messaging
tags:
- distributed-systems
- messaging
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Asynchronous messaging decouples senders from receivers via an intermediary broker that buffers messages. Producers "fire and forget" — they deliver to a queue and continue — while consumers retrieve and process at their own pace. This delivers responsiveness, spike-buffering, and loose coupling.

## How It Works

Messages are written to broker-managed queues or topics. Brokers (RabbitMQ, ActiveMQ, IBM MQ, Kafka) persist messages and deliver them to consumers via pull or push. Delivery guarantees range from at-most-once to exactly-once. Pub-sub topics fan out a single message to multiple subscribers.

## Key Parameters

- Persistence (durable vs. memory-only queues).
- Acknowledgment mode (auto/manual).
- Delivery guarantee.

## When To Use

Whenever producer and consumer can decouple in time, or to smooth request spikes ahead of slower downstream processing.

## Risks & Pitfalls

- Message loss in memory-only brokers.
- Duplicates from retries unless idempotency is built in.
- Poison messages can stall consumers.

## Related Concepts

- [[concepts/message-broker]]
- [[concepts/message-queue]]
- [[concepts/publish-subscribe]]
- [[concepts/competing-consumers]]

## Sources

- [[summaries/foundations-scalable-systems-04-part-i-the-basics]]
- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
