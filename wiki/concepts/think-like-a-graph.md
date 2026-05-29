---
title: Think Like a Graph
type: claim
id: claim-think-like-a-graph
tags:
- graph
- distributed-systems
- graph-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
confidence:
  base: 0.65
---

## Definition

"Think like a graph" is the framing introduced by IBM's Giraph++ for the block-centric (a.k.a. graph-centric) programming abstraction: instead of writing UDFs that operate on a single vertex, the programmer writes UDFs that operate on a whole partitioned subgraph (block) and may propagate state inside it serially before exchanging messages with other subgraphs.

## How It Works

A graph is pre-partitioned into blocks via a METIS-like algorithm; each block is owned by one worker. The user defines `graphPartition.compute(.)` rather than `vertex.compute(.)`. Inside a block the UDF can iterate over all vertices freely — for example, propagating shortest-path estimates via Dijkstra over the block — without paying network cost. Cross-block updates are still sent as messages, processed at superstep boundaries. Compared to vertex-centric programming, this collapses many "one-hop" supersteps into one block-internal pass.

## Key Parameters

- Block-cohesion quality (depends on partitioner).
- Whether the framework separates `Vertex::compute` from `Block::compute` (Blogel) or merges them (Giraph++).
- Vertex-ID encoding for cheap block/worker lookup.

## When To Use

- When the underlying graph has large diameter or strong locality and a vertex-centric implementation needs many supersteps.
- When block-internal computation is cheap relative to inter-block messages.
- When the user is comfortable writing inside-block algorithms (Dijkstra, BFS, asynchronous accumulation).

## Risks & Pitfalls

- Without high-quality partitioning, block boundaries cut too many edges and the optimization disappears.
- Programmer must reason about both global supersteps and local block iteration.
- Termination conditions become more subtle than vanilla Pregel halting.

## Related Concepts

- [[concepts/block-centric-computation]]
- [[concepts/vertex-centric-programming]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
