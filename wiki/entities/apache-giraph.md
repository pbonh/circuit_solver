---
title: Apache Giraph
type: entity
id: entity-apache-giraph
tags:
- graph
- distributed-systems
- big-data
- graph-processing
- pregel
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

Apache Giraph is the most popular open-source Pregel-like system. Started as a side project at Yahoo! and substantially improved by Facebook, it is written in Java and integrates with Hadoop (HDFS storage and YARN). Project contributors span Facebook, LinkedIn, Twitter, Pivotal, and HortonWorks. It aims primarily to open-source Pregel's semantics with practical optimizations rather than introducing fundamentally new computation models.

## Characteristics

- Java implementation; runs on JVM with HDFS and YARN integration.
- Vertex-centric BSP model identical to Pregel's at the API level.
- Facebook-driven improvements: multithreading, byte-array serialization of edges and messages (object reuse) to combat GC and per-object footprint, superstep splitting to bound message-buffer memory.
- Out-of-core capabilities configurable via Giraph properties (`giraph.org/ooc.html`).
- Supports `KeyValueTextInputFormat`-style formatting via VertexInputFormat / VertexReader classes (more verbose than BigGraph@CUHK's C++ API).

## Common Strategies

- Combine sender-side messages aggressively; use Facebook's byte-array vertex/message storage to keep large graphs in memory.
- Tune YARN containers to give workers enough heap for the per-vertex object overhead.
- Enable out-of-core support when memory is tight; expect a significant slowdown but graceful overflow.
- Use the V-mode Pregel API exposed by Giraph++ for "think-like-a-graph" extensions.

## Related Entities

- [[entities/pregel]]
- [[entities/graphx]]
- [[entities/giraph-plus-plus]]
- [[entities/hdfs]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
