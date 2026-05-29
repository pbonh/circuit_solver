---
title: Block-Centric Computation
type: claim
id: concepts/block-centric-computation
tags:
- graph
- distributed-systems
- big-data
- graph-processing
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

Block-centric computation (also called "think like a graph") is an extension of vertex-centric programming in which vertices are partitioned into cohesive connected blocks, each block is assigned to a single worker, and the UDF runs on whole blocks: state propagates serially within a block without messages and only crosses block boundaries via inter-block messages.

## How It Works

A preprocessing pass partitions the graph into connected blocks (e.g., URL grouping for web graphs, 2D spatial cells for spatial graphs, or graph Voronoi diagrams for general graphs). Each block becomes a first-class object with its own value, adjacency list of neighboring blocks, and `compute(.)` UDF. Vertex-to-worker mapping is recovered either by recoding vertex IDs (Giraph++) or by storing the worker ID inside the vertex ID itself (Blogel). At runtime the user can choose V-mode (pure vertex-centric over the partitioned graph), B-mode (only block-level computation), or VB-mode (vertices first, then blocks per superstep). DAIC-style accumulation can be implemented inside `block.compute(.)` for asynchronous in-block propagation.

## Key Parameters

- Block size and number of blocks per worker (overpartitioning improves balance).
- Cohesion metric (in-block edges vs. crossing edges).
- Partitioner choice: URL, 2D-spatial, GVD, or user-provided block IDs.
- Execution mode (V, B, or VB).

## When To Use

- Large-diameter graphs (road networks, terrain meshes, web crawls) where pure vertex-centric runs for thousands of supersteps.
- Algorithms where in-block serial propagation is cheap (single-source shortest paths via Dijkstra inside a block).
- Block-centric DAIC for prioritized asynchronous convergence with exact results.

## Risks & Pitfalls

- Graph partitioning itself is expensive on big graphs; Blogel's GVD partitioner scales near-linearly and is the only practical option in many cases.
- Stop conditions for VB-mode are more subtle than for pure vertex-centric.
- Recoding vertex IDs to encode worker/block IDs forces a one-time data transformation.
- Inappropriate partitioning destroys cohesion and erases benefits.

## Related Concepts

- [[concepts/vertex-centric-programming]]
- [[concepts/graph-partitioning]]
- [[concepts/graph-voronoi-diagram]]
- [[concepts/think-like-a-graph]]
- [[concepts/delta-accumulative-iterative-computation]]

## Sources

- [[summaries/systems-big-graph-analytics-01-1-introduction]]
- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
