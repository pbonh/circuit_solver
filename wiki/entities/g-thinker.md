---
title: G-thinker
type: entity
id: entities/g-thinker
tags:
- graph
- distributed-systems
- graph-processing
- graph-mining
- subgraph-centric
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
---

## Overview

G-thinker is a distributed subgraph-centric framework for computation-intensive graph mining, designed by Da Yan and colleagues. It targets problems like maximum clique, maximal clique enumeration, quasi-clique enumeration, dense subgraph finding, and graph matching, where each subgraph requires backtracking-heavy serial computation rather than the data-intensive message-passing typical of vertex-centric or block-centric models.

## Characteristics

- C++ implementation released at `cis.uab.edu/yanda/gthinker`; libraries include `partition` and `subgraph`.
- Per-worker components: local table T_local of assigned vertices, LRU cache T_cache for non-local pulled vertices, in-memory active-task buffer, disk-based task queue Q.
- Programming interface: subclass `Task<I,C,V,E>` (with UDFs `compute(frontier)`, `pull`, `add_task`) and `Worker<Task>` (UDFs `seedTask_gene(vertex)`, `respond(vertex)`, formatting, and `run(config_info)`).
- Optional `Aggregator` template argument with periodic synchronization to share global pruning state (e.g., current best clique size).
- Batch processing: a worker fetches a batch of tasks, sends combined pull requests (one per remote vertex, deduplicated), processes the batch, and pushes incomplete tasks back to disk.
- `respond(v)` UDF prunes Γ(v) before transmission to save bandwidth (e.g., return Γ_>(v) only).

## Common Strategies

- Adapt a serial backtracking algorithm (e.g., Tomita-Seki for maximum clique) directly inside `compute(frontier)`.
- Use seed vertex per task with 1-ego or k-ego neighborhood; recursively decompose oversized G_i for load balance.
- Enable aggregator synchronization at 10s intervals for tight pruning loops.
- Hash-partition vertices for initial deployment; switch to the system's other partitioners as needed.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/blogel]]
- [[entities/arabesque]]
- [[entities/nscale]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
