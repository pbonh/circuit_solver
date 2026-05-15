---
title: "Lost Update"
type: concept
tags: [well-established, transactions, concurrency, isolation]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

A lost update is a race condition in which two transactions concurrently perform a read-modify-write cycle on the same object, and the later write clobbers the earlier one without incorporating its changes. Classic example: two threads each incrementing a counter from 42 to 43, when the correct result is 44.

## How It Works

Scenarios:
- Counter or balance updates.
- JSON document local edits.
- Wiki page concurrent edits replacing whole content.

Solutions:
- **Atomic update operations**: `UPDATE counters SET value = value + 1` — built-in atomic increments / list-append in databases.
- **Explicit locks**: `SELECT ... FOR UPDATE` to lock the row before modifying.
- **Automatic detection**: snapshot-isolation implementations (PostgreSQL, Oracle, SQL Server) can detect lost updates and abort. MySQL InnoDB does not.
- **Compare-and-set**: succeed only if value matches the read.
- For replicated stores: CRDTs / commutative operations; never LWW alone.

## Key Parameters

- Whether the database's snapshot isolation includes lost-update detection.
- Application retry policy.
- Choice of atomic operation vs explicit lock.

## When To Use

Whenever data is updated via read-modify-write — i.e., almost always for mutable application state.

## Risks & Pitfalls

- ORMs often hide lost updates by issuing naive UPDATEs.
- Compare-and-set may be unsafe if the WHERE clause reads from a stale snapshot.
- Replicated stores need version vectors / CRDTs; LWW silently loses data.

## Related Concepts

- [[concepts/transaction]]
- [[concepts/snapshot-isolation]]
- [[concepts/write-skew]]
- [[concepts/crdt]]
- [[concepts/version-vector]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
