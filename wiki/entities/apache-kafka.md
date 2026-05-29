---
title: Apache Kafka
type: entity
id: entities/apache-kafka
tags:
- well-established
- streaming
- distributed-systems
- message-broker
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/03-part-i-foundations-of-data-systems.txt
---

## Overview

Apache Kafka is a distributed log-structured message broker originally developed at LinkedIn. It combines the durability properties of a database with the low-latency delivery of a message queue, making it a foundational component in modern stream-processing and event-driven architectures.

## Characteristics

- Topics are partitioned, replicated, append-only logs persisted to disk.
- Consumers read from offsets they manage themselves, enabling replay and multiple independent consumer groups.
- Provides at-least-once delivery by default and exactly-once semantics with transactional producers.
- Decouples producers from consumers in space, time, and rate.
- Often paired with schema registries (Confluent) to enforce Avro schema evolution rules.

## Common Strategies

- Use as the central event bus for change-data-capture, log aggregation, and stream-processing pipelines (covered in DDIA Part III).
- Partition by entity key for ordered per-key processing.
- Apply log compaction for keyed topics to retain the latest value per key indefinitely.
- Integrate Kafka Streams or Flink/Spark for stateful processing.

## Related Entities

- [[entities/cassandra]]
- [[entities/apache-avro]]

## Sources

- [[summaries/ddia-03-part-i-foundations-of-data-systems]]
- [[summaries/ddia-04-part-ii-distributed-data]]
- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
