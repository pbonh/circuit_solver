---
title: Exactly-Once Processing
type: claim
id: concepts/exactly-once-processing
tags:
- messaging
- fault-tolerance
- advanced
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/05-part-ii-scalable-systems.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Exactly-once processing guarantees that, despite retries, duplicates, and failures, every message has exactly one effective application. It is the strongest delivery guarantee in messaging systems.

## How It Works

Implementations combine producer idempotence (deduplication at the broker via idempotency keys) with consumer-side deduplication (using a key store or transactional state updates). Kafka achieves exactly-once with idempotent producers + transactional consumers; ActiveMQ Artemis dedupes at the broker.

## Key Parameters

- Idempotency-key TTL.
- Transactional scope (broker offset + state update).

## When To Use

Financial transactions, billing, any workflow where duplicate effects are unacceptable.

## Risks & Pitfalls

- The "exactly-once" guarantee is technology-specific and easily misconfigured.
- Performance cost is significant compared to at-most-once or at-least-once.

## Related Concepts

- [[concepts/idempotency]]
- [[concepts/asynchronous-messaging]]
- [[concepts/poison-message]]

## Sources

- [[summaries/foundations-scalable-systems-05-part-ii-scalable-systems]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
