---
title: "Systems for Big Graph Analytics — Part II: Think Like a Graph (Block-Centric and Subgraph-Centric)"
type: summary
tags: [graph, distributed-systems, big-data, graph-processing, graph-mining, parallel, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt"]
confidence: high
---

## Key Points

- Vertex-centric models propagate state one hop per superstep, which is prohibitively slow on large-diameter graphs (road networks, terrain meshes, web graphs with spatial locality); experiments show SSSP on the USA road network needing thousands of supersteps in Pregel.
- The block-centric model partitions vertices into cohesive blocks (each block assigned to one worker); inside a block, state propagates serially without messages, cutting both superstep count and network volume dramatically (e.g., SSSP 10,789 → 59 supersteps; 2832s → 11s).
- Two key challenges: (1) graph partitioning into blocks is expensive on big graphs; (2) the vertex-to-worker mapping is no longer a simple hash function — Blogel addresses this by embedding worker IDs in extended vertex IDs.
- Giraph++ pioneered "think like a graph"; Blogel goes further by giving each block its own state, adjacency list, and `compute(.)`, supporting V-mode, B-mode, and VB-mode execution.
- Blogel ships three partitioners: URL partitioner (web graphs by host/domain), 2D partitioner (spatial graphs via sample-based balanced rectangles plus per-cell BFS), and graph Voronoi diagram (GVD) partitioner for general graphs (multi-source BFS from sampled sources, with iterative re-sampling and a final Hash-Min pass).
- DAIC (Maiter's model) can be implemented in block-centric form for efficient asynchronous accumulation inside each block.
- GRACE (single-machine in-memory) applies the same block-centric idea at the CPU-cache granularity, processing each cache-resident block to convergence to reduce cache misses.
- Subgraph-finding problems (cliques, quasi-cliques, triangles, dense subgraphs, graph matching, motif search) decompose computation into many per-seed subgraphs of bounded size, each examined by a serial backtracking algorithm.
- Vertex-centric and block-centric systems are data-intensive and ill-suited for these computation-intensive problems; subgraphs can overlap and their total volume can far exceed the input graph.
- Existing distributed solutions: NScale (k-hop MapReduce neighborhoods packed into reducers, but heavy disk I/O), Arabesque (embedding-centric, requires the entire graph in every machine's RAM, materializes every candidate embedding), and G-thinker (the system this chapter recommends).
- G-thinker design criteria: subgraph-centric API; native computation-intensive workloads; no global synchronization; bounded memory through batched task scheduling; vertex sharing across tasks via a per-worker LRU cache.
- G-thinker components per worker: local table T_local (assigned vertices and adjacency lists), LRU vertex cache T_cache for non-local pulled vertices, in-memory active-task buffer, disk-based task queue Q.
- A G-thinker programmer subclasses `Task<I,C,V,E>` (UDF `compute(frontier)` plus `pull`/`add_task`) and `Worker<Task>` (UDFs `seedTask_gene`, `respond`, plus formatting and `run`). Tasks proceed independently, and an optional aggregator periodically syncs global state (e.g., current best clique size).
- Batch processing hides round-trip latency: a worker pulls a batch of tasks, sends all required vertex requests together, receives responses, then runs `compute` for the whole batch; requests can be deduplicated.

## Relevant Concepts

- [[concepts/block-centric-computation]] — the central model of Chapter 5.
- [[concepts/graph-partitioning]] — required preprocessing step for block-centric systems.
- [[concepts/graph-voronoi-diagram]] — Blogel's general-purpose partitioning via sampled sources and multi-source BFS.
- [[concepts/subgraph-centric-computation]] — Chapter 6's model for graph mining.
- [[concepts/maximal-clique-enumeration]] — the running example for subgraph-finding.
- [[concepts/computation-intensive-vs-data-intensive]] — the categorization that motivates subgraph-centric systems.
- [[concepts/think-like-a-graph]] — Giraph++'s framing of block-centric computation.
- [[concepts/task-batching]] — G-thinker's mechanism for hiding network round-trips and reducing redundant requests.
- [[concepts/lru-vertex-cache]] — G-thinker's per-worker cache for non-local vertices shared by tasks.
- [[entities/blogel]] — the state-of-the-art block-centric system in BigGraph@CUHK.
- [[entities/giraph-plus-plus]] — IBM's pioneering "think-like-a-graph" framework.
- [[entities/grace]] — single-machine block-centric system targeting CPU cache locality.
- [[entities/g-thinker]] — the subgraph-centric framework recommended by the authors.
- [[entities/arabesque]] — embedding-centric distributed graph mining system.
- [[entities/nscale]] — neighborhood-centric MapReduce-based mining system.

## Source Metadata

- Source type: book chapters
- Book title: Systems for Big Graph Analytics
- Chapters: 5 (Block-Centric Computation), 6 (Subgraph-Centric Graph Mining)
- File: raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
- Authors: Da Yan, Yingyi Bu, Yuanyuan Tian, Amol Deshpande (2017, Springer)
