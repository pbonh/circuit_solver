---
title: Checkpoint
type: claim
id: concepts/checkpoint
tags:
- streaming
- fault-tolerance
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

A checkpoint is a periodic durable snapshot of a stream-processing job's state. On failure, the system can roll back to the latest checkpoint, restore operator state, and resume processing from the corresponding source position — providing fault tolerance for stateful streaming jobs.

## How It Works

In Flink, the job manager injects barriers; operators snapshot their state to a configured backend (RocksDB by default) when barriers align. Source positions are recorded so on recovery, processing resumes from there.

## Key Parameters

- Checkpoint interval.
- Backend choice (RocksDB, filesystem, S3).
- Minimum pause between checkpoints.

## When To Use

Every long-running stateful streaming job.

## Risks & Pitfalls

- Disabled by default in Flink; common production-readiness oversight.
- Very large state slows checkpointing.

## Related Concepts

- [[concepts/stream-barrier]]
- [[concepts/stateful-stream]]
- [[concepts/stream-processing]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
