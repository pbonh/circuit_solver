---
title: Apache Samza
type: entity
id: entity-samza
tags:
- well-established
- distributed-systems
- streaming
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
---

## Overview

Apache Samza is a distributed stream-processing framework built at LinkedIn, designed tightly around Apache Kafka for transport and state. Each Samza job consumes from Kafka topics, processes records, and produces to Kafka topics; state is replicated via dedicated Kafka topics with log compaction.

## Characteristics

- One stream task per Kafka partition; per-task local state (RocksDB) replicated to a compacted Kafka topic for fault tolerance.
- At-least-once processing; deduplication and idempotence handled at application level.
- Tightly integrated with YARN for cluster scheduling (now also supports standalone mode).
- High-throughput, low-overhead — Samza was the engine behind LinkedIn's newsfeed and many internal pipelines.

## Common Strategies

- Use compacted Kafka topics for materializing state that other consumers may also need.
- Co-partition input topics so joins are local.

## Related Entities

- [[entities/apache-kafka]]
- [[entities/apache-flink]]
- [[entities/apache-storm]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
