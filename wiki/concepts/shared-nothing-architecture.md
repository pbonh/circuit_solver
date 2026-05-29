---
title: Shared-Nothing Architecture
type: claim
id: claim-shared-nothing-architecture
tags:
- distributed-systems
- scalability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
---

## Definition

In a shared-nothing architecture, each node has its own private memory and storage; coordination happens only through explicit message passing over the network. The default approach in modern NoSQL/NewSQL distributed databases.

## How It Works

Data is partitioned across nodes; each node serves reads and writes for its own partition. Cross-node operations require explicit messaging. There are no shared disks or shared memory pools.

## Key Parameters

- Partitioning strategy.
- Replication factor.

## When To Use

Internet-scale distributed databases, big-data engines (Spark, Hadoop), any system on commodity hardware.

## Risks & Pitfalls

- Cross-partition operations are expensive.
- Rebalancing data when nodes are added or removed is non-trivial.

## Related Concepts

- [[concepts/shared-everything-architecture]]
- [[concepts/sharding]]
- [[concepts/distributed-database]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
