---
title: Apache Flink
type: entity
id: entities/apache-flink
tags:
- streaming
- dataflow
- open-source
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/Foundations of Scalable Systems/_txt/07-part-iv-event-and-stream-processing.txt
---

## Overview

Apache Flink is a distributed stream-processing engine emerging from the EU Stratosphere project (~2014). It provides high-throughput, low-latency stateful streaming with exactly-once semantics, plus a more functional API than predecessors like Apache Storm.

## Characteristics

- DataStream API (Java/Scala), Table API, and SQL API.
- Compiles user code into a logical DAG and maps it onto Task Manager JVMs.
- Configurable parallelism per operator via `setParallelism()`.
- State backends: in-memory or RocksDB.
- Stream-barrier-based checkpointing for consistent fault tolerance.
- Job Manager coordinates the cluster; HA configurations run multiple Job Managers.

## Common Strategies

- Operator chaining to co-locate operators and minimize communication.
- Tune `taskmanager.numberOfTaskSlots` to match available CPU cores.
- Configure checkpoint intervals to balance throughput vs. recovery time.

## Related Entities

- [[entities/apache-storm]]
- [[entities/apache-kafka]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/foundations-scalable-systems-07-part-iv-event-and-stream-processing]]
