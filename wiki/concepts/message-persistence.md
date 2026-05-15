---
title: "Message Persistence"
type: concept
tags: [messaging, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: medium
---

## Definition

Message persistence is the ability of a broker to durably store messages on disk so they survive broker restarts, machine failures, or network outages. The alternative is memory-only queues, which lose pending messages on broker crash.

## How It Works

The broker writes message payloads (and acknowledgment state) to disk before confirming receipt to producers. On broker restart, the queue contents are reconstructed from the persistent store. Producers can opt in via flags like RabbitMQ's `durable` queue and `persistent` message-delivery-mode.

## Key Parameters

- Per-queue and per-message durability flags.
- fsync strategy.

## When To Use

Whenever message loss would corrupt application semantics — order placement, payment, audit trails.

## Risks & Pitfalls

- Persistent queues are slower than memory-only.
- Disk-IOPS bottleneck can become the throughput ceiling.

## Related Concepts

- [[concepts/message-broker]]
- [[concepts/asynchronous-messaging]]
- [[concepts/exactly-once-processing]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
