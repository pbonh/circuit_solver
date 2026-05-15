---
title: "Serializable Snapshot Isolation (SSI)"
type: concept
tags: [emerging, well-established, transactions, isolation, concurrency]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: high
---

## Definition

Serializable Snapshot Isolation, introduced by Cahill, Röhm, and Fekete in 2008, is an **optimistic** concurrency-control algorithm that provides full serializability on top of snapshot isolation. Instead of blocking transactions with locks, SSI lets them run optimistically against MVCC snapshots and aborts at commit time those whose execution would have violated serializability. Used in PostgreSQL since 9.1 and in FoundationDB.

## How It Works

- All reads come from a consistent snapshot (no read locks).
- The system tracks two kinds of read-write conflicts:
  - **Stale MVCC read**: a transaction read a value that was concurrently overwritten by another now-committed transaction.
  - **Write affecting prior read**: a transaction wrote a row that another transaction had read at snapshot time.
- At commit, if a transaction's reads would have been invalidated by a concurrent write, the offender is aborted and the application retries.
- Index entries serve as tripwires marking what data each transaction has read.

## Key Parameters

- Granularity of read tracking (per-row, per-index entry, per-table).
- Abort/retry rate (workload-dependent).
- Transaction duration limit.

## When To Use

For mixed read/write workloads needing serializability without the latency tail of 2PL. Especially good when long-running read-only queries coexist with short read-write transactions.

## Risks & Pitfalls

- High contention causes many aborts; not ideal for write-heavy workloads with hot keys.
- Bookkeeping memory grows with concurrent transaction count.
- Long-running read-write transactions are more likely to be aborted.
- Still a relatively recent technique; performance characteristics are workload-specific.

## Related Concepts

- [[concepts/serializability]]
- [[concepts/snapshot-isolation]]
- [[concepts/two-phase-locking]]
- [[concepts/transaction]]
- [[concepts/write-skew]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
