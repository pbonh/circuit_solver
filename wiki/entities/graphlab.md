---
title: "GraphLab"
type: entity
tags: [graph, distributed-systems, big-data, graph-processing, shared-memory, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Overview

GraphLab pioneered the vertex-centric shared-memory abstraction for graph analytics, with a strong emphasis on machine-learning applications. Originally a single-machine system (Low et al., UAI 2010), it was extended to Distributed GraphLab (Low et al., PVLDB 2012), and later evolved into PowerGraph. The team spun out the company Dato (later Turi), which was acquired by Apple.

## Characteristics

- Vertex-centric API where each vertex's UDF `update()` directly reads and writes full-scope data (in/out neighbors, in/out edge values, own value).
- Asynchronous execution by default, with synchronous mode also available; asymmetric convergence (e.g., PageRank with threshold) is supported via per-vertex scheduling.
- Ghost replication of overlapping scope data across machines, kept consistent via locking.
- Hash-based vertex partitioning (over-partitioned into "atoms") leads to high replication factors on real graphs and limits scalability; reported test graphs only ~200M edges in original work.

## Common Strategies

- Use for ML-style algorithms (collaborative filtering, belief propagation) where asymmetric vertex convergence pays off.
- Prefer synchronous mode when the algorithm does not benefit from asymmetry (Hash-Min) — it is often faster than async.
- Reserve generous memory budget per machine to hold ghost replicas.

## Related Entities

- [[entities/powergraph]]
- [[entities/maiter]]
- [[entities/pregel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
