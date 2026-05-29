---
title: Giraph++
type: entity
id: entity-giraph-plus-plus
tags:
- graph
- distributed-systems
- graph-processing
- block-centric
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
---

## Overview

Giraph++ (Tian et al., PVLDB 2013) is the IBM Research extension of Apache Giraph that pioneered the "think like a graph" (block-centric / graph-centric) programming abstraction. The user writes UDFs operating on a partition (block) rather than on a single vertex, enabling efficient in-partition state propagation without messages.

## Characteristics

- Java implementation extending Giraph.
- Partitions input via METIS-like algorithms; recodes vertex IDs in a MapReduce job so worker-of-vertex is derivable directly from the new ID.
- The "graph" abstraction is what Blogel later termed "block": a partition with strong intra-partition cohesion.
- Lacks Blogel's first-class block-with-state model (no block-level `compute`, no block-level adjacency list to other blocks).

## Common Strategies

- Use the partition-level UDF for shortest-path and traversal algorithms where in-partition propagation dominates.
- Combine with DAIC-style accumulation inside the block for fast asynchronous convergence.

## Related Entities

- [[entities/apache-giraph]]
- [[entities/blogel]]
- [[entities/grace]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
