---
title: "Google Cloud Dataflow / Apache Beam"
type: entity
tags: [well-established, distributed-systems, streaming, batch]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable, and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt"]
confidence: medium
---

## Overview

Google Cloud Dataflow is Google's managed service for batch and stream processing, derived from the internal Flume and MillWheel systems. Apache Beam is the open-source SDK extracted from Dataflow's API: a unified model for writing batch + stream pipelines that can run on multiple "runners" (Dataflow, Flink, Spark, Samza, etc.). The Dataflow Model paper (Akidau et al., 2015) is the canonical reference for handling unbounded, out-of-order data with event-time semantics.

## Characteristics

- Unified API for batch and stream — a batch is a bounded stream.
- Event-time windowing (tumbling, hopping, sliding, session) with watermarks and triggers.
- Exactly-once semantics via micro-coordination and idempotent writes.
- Runner abstraction lets the same pipeline run on multiple backends.
- Managed Dataflow service auto-scales workers based on backlog.

## Common Strategies

- Use Beam SDK to keep pipelines portable across runners.
- Choose session windows for user-activity sessionization, hopping windows for smoothed metrics.
- Use triggers to handle late data with corrections rather than dropping.

## Related Entities

- [[entities/apache-flink]]
- [[entities/apache-spark]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
