---
title: "Two-Phase Locking (2PL)"
type: concept
tags: [well-established, transactions, isolation, concurrency]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Two-phase locking is a pessimistic concurrency-control algorithm that provides serializable isolation. Transactions acquire shared (read) and exclusive (write) locks on data objects in a growing phase, then release all locks at commit/abort (shrinking phase). For decades 2PL was the only widely used serializability algorithm; it underlies MySQL InnoDB's serializable mode, SQL Server's serializable, and DB2's repeatable read.

## How It Works

- Read of object X acquires a shared lock; multiple readers can hold shared locks simultaneously.
- Write of X requires an exclusive lock that excludes both readers and writers.
- Shared lock can be upgraded to exclusive when a read is followed by a write.
- All locks are held until the transaction commits or aborts (strict 2PL, sometimes SS2PL).
- **Predicate locks** prevent phantoms by locking all (existing and future) rows matching a search condition; **index-range locks** are a practical approximation.
- Deadlocks are detected and resolved by aborting one transaction.

## Key Parameters

- Lock granularity (row, page, table, predicate).
- Deadlock detection interval.
- Lock-wait timeout.

## When To Use

When a serializability guarantee is required and SSI is unavailable or the workload would cause too many SSI aborts.

## Risks & Pitfalls

- Throughput and tail latency suffer significantly compared to weaker isolation.
- Deadlocks are common; aborted transactions must be retried.
- Predicate locks are expensive; index-range locks are simpler but less precise.
- Without a usable index, the system may fall back to table-level locks.

## Related Concepts

- [[concepts/serializability]]
- [[concepts/snapshot-isolation]]
- [[concepts/serializable-snapshot-isolation]]
- [[concepts/write-skew]]
- [[concepts/transaction]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
