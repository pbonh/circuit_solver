---
title: Dataflow Engine
type: claim
id: claim-dataflow-engine
tags:
- batch
- well-established
- distributed-systems
- performance
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Designing Data-Intensive Applications The Big Ideas Behind Reliable, Scalable,
  and Maintainable Systems by Martin Kleppmann/_txt/05-part-iii-derived-data.txt
confidence:
  base: 0.85
---

## Definition

A dataflow engine generalizes MapReduce by executing a directed acyclic graph (DAG) of operators with pipelining and in-memory intermediate state. Examples include Apache Spark, Apache Flink, Apache Tez, Naiad, and Google's internal Dremel/F1/Flume. Each operator consumes one or more input streams and produces output streams; the framework partitions data, handles fault tolerance, and schedules work.

## How It Works

- A job is expressed as a DAG of operators (map, filter, reduce, join, group, window) over input datasets.
- Operators pipeline intermediate results in memory rather than materializing every stage to disk (avoiding MapReduce's heavy shuffle cost).
- Fault tolerance via lineage (Spark RDDs recompute lost partitions) or checkpointing (Flink barriers).
- High-level APIs (Spark DataFrame/SQL, Flink Table API, Beam) compile down to DAGs.
- Batch and stream become two sides of the same engine — Spark adds streaming via microbatching; Flink adds batch via bounded streams.

## Key Parameters

- Parallelism / number of executors.
- Memory vs disk spill thresholds.
- Checkpoint interval and storage location.
- Fault-tolerance mode (lineage vs checkpoint).

## When To Use

For batch ETL, ad-hoc analytics, ML pipelines, stream processing, and graph computations on multi-terabyte datasets. The dominant family for modern data-intensive workloads.

## Risks & Pitfalls

- Skewed partitions cause stragglers; tuning required.
- Memory pressure causes spill to disk and dramatic slowdown.
- Long lineages without checkpointing are expensive to recompute on failure.
- Operational complexity (cluster manager, scheduler, executor tuning).

## Related Concepts

- [[concepts/mapreduce]]
- [[concepts/bulk-synchronous-parallel]]
- [[concepts/stream-processing]]
- [[concepts/distributed-filesystem]]
- [[entities/apache-spark]]
- [[entities/apache-flink]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
