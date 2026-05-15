---
title: "Causal Consistency"
type: concept
tags: [distributed-systems, well-established, consistency]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Causal consistency requires that operations that are causally related (one happens before another in the Lamport "happens-before" sense) are observed in the same order on every replica, while concurrent operations may appear in any order. It defines a partial order on operations, in contrast to linearizability's total order. Causal consistency is the strongest consistency model that remains available under network partitions.

## How It Works

- Each operation carries a causal context (version vector, dotted version vector, or vector clock) that records what the operation depends on.
- When a replica receives an operation, it must wait until all causally preceding operations have been applied before applying the new one.
- Snapshot isolation provides causal consistency: a transaction's reads are from a single point-in-time snapshot that respects "happens-before."
- Useful in geographically distributed settings (COPS, Eiger, others) where linearizability is too costly.

## Key Parameters

- Vector clock size (one entry per replica).
- Garbage-collection policy for old vector entries.
- Trade-off between metadata overhead and granularity.

## When To Use

For multi-region, low-latency applications that need ordering guarantees stronger than eventual consistency but cannot pay linearizability's price. Examples: social-media feeds where comment-reply ordering matters, collaborative editing under conflict-free data types.

## Risks & Pitfalls

- Cannot enforce real-time unique constraints (uniqueness needs linearizability or total-order broadcast).
- Causal metadata grows with replica count; pruning is necessary at scale.
- Few production databases offer it as a built-in mode; usually requires application-level tracking.

## Related Concepts

- [[concepts/linearizability]]
- [[concepts/eventual-consistency]]
- [[concepts/version-vector]]
- [[concepts/lamport-timestamp]]
- [[concepts/snapshot-isolation]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/ddia-05-part-iii-derived-data]]
