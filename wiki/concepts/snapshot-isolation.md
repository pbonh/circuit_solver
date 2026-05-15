---
title: "Snapshot Isolation"
type: concept
tags: [well-established, transactions, isolation, mvcc]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Snapshot isolation is a transaction-isolation level in which each transaction reads from a consistent snapshot of the database taken at the start of the transaction. Writes use locks to prevent dirty writes, but readers never block writers and writers never block readers — implemented via **multi-version concurrency control (MVCC)**. PostgreSQL, MySQL InnoDB, Oracle, and SQL Server all support it (sometimes calling it "repeatable read" or "serializable" depending on vendor).

## How It Works

- Each row keeps multiple versions, tagged with the transaction ID that created and (if applicable) deleted it.
- On transaction start, the system records the set of in-progress transactions; visibility rules ignore writes from later or in-progress transactions.
- Updates become a delete plus a create (new version). Garbage collection removes obsolete versions once no transaction needs them.
- Long-running read-only transactions can run alongside writes without lock contention — great for backups, analytics, integrity checks.
- Prevents dirty reads and read skew but does not prevent write skew or phantoms.

## Key Parameters

- Garbage-collection cadence (controls space cost of old versions).
- Whether lost-update detection is enabled (PostgreSQL/Oracle/SQL Server yes; MySQL no).
- Naming: "repeatable read" (PostgreSQL/MySQL), "serializable" (Oracle 11g), "snapshot" (SQL Server).

## When To Use

For mixed read-write workloads where consistent snapshots matter (analytic queries, backups) and the application can tolerate write skew or handle it explicitly with `SELECT FOR UPDATE`.

## Risks & Pitfalls

- Write skew goes undetected (two transactions read overlapping data, write disjoint rows that violate each other's premises).
- Phantoms in write-write conflicts are not prevented.
- MVCC requires version cleanup; long-running transactions block GC and bloat storage.
- Snapshot isolation is not linearizable.

## Related Concepts

- [[concepts/transaction]]
- [[concepts/serializability]]
- [[concepts/serializable-snapshot-isolation]]
- [[concepts/write-skew]]
- [[concepts/lost-update]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
