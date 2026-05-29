---
title: Horizontal Partitioning
type: claim
id: claim-horizontal-partitioning
tags:
- databases
- scalability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.65
---

## Definition

Horizontal partitioning splits a table by rows: each partition contains a subset of the table's rows, assigned by a partition key (hash, range, or value). It is the basis of sharding in most distributed databases.

## How It Works

The database engine routes inserts and updates to the partition that owns the row's partition-key value. Queries either target a single partition or fan out and merge results.

## Key Parameters

- Partition key.
- Partition count.
- Routing strategy.

## When To Use

Very large tables that exceed single-node capacity or have natural partitioning along an attribute (region, tenant, time).

## Risks & Pitfalls

- Cross-partition queries (joins, scans) are slow.
- Hotspots if the key has skewed distribution.

## Related Concepts

- [[concepts/sharding]]
- [[concepts/vertical-partitioning]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
