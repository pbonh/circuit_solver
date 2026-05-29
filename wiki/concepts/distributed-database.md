---
title: Distributed Database
type: claim
id: claim-distributed-database
tags:
- distributed-systems
- databases
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/06-part-iii-scalable-distributed-databases.txt
confidence:
  base: 0.85
---

## Definition

A distributed database stores and serves data from multiple cooperating nodes, typically with partitioning (sharding) for scale and replication for availability. It exposes a single logical data store to clients while distributing work across the cluster internally.

## How It Works

The engine partitions tables/collections across nodes by a partition key, replicates each partition for fault tolerance, and routes client queries to the owning node(s). Consensus protocols, version vectors, or last-writer-wins reconcile replicas.

## Key Parameters

- Partition strategy.
- Replication factor and consistency model.
- Cross-partition transaction support.

## When To Use

Whenever single-node databases can no longer meet capacity, throughput, latency, or availability requirements.

## Risks & Pitfalls

- Consistency, joins, and transactions become hard.
- Operational complexity is significantly higher.

## Related Concepts

- [[concepts/sharding]]
- [[concepts/replication]]
- [[concepts/eventual-consistency]]
- [[concepts/strong-consistency]]

## Sources

- [[summaries/foundations-scalable-systems-06-part-iii-scalable-distributed-databases]]
