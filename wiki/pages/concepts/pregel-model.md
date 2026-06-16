---
title: Pregel Model
type: concept
slug: pregel-model
created: 2026-06-16
updated: 2026-06-16
summary: Google's vertex-centric BSP computation model for distributed graph processing; each vertex runs compute() each superstep, exchanging messages with neighbors.
tags: [graph-analytics, distributed-systems, bsp, pregel, giraph]
sources: [systems-for-big-graph-analytics]
status: active
---

# Pregel Model

Pregel (Google, 2010) is a Bulk Synchronous Parallel (BSP) model for distributed graph computation. Graphs are partitioned across workers; computation proceeds in supersteps separated by global barriers.

## Execution Model

Each superstep:
1. Every active vertex runs `compute(messages)` — receives messages from the previous superstep
2. Vertex sends messages to neighbors (any vertex, not just adjacent)
3. Vertex votes to halt or remains active
4. Global barrier synchronizes all workers
5. Repeat until all vertices halt and message queue is empty

`compute()` can read/write vertex and edge values, send messages, mutate graph structure (add/remove vertices/edges).

## Design Choices

**Combiners**: aggregate messages with an associative/commutative operation before sending — reduces network traffic (analogous to map-reduce combiners). Example: min-combiner for SSSP.

**Aggregators**: global reduction across vertices per superstep — used to compute global statistics (total error, active count, termination condition).

**Partitioning**: hash-by-vertex-id (default, simple, poor locality) vs. edge-cut / vertex-cut (better for power-law graphs). PowerGraph improves on this with vertex-cut for high-degree hubs.

**Communication**: push model (senders decide recipients) default; pull model reduces communication but requires neighbor awareness.

## Algorithm Examples

- **PageRank**: O(constant) supersteps per iteration; inherently BSP-friendly
- **SSSP (Dijkstra/Bellman-Ford)**: O(diameter) supersteps — expensive on sparse real-world graphs
- **Connected components**: label propagation converges in O(diameter)
- **Spanning tree**: modified SSSP

## Limitations

- O(diameter) supersteps for distance/path problems → motivation for block-centric systems
- Power-law degree distributions create load imbalance in vertex-cut partitioning → PowerGraph
- Synchronous barrier is wasteful when few vertices are active late in computation → Maiter/async models

## Related concepts and entities

- [[big-graph-systems]] - Pregel is the foundational paradigm
- [[graph-algorithms]] - algorithms implemented in Pregel
