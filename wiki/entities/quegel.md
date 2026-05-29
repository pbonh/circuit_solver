---
title: Quegel
type: entity
id: entities/quegel
tags:
- graph
- distributed-systems
- graph-processing
- query-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt
---

## Overview

Quegel (Yan et al., PVLDB 2016; Zhang et al., SIGMOD 2016) is a query-centric distributed graph-processing framework in the BigGraph@CUHK toolkit. Users write vertex-centric UDFs for a generic query; Quegel batches many online queries via its superstep-sharing execution model, advancing each active query by one superstep per super-round.

## Characteristics

- Three data classes per machine: V-data (per-vertex topology), VQ-data (per-vertex per-query state, lazily allocated), Q-data (per-query state replicated on every machine).
- A query taking n supersteps finishes in n+1 super-rounds (the last super-round prints/dumps results).
- New queries are appended to the master's queue and started at super-round boundaries.
- Provides API for users to build per-machine distributed indexes (e.g., inverted label index for graph matching) before query processing begins.
- Same C++/MPI/libhdfs base as Pregel+, Blogel, and GraphD.

## Common Strategies

- Use for online graph-query workloads where each query touches a small fraction of vertices (shortest path, k-hop reachability, graph matching).
- Build label/keyword indexes once after graph load to expedite all subsequent queries.
- Tune VQ-data lazy allocation to bound memory under heavy concurrent query load.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/pregel-plus]]
- [[entities/pregel]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
