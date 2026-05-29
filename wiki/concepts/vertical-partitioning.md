---
title: Vertical Partitioning
type: claim
id: claim-vertical-partitioning
tags:
- databases
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.65
---

## Definition

Vertical partitioning ("row splitting") divides a table by columns: groups of columns from each row are stored in separate physical partitions. Often used to separate static, read-mostly fields from dynamic, frequently updated fields.

## How It Works

A logical row is split into pieces stored on different disks or tables. Queries that need columns from multiple partitions must join. A common variant separates frequently changing fields from large blobs.

## Key Parameters

- Column-grouping strategy.
- Join cost between partitions.

## When To Use

Tables with mixed access patterns where co-locating all columns wastes I/O.

## Risks & Pitfalls

- Reconstructing a full row requires joins.
- Cross-partition transactions complicate writes.

## Related Concepts

- [[concepts/horizontal-partitioning]]
- [[concepts/sharding]]
- [[concepts/denormalization]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
