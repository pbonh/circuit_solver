---
title: "Dataflow"
type: concept
tags: [streaming, distributed-systems, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt"]
confidence: medium
---

## Definition

Dataflow is a programming model in which a computation is expressed as a directed acyclic graph (DAG) of operators that transform data as it flows from sources to sinks. Stream-processing engines like Flink, Storm, Spark, and Beam all use this model.

## How It Works

The application code defines operators (map, filter, reduce, window, join) connected by streams. The engine compiles the logical DAG into a physical execution plan, assigning operator instances to compute resources and routing data between them. Parallelism is configured per operator.

## Key Parameters

- Operator parallelism.
- Partitioning strategy (key, broadcast, rebalance).
- Backend (memory, RocksDB) for stateful operators.

## When To Use

Real-time and batch analytics pipelines, ETL, ML feature computation.

## Risks & Pitfalls

- Skewed partitioning produces hotspots.
- State-heavy operators require careful checkpointing.

## Related Concepts

- [[concepts/stream-processing]]
- [[concepts/checkpoint]]
- [[concepts/stream-barrier]]

## Sources

- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
