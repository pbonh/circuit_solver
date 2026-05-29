---
title: Poison Message
type: claim
id: claim-poison-message
tags:
- messaging
- fault-tolerance
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.65
---

## Definition

A poison message is a message that consumers cannot process successfully — typically because of a malformed payload, unexpected state, or consumer bug. Without intervention, brokers redeliver it indefinitely, stalling progress and potentially crashing consumers.

## How It Works

Brokers cap the number of delivery attempts per message (e.g., SQS `maxReceiveCount`, RabbitMQ delivery count). Once the cap is reached, the message is moved to a dead-letter queue for offline diagnosis.

## Key Parameters

- Max delivery attempts (commonly 3-5).
- Dead-letter queue destination.
- Alert thresholds on DLQ depth.

## When To Use

Always — every production queue should have a DLQ and a monitoring alert.

## Risks & Pitfalls

- Unlimited redelivery can take down consumers.
- DLQs that nobody monitors accumulate silently.

## Related Concepts

- [[concepts/dead-letter-queue]]
- [[concepts/idempotency]]
- [[concepts/competing-consumers]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
