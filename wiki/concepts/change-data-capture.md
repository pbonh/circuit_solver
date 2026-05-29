---
title: Change Data Capture (CDC)
type: claim
id: concepts/change-data-capture
tags:
- well-established
- distributed-systems
- streaming
- integration
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Change data capture is the practice of observing all data changes written to a database and exposing them as a stream that other systems can consume. It turns a database into the leader and derived stores (search indexes, caches, warehouses, analytics) into followers, avoiding the race conditions inherent in dual writes.

## How It Works

- The source DB exposes its replication log (PostgreSQL WAL, MySQL binlog, MongoDB oplog) or implements an explicit changes API.
- A CDC pipeline parses the log and emits an event per change to a message broker (typically Kafka).
- An **initial snapshot** captures the database state at a known log offset; subsequent changes are applied incrementally.
- **Log compaction** retains the latest value per key, providing a snapshot-on-demand without needing a full source dump.
- Consumers update their derived views in the order changes were applied at the source.

Implementations: LinkedIn Databus, Yahoo Sherpa, Facebook Wormhole, Bottled Water (PostgreSQL), Maxwell and Debezium (MySQL), Mongoriver (MongoDB), Oracle GoldenGate, Kafka Connect.

## Key Parameters

- Replication-log retention at the source.
- Snapshot trigger and frequency.
- Schema evolution handling.
- Consumer offset tracking.

## When To Use

To keep derived data systems (search, caching, analytics) consistent with a source database; to integrate legacy databases with modern stream-processing pipelines; to build event-driven architectures without modifying application code.

## Risks & Pitfalls

- Asynchronous by default — derived views lag the source.
- Schema changes at the source can break downstream consumers if not coordinated.
- Trigger-based CDC adds source-side overhead; log-based CDC is generally preferred.
- Long-lived consumers need replayable retention.

## Related Concepts

- [[concepts/event-sourcing]]
- [[concepts/log-compaction]]
- [[concepts/log-based-message-broker]]
- [[concepts/derived-data]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
