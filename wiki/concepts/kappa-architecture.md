---
title: Kappa Architecture
type: claim
id: concepts/kappa-architecture
tags:
- streaming
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.85
  source_count: 1
  contradicted: false
  effective: 0.85
  inputs_hash: 87cc4b0a8c906cba
---

## Definition

The Kappa architecture is a simplification of the Lambda architecture that uses only stream processing. All events are stored in an immutable, durable event log (e.g., Kafka), and a single stream-processing pipeline produces the current views. Reprocessing replays the log from the beginning.

## How It Works

Producers write events to a Kafka topic. Stream-processing jobs consume from the topic and maintain materialized views in a serving store. To handle a schema or logic change, you start a new processing job that consumes the log from offset 0 and writes to a new view.

## Key Parameters

- Log retention period.
- Stream-processing engine.

## When To Use

When event logs can be retained long enough to enable full replay and a single processing model suffices for both historical and current use cases.

## Risks & Pitfalls

- Very long-retention logs are expensive.
- Replays for very large logs take a long time.

## Related Concepts

- [[concepts/lambda-architecture]]
- [[concepts/event-log]]
- [[concepts/stream-processing]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
