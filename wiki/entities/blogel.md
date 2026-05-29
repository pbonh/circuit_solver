---
title: Blogel
type: entity
id: entity-blogel
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

Blogel (Yan et al., PVLDB 2014) is the block-centric framework of the BigGraph@CUHK toolkit, written in C++ on top of MPI and HDFS. It extends Pregel-style processing with first-class block objects: each block contains a value, an adjacency list of neighboring blocks, and a `compute(.)` UDF, in addition to its vertices. Blogel processes vertex-centric, block-centric, and mixed VB-mode algorithms, and ships three graph partitioners.

## Characteristics

- Three execution modes: V-mode (pure vertex-centric over blocks), B-mode (only blocks compute and exchange messages), VB-mode (vertices then blocks per superstep).
- Vertex IDs extended to encode block ID and worker ID, eliminating the need for a separate lookup function.
- Three partitioners: URL partitioner (web graphs by host), 2D partitioner (spatial graphs via sample-based rectangles and per-cell BFS), and Graph Voronoi Diagram (GVD) partitioner (general graphs via multi-source BFS from sampled sources, with re-sampling and Hash-Min finalization).
- Block classes templated on BValT (block value), BVertexT (vertex type), and BMsgT (block message type).
- Supports per-vertex and per-block message combiners.

## Common Strategies

- Use B-mode for algorithms like Hash-Min where block-level state suffices.
- Use VB-mode for algorithms like SSSP where blocks run Dijkstra internally and only minimal inter-block traffic is needed.
- Choose the partitioner: URL for web graphs, 2D for spatial, GVD for everything else.
- Read documentation on `BGlobal.h` parameters before launching GVD partitioning on a new graph.

## Related Entities

- [[entities/biggraph-cuhk]]
- [[entities/pregel-plus]]
- [[entities/giraph-plus-plus]]
- [[entities/grace]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
