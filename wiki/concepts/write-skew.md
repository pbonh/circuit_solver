---
title: "Write Skew"
type: concept
tags: [well-established, transactions, concurrency, isolation]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Write skew is a race condition in which two transactions read overlapping data, each makes a decision based on that read, then writes disjoint rows that together invalidate each other's premise. Snapshot isolation does not prevent write skew; only true serializable isolation does.

## How It Works

Canonical examples:
- **On-call doctors**: each transaction sees two doctors on call, decides it's safe for one to go off, both write their own off-call status. Result: zero doctors on call.
- **Meeting-room double-booking**: both transactions see no conflicting booking and each inserts its own.
- **Multiplayer chess**: two players write disjoint board squares that violate a game rule.
- **Username uniqueness**: two registrations check absence and each insert.
- **Double-spending**: two debits each leave the apparent balance positive but together overdraw.

The general pattern: a SELECT-based precondition, then a write that changes the precondition. Often the writes touch rows the precondition didn't return — so SELECT FOR UPDATE has nothing to lock — leading to **phantoms**.

Mitigations:
- True serializable isolation (SSI, 2PL with predicate or index-range locks, actual serial execution).
- Materializing conflicts: pre-create lockable rows representing the resource (e.g., a row per (room, 15-minute window)).
- Database constraints (foreign keys, uniqueness) when the rule fits the model.

## Key Parameters

- Available isolation level.
- Lock granularity for materialized conflict rows.

## When To Use

Awareness rather than usage: when designing application invariants enforced across multiple rows, recognize the write-skew pattern and pick an isolation level that defends against it.

## Risks & Pitfalls

- Hardest of the standard race conditions to detect by testing.
- Often surfaces in financial and scheduling systems with real money/safety consequences.
- Materializing conflicts is ugly and leaks concurrency control into the data model.

## Related Concepts

- [[concepts/serializability]]
- [[concepts/serializable-snapshot-isolation]]
- [[concepts/two-phase-locking]]
- [[concepts/snapshot-isolation]]
- [[concepts/lost-update]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
