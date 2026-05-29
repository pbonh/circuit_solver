---
title: Write-Ahead Log (WAL)
type: claim
id: claim-write-ahead-log
tags:
- storage
- well-established
- durability
- recovery
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

A write-ahead log (WAL), also called a redo log, is an append-only file to which every modification is written before the underlying data structure (typically a B-tree's in-place pages, or an LSM-tree's memtable) is updated. After a crash, the log is replayed to restore the data structure to a consistent state.

## How It Works

- Every operation that mutates persistent state writes its intent to the WAL first, with checksums for corruption detection.
- The data structure (page, memtable) is then updated in memory; eventually durable writes are flushed.
- On restart, the engine reads the WAL and replays operations whose effects were not yet persisted, restoring the in-memory state and any partially-applied multi-page changes.
- For B-trees: the WAL prevents partial split situations (orphan pages) by recording the whole operation atomically.
- For LSM-trees: the WAL captures memtable contents lost on crash; after a successful memtable flush to an SSTable, the corresponding log segment can be discarded.

## Key Parameters

- Sync policy (fsync per record, per batch, periodic — durability vs throughput).
- Log segment size and rotation policy.
- Retention after a checkpoint.

## When To Use

In any storage engine that performs in-place mutations of complex on-disk structures, or that holds dirty state in memory before flushing.

## Risks & Pitfalls

- Sync policy directly trades durability for write throughput; misconfiguration can lose committed data.
- Long replay times after crash can cause unacceptable downtime; periodic checkpoints mitigate this.
- WAL is a sequential bottleneck; high-throughput systems often use group commit or multiple logs.

## Related Concepts

- [[concepts/b-tree]]
- [[concepts/lsm-tree]]
- [[concepts/fault-tolerance]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
