---
title: Dead-Letter Queue
type: claim
id: concepts/dead-letter-queue
tags:
- messaging
- fault-tolerance
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

A dead-letter queue (DLQ) is a destination for messages that exceed a broker's redelivery limit or fail validation. Sidelining poison messages prevents them from blocking the main queue and provides a structured handoff for offline diagnosis.

## How It Works

Each queue references a DLQ destination and a maximum-receives setting. Once a message has been delivered (and unacknowledged) that many times, the broker moves it to the DLQ instead of redelivering. Operators monitor DLQ depth and investigate root causes (data corruption, consumer bug).

## Key Parameters

- Max delivery attempts.
- DLQ retention period.
- Monitoring alert thresholds.

## When To Use

Every production queue.

## Risks & Pitfalls

- Unmonitored DLQs hide real failures.
- Without root-cause fix, DLQ keeps growing.

## Related Concepts

- [[concepts/poison-message]]
- [[concepts/message-queue]]
- [[concepts/asynchronous-messaging]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
