---
title: ACID Transactions
type: claim
id: claim-acid-transactions
tags:
- databases
- consistency
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
---

## Definition

ACID describes the four guarantees provided by classical database transactions: **Atomicity** (all changes commit or none do), **Consistency** (the transaction leaves the database in a valid state per defined invariants), **Isolation** (concurrent transactions appear to execute serially), and **Durability** (committed changes survive crashes).

## How It Works

Engines use locking or MVCC for isolation, a write-ahead log and journal for durability, and either undo logs or shadow-paging for atomic commit/rollback. In distributed databases ACID extends across partitions using two-phase commit + consensus.

## Key Parameters

- Isolation level (read-committed, repeatable read, snapshot, serializable).
- Transaction timeout.
- Lock-acquisition order.

## When To Use

Any workload where partial updates would corrupt invariants — financial systems, inventory, billing, multi-row updates.

## Risks & Pitfalls

- Serializable isolation is expensive at scale.
- Distributed ACID multiplies coordination cost.
- Many NoSQL systems offer only weaker guarantees per request.

## Related Concepts

- [[concepts/serializability]]
- [[concepts/snapshot-isolation]]
- [[concepts/two-phase-commit]]
- [[concepts/strong-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
