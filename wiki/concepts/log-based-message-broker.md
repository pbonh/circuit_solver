---
title: "Log-Based Message Broker"
type: concept
tags: [well-established, streaming, distributed-systems, messaging]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt"]
confidence: high
---

## Definition

A log-based message broker stores messages in partitioned append-only logs persisted to disk, combining the durability of a database with the low-latency notification of a message queue. Consumers read sequentially by offset; multiple consumer groups can independently read the same log. Examples: Apache Kafka, Amazon Kinesis Streams, Twitter DistributedLog, Apache Pulsar.

## How It Works

- Topics are split into ordered partitions; each partition is an append-only log on disk, replicated across brokers.
- Producers append messages; the broker assigns each message a monotonically increasing offset within the partition.
- Consumers checkpoint their progress by recording offsets. Multiple consumer groups can read at independent positions.
- Old messages can be retained for days or weeks (a hard drive can buffer ~11 hours at 150 MB/s of writes); slow consumers can fall behind until offsets fall outside retention.
- **Log compaction** retains the latest value per key indefinitely, supporting use as durable state.
- Fan-out is trivial; load balancing is via partition assignment within a consumer group.

## Key Parameters

- Partition count (sets the maximum consumer-group parallelism).
- Retention policy (time-based, size-based, compacted).
- Replication factor per partition.
- Acknowledgment level for producer writes.

## When To Use

As the backbone of event-driven architectures, change-data-capture pipelines, stream processing, and inter-service event buses. Especially good when replay, fan-out to many consumers, and ordering matter.

## Risks & Pitfalls

- Per-partition ordering only — cross-partition events have undefined order.
- Slow consumer can drop behind retention and miss messages.
- Single-partition throughput limit forces partition-key planning.
- Operational overhead (broker management, replication, partition rebalancing).

## Related Concepts

- [[concepts/message-broker]]
- [[concepts/change-data-capture]]
- [[concepts/event-sourcing]]
- [[concepts/total-order-broadcast]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
