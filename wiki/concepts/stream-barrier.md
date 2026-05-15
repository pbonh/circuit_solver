---
title: "Stream Barrier"
type: concept
tags: [streaming, advanced]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: medium
---

## Definition

Stream barriers are special control events injected by Apache Flink's job manager into the source stream to coordinate consistent distributed checkpoints across stateful operators. They flow strictly in order with regular events and trigger state snapshots.

## How It Works

Periodically, the job manager injects a barrier with a checkpoint identifier into each source partition. The barrier propagates through the DAG. When a stateful operator has received the barrier on all input streams, it snapshots its local state to durable storage and forwards the barrier to its outputs. Once the barrier reaches all sinks, the checkpoint is complete.

## Key Parameters

- Checkpoint interval.
- Barrier alignment timeout.

## When To Use

Built-in to Flink and similar dataflow engines that provide exactly-once stateful semantics.

## Risks & Pitfalls

- Long barrier alignment delays under skewed input rates.
- Frequent checkpoints amplify state-store I/O.

## Related Concepts

- [[concepts/checkpoint]]
- [[concepts/stream-processing]]
- [[concepts/stateful-stream]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
