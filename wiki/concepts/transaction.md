---
title: Transaction
type: claim
id: concepts/transaction
tags:
- foundational
- well-established
- transactions
- concurrency
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

A transaction is a logical unit grouping reads and writes that the database treats as a single operation: either the entire group commits successfully or it aborts with no effect (rollback). Transactions, introduced in IBM's System R in 1975, exist to simplify the application programming model by hiding partial failure, concurrency, and crash recovery.

## How It Works

ACID properties:
- **Atomicity**: if anything fails mid-transaction, the entire transaction is discarded. The application can safely retry.
- **Consistency**: invariants the application defines are preserved. (This is the application's responsibility; databases enforce only constraints.)
- **Isolation**: concurrently running transactions don't interfere; ideally as if executed serially.
- **Durability**: once committed, writes survive crashes (typically via write-ahead log).

Implementation: multi-object transactions are demarcated with BEGIN/COMMIT statements over a client connection; row-level write locks prevent dirty writes; MVCC stores multiple versions for snapshot isolation; 2PL or SSI enforce serializability.

## Key Parameters

- Isolation level (read uncommitted, read committed, repeatable read / snapshot isolation, serializable).
- Lock granularity and timeout.
- Retry policy on abort.
- Distributed vs single-node scope.

## When To Use

Always for any operation that must preserve invariants across multiple writes or reads. Single-object writes are often atomic by default; multi-object operations need explicit transactions.

## Risks & Pitfalls

- "ACID" is a marketing term; implementations vary widely on isolation in particular.
- Most databases default to weak isolation (read committed or snapshot isolation), missing race conditions like write skew.
- Distributed transactions amplify failures and have heavy operational cost (see 2PC).
- ORMs often don't retry aborted transactions, defeating the safety guarantee.

## Related Concepts

- [[concepts/acid]]
- [[concepts/snapshot-isolation]]
- [[concepts/serializability]]
- [[concepts/two-phase-locking]]
- [[concepts/serializable-snapshot-isolation]]
- [[concepts/lost-update]]
- [[concepts/write-skew]]
- [[concepts/two-phase-commit]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
