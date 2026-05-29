---
title: GraphX
type: entity
id: entity-graphx
tags:
- graph
- distributed-systems
- big-data
- graph-processing
- spark
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

GraphX (Gonzalez et al., OSDI 2014) is a vertex-centric graph-processing API built on top of Apache Spark. It lets analysts run graph algorithms within end-to-end Spark pipelines without exporting graph data to a dedicated graph engine, at the cost of running on a general-purpose dataflow runtime rather than a specialized one.

## Characteristics

- Scala implementation on Spark RDDs/DataFrames.
- Vertex-centric BSP-style API similar to Pregel.
- Reuses Spark's fault-tolerance machinery, which falls back to checkpointing for the wide-dependency message-passing pattern of Pregel.
- Reported to consume an order of magnitude more memory than other Pregel-like systems for the same workload.

## Common Strategies

- Use when the broader pipeline (ETL, ML, analytics) is already in Spark and graph processing is one step among many.
- Plan for higher cluster cost (memory) relative to specialized engines.
- Prefer Giraph or BigGraph@CUHK for graph-only workloads when efficiency matters.

## Related Entities

- [[entities/apache-spark]]
- [[entities/apache-giraph]]
- [[entities/pregel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
