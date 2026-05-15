---
title: "Out-of-Core Graph Processing"
type: concept
tags: [graph, distributed-systems, big-data, graph-processing, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/02-part-i-think-like-a-vertex.txt"]
confidence: high
---

## Definition

Out-of-core graph processing refers to systems and algorithms that process graphs whose vertices, edges, and intermediate state (messages, updates) exceed available RAM by streaming this data to and from secondary storage — local disk, SSD, or a distributed file system.

## How It Works

Distributed out-of-core systems (Pregelix, GraphD, Chaos) partition the graph across many machines and stream each machine's portion through a small memory buffer; GraphD specifically hides sequential-disk streaming time inside MPI message-transmission time on Gigabit-Ethernet clusters. Single-PC systems take several approaches: GraphChi uses sorted in-edge shards with parallel sliding windows over out-edges; X-Stream streams unordered edges twice (scatter then gather); VENUS separates static structure from mutable vertex values to maximize value caching; GridGraph stores edges in a 2-D grid of blocks and pins source/destination vertex chunks during column-oriented streaming-apply passes.

## Key Parameters

- Number of partitions/shards/blocks and their sizes (must fit memory budget).
- Streaming-buffer size.
- Whether to support graph mutation (GraphChi yes, VENUS no).
- Skip mechanism for inactive vertices (GraphD streams sparsely; X-Stream always reads all edges).
- Network-to-disk bandwidth ratio (Chaos assumes network >> disk).

## When To Use

- Graphs too large to fit in cluster RAM.
- Academic / small-business clusters with commodity disks.
- Single-PC processing of moderate-size graphs (millions to low billions of edges).

## Risks & Pitfalls

- Streaming all edges every iteration is wasteful when only a few vertices are active (X-Stream's main weakness).
- Disk seek costs dominate when shards/blocks are too small.
- Random-write workloads on hard disks (versus SSD) destroy performance.
- Preprocessing cost (vertex re-numbering, in-edge sorting) can outweigh the speedup for one-shot jobs.

## Related Concepts

- [[concepts/shared-memory-graph-abstraction]]
- [[concepts/vertex-centric-programming]]
- [[concepts/gas-model]]

## Sources

- [[summaries/systems-big-graph-analytics-02-part-i-think-like-a-vertex]]
