---
title: Graph Voronoi Diagram (GVD) Partitioner
type: claim
id: claim-graph-voronoi-diagram
tags:
- graph
- graph-partitioning
- algorithm
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/SystemsForBigGraphAnalytics/_txt/03-part-ii-think-like-a-graph.txt
confidence:
  base: 0.85
---

## Definition

A graph Voronoi diagram (GVD) partitions the vertices of an undirected graph by assigning each vertex to its nearest sampled source under graph-hop distance, producing connected blocks of vertices. Blogel's GVD partitioner uses this to build cohesive blocks for block-centric computation.

## How It Works

1. Sample a small fraction of vertices uniformly at random as "sources" (solid circles).
2. Run multi-source breadth-first traversal in vertex-centric mode: each vertex v records the source it heard from first; on first arrival it broadcasts that source ID to its neighbors and votes to halt. Total messages are O(|E|).
3. If some blocks are unbalanced (too many vertices in one), mark those blocks' vertices as unassigned, raise the sampling probability, and re-run multi-source BFS over the unassigned vertices.
4. After iterating until quality is acceptable, run a final Hash-Min pass over still-unassigned vertices so that each remaining connected component becomes its own block.

For directed graphs, the partitioner first converts to undirected (intersecting in/out adjacency lists) before applying GVD.

## Key Parameters

- Initial sampling probability and growth schedule across rounds.
- Block-size quality threshold for triggering re-sampling.
- Maximum number of rounds.

## When To Use

- Partitioning a general graph (no spatial coordinates, no URL host names) for block-centric processing.
- Workloads where partitioning time must be linear in graph size; experiments report GVD partitioning comparable in time to graph loading/dumping.

## Risks & Pitfalls

- Random sampling can miss small connected components, requiring the final Hash-Min step.
- Block sizes vary; achieving strict balance requires multiple re-sampling rounds.
- A directed graph loses some structure when symmetrized for GVD partitioning.

## Related Concepts

- [[concepts/graph-partitioning]]
- [[concepts/block-centric-computation]]

## Sources

- [[summaries/systems-big-graph-analytics-03-part-ii-think-like-a-graph]]
