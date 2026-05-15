---
title: "MapReduce"
type: entity
tags: [distributed-systems, big-data, batch-processing, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt"]
confidence: high
---

## Overview

MapReduce (Dean and Ghemawat, OSDI 2004) is Google's foundational distributed batch-processing model: a job is expressed as a `map(key, value) → [(key', value')]` function followed by a `reduce(key, [value']) → output` function. Apache Hadoop is the open-source implementation. PEGASUS, GBASE, and SystemML (in MR backend mode) all execute graph operations as one or many sequential MR jobs.

## Characteristics

- Each MR job has a shuffle phase that sorts and groups intermediate (key', value') pairs across the cluster.
- Strong fault tolerance via re-execution of failed map/reduce tasks.
- Materializes intermediate output to HDFS between jobs — wasteful for iterative graph algorithms.
- Object reuse pattern reduces per-record allocation cost in JVM.

## Common Strategies

- Use for one-shot ETL and analytics where job latency is dwarfed by data size.
- Avoid as the direct execution engine for iterative graph computation (Pregel was motivated by this weakness).
- Combine multiple LOPs into composite jobs (SystemML's piggybacking) to amortize startup cost.

## Related Entities

- [[entities/hdfs]]
- [[entities/apache-spark]]
- [[entities/pegasus]]
- [[entities/gbase]]
- [[entities/systemml]]
- [[entities/nscale]]

## Sources

- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
