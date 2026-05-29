---
title: Stateful Stream
type: claim
id: claim-stateful-stream
tags:
- streaming
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
confidence:
  base: 0.65
---

## Definition

A stateful streaming application maintains state that persists across the processing of individual events — windowed aggregates, joins, lookup tables, machine-learning model parameters, deduplication caches. The state lives in operator instances and is checkpointed for fault tolerance.

## How It Works

Operators store state per-key in an in-memory or embedded (RocksDB) backend. On checkpoint barriers, the runtime snapshots state to durable storage. On failure, the operator restores from the latest checkpoint and resumes from the corresponding source offset.

## Key Parameters

- State backend (in-memory, RocksDB).
- Checkpoint interval.
- Maximum state size per key.

## When To Use

Sliding-window aggregates, real-time joins, sessionization, ML inference.

## Risks & Pitfalls

- Large state slows checkpoints.
- Skewed key distribution makes some operators much heavier than others.

## Related Concepts

- [[concepts/stream-processing]]
- [[concepts/stateless-stream]]
- [[concepts/checkpoint]]
- [[concepts/sliding-window]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
