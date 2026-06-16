---
title: Big Graph Systems
type: concept
slug: big-graph-systems
created: 2026-06-16
updated: 2026-06-16
summary: Distributed and disk-based computation frameworks for processing graphs with billions of vertices and edges, organized around vertex-centric, block-centric, and matrix-based paradigms.
tags: [graph-analytics, distributed-systems, pregel, graphlab, sparse-linear-algebra]
sources: [systems-for-big-graph-analytics]
status: active
---

# Big Graph Systems

Frameworks for processing graphs too large to fit in the memory of a single machine (or to solve in reasonable time with sequential algorithms). Three dominant paradigms: vertex-centric (BSP), block-centric, and matrix-based.

## Computation Paradigms

| Paradigm | Model | Systems | Circuit Relevance |
|---|---|---|---|
| Vertex-centric | Each vertex executes compute(); BSP supersteps | Pregel, Giraph, GraphLab, PowerGraph | Iterative solvers per node |
| Block-centric | Partition into subgraphs; local sequential solve | Blogel | Hierarchical circuit partitioning |
| Subgraph-centric | Enumerate/mine subgraph patterns | G-Thinker | Motif finding in netlists |
| Matrix-based | Sparse matrix-vector multiply (GIM-V) | PEGASUS, SystemML | Spectral, Laplacian solvers |

## Pregel / BSP Model

Bulk Synchronous Parallel: superstep = all vertices compute → synchronize → exchange messages → repeat. Simple programming model; inherently handles distributed graph state. Key limitation: requires O(graph diameter) supersteps for algorithms like SSSP. Combiners aggregate messages to reduce communication volume.

**PowerGraph** addresses power-law degree distributions (few very high-degree vertices) by vertex-cut partitioning — splitting high-degree vertices across machines. VLSI netlists often have power-law-like degree distributions (clock nets, power rails).

## Asynchronous Models

GraphLab's GAS (gather-apply-scatter) and Maiter's delta-based asynchronous updates converge faster than BSP for iterative algorithms like PageRank and belief propagation. Delta-based updates: only propagate changes above a threshold, avoiding full sweeps.

## Single-Machine Disk-Based

GraphChi (parallel sliding windows), X-Stream (edge-centric streaming), GridGraph (2D grid partitioning): process billion-edge graphs on a laptop with SSD. Critical for VLSI: design databases are large but rarely require full cluster infrastructure.

## Matrix View of Graphs

Adjacency matrix A, degree matrix D, Laplacian L = D - A. Many graph algorithms are sparse matrix-vector multiplications:
- PageRank: x_{k+1} = αA^T D^{-1} x_k + (1-α)/n
- Effective resistance (VLSI power grid): Rv = L^+ e_uv (requires Laplacian pseudoinverse)
- Spectral partitioning: eigenvectors of L

PEGASUS implements generalized GIM-V on MapReduce; applicable to circuit Laplacian computations.

## Connection to Circuit Simulation

- [[graphs-in-vlsi]] (from another source): VLSI circuits are graphs — nodes, edges, Laplacian resistance computations
- Large netlists are big graphs; scalable simulation requires distributed or streaming computation
- The Laplacian of a resistive circuit IS the graph Laplacian — effective resistance = Laplacian pseudoinverse
- Hierarchical partitioning (block-centric) mirrors circuit partitioning for parallel SPICE

## Related concepts and entities

- [[pregel-model]] - vertex-centric BSP computation model
- [[graph-algorithms]] - algorithms that run on these systems
- [[circuit-simulation]] - application domain for big graph computation
