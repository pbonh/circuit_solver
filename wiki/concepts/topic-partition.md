---
title: Topic Partition
type: claim
id: concepts/topic-partition
tags:
- streaming
- scalability
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

In Kafka, a topic partition is an independently ordered log within a topic. Distributing partitions across brokers enables horizontal scaling of throughput; multiple partitions allow parallel production and consumption.

## How It Works

Producers choose partitions either via round-robin (no key) or by hashing the message key (semantic partitioning). Ordering is preserved within each partition but not across partitions. Each partition is owned by a leader broker; replicas are followers.

## Key Parameters

- Partition count (set at topic creation, can only increase).
- Replication factor.
- Partitioner class.

## When To Use

Always: even a single-partition Kafka topic is technically partitioned. More partitions enable more parallel consumers.

## Risks & Pitfalls

- Increasing partition count post-deployment can route the same key to a different partition.
- Too many partitions strain broker metadata.

## Related Concepts

- [[concepts/event-log]]
- [[concepts/consumer-group]]
- [[concepts/in-sync-replica]]
- [[concepts/sharding]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
