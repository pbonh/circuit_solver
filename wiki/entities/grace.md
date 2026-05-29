---
title: GRACE
type: entity
id: entities/grace
tags:
- graph
- graph-processing
- single-machine
- block-centric
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
---

## Overview

Per Systems For Big Graph Analytics Chapter 5 (Part II / Think Like a Graph): "The block-centric model has also been applied in single-machine in-memory graph processing. For example, GRACE [4] partitions vertices into blocks by METIS, so that each block fits in the CPU cache. All vertices in a block are processed together (possibly until convergence) without cache miss, before processing another block. This block-centric solution improves cache locality and mitigates the problem of limited memory bandwidth. Unlike Giraph++ and Blogel, GRACE only requires a user to specify the vertex-centric computation logic, and the block-centric computation is treated as a proper scheduling of vertex-centric computation inside each block."

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
