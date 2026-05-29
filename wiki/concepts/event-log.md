---
title: Event Log
type: claim
id: concepts/event-log
tags:
- distributed-systems
- streaming
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

An event log is an append-only, immutable, totally ordered sequence of events. Unlike a queue, consumption is non-destructive: readers specify an offset and can replay history. Kafka topics are the canonical implementation.

## How It Works

Producers append events to the tail. Each event gets a sequence number (offset). Consumers track their own offsets and read in order, independently from other consumers. Retention is time- or size-bounded, or compacted (only latest value per key kept).

## Key Parameters

- Retention period.
- Compaction policy.
- Partition count.

## When To Use

Source of truth for state changes, replicated change-data-capture streams, event sourcing, stream-processing pipelines.

## Risks & Pitfalls

- Without compaction, indefinite retention is unsustainable.
- Schema evolution: old events must remain parseable.

## Related Concepts

- [[concepts/event-driven-architecture]]
- [[concepts/log-compaction]]
- [[concepts/topic-partition]]
- [[concepts/kappa-architecture]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
