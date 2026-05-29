---
title: SSTable (Sorted String Table)
type: claim
id: claim-sstable
tags:
- storage
- well-established
- indexing
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
confidence:
  base: 0.85
---

## Definition

An SSTable is an on-disk file holding key-value pairs sorted by key, with each key appearing at most once. Introduced by Google's Bigtable, SSTables are the persistent storage primitive used by LSM-tree engines (LevelDB, RocksDB, Cassandra, HBase) and by Lucene for its term dictionary.

## How It Works

- Pairs are written in sorted order, so merging two SSTables is a streaming merge-sort: read the head of each, emit the smaller key, advance.
- A sparse in-memory index points at one key every few kilobytes; the rest is found by sequential scan within the block.
- Blocks can be compressed independently, saving disk space and I/O bandwidth.
- SSTables are immutable once written; new data goes to a new file, deletes are recorded as tombstones.
- A background compaction process merges adjacent SSTables and discards old or deleted entries.

## Key Parameters

- Block size (controls compression efficiency vs random-access granularity).
- Sparse index density (keys per kilobyte).
- Compression codec (Snappy, LZ4, Zstd).
- Bloom filter bits per key.

## When To Use

As the on-disk storage primitive for any log-structured engine. Also used by inverted-index systems (Lucene) for sorted term dictionaries.

## Risks & Pitfalls

- Variable-length keys/values mean the sparse index is required for efficient seeks.
- Compaction must be tuned to avoid runaway file counts.
- Reads may need to consult many SSTables; combine with Bloom filters.

## Related Concepts

- [[concepts/lsm-tree]]
- [[concepts/b-tree]]
- [[concepts/bloom-filter]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
