---
title: Exactly-Once Semantics
type: claim
id: concepts/exactly-once-semantics
tags:
- streaming
- distributed-systems
- well-established
- fault-tolerance
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Exactly-once (or "effectively-once") semantics is the property that each event in a stream is processed such that the observable output is the same as if it had been processed exactly one time, even if internal retries cause the underlying handler to execute multiple times. It is achieved by combining at-least-once delivery with idempotent operations and/or atomic commits of state and output.

## How It Works

Two principal approaches:

- **Atomic commit**: an output, a state update, and an offset checkpoint are committed together as a transaction. Failure rolls back partial effects. Used in Google Cloud Dataflow, VoltDB, Kafka's transactional producers.
- **Idempotence with end-to-end IDs**: every operation carries a unique identifier; downstream consumers deduplicate based on the ID, and side effects (DB writes, message emissions) are written conditionally on the ID being new.

Microbatching and Flink-style checkpointing implement exactly-once within the framework's boundary; once data leaves the framework (DB write, email send), end-to-end IDs are needed to extend the guarantee.

## Key Parameters

- ID generation strategy (UUID, hash, sequential).
- Deduplication window (how long to remember seen IDs).
- Transaction scope (single-system vs distributed).

## When To Use

Whenever duplicate processing would cause harm: payments, billing, counters, inventory adjustments, sensitive notifications.

## Risks & Pitfalls

- "Exactly-once" is a misleading marketing term; the underlying mechanism is at-least-once + dedup. Failed nodes can still execute handlers multiple times before dedup kicks in.
- Idempotence depends on the operation: setting a value is idempotent; incrementing a counter is not.
- Side effects outside the system (sending an email, calling an external API) require careful handling.
- Network partitions and clock skew complicate dedup window sizing.

## Related Concepts

- [[concepts/idempotency]]
- [[concepts/microbatching]]
- [[concepts/stream-processing]]
- [[concepts/two-phase-commit]]
- [[concepts/end-to-end-argument]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
