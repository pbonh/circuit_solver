---
title: Microbatching
type: claim
id: concepts/microbatching
tags:
- streaming
- well-established
- fault-tolerance
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

Microbatching is a stream-processing technique that breaks an input stream into small fixed-duration batches (typically around one second) and processes each as a miniature batch job. Pioneered by Spark Streaming, it inherits the fault-tolerance and exactly-once semantics of batch processing while reducing latency far below traditional batch.

## How It Works

- Incoming events accumulate in a buffer for the batch interval (e.g., 1 second).
- At the end of the interval, the accumulated batch is processed as a normal job; failed tasks restart from input data still buffered upstream.
- State carried across batches is checkpointed to durable storage.
- Provides a tumbling window by processing time equal to the batch size; larger windows require explicit state.

## Key Parameters

- Batch interval (latency vs scheduling overhead trade-off).
- Checkpoint frequency.
- Buffer size in the source broker.

## When To Use

When you need stream-like latency (~1 second) but want the simple fault-tolerance model of batch jobs, or when you already run a batch processing engine (Spark) and want to reuse it for streams.

## Risks & Pitfalls

- True low-latency (sub-100 ms) requires record-at-a-time engines like Flink.
- Hopping/sliding event-time windows that don't align with the batch interval are awkward.
- A late-arriving event in the previous batch requires either being dropped or moving to the current batch.

## Related Concepts

- [[concepts/stream-processing]]
- [[concepts/dataflow-engine]]
- [[concepts/exactly-once-semantics]]
- [[entities/apache-spark]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
