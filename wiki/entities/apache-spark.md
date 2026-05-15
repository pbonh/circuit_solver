---
title: "Apache Spark"
type: entity
tags: [distributed-systems, big-data, in-memory, dataflow, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/04-part-iii-think-like-a-matrix.txt"]
confidence: high
---

## Overview

Apache Spark (Zaharia et al., NSDI 2012) is a general-purpose distributed dataflow engine built around Resilient Distributed Datasets (RDDs). It provides in-memory caching across iterations, fine-grained recovery via lineage, and a unified API (Scala/Java/Python/R) spanning SQL (Spark SQL), streaming, ML (MLlib), and graphs (GraphX). SystemML can target Spark as a backend; GraphX provides Pregel-like semantics on top of Spark.

## Characteristics

- Lazy RDD/DataFrame transformations and explicit actions trigger DAG scheduling.
- Caching reduces I/O for iterative workloads (matrix multiplications, ML training, vertex-centric BFS).
- Narrow-dependency operations (filter, map) achieve fault tolerance via partition recomputation; wide-dependency operations (groupBy, joins, Pregel messaging) fall back to checkpointing.
- YARN, Kubernetes, Mesos, and standalone cluster managers.

## Common Strategies

- Use Spark as the analytics backend for SystemML when a graph pipeline mixes with broader data processing.
- Prefer dedicated Pregel-like systems for graph-only workloads — GraphX consumes much more memory.
- Use MLContext API to interact with SystemML matrices from Spark Shell/Notebooks.

## Related Entities

- [[entities/mapreduce]]
- [[entities/hdfs]]
- [[entities/graphx]]
- [[entities/systemml]]

## Sources

- [[summaries/ddia-05-part-iii-derived-data]]
- [[summaries/systems-big-graph-analytics-04-part-iii-think-like-a-matrix]]
