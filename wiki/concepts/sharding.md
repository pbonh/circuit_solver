---
title: "Sharding"
type: concept
tags: [databases, scalability, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt"]
confidence: high
---

## Definition

Sharding (a.k.a. partitioning) splits a logical data set across multiple physical nodes so that each node manages a subset. It enables data-tier scale-out by distributing storage and request load.

## How It Works

A shard key is chosen for each item. Sharding strategies map keys to nodes via hashing (modulo or consistent hashing), value-based assignment, or range partitioning. Reads and writes are routed to the owning shard. Replication on top of sharding gives both scalability and availability.

## Key Parameters

- Shard key.
- Sharding strategy (hash, range, value).
- Number of shards.
- Resharding/migration policy.

## When To Use

Datasets exceeding the storage or throughput limits of a single node; geographically partitioned applications.

## Risks & Pitfalls

- Cross-shard joins and transactions are expensive.
- Hotspots if the shard key is skewed.
- Resharding is operationally painful unless planned for.

## Related Concepts

- [[concepts/horizontal-partitioning]]
- [[concepts/vertical-partitioning]]
- [[concepts/replication]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
