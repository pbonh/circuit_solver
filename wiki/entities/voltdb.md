---
title: "VoltDB"
type: entity
tags: [well-established, distributed-systems, oltp, in-memory, sql]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/04-part-ii-distributed-data.txt"]
confidence: medium
---

## Overview

VoltDB is an in-memory, partitioned, serializable SQL database derived from the H-Store research project led by Michael Stonebraker. It implements serializable isolation through **actual serial execution**: each partition runs a single-threaded transaction loop, and all transactions are submitted as deterministic stored procedures.

## Characteristics

- All data in memory (with command-log persistence for durability).
- One transaction processing thread per partition; partitions distributed across cores and nodes.
- Stored procedures in Java or Groovy, required to be deterministic so the same procedure produces identical results on every replica.
- Cross-partition transactions exist but are far slower (~1000/sec vs partition-local throughput).
- Replication via deterministic stored-procedure re-execution rather than log shipping.

## Common Strategies

- Design schemas around a partition key chosen to keep most transactions partition-local.
- Use stored procedures to keep transactions short and avoid client round-trips.
- Suitable for OLTP workloads with high write volume and well-known query shapes.

## Related Entities

- [[entities/postgresql]]
- [[entities/mongodb]]

## Sources

- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
