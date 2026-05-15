---
title: "GRACE"
type: entity
tags: [graph, graph-processing, single-machine, block-centric, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: low
---

## Overview

GRACE (Xie et al., PVLDB 2013) is a single-machine in-memory block-centric graph processing system. Vertices are partitioned into blocks via METIS so that each block fits in the CPU last-level cache; all vertices in a block are processed together (often to convergence) before moving on to the next block, exploiting cache locality and limited memory bandwidth.

## Characteristics

- Block sizing tuned to CPU cache (vs. Blogel's block sizing for in-memory worker capacity).
- User specifies only the vertex-centric computation logic; block-centric scheduling is handled by the framework.
- Targets the memory-bandwidth wall on a single multi-core machine.

## Common Strategies

- Use to extract additional throughput from a single-node Pregel-style program when memory bandwidth is the bottleneck.
- Combine with selective scheduling so converged blocks are not revisited.

## Related Entities

- [[entities/giraph-plus-plus]]
- [[entities/blogel]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
