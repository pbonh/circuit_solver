---
title: B-Tree
type: claim
id: claim-b-tree
tags:
- storage
- foundational
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

A B-tree is a self-balancing on-disk tree data structure that keeps key-value pairs sorted and supports efficient point lookups, range scans, and updates. Introduced in 1970 by Bayer and McCreight and described as "ubiquitous" by 1979, it remains the standard index implementation in essentially all relational databases and many non-relational ones.

## How It Works

- The database is divided into fixed-size pages (commonly 4 KB), each holding sorted keys and either inline values or references to child pages.
- A root page points to child pages whose keys are bounded between the keys in the root; tree depth grows logarithmically with key count. Branching factor is typically several hundred, so most trees are 3–4 levels deep.
- Lookups traverse from root to leaf, following the page references for the relevant key range.
- Inserts/updates may require splitting an overfull page (writing two children plus updating the parent). To survive crashes, a B-tree uses a write-ahead log (WAL/redo log).
- Concurrency requires latches (lightweight locks) protecting tree pages while threads navigate or modify.
- Optimizations: copy-on-write (LMDB), key abbreviation in interior pages, leaf-to-leaf sibling pointers, sequential-layout hints to reduce seeks, fractal trees borrowing log-structured ideas.

## Key Parameters

- Page size (commonly 4–16 KB).
- Branching factor (depends on key size).
- WAL durability mode (sync vs async).
- Fill factor (target page utilization).

## When To Use

When predictable, low-variance read latency is important; when range scans and ordered traversal are common; when transactional isolation via range locks is needed. The default choice for relational OLTP storage.

## Risks & Pitfalls

- In-place page updates risk corruption on crash without a WAL; multi-page operations (splits) are especially dangerous.
- Write amplification: each modification writes the page twice (once to WAL, once to tree), plus possible splits.
- Fragmentation leaves unused space in pages, increasing on-disk footprint.
- Concurrency control via latches is more complex than the lock-free background merging of LSM-trees.

## Related Concepts

- [[concepts/lsm-tree]]
- [[concepts/sstable]]
- [[concepts/write-ahead-log]]
- [[concepts/secondary-index]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
