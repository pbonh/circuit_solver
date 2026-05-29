---
title: Strong Consistency
type: claim
id: concepts/strong-consistency
tags:
- distributed-systems
- consistency
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Strong consistency in a distributed database means that once an update has been confirmed, every subsequent read by any client returns the new value. In the strongest variant — strict serializability — this is the conjunction of transactional serializability and per-object linearizability.

## How It Works

Achieved via consensus algorithms (Paxos, Raft) for replica agreement, two-phase commit (or its consensus-backed variant) for multi-partition transactions, and bounded-uncertainty time sources (Spanner's TrueTime) for global ordering.

## Key Parameters

- Replica consistency protocol.
- Transaction isolation level.
- Time-source accuracy.

## When To Use

When inconsistent reads would lead to data loss, business-logic violations, or compliance failures — financial systems, inventory, identity management.

## Risks & Pitfalls

- Latency cost is significant compared to eventual consistency.
- CAP trade-off: must sacrifice availability under partition.

## Related Concepts

- [[concepts/serializability]]
- [[concepts/linearizability]]
- [[concepts/two-phase-commit]]
- [[concepts/eventual-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
