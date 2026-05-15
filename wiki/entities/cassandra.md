---
title: "Apache Cassandra"
type: entity
tags: [well-established, nosql, distributed-systems, lsm-tree, open-source]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt"]
confidence: medium
---

## Overview

Apache Cassandra is a distributed wide-column store inspired by Google's Bigtable and Amazon's Dynamo. It uses an LSM-tree storage engine with SSTables and supports both size-tiered and leveled compaction. DDIA cites it as a canonical example of the LSM family and discusses its "column families" (which, despite the name, store rows together rather than being true column-oriented storage).

## Characteristics

- LSM-tree storage with SSTables per column family.
- Tunable compaction strategy (size-tiered, leveled).
- Peer-to-peer architecture with consistent hashing and tunable consistency (per-operation quorum).
- Schema-on-write at the column-family level, but columns can be sparse and dynamically added.
- Column families inherited from Bigtable; not the same as column-oriented OLAP storage.

## Common Strategies

- Choose partition keys carefully to avoid hotspots.
- Pick a compaction strategy based on write/read mix and disk space budget.
- Use Bloom filters and key/row caches to mitigate read amplification.
- Run repair operations to reconcile replicas after partitions or extended downtime.

## Related Entities

- [[entities/mongodb]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
