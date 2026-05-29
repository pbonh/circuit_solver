---
title: LSM-Tree (Log-Structured Merge-Tree)
type: claim
id: claim-lsm-tree
tags:
- storage
- well-established
- indexing
- write-optimized
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

An LSM-tree is a write-optimized storage structure built from an in-memory sorted buffer (the memtable) plus a cascade of immutable sorted files on disk (SSTables) that are merged in the background. Introduced by O'Neil et al. in 1996 and popularized by Google's Bigtable, LSM-trees underpin LevelDB, RocksDB, Cassandra, HBase, and Lucene.

## How It Works

- Writes go to an in-memory balanced tree (red-black, AVL, skip list) — the memtable — and to a separate on-disk write-ahead log for crash recovery.
- When the memtable exceeds a threshold (typically a few MB), it is flushed to disk as a new SSTable: a sorted, immutable key-value file. The WAL can then be discarded.
- Reads check the memtable first, then SSTables newest-to-oldest. A Bloom filter on each SSTable lets the engine skip files that definitely do not contain the key.
- A background compaction process merges multiple SSTables into a smaller number, discarding overwritten keys and tombstones. Strategies include size-tiered (Cassandra, HBase) and leveled (LevelDB, RocksDB) compaction.
- Range queries are efficient because data is sorted; sequential writes yield high throughput on both spinning disks and SSDs.

## Key Parameters

- Memtable size threshold.
- Compaction strategy (size-tiered vs leveled) and level multipliers.
- Number of Bloom filter bits per key.
- Write-ahead log sync policy.

## When To Use

For write-heavy workloads where throughput matters more than read tail latency; when storage compactness and SSD endurance are concerns; when range scans are needed but in-place updates are not required; full-text indexes (Lucene) use a similar structure.

## Risks & Pitfalls

- Reads may consult many SSTables; without Bloom filters, missing-key lookups are expensive.
- Compaction competes with foreground I/O; under sustained high write throughput, compaction can fall behind, causing read amplification, disk-space growth, and unbounded queueing of pending segments.
- High-percentile read latency is less predictable than B-trees due to compaction interference.
- Transactional range locking is awkward because the same key can live in multiple segments.

## Related Concepts

- [[concepts/b-tree]]
- [[concepts/sstable]]
- [[concepts/bloom-filter]]
- [[concepts/write-ahead-log]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
