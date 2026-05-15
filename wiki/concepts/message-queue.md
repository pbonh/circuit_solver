---
title: "Message Queue"
type: concept
tags: [messaging, distributed-systems, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt"]
confidence: high
---

## Definition

A message queue is a FIFO buffer hosted by a broker. Producers append messages; consumers retrieve them, typically destructively (the message is removed once acknowledged). Each message is delivered to exactly one consumer of a queue (point-to-point semantics).

## How It Works

Messages enter the tail and exit the head. Memory-only queues are fast but lose messages on broker crash; durable queues persist messages to disk. Consumers acknowledge messages explicitly or automatically; unacknowledged messages can be redelivered.

## Key Parameters

- Persistence (durable or transient).
- Maximum queue length.
- Acknowledgment mode.

## When To Use

Task queues, point-to-point integration, work distribution among workers (competing consumers).

## Risks & Pitfalls

- Unbounded queues amplify outages.
- Poison messages block progress unless redirected to a dead-letter queue.

## Related Concepts

- [[concepts/message-broker]]
- [[concepts/competing-consumers]]
- [[concepts/dead-letter-queue]]
- [[concepts/publish-subscribe]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
