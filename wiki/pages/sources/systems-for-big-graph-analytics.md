---
title: "Systems for Big Graph Analytics"
type: source
slug: systems-for-big-graph-analytics
created: 2026-06-16
updated: 2026-06-16
summary: Yan et al. survey of big graph processing systems organized around vertex-centric (Pregel), block-centric (Blogel), subgraph-centric (G-Thinker), and matrix-based (PEGASUS, SystemML) paradigms.
source_file: Books/SystemsForBigGraphAnalytics
tags: [graph-analytics, distributed-systems, pregel, graphlab, sparse-linear-algebra, big-data]
status: active
---

# Systems for Big Graph Analytics

- **Source file:** `sources/Books/SystemsForBigGraphAnalytics/`
- **Author / origin:** Da Yan et al., SpringerBriefs in Computer Science, 2017 (based on SIGMOD 2016 tutorial)
- **Date:** 2017

## Summary

A concise survey and tutorial on computation models for big graph analytics systems — the systems used to process graphs with billions of vertices and edges. Organized into three "think like" paradigms.

### Part I: Think Like a Vertex

**Google Pregel (Ch. 2)**: BSP (Bulk Synchronous Parallel) model. Each superstep: each active vertex executes a user-defined `compute()` function receiving messages from the previous superstep, then sends messages to neighbors. Vertices vote to halt; computation ends when all are halted. Highly scalable; implemented in Apache Giraph, GPS, Mizan, Pregelix.

Key Pregel-like system design dimensions:
- **Communication**: push (sender-driven) vs. pull (receiver-driven); combiner functions to aggregate messages before sending
- **Load balancing**: hash partitioning (easy but poor locality) vs. edge-cut/vertex-cut partitioning
- **Out-of-core**: streaming from disk for graphs that don't fit in RAM (GraphChi uses parallel sliding windows)
- **Fault recovery**: checkpoint + rollback vs. partial fault recovery
- **On-demand querying**: interactive query support on top of Pregel-style computation

**BigGraph@CUHK (Ch. 3)**: Hands-on tutorial on a Pregel implementation. API: Vertex class with `compute()`, aggregators, message combiners. Worker-master architecture; HDFS for graph storage.

**Shared Memory Abstraction (Ch. 4)**: 
- **GraphLab**: gather-apply-scatter (GAS) model; asynchronous execution; consistency constraints (full, edge, vertex); better for iterative ML algorithms
- **PowerGraph**: vertex-cut partitioning (rather than edge-cut); factor graph structure; splits high-degree vertices across machines; handles power-law graphs efficiently
- **Maiter**: asynchronous model with delta-stepping; converges faster than synchronous BSP for many algorithms (PageRank, SSSP)
- **Single-PC disk-based** (GraphChi, X-Stream, VENUS, GridGraph): enables large-graph computation on a single machine with SSD — avoids cluster overhead for moderate-size graphs

### Part II: Think Like a Graph

**Block-Centric Computation (Ch. 5)** via Blogel: partitions graph into subgraph blocks; each block runs a local sequential algorithm, then communicates with neighboring blocks. Dramatically fewer supersteps than vertex-centric (e.g., SSSP: O(diameter) supersteps vertex-centric vs. O(#blocks) block-centric). Better computation locality; leverages graph structure for pruning.

**Subgraph-Centric Graph Mining (Ch. 6)** via G-Thinker: designed for computationally intensive graph mining (clique finding, dense subgraph discovery, frequent subgraph mining). Tasks = subgraph exploration trees; work-stealing for load balance; avoids redundant computation across workers.

### Part III: Think Like a Matrix

**Matrix-Based Systems (Ch. 7)**: Represent graphs as adjacency/Laplacian matrices; exploit sparse linear algebra.
- **PEGASUS**: MapReduce-based generalized matrix-vector multiplication (GIM-V); used for PageRank, SSSP, connected components. Highly scalable on Hadoop.
- **GBASE**: graph storage and compression in matrix form; compressed matrix-vector multiplication
- **SystemML**: high-level linear algebra DSL compiled to MapReduce/Spark; optimizes sparse/dense matrix operations; used for machine learning on graphs

**Matrix vs. Vertex-Centric**: Matrix methods excel when the algorithm is expressible as (repeated) sparse matrix-vector multiplication — PageRank, spectral methods, random walks. Vertex-centric is more expressive for irregular access patterns (e.g., belief propagation with non-uniform convergence).

## Key takeaways

- Pregel's BSP model is simple but can require O(diameter) supersteps for shortest-path problems — block-centric approaches collapse this
- PowerGraph's vertex-cut is critical for power-law graphs (social networks, VLSI netlists share this skewed degree distribution)
- Matrix-based computation (PEGASUS, SystemML) aligns naturally with spectral graph methods and linear solvers — relevant to circuit simulation Laplacians
- Single-machine disk-based systems (GraphChi) are practical for VLSI-scale netlists (hundreds of millions of edges) without cluster infrastructure
- Asynchronous computation (Maiter, GraphLab) converges faster for many iterative algorithms vs. synchronous BSP

## Pages updated from this source

- [[big-graph-systems]] - concept created
- [[pregel-model]] - concept created
- [[graph-algorithms]] - topic updated/created
- [[overview]] - updated with scalable graph computation
