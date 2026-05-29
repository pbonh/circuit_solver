---
title: Stream Processing
type: claim
id: claim-stream-processing
tags:
- streaming
- distributed-systems
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
---

## Definition

Stream processing is a paradigm in which data is processed event-by-event (or in small microbatches) as it arrives, producing low-latency results without persisting to a database first. Examples: fraud detection, real-time route planning, trending-topic identification.

## How It Works

A stream-processing platform (Flink, Storm, Kafka Streams, Spark Streaming) executes a DAG of operators that ingest from sources (Kafka topics, files, S3) and write to sinks. Operators may be stateless (transform each event) or stateful (maintain windowed aggregates, joins, model parameters).

## Key Parameters

- Window size and slide.
- State backend (RocksDB, in-memory).
- Checkpoint interval.
- Parallelism.

## When To Use

Real-time analytics, fraud detection, monitoring, event-driven workflows.

## Risks & Pitfalls

- Late or out-of-order events complicate window semantics.
- Recovery from failure requires checkpointing.

## Related Concepts

- [[concepts/batch-processing]]
- [[concepts/dataflow]]
- [[concepts/lambda-architecture]]
- [[concepts/kappa-architecture]]
- [[concepts/checkpoint]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
